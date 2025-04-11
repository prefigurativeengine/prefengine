import time
from typing import OrderedDict
import socket
import json
import os
import sys
import logging

import RNS

APP_NAME = 'prefengine'
ASPECTS = 'main'

# TODO: make engine configuration that specifies this
RET_DATA_PATH = os.path.join(os.path.expanduser('~'), '.prefengine', 'reticulum')

log = logging.getLogger(__name__)
logging.basicConfig(encoding='utf-8', level=logging.DEBUG)


class AnnounceHandler:
    def __init__(self, aspect_filter=None):
        self.aspect_filter = aspect_filter

    def received_announce(self, destination_hash, announced_identity, app_data):
        log.info(
            "Received an announce from "+
            RNS.prettyhexrep(destination_hash)
        )



# TODO: mark private methods, add timeouts, make async
class RNSApi:
    identity: RNS.Identity
    new_peer_dest: RNS.Destination
    reconnect_dest: RNS.Destination

    # TODO: peer_conns only uses the parent_id (RNS.Identity in reticulum terminology) to identify device endpoints; 
    # this will not work when prefengine will have more than one destination that peers can connect to
    peer_conns: dict[str, RNS.Link]

    client_socket: socket.socket
    ret_instance: RNS.Reticulum

    def __init__(self):
        self.ret_instance = RNS.Reticulum(configdir=RET_DATA_PATH)
        
        # TODO: use env variables
        self.identity = RNS.Identity.from_file(os.path.join(RET_DATA_PATH, ".prefengine-secret"))

        # two sides of the same theoretical endpoint
        self.create_new_peer_dest()
        self.create_reconnect_dest()

        self.new_peer_dest.set_link_established_callback(self.handle_remote_new)

        self.peer_conns = {}
        self.client_socket = None

        # announce functionality for learning of peers
        announce_handler = AnnounceHandler()

        RNS.Transport.register_announce_handler(announce_handler)

        log.info("Sending announce on all interfaces")
        self.new_peer_dest.announce()


    def client_listen(self, host='127.0.0.1', port=3502):
        server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server_socket.bind((host, port))
        server_socket.listen(0)

        log.info(f"Server listening on {host}:{port}")
        while True:
            self.client_socket, _ = server_socket.accept()
            log.info("Client connection recieved")
            
            try:
                data = b''
                while True:
                    chunk = self.client_socket.recv(1024)
                    if not chunk:
                        break
                    
                    data += chunk
                    try:
                        # Try to parse what we have so far
                        json_data = json.loads(data.decode('utf-8'))
                        log.info("Client JSON request parsed")

                        # TODO: send info on remote send success or failure
                        self.handle_json(json_data)
                    except json.JSONDecodeError:
                        # Might be incomplete data, continue receiving
                        pass
                        
            except Exception as e:
                log.error(f"Error: {e}", file=sys.stderr)
            

    def handle_json(self, json_req: dict):
        # validate
        if not json_req["action"]:
            log.error("action in JSON not set.")
            return
        
        elif not isinstance(json_req["action"], str):
            log.error("action in JSON not a string.")
            return
        
        # init request and reticulum
        action: str = json_req["action"]
        
        if action == "fo_reconnect":
            if not json_req["id"]:
                log.error("id in JSON not set.")
                return
            
            log.info('Reconnect request recieved and parsed')
            self.fo_reconnect(json_req["id"])
        
        if action == "send":
            if not json_req["change"]:
                log.error("change in JSON not set.")
                return
            
            elif not isinstance(json_req["change"], dict):
                log.error("change in JSON not an object.")
                return
            
            log.info('Data request recieved and parsed')
            self.send_remote_res(json_req["change"])


    def create_reconnect_dest(self):
        if not self.identity:
            return
        
        self.reconnect_dest = RNS.Destination(
            self.identity,
            RNS.Destination.OUT,
            RNS.Destination.SINGLE,
            APP_NAME,
            ASPECTS
        )

        # TODO: test the computational and bandwidth cost of proving all 
        # self.reconnect_dest.set_proof_strategy(RNS.Destination.PROVE_ALL)

        # self.reconnect_dest.enable_ratchets(self.RATCHET_PATH)
        # self.reconnect_dest.enforce_ratchets()


    def create_new_peer_dest(self):
        if not self.identity:
            return
        
        self.new_peer_dest = RNS.Destination(
            self.identity,
            RNS.Destination.IN,
            RNS.Destination.SINGLE,
            APP_NAME,
            ASPECTS
        )

        log.info('New peer dest hash: ' + self.new_peer_dest.hexhash)


        # TODO: test the computational and bandwidth cost of proving all 
        # self.new_peer_dest.set_proof_strategy(RNS.Destination.PROVE_ALL)

        # self.new_peer_dest.enable_ratchets(self.RATCHET_PATH)
        # self.new_peer_dest.enforce_ratchets()
        
    
    # REMOTE FUNCS

    # only handles remote from-off reconnects
    def handle_remote_new(self, link: RNS.Link):
        link.set_link_closed_callback(self.link_closed)

        log.info('New remote link request recieved: ' + self.get_source_dest_from_link(link))
        n_dto = self.convert_to_recieved_conn(link)

        # rust will make sure this destination is actually apart of the group
        resp = self.client_send_from_remote_thread(n_dto.encode(), True)

        # only put in conn dict if a valid peer
        if resp["accepted"] == 0:
            self.peer_conns[n_dto['id']] = link
            
            # TODO: use accept_app
            link.set_resource_strategy(RNS.Link.ACCEPT_ALL)
            link.set_resource_concluded_callback(self.handle_remote_res_fin)

        else:
            log.info('New remote link request rejected')
            link.teardown()


    def handle_remote_res_fin(self, resource: RNS.Resource):
        log.info('New resource request recieved')
        remote_json = self.convert_to_recieved_res(resource)

        self.client_send_from_remote_thread(remote_json.encode())


    def send_remote_res(self, data):
        if len(self.peer_conns) == 0:
            log.error("Send action was called when no peer connections exist")
            return
        
        for id in self.peer_conns.keys():
            res = RNS.Resource(data, self.peer_conns[id])

            # TODO: add msg back to rust client if res was accepted or not
            res.advertise()

    
    def fo_reconnect(self, id):
        rc_id = RNS.Identity.recall(bytes.fromhex(id))

        if not rc_id:
            log.error('Reconnect request recipient not known for <' + id + '>')
            return
        
        rc_dest = RNS.Destination(
            rc_id,
            RNS.Destination.OUT,
            RNS.Destination.SINGLE,
            APP_NAME,
            ASPECTS
        )

        # TODO: somehow make this less intrusive to the devices private key?
        def identify_self(link: RNS.Link):
            log.info('Link has been established to <' + link.destination.hexhash + '>')
            link.identify(self.identity)
            log.info('Identified self to <' + link.destination.hexhash + '>')

        r_link = RNS.Link(rc_dest, identify_self)

        r_link.set_resource_strategy(RNS.Link.ACCEPT_ALL)
        r_link.set_resource_concluded_callback(self.handle_remote_res_fin)
        r_link.set_link_closed_callback(self.link_closed)

        self.peer_conns[id] = r_link


    def link_closed(self, link):
        if link.teardown_reason == RNS.Link.TIMEOUT:
            log.info("The link timed out")
        elif link.teardown_reason == RNS.Link.DESTINATION_CLOSED:
            log.info("The link was closed by the remote peer")
        else:
            log.info("Link closed")


    def client_send_from_remote_thread(self, data, recv_after=False):
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        #s.bind(('127.0.0.1', 0))
        s.connect(('127.0.0.1', 3501))

        s.sendall(data)
        log.info("Sent data to client of char len size: " + str(len(data)))

        if recv_after:
            json_obj = self.recv_then_parse(s)
            return json_obj
        return {}
    

    def recv_then_parse(self, s):
        data = b''
        while True:
            log.info("Listening for data from client")
            chunk = s.recv(1024)
            if not chunk:
                break
            
            data += chunk
            try:
                # Try to parse what we have so far
                return json.loads(data.decode('utf-8'))
            
            except json.JSONDecodeError:
                # Might be incomplete data, continue receiving
                pass
            
            except UnicodeDecodeError:
                pass

        log.error("Failed to decode recieved client data after remote message")
        return {"accepted": 1}
    
    def convert_to_recieved_conn(self, link: RNS.Link):
        remote_json = OrderedDict()
        remote_json['action'] = "new_peer"
        remote_json['id'] = self.get_source_dest_from_link(link)
        # hardcoded to tcp for now
        remote_json['ptp_conn'] = {"physical_type": "tcp"}

        return json.dumps(remote_json)
    
    def get_source_dest_from_link(self, link: RNS.Link):
        if not link.get_remote_identity():
            log.warning("Link remote identity was requested when remote peer wasn't identified, waiting")

        while True:
            id = link.get_remote_identity()
            if not id:
                time.sleep(0.5)
                continue

            hex = RNS.Destination.hash(id, APP_NAME, ASPECTS).hex()
            log.info(hex + "  Peer identified")
            return hex
    
    def convert_to_recieved_res(self, res: RNS.Resource):
        remote_json = OrderedDict()
        remote_json['action'] = "resc_fin"
        r_data = str(res.data.read(), encoding='utf-8')
        remote_json['data'] = r_data

        return json.dumps(remote_json)
        

def start_api():
    api = RNSApi()
    print('Reticulum reverse proxy initialized')

    api.client_listen()

if __name__ == "__main__":
    if len(sys.argv) == 2 and sys.argv[1] == "first_start":
        # turn off stdout for rust to only capture hash
        nullout = open(os.devnull, 'w')
        sys.stdout = nullout

        test_instance = RNS.Reticulum(RET_DATA_PATH)

        identity = RNS.Identity()
        identity.to_file(os.path.join(RET_DATA_PATH, ".prefengine-secret"))

        # sends self peer id to rust
        sys.stdout = sys.__stdout__
        hash = RNS.Destination(
            identity,
            RNS.Destination.IN,
            RNS.Destination.SINGLE,
            APP_NAME,
            ASPECTS
        ).hexhash
        print(hash)

        sys.stdout = nullout
        # restarted by rust later 
        sys.exit(0)
        
    start_api()
