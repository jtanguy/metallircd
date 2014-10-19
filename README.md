MetallIRCd
==========

Metallircd (the metallic IRCd) is a new IRCd made using the Rust language, currently still in the early stages of developpment.

It's final goal is to get rid of netsplits in a IRC Network. To do so, we will discard the tree structure of the IRC server to server protocol in profit of the future lord of the Nodes library, providing a distributed way of transmitting messages.

The (current) first step of developpement is implementing the base functions of a lonely IRC server towards its clients.

Building
========

The project uses cargo the rust package manager. So a simple `cargo build` should do.

Documentation
=============

The documentation is moslty par of the sources. You can build it with `cargo doc` and then find it at `./target/doc/metallircd/index.html`.
