# wireguard-conf

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/idkncc/wireguard-conf/build.yml)](https://github.com/idkncc/wireguard-conf/actions)
[![docs.rs](https://img.shields.io/docsrs/wireguard-conf)](https://docs.rs/wireguard-conf)
[![Crates.io Version](https://img.shields.io/crates/v/wireguard-conf)](https://crates.io/crates/wireguard-conf)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/wireguard-conf)](https://crates.io/crates/wireguard-conf)
[![Crates.io MSRV](https://img.shields.io/crates/msrv/wireguard-conf)](https://crates.io/crates/wireguard-conf)
[![Quickstart Tutorial](https://img.shields.io/badge/Quickstart_Tutorial-blue)](./docs/QUICKSTART.md)

Easy to use library for creating wireguard configs.

## Installation & Usage

Install `wireguard-conf`

```shell
cargo add wireguard-conf
```

### Usage

The best way to understand library: [Quickstart](./docs/QUICKSTART.md)!

More usage examples you can find in [tests/tutorial.rs](tests/tutorial.rs), [tests folder](tests/) and documentation examples on [docs.rs](https://docs.rs/wireguard-conf).

```rust
use wireguard_conf::prelude::*;
use wireguard_conf::as_ipnet;

let peer = Peer::builder()
    .allowed_ips([as_ipnet!("10.0.0.2/24")])
    .build();

let interface = Interface::builder()
    .address([as_ipnet!("10.0.0.1/24")])
    .peers([peer.clone()])
    .build();

// to export configs, use `println!()`, `writeln!()`, `.to_string()`, etc.

println!("Server's config:");
println!("{}\n", interface);

println!("Client's config:");
println!("{}", peer.to_interface(&interface, ToInterfaceOptions::new()).unwrap());
```

### Features

- `amneziawg`: adds support for generating and manually building amnezia settings via
  [`AmneziaWG`](https://docs.rs/wireguard-conf/latest/wireguard-conf/enum.AmneziaWG.html) enum.
  AmneziaWG **1.0**, **2.0** and **3.1** are supported (`AmneziaWG::random_v1`,
  `AmneziaWG::random_v2`, `AmneziaWG::random_v3`).

  > **AmneziaWG** is a fork of wireguard, that less recognizable by DPI systems by
  > randomizing packet headers with packet sizes.

- `keepalive-range`: lets `Peer`'s and `ToInterfaceOptions`' `persistent_keepalive`
  accept a range (`20-30`) instead of a single number, that requires AmneziaWG 3.1.
  Implies `amneziawg`.

- `serde`: adds implementions of `serde::Serialize` and `serde::Deserialize` for all structs.

## Misc

- [Contributing](./CONTRIBUTING.md)
- [Major releases migration guides](./docs/MIGRATION.md)
- Licensed under either [MIT](./LICENSE-MIT) or [Apache 2.0](./LICENSE-APACHE)
