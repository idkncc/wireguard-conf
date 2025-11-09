//! Easy to use Wireguard config generator.
//!
//! - Use [`InterfaceBuilder`] and [`PeerBuilder`] for interface/peers creation.
//! - Use [`Interface`]'s and [`Peer`]'s [`std::fmt::Display`] for exporting  Wireguard config (`.to_string()`, [`write!()`], etc).
//! - Use [`PrivateKey`], [`PublicKey`] and [`PresharedKey`] for generating, importing and
//!   exporting keys.
//! - Use [`AmneziaSettings`] for generating/using AmneziaWG obfuscation values.
//!
//! # Features
//!
//! - `amneziawg`: Adds AmneziaWG obfuscation values support.
//!
//! # Example
//!
//! ```
//! use wireguard_conf::prelude::*;
//! use wireguard_conf::as_ipnet;
//!
//! let peer = PeerBuilder::new()
//!     .allowed_ips([as_ipnet!("10.0.0.2/24")])
//!     .build();
//!
//! let interface = InterfaceBuilder::new()
//!     .address(as_ipnet!("10.0.0.1/24"))
//!     .peers([peer.clone()])
//!     .build();
//!
//! // to export configs, use `println!()`, `writeln!()`, `.to_string()`, etc.
//!
//! println!("Server's config:");
//! println!("{}\n", interface);
//!
//! println!("Client's config:");
//! println!("{}", peer.to_interface(&interface).unwrap());
//! ```

#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod macros;
mod models;
mod utils;

pub mod prelude;

pub use models::*;
pub use utils::*;
