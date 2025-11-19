use derive_builder::Builder;
use either::Either;
use ipnet::{IpNet, Ipv4Net};

use std::convert::Infallible;
use std::fmt;

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// Struct, that represents `[Peer]` section in configuration.
///
/// [Wireguard docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#peer)
#[must_use]
#[derive(Clone, Debug, PartialEq, Builder)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[builder(build_fn(private, name = "fallible_build", error = "Infallible"))]
pub struct Peer {
    /// Peer's endpoint.
    ///
    /// [Wireguard Docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#endpoint)
    #[builder(setter(into, strip_option), default)]
    pub endpoint: Option<String>,

    /// Peer's allowed IPs.
    ///
    /// - */32 and */128 ipnets will be generated as regular ips (f.e. 1.2.3.4/32 -> 1.2.3.4)
    ///
    /// [Wireguard Docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#allowedips)
    #[builder(setter(into), default)]
    pub allowed_ips: Vec<IpNet>,

    /// Peer's persistent keepalive.
    ///
    /// Represents in seconds how often to send an authenticated empty packet to the peer, for the
    /// purpose of keeping a stateful firewall or NAT mapping valid persistently.
    ///
    /// Setting this value to `0` omits it in config.
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#persistentkeepalive)
    #[builder(default)]
    pub persistent_keepalive: u16,

    /// Peer's key.
    ///
    /// If [`PrivateKey`] is provided, then peer can be exported to interface & full config.
    /// Otherwise only to peer section of config.
    #[builder(default = Either::Left(PrivateKey::random()))]
    pub key: Either<PrivateKey, PublicKey>,

    /// Peer's preshared-key.
    #[builder(setter(strip_option), default)]
    pub preshared_key: Option<PresharedKey>,

    /// AmneziaWG settings.
    ///
    /// Used for packet obfuscation.
    #[cfg(feature = "amneziawg")]
    #[cfg_attr(docsrs, doc(cfg(feature = "amneziawg")))]
    #[builder(setter(strip_option), default)]
    pub amnezia_settings: Option<AmneziaSettings>,
}

impl PeerBuilder {
    /// Create new `InterfaceBuilder`.
    ///
    /// ```rust
    /// # use wireguard_conf::prelude::*;
    /// # use wireguard_conf::as_ipnet;
    /// #
    /// let interface = PeerBuilder::new()
    ///     .allowed_ips([as_ipnet!("0.0.0.0/0")])
    ///     // <snip>
    ///     .build();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets private key.
    ///
    /// Shorthand for `.key(Either::Left(value))`.
    pub fn private_key(&mut self, value: PrivateKey) -> &mut Self {
        self.key = Some(Either::Left(value));
        self
    }

    /// Sets public key.
    ///
    /// Shorthand for `.key(Either::Right(value))`.
    pub fn public_key(&mut self, value: PublicKey) -> &mut Self {
        self.key = Some(Either::Right(value));
        self
    }

    /// Builds an `Interface`.
    pub fn build(&self) -> Peer {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl Peer {
    /// Get Peer's [`Interface`].
    ///
    /// Pass server's interface to `interface` argument.
    ///
    /// # Errors
    ///
    /// - [`WireguardError::NoPrivateKeyProvided`] -- peer don't have private key.
    ///   You need to provide [`PrivateKey`] for creating interfaces from peers.
    /// - [`WireguardError::NoAssignedIP`] -- no assigned ip found.
    ///   This means that your peer doesn't have allowed ip, that is in interface's addresses
    ///   network.
    pub fn to_interface(&self, interface: &Interface) -> WireguardResult<Interface> {
        let Either::Left(private_key) = self.key.clone() else {
            return Err(WireguardError::NoPrivateKeyProvided);
        };

        let assigned_ips: Vec<IpNet> = self
            .allowed_ips
            .iter()
            .filter_map(|allowed_ip| {
                for server_address in &interface.address {
                    if server_address.contains(allowed_ip) {
                        return IpNet::new(allowed_ip.addr(), server_address.prefix_len()).ok();
                    }
                }

                None
            })
            .collect();

        Ok(Interface {
            endpoint: None,

            address: assigned_ips,
            listen_port: None,
            private_key,
            dns: interface.dns.clone(),

            table: None,
            mtu: None,

            #[cfg(feature = "amneziawg")]
            amnezia_settings: self.amnezia_settings.clone(),

            pre_up: vec![],
            pre_down: vec![],
            post_up: vec![],
            post_down: vec![],

            peers: vec![interface.to_peer()],
        })
    }
}

/// Implements [`fmt::Display`] for exporting peer.
///
/// # Note
///
/// It exports only `[Peer] ...` part. To export full interface, use [`Peer::to_interface()`]
/// and then `.to_string()`
impl fmt::Display for Peer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[Peer]")?;
        if let Some(endpoint) = self.endpoint.clone() {
            writeln!(f, "Endpoint = {endpoint}")?;
        }
        writeln!(
            f,
            "AllowedIPs = {}",
            self.allowed_ips
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join(",")
        )?;
        writeln!(
            f,
            "PublicKey = {}",
            self.key.clone().right_or_else(|key| PublicKey::from(&key))
        )?;
        if let Some(preshared_key) = &self.preshared_key {
            writeln!(f, "PresharedKey = {preshared_key}")?;
        }
        if self.persistent_keepalive != 0 {
            writeln!(f, "PersistentKeepalive = {}", self.persistent_keepalive)?;
        }

        Ok(())
    }
}
