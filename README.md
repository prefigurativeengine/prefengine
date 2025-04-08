# Prefigurative Engine

Prefengine is an application designed to provide a unified interface to create business software similar to an ERP or BPM, but for grassroots, activist organizations.  By solving issues present in horizontal organizing, the project hopes to make organizing less exhausting and more accessible for everyone involved. In more technical terms, it is engine software that will construct and run organizational p2p app, made in Rust.

## Roadmap

The following broad milestones listed represent the kind of app prefengine should be capable of producing. Subject to change.

0. Protoype 
abilities of all three layers

1. Basic Team Collaboration App
abilities of all three layers

 - Reliable, Partially-synchronous Messaging
 - Reliable, Partially-synchronous Replication
 - SQL Data Storage

 - Basic Team Chat
 - Basic Team Project-management
 - Basic Team Decision-making

2. Basic Organization App
abilities of all three layers

 - Multi-Database Replication 

 - Basic Member
 - Basic Calender
 - Basic Environment Modeling

## Compiling from source

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install), [Python 3](https://www.python.org/downloads/), and Reticulum (with ```pip install RNS```) installed.
2. Clone the repository:
    ```bash
    git clone https://github.com/your-username/your-repo.git
    cd your-repo
    ```
3. Build the project:
Linux:
    ```bash
    .\\linux_build.bat
    cargo build
    ```

Windows:
    ```bash
    .\\win_build.bat
    cargo build
    ```

4. Run the project:
    ```bash
    cargo run
    ```

If the server is running, the web ui can be accessed at http://localhost:3500 by default.


## Contributing

### Asking Questions

If you want to ask a question, do not open an issue.

Instead, ask away on the discussions or on the Discord at https://discord.gg/Gdk63XHeGdk63XHe

### Providing Feedback & Ideas

Likewise, feedback, ideas and feature requests are a very welcome way to contribute (especially from real-life organizers), and should also be posted on the discussions, or on the Discord at https://discord.gg/Gdk63XHeGdk63XHe

Please do not post feature requests or general ideas on the issue tracker, or in direct messages to the primary developers. You are much more likely to get a response and start a constructive discussion by posting your ideas in the public channels created for these purposes.

### Reporting Issues

If you have found a bug or issue in this project, please report it using the issue tracker. If at all possible, be sure to include details on how to reproduce the bug.

### Writing Code

If you are interested in contributing significant code to the project, please coordinate the effort with one of the main developers in Discord before submitting a pull request. Before deciding to contribute, it is also a good idea to ensure your efforts are in alignment with the Roadmap and current development focus.

By contributing code to this project, you agree that copyright for the code is transferred to the Reticulum maintainers and that the code is irrevocably placed under the Apache license.

