# v3.0.0

## Breaking changes

- Removed `AmneziaWG::strip_server_data`, `AmneziaWG2::strip_server_data` and
  `ToInterfaceOptions::strip_server_data`: there are no server-only AmneziaWG
  settings, `Jc`/`Jmin`/`Jmax` and `I1`-`I5` are sent by the client before the handshake.
- `WireguardError::InvalidAmneziaSetting` now holds `&'static str` instead of `String`.
- `Peer`'s `Display` now exports `AllowedIPs` normalized to network addresses, so
  configs with host bits in `AllowedIPs` don't trigger `AllowedIP has nonzero host part`.
- `AmneziaWG` deserialization now picks the newest variant that can parse the
  config (`V3`, then `V2`, then `V1`) instead of the oldest. Valid AmneziaWG 2.0
  configs are now read as `V3`, as are AmneziaWG 1.0 configs whose values fit the
  AmneziaWG 3.1 types. The exported config
  doesn't change, only the variant does — see [the migration guide](./docs/MIGRATION.md#amneziawg-deserialization).

## New features

- Added support for AmneziaWG 3.1.
- Added `AmneziaWG::strip_client_data` (with `AmneziaWG2`/`AmneziaWG3` implementations)
  and `Interface::strip_client_data` for generating server configuration files:
  removes client-side junk packets (`Jc`, `Jmin`, `Jmax`) and the CPS-chain
  (`I1`-`I5`), keeps settings that must match on both sides. The AmneziaWG 3.1
  timers are removed as well, because they are marked as client-side; whether
  they really have to be stripped from the server's config is not decided yet.
- `Peer::persistent_keepalive` and `ToInterfaceOptions::persistent_keepalive`
  both accept a range with the `keepalive-range` feature (implies `amneziawg`):
  they are `UintRange` instead of `u16`. A single value is still exported
  as a plain number, and a zero range (`0-0`) is omitted, as before.
  Without the feature the fields are still `u16`, and a single-value range
  is serialized as a plain number too, so such a config stays portable;
  a multi-value range is serialized as a string and requires the feature.

# v2.0.0

## Breaking changes

- [Rewritten how AmneziaWG values are handled](./docs/MIGRATION.md).

  Now `amneziawg` feature has both versions of AmneziaWG.

# v1.1.0

## New features

- ~~Added separate `amneziawg-1` and `amneziawg-2` for AmneziaWG major versions~~
- Added support for AmneziaWG 2.0 (by [@Im-a-rabbit](https://github.com/Im-a-rabbit))
- Added 1.88 as MSRV
- Added Apache 2.0 license
