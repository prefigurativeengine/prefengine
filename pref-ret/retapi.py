import RNS
import socket
import json
import os


# TODO: mark private methods, add timeouts, make async
class RNSApi:
    identity: RNS.Identity
    new_peer_dest: RNS.Destination
    reconnect_dest: RNS.Destination

    # TODO: peer_conns only uses the parent_id (RNS.Identity in reticulum terminology) to identify device endpoints; 
    # this will not work when prefengine will have more than one destination that peers can connect to
    peer_conns: dict[str, RNS.Link]

    client_socket: socket.socket
    APP_NAME: str
    ret_instance: RNS.Reticulum

    def __init__(self, name, config_p, first_start: bool):
        self.APP_NAME = name

        self.ret_instance = RNS.Reticulum(configdir=config_p)
        
        # TODO: use env variables
        if first_start:
            self.identity = RNS.Identity()
            self.identity.to_file('secretid')
        else:
            self.identity = RNS.Identity.from_file('secretid')

        # two sides of the same theoretical endpoint
        self.create_new_peer_dest()
        self.create_reconnect_dest()

        self.new_peer_dest.set_link_established_callback(self.handle_remote_new)


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
                print("Error: Received invalid JSON data", sys.stderr)
            

    def handle_json(self, json_req: dict):
        # validate
        if not json_req["action"]:
            print("action in JSON not set.")
            return
        
        elif not isinstance(json_req["action"], str):
            print("action in JSON not a string.")
            return
        
        if not json_req["id"]:
            print("id in JSON not set.")
            return
        
        # init request and reticulum
        action: str = json_req["action"]
        
        if action == "fo_reconnect"
            self.fo_reconnect(json_req["id"])
        
        if action == "send"
            if not json_req["id"]:
                print("id in JSON not set.")
                return
            
            if not json_req["change"]:
                print("change in JSON not set.")
                return
            
            elif not isinstance(json_req["change"], dict):
                print("change in JSON not an object.")
                return
            
            self.send_remote_res(json_req["id"], json_req["change"])


    def create_reconnect_dest(self):
        if not self.identity:
            return
        
        self.reconnect_dest = RNS.Destination(
            self.identity,
            RNS.Destination.OUT,
            RNS.Destination.SINGLE,
            self.APP_NAME,
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
            self.APP_NAME,
        )

        # TODO: test the computational and bandwidth cost of proving all 
        self.new_peer_dest.set_proof_strategy(RNS.Destination.PROVE_ALL)

        # self.new_peer_dest.enable_ratchets(self.RATCHET_PATH)
        # self.new_peer_dest.enforce_ratchets()
        
    
    # REMOTE FUNCS

    # only handles remote from-off reconnects
    def handle_remote_new(self, link: RNS.Link):
        remote_json = {'action': "new_peer"}
        remote_json['data'] = link

        n_dto = self.convert_to_recieved_conn(link)

        # rust will make sure this destination is actually apart of the group
        resp = self.client_send_from_remote_thread(n_dto, True)

        # only put in conn dict if a valid peer
        if resp["accepted"] == 0:

            # TODO: use remote identified callback here..
            while True:
                get_id_result = link.get_remote_identity()

                if get_id_result == None:
                    continue
                else:
                    # TODO: use accept_app
                    link.set_resource_strategy(RNS.Link.ACCEPT_ALL)
                    link.set_resource_concluded_callback(self.handle_remote_res_fin)

                    # TODO: send another req to rust to save parent id (to also confirm 
                    # the identity is also apart of the group) 
                    pub_key = get_id_result.get_public_key()
                    self.peer_conns[str(pub_key)] = link
                    break

        else:
            link.teardown()


    def handle_remote_res_fin(self, resource: RNS.Resource):
        remote_json = {'action': "resc_fin"}
        
        r_data = str(resource.data.read(), encoding='utf-8')
        remote_json['data'] = r_data

        self.client_send_from_remote_thread(r_data)


    def send_remote_res(self, remote_id, data):
        res = RNS.Resource(data, self.peer_conns[remote_id])

        # TODO: add msg back to rust client if res was accepted or not
        res.advertise()

    
    def fo_reconnect(self, id):
        rc_dest_hash = id["child_id_endpoints"][0]

        # TODO: check for path exists for remote dest
        rc_id = RNS.Identity.recall(rc_dest_hash)
        rc_dest = RNS.Destination(
            rc_id,
            RNS.Destination.OUT,
            RNS.Destination.SINGLE,
            self.APP_NAME,
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
    

    def recv_then_parse(s):
        data = b''
        while True:
            chunk = s.recv(1024)
            if not chunk:
                break
            data += chunk
        return json.loads(data.decode('utf-8'))
    
    def convert_to_recieved_conn(link: RNS.Link):
        rssi = link.get_rssi()
        if not rssi:
            rssi = 0
        
        json_d = {
            "id": {
                "child_id_endpoints": link.destination.hash
            },
            "ptp_conn": {
                "physical_type": "tcp",
            }
        }

        return json.dumps(json_d)




import sys

def start_api(first_start):
    api = RNSApi('prefengine', config_p, first_start)
    api.client_listen()

if __name__ == "__main__":
    # TODO: make this path concat better
    config_p = os.getcwd() + "\\" + "reticulum_config.conf"

    if len(sys.argv) = 1:
        start_api(True)

    elif len(sys.argv) = 0:
        start_api(False)

    else:
        print('too many arguments', file=sys.stderr)
        sys.exit(1)
