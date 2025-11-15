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

let peer = PeerBuilder::new()
    .allowed_ips([as_ipnet!("10.0.0.2/24")])
    .build();

let interface = InterfaceBuilder::new()
    .address(as_ipnet!("10.0.0.1/24"))
    .peers([peer.clone()])
    .build();

// to export configs, use `println!()`, `writeln!()`, `.to_string()`, etc.

println!("Server's config:");
println!("{}\n", interface);

println!("Client's config:");
println!("{}", peer.to_interface(&interface).unwrap());
```

### Features

- `amneziawg`: adds support for generating/using [AmneziaWG](https://docs.amnezia.org/documentation/amnezia-wg/) obfuscation values.

### Contributing

1. Fork & clone
2. Install Rust, Cargo, etc. On nix you can start devshell (`nix develop -c $SHELL`)
3. Make changes
4. Format and lint code:
   ```
   just fmt
   just lint
   # or fix automatically: just lint-fix
   ```
5. Commit changes (use [Conventional commits](https://www.conventionalcommits.org/en/v1.0.0/))
   ```shell
   git commit -m "feat: did something"
   ```
6. Send ~patches~ PR
