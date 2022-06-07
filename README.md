# dynhost

[![CI](https://github.com/willdurand/dynhost/actions/workflows/ci.yml/badge.svg)](https://github.com/willdurand/dynhost/actions/workflows/ci.yml)

`dynhost` is a tiny CLI that updates a dynamic DNS record (DynHost) for a domain
hosted by [OVH](https://www.ovh.com). It relies on
[icanhazip.com](http://ipv4.icanhazip.com/) to retrieve the public IP and calls
the OVH "API" to perform the update.

## Usage

This tool expects the following environment variables:

- `DYNHOST_USERNAME`: DynHost username
- `DYNHOST_PASSWORD`: DynHost password
- `DYNHOST_HOSTNAME`: the domain to update

To update the dynamic DNS record, run:

```
dynhost
```

That's it!

### Other commands

You can print the version by running:

```
dynhost -V
```

### Systemd integration

You can find two configuration files for
[systemd](https://en.wikipedia.org/wiki/Systemd) to automatically run `dynhost`
(every hour by default).

## Installation

:warning: You should have [Rust installed](https://rustup.rs/) to build this
project. Ready?

Clone this project, then run:

```
cargo build --release
```

You will find the binary in `target/release/`.

### Cross-compilation for ARMv6 (Raspberry Pi Model 1)

This repository contains a set of tools to cross-compile `dynhost` for Raspberry
Pi Model 1 (ARMv6). If you have a Raspberry Pi 2 or 3, the architecture is
different (ARMv7).

Run the command below to cross-compile `dynhost` for ARMv6:

```
./cross-compile-armv6 --release
```

You will find the binary in `target/arm-unknown-linux-gnueabihf/release/`.

**Note:** the Docker image can be built with the following command:

```
docker build -t "willdurand/rust-armv6:latest" -f Dockerfile.armv6 .
```

### Cross-compilation for ARMv7

This repository contains a set of tools to cross-compile `dynhost` for Orange Pi
(ARMv7).

Run the command below to cross-compile `dynhost` for ARMv7:

```
./cross-compile-armv7 --release
```

You will find the binary in `target/armv7-unknown-linux-gnueabihf/release/`.

**Note:** the Docker image can be built with the following command:

```
docker build -t "willdurand/rust-armv7:latest" -f Dockerfile.armv7 .
```

## License

This project is released under the MIT License. See the bundled [LICENSE
file](./LICENSE) for details.
