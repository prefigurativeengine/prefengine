import RNS
import socket
import json
import os
import base64

APP_NAME = 'prefengine'
ASPECTS = 'main'

# TODO: make engine configuration that specifies this
RET_DATA_PATH = os.path.expanduser('~') + '\\' + '.prefengine\\reticulum'

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
        self.identity = RNS.Identity.from_file(RET_DATA_PATH + '\\.prefengine-secret')

        # two sides of the same theoretical endpoint
        self.create_new_peer_dest()
        self.create_reconnect_dest()

        self.new_peer_dest.set_link_established_callback(self.handle_remote_new)

        self.peer_conns = {}
        self.client_socket = None


    def client_listen(self, host='127.0.0.1', port=3502):
        server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server_socket.bind((host, port))
        server_socket.listen(0)

        print(f"Server listening on {host}:{port}")
        while True:
            self.client_socket, _ = server_socket.accept()
            
            data = b''
            while True:
                chunk = self.client_socket.recv(1024)
                if not chunk:
                    break
                data += chunk
                
            try:
                json_data = json.loads(data.decode('utf-8'))

                # TODO: send info on remote send success or failure
                self.handle_json(json_data)
            except json.JSONDecodeError:
                print("Error: Received invalid JSON data", file=sys.stderr)
            

    def handle_json(self, json_req: dict):
        # validate
        if not json_req["action"]:
            print("action in JSON not set.")
            return
        
        elif not isinstance(json_req["action"], str):
            print("action in JSON not a string.")
            return
        
        # init request and reticulum
        action: str = json_req["action"]
        
        if action == "fo_reconnect":
            if not json_req["id"]:
                print("id in JSON not set.")
                return
            
            self.fo_reconnect(json_req["id"])
        
        if action == "send":
            if not json_req["change"]:
                print("change in JSON not set.")
                return
            
            elif not isinstance(json_req["change"], dict):
                print("change in JSON not an object.")
                return
            
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
        self.reconnect_dest.set_proof_strategy(RNS.Destination.PROVE_ALL)

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

        # TODO: test the computational and bandwidth cost of proving all 
        self.new_peer_dest.set_proof_strategy(RNS.Destination.PROVE_ALL)

        # self.new_peer_dest.enable_ratchets(self.RATCHET_PATH)
        # self.new_peer_dest.enforce_ratchets()
        
    
    # REMOTE FUNCS

    # only handles remote from-off reconnects
    def handle_remote_new(self, link: RNS.Link):
        n_dto = self.convert_to_recieved_conn(link)

        # rust will make sure this destination is actually apart of the group
        resp = self.client_send_from_remote_thread(n_dto, True)

        # only put in conn dict if a valid peer
        if resp["accepted"] == 0:
            self.peer_conns[n_dto['id']] = link
            
            # TODO: use accept_app
            link.set_resource_strategy(RNS.Link.ACCEPT_ALL)
            link.set_resource_concluded_callback(self.handle_remote_res_fin)

        else:
            link.teardown()


    def handle_remote_res_fin(self, resource: RNS.Resource):
        remote_json = self.convert_to_recieved_res(resource)

        self.client_send_from_remote_thread(remote_json)


    def send_remote_res(self, data):
        for id in self.peer_conns.keys():
            res = RNS.Resource(data, self.peer_conns[id])

            # TODO: add msg back to rust client if res was accepted or not
            res.advertise()

    
    def fo_reconnect(self, id):
        # TODO: check for path exists for remote dest
        rc_id = RNS.Identity.recall(id)
        rc_dest = RNS.Destination(
            rc_id,
            RNS.Destination.OUT,
            RNS.Destination.SINGLE,
            APP_NAME,
        )

        # TODO: somehow make this less intrusive to the devices private key?
        def identify_self(link):
            link.identify(self.identity)

        r_link = RNS.Link(rc_dest, identify_self)

        r_link.set_resource_strategy(RNS.Link.ACCEPT_ALL)
        r_link.set_resource_concluded_callback(self.handle_remote_res_fin)

        self.peer_conns[id] = r_link


    def client_send_from_remote_thread(self, data, recv_after=False):
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.bind(('127.0.0.1', 0))
        s.connect(('127.0.0.1', 3501))

        s.sendall(data)

        if recv_after:
            json_obj = self.recv_then_parse(s)
            return json_obj
        return {}
    

    def recv_then_parse(self, s):
        data = b''
        while True:
            chunk = s.recv(1024)
            if not chunk:
                break
            data += chunk
        return json.loads(data.decode('utf-8'))
    
    def convert_to_recieved_conn(self, link: RNS.Link):
        remote_json = {'action': "new_peer"}
        remote_json['id'] = str(base64.b64encode(link.destination.hash))
        # hardcoded to tcp for now
        remote_json['ptp_conn'] = {"physical_type": "tcp"}

        return json.dumps(remote_json)
    
    def convert_to_recieved_res(self, res: RNS.Resource):
        remote_json = {'action': "resc_fin"}
        
        r_data = str(res.data.read(), encoding='utf-8')
        remote_json['data'] = r_data

        return remote_json
        

import sys

def start_api():
    api = RNSApi()
    api.client_listen()

if __name__ == "__main__":
    if len(sys.argv) == 2 and sys.argv[1] == "first_start":
        # turn off stdout for rust to only capture hash
        nullout = open(os.devnull, 'w')
        sys.stdout = nullout

        test_instance = RNS.Reticulum(RET_DATA_PATH)

        identity = RNS.Identity()
        identity.to_file(RET_DATA_PATH + '\\.prefengine-secret')

        # sends self peer id to rust
        sys.stdout = sys.__stdout__
        hash = RNS.Destination.hash(identity, APP_NAME, ASPECTS)
        print(hash)

        sys.stdout = nullout
        # restarted by rust later
        sys.exit(0)
        
    start_api()
