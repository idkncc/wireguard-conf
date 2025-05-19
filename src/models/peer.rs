use either::Either;
use ipnet::Ipv4Net;

use std::fmt;

use crate::prelude::*;

/// Struct, that represents `[Peer]` section in configuration.
///
/// [Wireguard docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#peer)
#[must_use]
#[derive(Clone, Debug)]
pub struct Peer {
    /// Peer's endpoint.
    pub endpoint: Option<String>,

    /// Peer's allowed IPs.
    pub allowed_ips: Vec<Ipv4Net>,

    /// Peer's key.
    ///
    /// If [`PrivateKey`] is provided, then peer can be exported to interface & full config.
    /// Otherwise only to peer section of config.
    pub key: Either<PrivateKey, PublicKey>,

    #[cfg(feature = "amneziawg")]
    #[cfg_attr(docsrs, doc(cfg(feature = "amneziawg")))]
    pub amnezia_settings: Option<AmneziaSettings>,
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

        let assigned_ip = *self
            .allowed_ips
            .iter()
            .find(|&net| interface.address.contains(net))
            .ok_or(WireguardError::NoAssignedIP)?;

        Ok(Interface {
            address: assigned_ip,
            listen_port: None,
            private_key,
            dns: interface.dns.clone(),

            #[cfg(feature = "amneziawg")]
            amnezia_settings: self.amnezia_settings.clone(),

            endpoint: None,
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

        Ok(())
    }
}
