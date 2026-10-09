//! Easy to use Wireguard config generator.
//!
//! - Use [`InterfaceBuilder`] and [`PeerBuilder`] for interface/peers creation.
//! - Use [`Interface`]'s and [`Peer`]'s [`std::fmt::Display`] for exporting  Wireguard config (`.to_string()`, [`write!()`], etc).
//! - Use [`PrivateKey`], [`PublicKey`] and [`PresharedKey`] for generating, importing and
//!   exporting keys.
//! - Use [`AmneziaWG`] to generate and use AmneziaWG obfuscation values.
//!
//! # Features
//!
//! - `amneziawg`: adds support for generating and manually building amnezia settings via
//!   the [`AmneziaWG`] enum, including AmneziaWG 1.0, 2.0 and 3.1.
//!
//!   > **AmneziaWG** is a fork of wireguard, that less recognizable by DPI systems by
//!   > randomizing packet headers with packet sizes.
//!
//! - `keepalive-range`: lets [`Peer`]'s and [`ToInterfaceOptions`]'
//!   `persistent_keepalive` accept a range (`20-30`) instead of a single number.
//!   Implies `amneziawg`.
//!
//! - `serde`: adds implementions of [`serde::Serialize`] and [`serde::Deserialize`] for all structs.
//!
//! # Example
//!
//! ```rust
//! use wireguard_conf::prelude::*;
//! use wireguard_conf::as_ipnet;
//!
//! let peer = Peer::builder()
//!     .allowed_ips([as_ipnet!("10.0.0.2/24")])
//!     .build();
//!
//! let interface = Interface::builder()
//!     .address([as_ipnet!("10.0.0.1/24")])
//!     .peers([peer.clone()])
//!     .build();
//!
//! // to export configs, use `println!()`, `writeln!()`, `.to_string()`, etc.
//!
//! println!("Server's config:");
//! println!("{}\n", interface);
//!
//! println!("Client's config:");
//! println!("{}", peer.to_interface(&interface, ToInterfaceOptions::new()).unwrap());
//! ```

#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![allow(clippy::doc_markdown)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(auto_cfg))]

mod error;
mod keys;
mod macros;
mod models;

#[cfg(feature = "amneziawg")]
mod amneziawg;
#[cfg(feature = "amneziawg")]
pub use amneziawg::*;

pub mod prelude;

pub use ipnet;

pub use error::*;
pub use keys::*;
pub use models::*;
