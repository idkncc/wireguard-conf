# Wireguard Conf

Easy to use library for creating wireguard configs.

## Installation

Install `wireguard-conf` and `ipnet` (for parsing ip networks)

```shell
cargo add wireguard-conf ipnet
```

### Usage

More usage examples in [tests](tests/) and on [docs.rs](https://docs.rs/wireguard-conf)

```rust
use wireguard_conf::prelude::*;
use wireguard_conf::as_ipnet;

use ipnet::Ipv4Net;

// create peer:
let peer = PeerBuilder::new()
    .add_allowed_ip(as_ipnet!("10.0.0.2/24"))
    .build();

// create interface with that peer:
let interface = InterfaceBuilder::new()
    .address(as_ipnet!("10.0.0.1/24"))
    .add_peer(peer.clone())
    .build();

// to export configs, use `println!()`, `writeln!()`, `.to_string()`, etc.

println!("Server's config:");
println!("{}\n", interface);

println!("Client's config:");
println!("{}", peer.to_interface(&interface).unwrap());
```
