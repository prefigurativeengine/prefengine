import RNS
import socket
import json
import os
import threading

# TODO: mark private methods
class RNSApi:
    identity: RNS.Identity
    new_peer_dest: RNS.Destination
    reconnect_dest: RNS.Destination
    peer_conns: dict[str, RNS.Link]
    client_socket: socket.socket
    client_lock: threading.Lock

    # prefengine for now, should probably be user-input in future
    APP_NAME: str

    # correlates to capability_type of device for now
    # APP_ASPECTS: list[str]

    RATCHET_PATH: str

    def __init__(self, name, config_p, ratchet_p, peer_ids: list[str]):
        self.APP_NAME = name
        self.RATCHET_PATH = ratchet_p

        ret = RNS.Reticulum(configdir=config_p)

        self.identity = RNS.Identity()

        self.client_lock = threading.Lock()

        # two sides of the same theoretical endpoint
        self.create_new_peer_dest()
        self.create_reconnect_dest()

        # create links
        self.new_peer_dest.set_link_established_callback(self.handle_remote)

        for id in peer_ids:
            # TODO: check for path exists for remote dest
            id_obj = RNS.Identity.recall(id)
            r_dest = RNS.Destination(
                id_obj,
                RNS.Destination.OUT,
                RNS.Destination.SINGLE,
                self.APP_NAME,
            )
            r_link = RNS.Link(r_dest)
            r_link.set_resource_concluded_callback(self.handle_remote_res_fin)

            self.peer_conns[id] = r_link
        
        self.client_listen()


    def client_listen(self, host='127.0.0.1', port=3502):
        server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server_socket.bind((host, port))
        server_socket.listen(0)
        print(f"Server listening on {host}:{port}")

        while True:
            self.client_socket, addr = server_socket.accept()
            print(f"Connection received from {addr}")
            
            data = b''
            while True:
                chunk = self.client_socket.recv(1024)
                if not chunk:
                    break
                data += chunk
                
            try:
                json_data = json.loads(data.decode('utf-8'))
                print("Received JSON:", json.dumps(json_data, indent=2))

                self.handle_json(json_data)
            except json.JSONDecodeError:
                print("Error: Received invalid JSON data")
            
            self.client_socket.close()


    def handle_json(self, json_req: dict):
        # parse json, init ret then decide which method to run: identity -> dest -> instance/transport -> link api -> resource

        # validate
        if not json_req["action"]:
            print("action in JSON not set.")
            return
        
        elif not isinstance(json_req["action"], str):
            print("action in JSON not a string.")
            return
        
        if not json_req["obj"]:
            print("obj in JSON not set.")
            return
        
        elif not isinstance(json_req["obj"], dict):
            print("obj in JSON not an object.")
            return
        
        # init request and reticulum
        action: str = json_req["action"]
        # obj: dict = json_req["obj"]
        
        if action == "send":
            if not json_req["id"]:
                print("id in JSON not set.")
                return
            self.send_remote(json_req["id"], json_req["obj"])
        
        else:
            print("action in JSON not recongnized.")
            return


    def create_reconnect_dest(self):
        if not self.identity:
            print("destination was called, but identity has not been set.")
            return
        
        self.reconnect_dest = RNS.Destination(
            self.identity,
            RNS.Destination.OUT,
            RNS.Destination.SINGLE,
            self.APP_NAME,
        )

        # TODO: test the computational and bandwidth cost of proving all 
        self.reconnect_dest.set_proof_strategy(RNS.Destination.PROVE_ALL)

        # req handler
        self.reconnect_dest.register_request_handler(
            "/sync-db",
            self.handle_sync_db,
            RNS.Destination.ALLOW_ALL
        )

        # enable ratchets, enforce
        self.reconnect_dest.enable_ratchets(self.RATCHET_PATH)
        self.reconnect_dest.enforce_ratchets()


    def create_new_peer_dest(self):
        if not self.identity:
            print("destination was called, but identity has not been set.")
            return
        
        self.new_peer_dest = RNS.Destination(
            self.identity,
            RNS.Destination.IN,
            RNS.Destination.SINGLE,
            self.APP_NAME,
        )

        # TODO: test the computational and bandwidth cost of proving all 
        self.new_peer_dest.set_proof_strategy(RNS.new_peer_destination.PROVE_ALL)

        # req handler
        self.new_peer_dest.register_request_handler(
            "/new-peer",
            self.handle_new_peer,
            RNS.new_peer_destination.ALLOW_ALL
        )

        # enable ratchets, enforce
        self.new_peer_dest.enable_ratchets(self.RATCHET_PATH)
        self.new_peer_dest.enforce_ratchets()
    

    def get_direction(json_direction):
        if json_direction == 1:
            return RNS.Destination.IN
        elif json_direction == 2:
            return RNS.Destination.OUT
        else:
            return 0
        

    def client_send(self, data):
        self.client_socket.sendall(data)
    
    # REQUEST HANDLERS

    def handle_remote_new(self, link: RNS.Link):
        remote_json = {'action': "new_peer"}
        remote_json['data'] = link

        resp = self.client_send_from_remote_thread(json.dumps(remote_json), True)

        # only put in conn dict if a valid peer
        if resp["accepted"] == 0:
            pub_key = link.get_remote_identity().get_public_key()
            self.peer_conns[str(pub_key)] = link
        

    def handle_remote_res_fin(self, resource):
        remote_json = {'action': "res_fin"}
        remote_json['data'] = resource

        self.client_send_from_remote_thread(json.dumps(remote_json))


    def send_remote(self, remote_id, data):
        res = RNS.Resource(data, self.peer_conns[remote_id])

        # TODO: add msg back to rust client if res was accepted or not
        res.advertise()


    def client_send_from_remote_thread(self, data, recv_after=False):
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.bind(('127.0.0.1', 0))
        s.connect(('127.0.0.1', 3502))

        s.sendall(data)

        if recv_after:
            json_obj = self.recv_then_parse(s)
            return json_obj
        return {}
    

    def recv_then_parse(s):
        data = b''
        while True:
            chunk = s.recv(1024)
            if not chunk:
                break
            data += chunk
        return json.loads(data.decode('utf-8'))


    # how handle link:

    # new_peer - 
    # will listen for link establishments, do extra checks, then put peer in hashmap of links, or acknoledging a 
    # new group member; if new, also starting replication process so new peer can connect to all other peers 

    # reconnect - 
    # will recall identities and dests from disk, then establish links for each peer, putting all links in a hasmap 

    # handles previously off org members (peers) as well as newly added org members (temp_peers)

if __name__ == "__main__":
    # TODO: add platform check for slash here
    config_p = os.getcwd() + "\\" + "retconfig.conf"
    
