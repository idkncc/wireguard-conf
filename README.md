# Wireguard Conf

Easy to use library for creating wireguard configs.

## Installation

Install `wireguard-conf`

```shell
cargo add wireguard-conf
```

### Usage

The best way to understand library: [Quickstart](./QUICKSTART.md)!

More usage examples you can find in [tests/tutorial.rs](tests/tutorial.rs), [tests folder](tests/) and documentation examples on [docs.rs](https://docs.rs/wireguard-conf).

```rust
use wireguard_conf::prelude::*;
use wireguard_conf::as_ipnet;

let peer = PeerBuilder::new()
    .allowed_ips([as_ipnet!("10.0.0.2/24")])
    .build();

let interface = InterfaceBuilder::new()
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

- `amneziawg`: adds support for generating/using [AmneziaWG](https://docs.amnezia.org/documentation/amnezia-wg/) obfuscation values.
- `serde`: adds implementions of [`serde::Serialize`] and [`serde::Deserialize`] for all structs.

### Contributing

1. Fork & clone
2. Install Rust, Cargo and [just](https://just.systems/). 

   Or start nix shell: `direnv allow` or just `nix develop`
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
