![Rust](https://github.com/realaltffour/PaperTrader/workflows/Rust/badge.svg)
# PaperTrader
The opensource cross-platform paper trader for learning to trade assets.
# Getting Started

## Developers
Make Sure you have `Cargo`. 
To get a copy of the source code for development:
```shell
$ git clone https://github.com/realaltffour/PaperTrader.git
```

Build command, Server:
```shell
$ cargo build --features "server"
```

Build command, Client:
```shell
$ cargo build --features "client"
```

Build command, Hybrid:
```shell
$ cargo build --features "server,client"
```

Running server/client (depends on your build command):
```shell
$ ./target/debug/sandbox
```

## Built With

* [Rust](https://www.rust-lang.org/) - Language
* [Docker](https://www.docker.com/) - Deployment System
* [PostgreSQL](https://www.postgresql.org/) - Database System

## Authors:
* **altffour** - *Owner* - [realaltffour](https://github.com/realaltffour)
* See also the list of [contributors](https://github.com/realaltffour/PaperTrader/graphs/contributors) who participated in this project.

## License
This project licensed under GPL v3.0 - see the [LICENSE](LICENSE) file for details
