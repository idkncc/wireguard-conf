# Quickstart/Tutorial

First thing to undestand is **small terminology**, that will be used here and in the documentation:

 - Interface -- is a complete config, that starts with `[Interface]` and includes Peers (`[Peer]` sections).
 - Peer -- is a [`Peer`] section of config.
 - Server -- remote instance of Wireguard (e.g. VPS, Office, etc.)
 - Client -- local instance of Wireguard (e.g. Your PC, etc.)

Also, I'm assuming, that you already know and understand Wireguard, networking and IP.

## Installing & Importing

Usual stuff, nothing very special

```shell
$ cargo add wireguard_conf
```

```rust
use wireguard_conf::{prelude::*}; // prelude has all commonly used stuff

fn main() {
}
```

## Keys

First thing to do: generate keys.

Even though it's optional, but you need to save private keys somewhere and also load them sometime.

Lets start with generating ones:

```rust
fn main() {
    let server_private_key = PrivateKey::random();
    let client_private_key = PrivateKey::random();
}
```

In this example, we wont save and load private keys, but it can be implemented by using `PrivateKey::to_string()` and `PrivateKey::try_from(&str)`. Same methods can be used on public keys.

> [!TIP]
>
> You can also use `serde` feature! It will add `Serialize` and `Deserialize` traits to all structs (including private and public keys).

## Creating Server's Interface.

For creating one, we'll use `InterfaceBuilder`. It has convenient methods and design. To create 
one you can use either `Interface::builder()` or `InterfaceBuilder::new()`.

We'll make some-what realistic example, and it will have:
**Address**, **ListenPort**, **PrivateKey**, **DNS**, **Endpoint**, **PostUp** and **PostDown** scripts.

<details>

<summary>Details about each field, that will be used</summary>

- **Address**

  We have multiple choices to do it: `.address()`, `.add_network()` or `.add_address()`. Usually,
  you will use either `.address()` or `.add_network()`. 

  > `.add_address(value)` will add /32 *(or /128)* network (because it contains **only 1 address**).
  >
  > For example, `.add_address(as_ipaddr!("1.2.3.4"))` will have same result as `.add_network(as_ipnet!("1.2.3.4/32"))`

  Also worth pointing out, that `as_ipnet!()` macro is a shorthand for parsing expression as `ipnet::IpNet`.
  `as_ipaddr!()` is smillar and parses expression as `std::net::IpAddr`.

  In this example we'll use `.add_network()`.

  ```rust
  let server_interface = InterfaceBuilder::new()
      .add_network(as_ipnet!("10.0.0.1/24"))
  ```

- **ListenPort**, **PrivateKey**, **DNS** and **Endoint**
 
  
  I'm showing them together, because they don't have some super special differences.

  They set value for corresponding property.

  ```rust
      .listen_port(51820)
      .private_key(server_private_key)
      .dns(["1.1.1.1".to_string(), "1.0.0.1".to_string()]) // NOTE: they will be joined by `,`
      .endpoint("network.office.com")
  ```

- **PostUp** and **PostDown**
  
  All script fields (**PreUp**, **PreDown**, **PostUp**, **PostDown**) are arrays of fields.
  Note, that they are joined by repeating (this will be clear at the end), but **not by `;`**.

  ```rust
  // Server's output interface.
  let output_interface = "ens0";

  let server_interface = InterfaceBuilder::new()
      // <snip>
      .post_up([
          "iptables -A FORWARD -i %i -j ACCEPT".to_string(),
          "iptables -A FORWARD -o %i -j ACCEPT".to_string(),
          format!(
              "iptables -t nat -A POSTROUTING -o {} -j MASQUERADE",
              output_interface
          ),
      ])
      .post_down([
          "iptables -D FORWARD -i %i -j ACCEPT".to_string(),
          "iptables -D FORWARD -o %i -j ACCEPT".to_string(),
          format!(
              "iptables -t nat -D POSTROUTING -o {} -j MASQUERADE",
              output_interface
          ),
      ])
  ```

</details>

And after adding these methods we have this:

```rust
// Server's output interface.
let output_interface = "ens0";

let mut server_interface = Interface::builder()
    .address([as_ipnet!("10.0.0.1/24")])
    .listen_port(51820)
    .private_key(server_private_key.clone())
    .dns(["1.1.1.1".to_string(), "1.0.0.1".to_string()])
    .endpoint("network.office.com")
    // Firewall configuration
    //  Sets up firewall forwards and NAT
    .post_up([
        "iptables -A FORWARD -i %i -j ACCEPT".to_string(),
        "iptables -A FORWARD -o %i -j ACCEPT".to_string(),
        format!(
            "iptables -t nat -A POSTROUTING -o {} -j MASQUERADE",
            output_interface
        ),
    ])
    .post_down([
        "iptables -D FORWARD -i %i -j ACCEPT".to_string(),
        "iptables -D FORWARD -o %i -j ACCEPT".to_string(),
        format!(
            "iptables -t nat -D POSTROUTING -o {} -j MASQUERADE",
            output_interface
        ),
    ])
    .build();
```

## Adding Peer to Interface

For server to understand and differentiate clients, they use `[Peer]` sections.
They can be generated by using `Peer` struct.

```
let client_peer = Peer::builder()
```

Here's details about fields, that we'll be using:

- `key`
  
  It can be strange, that `Peer` can have either `PublicKey` or `PrivateKey`, because in the complete config peers can have only public keys.

  But `wireguard_conf` have support for defining either, because of the most useful helper function: `Peer::to_interface()`. We'll talk about it in the next section, but summarizing: it helps generate full client config from Client `Peer` and Server `Interface`, and that full client config contains `PrivateKey`.

  In the most cases, you will use `.private_key()` (to set it), but there's also `.public_key()` and `.key`, that takes `Either` of them.

  ```rust
      .private_key(client_private_key.clone())
  ```

- `allowed_ips`

  The allowed ips for the Peer.

  ```rust
      .allowed_ips([as_ipnet!("10.0.0.2/32")])
  ```

And thats configuration for your client `Peer`!

```rust
let peer = Peer::builder()
    .private_key(client_private_key.clone())
    .allowed_ips([as_ipnet!("10.0.0.2/32")])
    .build();
```

The last thing to do: add peer to interface.

`Interface` has `peers` property, which is of type `Vec<Peer>`, so:

```rust
server_interface.peers.push(client_peer.clone());
```
    
## `Peer::to_interface()`

This method is a great helper function, that constructs Client's `Interface` from Client's `Peer`
and Server's `Interface`.

As arguments, it takes reference to server `Interface` and `ToInterfaceOptions`.

With `ToInterfaceOptions` you can:

- Make server a default gateway (`.default_gateway(true)`)
- Set PersistentKeepalive to Server's `Peer` (`.persistent_keepalive(value)`)

```
let client_interface = client_peer
    .to_interface(
        &server_interface,
        ToInterfaceOptions::new()
            .default_gateway(true)
            .persistent_keepalive(25),
    )
    .expect("failed to get interface from peer");
```

And now we have full Client `Interface`, that we can actually use!

## Exporting and testing it

To export config you can use the `fmt::Display` trait: `write!()`, `println!()` or `.to_string()` it.

```
let server_conf = server_interface.to_string();
let client_conf = peer1_interface.to_string();
```

And let's add testing `println`s

```
println!("=== SERVER CONFIG ===");
println!("{server_conf}");
println!("=== END SERVER CONFIG ===");

println!();

println!("=== CLIENT CONFIG ===");
println!("{client_conf}");
println!("=== END CLIENT CONFIG ===");
```

And here's the output:

```
=== SERVER CONFIG ===
[Interface]
# Name = network.office.com
Address = 10.0.0.1/24
ListenPort = 51820
PrivateKey = jsO+dhTXsoS7mg8vRPnZGPWhfz0+PttpIJb23NYFpVw=
DNS = 1.1.1.1,1.0.0.1

PostUp = iptables -A FORWARD -i %i -j ACCEPT
PostUp = iptables -A FORWARD -o %i -j ACCEPT
PostUp = iptables -t nat -A POSTROUTING -o ens0 -j MASQUERADE

PostDown = iptables -D FORWARD -i %i -j ACCEPT
PostDown = iptables -D FORWARD -o %i -j ACCEPT
PostDown = iptables -t nat -D POSTROUTING -o ens0 -j MASQUERADE

[Peer]
AllowedIPs = 10.0.0.2/32
PublicKey = cTbbaEmcwS4t8200uZeiyidQm3ZWoYfNGGntaShSf38=


=== END SERVER CONFIG ===
```

Server is absolutely right, let's also check client:

```
=== CLIENT CONFIG ===
[Interface]
Address = 10.0.0.2/24
PrivateKey = M4WOJmmjt9aWR0jcvnyZzZXSwzWaZLeRixgxhHUj5CI=
DNS = 1.1.1.1,1.0.0.1

[Peer]
Endpoint = network.office.com:51820
AllowedIPs = 0.0.0.0/0
PublicKey = khsgcz3SXuc2PyIIupPZ4YjmRJMAHPoLHNEhK6RA1U8=
PersistentKeepalive = 25


=== END CLIENT CONFIG ===
```

It's right too!
