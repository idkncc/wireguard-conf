//! Easy to use Wireguard config generator.
//!
//! - Use [`InterfaceBuilder`] and [`PeerBuilder`] for interface/peers creation.
//! - Use [`Interface`]'s and [`Peer`]'s [`std::fmt::Display`] for exporting  Wireguard config (`.to_string()`, [`write!()`], etc).
//! - Use [`PrivateKey`] and [`PublicKey`] for generating, importing keys.
//!
//! # Example
//!
//! ```
//! use wireguard_conf::prelude::*;
//! use wireguard_conf::as_ipnet;
//!
//! use ipnet::Ipv4Net;
//!
//! let peer = PeerBuilder::new()
//!     .add_allowed_ip(as_ipnet!("10.0.0.2/24"))
//!     .build();
//!
//! let interface = InterfaceBuilder::new()
//!     .address(as_ipnet!("10.0.0.1/24"))
//!     .add_peer(peer.clone())
//!     .build();
//!
//! println!("Server's config:");
//! println!("{}\n", interface);
//!
//! println!("Client's config:");
//! println!("{}", peer.as_interface(&interface).unwrap());
//! ```

mod models;
mod utils;
mod macros;

pub mod prelude;

pub use models::*;
pub use utils::*;

