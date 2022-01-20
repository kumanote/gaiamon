# gaiamon

> gaiamon is a gaia daemon node monitoring tool.

# Features

- multiple node monitoring
  - you can monitor your validator node via your sentry nodes
- using only grpc endpoints of gaia daemon(you need to open grpc port internally to this tool in advance.) 
- check node syncing status
- check new proposal
- check if validator node missed sign for block
- check validator status
- check slashes
- alert to [Airbrake](https://airbrake.io/) (or [Errbit](https://github.com/errbit/errbit))
  - you can customize [logger](https://github.com/kumanote/logger-rs) to change how and where to report the alerting log to.

# How to install

## Prerequisite

- [Rust with Cargo](http://rust-lang.org) `1.57.0` or later.

## Install

```bash
# download
$ git clone git@github.com:kumanote/gaiamon.git
# install
$ cd gaiamon
$ cargo build --release
# then you will find an executable in the following path
$ ls -ls ./target/release/gaiamon
```

# Docker build (optional)

```bash
# download
$ git clone git@github.com:kumanote/gaiamon.git
# build
$ docker build -t gaiamon:latest .
```

# Run

Please set up config files before running the tool.
See [config.toml.example](config.toml.example) for configuration file example.

```bash
$ gaiamon -c /path/to/config.toml
```
