# Migration

## `v3.0.0`

### AmneziaWG 3.1

`AmneziaWG` enum now has `V3` variant for **AmneziaWG 3.1**:

```rust
let settings = AmneziaWG::random_v3();
let settings = AmneziaWG::builder_v3()
    .jc(9)
    // <snip>
    .build()
    .expect("valid settings");
```

**New in AmneziaWG 3.1**, besides the `V3` variant itself:

- `HeaderProtectionKey` — a 32-byte key for header protection. An invalid key is
  reported with `WireguardError::InvalidHeaderProtectionKey`.
- `UintRange` and `HeaderRange` types for the settings, that accept a range
  (see [Ranges](#ranges)).

### `AmneziaWG` deserialization

`AmneziaWG` has no version tag in the `serde` format, so the variant is picked by
trying the variants in order, from the newest to the oldest: `V3`, then `V2`, then
`V1`, and an error if none of them matches.

So valid AmneziaWG 2.0 configs are now read as `V3`, because every `V2` setting
has the same name in `V3`. Valid AmneziaWG 1.0 configs are read as `V3` too, as
long as their values fit `V3`'s types (`V1` stores them as `usize`, so e.g.
`S1`/`S2` above `u8::MAX` stay `V1`).
The exported config doesn't change, only the variant does:

```rust
let settings = AmneziaWG::random_v2();

let json = serde_json::to_string(&settings)?;
let decoded: AmneziaWG = serde_json::from_str(&json)?;

// same config
assert_eq!(decoded.to_string(), settings.to_string());
// but a different variant
assert!(matches!(decoded, AmneziaWG::V3(_)));
```

### Removed `strip_server_data`

`AmneziaWG::strip_server_data`, `AmneziaWG2::strip_server_data` and
`ToInterfaceOptions::strip_server_data` are removed.

There are no server-only AmneziaWG settings: the CPS-chain (`I1`-`I5`) and junk
packets (`Jc`, `Jmin`, `Jmax`) are sent by the client before the handshake and
are not needed in the server's config.

To generate a server config, use `AmneziaWG::strip_client_data` (or
`Interface::strip_client_data`) instead. It removes junk packets
(`Jc`, `Jmin`, `Jmax`) and the CPS-chain (`I1`-`I5`), keeps the rest:

```rust
let interface = InterfaceBuilder::new()
    .amnezia_settings(settings)
    .build();

// the full config, with the client-only settings
let full_conf = interface.to_string();
// the config for the server
let server_conf = interface.strip_client_data().to_string();
```

`AmneziaWG::strip_client_data` modifies the settings in place (it takes `&mut self`;
for `V1` it is a no-op). `Interface::strip_client_data()` takes `&self` and returns a
copy with client-only settings removed, so the interface itself stays intact and a
single one can be exported both as is and stripped, right before each output.

### Ranges

AmneziaWG 3.1 settings, that accept a range of values, use range types instead of
plain numbers. There are two of them: `UintRange` for the settings, that
`amneziawg-tools` reads as 16-bit values, and `HeaderRange` for the 32-bit ones.

- `AmneziaWG3`'s `H1`-`H4` are `HeaderRange` (`u32`).
- `AmneziaWG3`'s `ContentPaddingAddition`, `RekeyAfterTime`, `RekeyTimeout`,
  `RejectAfterTime`, `KeepaliveTimeout` and `MaxHandshakeAttempts` are
  `UintRange` (`u16`).

> **Note**
>
> `AmneziaWG2` is not affected: its `H1`-`H4` are still `HRange`, that is a
> different type and is not replaced by `HeaderRange`.

A range is exported as `lo-hi`, so `4-4` is exported as `4` and doesn't require
AmneziaWG 3.1. With `serde` a single-value range is serialized as a plain number,
so `4-4` stays `4`, while a multi-value range is a string: `"20-30"`.

Bounds are available with `lo()` and `hi()`, and a range always has `lo <= hi`,
so there's nothing to validate.

#### Building a range

A range is built either from a single number (a range with equal bounds), from a
parsed string, or from a pair of bounds with `TryFrom`:

```rust
let range = HeaderRange::try_from((2, 4))?;
let range: HeaderRange = "2-4".parse()?;
```

Bounds, parsed from a string, need a fallible conversion, so in a builder they can
only be set with the `try_`-prefixed setters. A plain number goes through the
infallible setters:

```rust
let settings = AmneziaWG::builder_v3()
    .h1(1) // still a number
    .try_h2("2-4")? // now a range
    // <snip>
    .build()?;
```

### `Peer::persistent_keepalive`

`PersistentKeepalive` accepts a range in AmneziaWG 3.1, so with `keepalive-range`
feature (that implies `amneziawg`) `Peer::persistent_keepalive` is `UintRange`
instead of `u16`:

```diff
 let client_interface = client_peer.to_interface(
     &server_interface,
     ToInterfaceOptions::new()
-        .persistent_keepalive(25),
+        .persistent_keepalive(25) // still works
+        // or, a range:
+        .persistent_keepalive("20-30".parse::<UintRange>()?),
 );
```

`ToInterfaceOptions` is not a `derive_builder` builder, so it has no `try_`-prefixed
setters: it takes `impl Into<UintRange>`, and a range has to be parsed or built
with `TryFrom` beforehand.

Without `keepalive-range` feature the field is still `u16`, since a range is not
valid for the vanilla WireGuard. With `serde` a single-value range is serialized as
a plain number, so `25` stays `25` in both builds. A multi-value range is serialized
as a string (`"20-30"`), so only it depends on the feature: a config with such a
range can't be read back by a build with the feature disabled. See
[Ranges](#ranges).

A zero range (`0-0`) is not exported at all, just like a `0` value was not.

### `WireguardError::InvalidAmneziaSetting`

Now holds `&'static str` instead of `String`:

```diff
-Err(WireguardError::InvalidAmneziaSetting("Jc".to_string()))
+Err(WireguardError::InvalidAmneziaSetting("Jc"))
```

## `v2.0.0`

### AmneziaWG

If you use AmneziaWG, now there's `AmneziaWG` enum, that contains `V1` and `V2` for **AmneziaWG 1.0** and **AmneziaWG 2.0** respectively.

```diff
-// old:
-let amneziawg = AmneziaSettings::random();
-let amneziawg = AmneziaSettings { ... };
+// new:
+let amneziawg = AmneziaWG::random_v1();
+
+let amneziawg = AmneziaWG::random_v2();
+let amneziawg = AmneziaWG::builder_v2()
+    .jc(5)
+    .jmin(65)
+    // <snip>
+    .build();
+let amneziawg = AmneziaWG::empty_v2();
 
 let interface = InterfaceBuilder::new()
     .amnezia_settings(amneziawg)
     .build();
```

## `v1.0.0`

First major release brought many changes. Breaking changes are builder structure and `Peer::to_interface` signature.

### Builders & models themselves

Now they are generated with `derive_builder`.

1. Removal of `set_*` and `add_*` builder methods.

   ```diff
    let interface = InterfaceBuilder::new()
   -    .add_dns("1.1.1.1".to_string())
   -    .add_dns("1.0.0.1".to_string())
   -    .set_peers(vec![peer1, peer2])
   -    .add_peer(somepeer)
   -    .add_post_up("rm -rf /".to_string())
   +    .dns(["1.1.1.1".to_string(), "1.0.0.1".to_string()])
   +    .peers([peer1, peer2, somepeer])
   +    // or .peers(vec![peer1, peer2, somepeer])
   +    .post_up(["rm -rf /".to_string()])
        .build();
   ```

2. Support for multiple addresses

    ```diff
     let interface = InterfaceBuilder::new()
    -    .address(as_ipnet!("10.0.0.1/24"))
    +    .address([as_ipnet!("10.0.0.1/24")])
    +    // or .add_network(as_ipnet!("10.0.0.1/24"))
         .build();
    ```

### `Peer::to_interface()`

Now this method requires additional 2nd argument `ToInterfaceOptions`.

```diff
 let client_interface = client_peer.to_interface(
     &server_interface,
+    ToInterfaceOptions::new(),
 );
```

With these options, you skip writing boilerplate.

- Default gateway

  ```diff
   let client_interface = client_peer.to_interface(
       &server_interface,
  +    ToInterfaceOptions::new()
  +        .default_gateway(true),
   );

  -client_interface.peers[0].allowed_ips = vec![as_ipnet("0.0.0.0/0")];
  ```

- `PersistentKeepalive`

  ```diff
   let client_interface = client_peer.to_interface(
       &server_interface,
  +    ToInterfaceOptions::new()
  +        .persistent_keepalive(25),
   );

  -client_interface.peers[0].persistent_keepalive = 25;
  ```
