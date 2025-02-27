
# Let's define an app name. We'll use this for all
# destinations we create. Since this basic example
# is part of a range of example utilities, we'll put
# them all within the app namespace "example_utilities"
# APP_NAME = "example_utilities"

# # This initialisation is executed when the program is started
# def program_setup(configpath):
#     # We must first initialise Reticulum
#     reticulum = RNS.Reticulum(configpath)
    
#     # Randomly create a new identity for our example
#     identity = RNS.Identity()

#     # Using the identity we just created, we create a destination.
#     # Destinations are endpoints in Reticulum, that can be addressed
#     # and communicated with. Destinations can also announce their
#     # existence, which will let the network know they are reachable
#     # and automatically create paths to them, from anywhere else
#     # in the network.
#     destination = RNS.Destination(
#         identity,
#         RNS.Destination.IN,
#         RNS.Destination.SINGLE,
#         APP_NAME,
#         "minimalsample"
#     )

#     # We configure the destination to automatically prove all
#     # packets addressed to it. By doing this, RNS will automatically
#     # generate a proof for each incoming packet and transmit it
#     # back to the sender of that packet. This will let anyone that
#     # tries to communicate with the destination know whether their
#     # communication was received correctly.
#     destination.set_proof_strategy(RNS.Destination.PROVE_ALL)
    
#     # Everything's ready!
#     # Let's hand over control to the announce loop
#     announceLoop(destination)


# def announceLoop(destination):
#     # Let the user know that everything is ready
#     RNS.log(
#         "Minimal example "+
#         RNS.prettyhexrep(destination.hash)+
#         " running, hit enter to manually send an announce (Ctrl-C to quit)"
#     )

#     # We enter a loop that runs until the users exits.
#     # If the user hits enter, we will announce our server
#     # destination on the network, which will let clients
#     # know how to create messages directed towards it.
#     while True:
#         entered = input()
#         destination.announce()
#         RNS.log("Sent announce from "+RNS.prettyhexrep(destination.hash))


import RNS
import socket
import json
import os


class RNSApi:
    identity: RNS.Identity
    dest: RNS.Destination

    # prefengine for now, should probably be user-input in future
    APP_NAME: str

    # correlates to capability_type of device for now
    APP_ASPECTS: list[str]

    RATCHET_PATH: str

    def __init__(self, name, aspects):
        APP_NAME = name
        APP_ASPECTS = aspects

    @staticmethod
    def start_server(config_p, host='127.0.0.1', port=3502):
        server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server_socket.bind((host, port))
        server_socket.listen(1)
        print(f"Server listening on {host}:{port}")

        ret = RNS.Reticulum(configdir=config_p)

        while True:
            client_socket, addr = server_socket.accept()
            print(f"Connection received from {addr}")
            
            data = b''
            while True:
                chunk = client_socket.recv(1024)
                if not chunk:
                    break
                data += chunk
            
            try:
                json_data = json.loads(data.decode('utf-8'))
                print("Received JSON:", json.dumps(json_data, indent=2))
                handle_json(json_data)
            except json.JSONDecodeError:
                print("Error: Received invalid JSON data")
            
            client_socket.close()

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
        obj: dict = json_req["obj"]
        
        if action == "identity":
            self.identity = RNS.Identity()

        elif action == "destination":
            if not self.identity:
                print("destination was called, but identity has not been set.")
                return
            
            direct = self.get_direction(obj["direction"]) 
            if direct == 0:
                print("direction in obj not recongnized.")
                return
            
            self.dest = RNS.Destination(
                self.identity,
                direct,
                RNS.Destination.GROUP,
                self.APP_NAME,
                *self.APP_ASPECTS
            )

            # TODO: test the computational and bandwidth cost of proving all 
            self.dest.set_proof_strategy(RNS.Destination.PROVE_ALL)

            # req handler
            self.dest.register_request_handler(
                "/new-peer",
                self.handle_new_peer,
                RNS.Destination.ALLOW_ALL
            )

            # enable ratchets, enforce
            self.dest.enable_ratchets(self.RATCHET_PATH)
            self.dest.enforce_ratchets()

        else:
            print("action in JSON not recongnized.")
            return
    
    def get_direction(json_direction):
        if json_direction == 1:
            return RNS.Destination.IN
        elif json_direction == 2:
            return RNS.Destination.OUT
        else:
            return 0
    
    # REQUEST HANDLERS

    def handle_new_peer(self):
        pass


# consider maintaining as much state as possible in rust, so that only create and updates would be needed here
if __name__ == "__main__":
    # TODO: add platform check for slash here
    config_p = os.getcwd() + "\\" + "retconfig.conf"
    start_server(config_p)
