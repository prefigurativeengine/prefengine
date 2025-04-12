# Prefigurative Engine

Prefengine is an application designed to provide a unified interface to create business software similar to an ERP or BPM, but for grassroots, activist organizations.  By solving issues present in horizontal organizing with a private peer-to-peer network, the project hopes to make organizing less exhausting and more accessible for everyone involved. In more technical terms, it is engine software that will construct and run an organizational p2p app, made in Rust.

Right now the project has been planned out extensively, however the actual code is very minimal and the few basic features it does have have not been made robust. The roadmap below lays out where the project is currently expected to go in the future.

## Features

Currently the program is capable of basic functions like a startup process (constructing config and starting certain important network systems), and take basic input about peers through the API, but can only do low-level p2p networking through the reticulum library, and not actually construct a connection to send data over.

## Roadmap

The following broad milestones listed represent the kind of app prefengine should be capable of producing. Subject to change.

0. Protoype (Basic implementation of essential functions)

1. Basic Team Collaboration App
 - [ ] Reliable, Partially-synchronous Messaging
 - [ ] Reliable, Partially-synchronous Replication
 - [ ] SQL Data Storage

 - [ ] Basic Team Chat
 - [ ] Basic Team Project-management
 - [ ] Basic Team Decision-making

2. Basic Organization App
 - [ ] Multi-Database Replication 

 - [ ] Basic Member System
 - [ ] Basic Calender
 - [ ] Basic Environment Modeling

Unorganized

 - [ ] Rust implementation of Reticulum

## Compiling from source

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install), [Python 3](https://www.python.org/downloads/), and Reticulum (with ```pip install RNS```) installed. Reticulum in particular is the basic p2p networking tool for prefengine.
2. Clone the repository:
    ```bash
    git clone https://github.com/prefigurativeengine/prefengine.git
    cd your-repo
    ```
3. Build the project:
Linux:
    ```bash
    ./linux_build.sh overwrite_data
    ```

Windows:
    ```bash
    .\\win_build.bat overwrite_data
    ```

In the future, the build script must be run without the ```overwrite_data``` argument for peer and config data to not be overwritten.

4. Run the project:
    ```
    cd target/debug
    cargo run
    ```

If the server is running, the endpoints (listed in webserver.rs) can be accessed at http://localhost:3500 by default.


## Source Tree Map

See the docs repo for more information on design choices.

```
├───api # Localhost server, representing the API layer. Only has a few testing endpoints that serve plaintext as of right now.
│   └───src
├───data # Config and serialized peer data. Used by build system for accessing these files at runtime
├───pref # Main functionalities of technical layer; nat traversal in discovery, other basic functions
│   └───src
│       ├───app # Entry point & high-level funcs
│       ├───core # Logging, etc.
│       └───peer_server # p2p management, database management
├───pref-ret # Python script acting as a reverse proxy for rust to communicate with to enable usage of reticulum for now. includes management of config, connections, addressing, etc.

```

## Contributing

### Asking Questions

If you want to ask a question, do not open an issue.

Instead, ask away on the discussions or on the Discord at https://discord.gg/eaERWJS6hM

### Providing Feedback & Ideas

Likewise, feedback, ideas and feature requests are a very welcome way to contribute (especially from real-life organizers), and should also be posted on the discussions, or on the Discord at https://discord.gg/eaERWJS6hM

Please do not post feature requests or general ideas on the issue tracker. You are much more likely to get a response and start a constructive discussion by posting your ideas in the public channels created for these purposes.

### Reporting Issues

If you have found a bug or issue in this project, please report it using the issue tracker. If at all possible, be sure to include details on how to reproduce the bug.

### Writing Code

If you are interested in contributing significant code to the project, please coordinate the effort with one of the main developers in Discord before submitting a pull request. Before deciding to contribute, it is also a good idea to ensure your efforts are in alignment with the Roadmap and current development focus.

By contributing code to this project, you agree that copyright for the code is placed under the Apache license.

