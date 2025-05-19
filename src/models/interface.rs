use either::Either;
use ipnet::Ipv4Net;

use std::fmt;

use crate::prelude::*;

/// Struct, that represents complete configuration (contains both `[Interface]` and `[Peer]`
/// sections).
///
/// Use [`InterfaceBuilder`] to create interface.
///
/// [Wireguard docs](https://github.com/pirate/wireguard-docs#interface)
#[must_use]
#[derive(Clone, Debug)]
pub struct Interface {
    /// Interface's address.
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs#address)
    pub address: Ipv4Net,

    /// Port to listen for incoming VPN connections.
    ///
    /// [Wireguard conf](https://github.com/pirate/wireguard-docs#listenport)
    pub listen_port: Option<u16>,

    /// Node's private key.
    ///
    /// [Wireguard conf](https://github.com/pirate/wireguard-docs#privatekey)
    pub private_key: PrivateKey,

    /// The DNS servers to announce to VPN clients via DHCP.
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs#dns-2)
    pub dns: Vec<String>,

    /// Endpoint.
    ///
    /// - `[Interface]` section will have `# Name = <endpoint>` comment at the top.
    /// - Exported [`Peer`] (via [`Interface::to_peer`]) will have this endpoint.
    ///
    /// [Wireguard Docs for `# Name`](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#-name-1);
    /// [Wireguard Docs for endpoint](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#endpoint)
    pub endpoint: Option<String>,

    /// AmneziaWG obfuscation values.
    ///
    /// [AmneziaWG Docs](https://github.com/amnezia-vpn/amneziawg-linux-kernel-module?tab=readme-ov-file#configuration)
    #[cfg(feature = "amneziawg")]
    #[cfg_attr(docsrs, doc(cfg(feature = "amneziawg")))]
    pub amnezia_settings: Option<AmneziaSettings>,

    /// Peers.
    ///
    /// Create them using [`PeerBuilder`] or [`Interface::to_peer`] method.
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs#peer)
    pub peers: Vec<Peer>,
}

impl Interface {
    /// Get [`Peer`] from interface.
    ///
    /// # Examples
    ///
    /// ```
    /// # use wireguard_conf::prelude::*;
    /// // Create server node
    /// let mut server = InterfaceBuilder::new()
    ///     // <snip>
    ///     .build();
    ///
    /// // Create client node, and add server to client's peers
    /// let client = InterfaceBuilder::new()
    ///     // <snip>
    ///     .add_peer(server.to_peer()) // convert `Interface` to `Peer` using `.to_peer()` method.
    ///     .build();
    ///
    /// // Add client to server's peers
    /// server.peers.push(client.to_peer());
    ///
    /// println!("Server config:\n{server}");
    /// println!("Client config:\n{client}");
    /// ```
    pub fn to_peer(&self) -> Peer {
        Peer {
            endpoint: self.endpoint.clone(),
            allowed_ips: vec![self.address],
            key: Either::Left(self.private_key.clone()),

            #[cfg(feature = "amneziawg")]
            amnezia_settings: self.amnezia_settings.clone(),
        }
    }
}

impl fmt::Display for Interface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[Interface]")?;
        if let Some(endpoint) = &self.endpoint {
            writeln!(f, "# Name = {endpoint}")?;
        }
        writeln!(f, "Address = {}", self.address)?;
        if let Some(listen_port) = self.listen_port {
            writeln!(f, "ListenPort = {listen_port}")?;
        }
        writeln!(f, "PrivateKey = {}", self.private_key)?;
        if !self.dns.is_empty() {
            writeln!(f, "DNS = {}", self.dns.join(","))?;
        }

        #[cfg(feature = "amneziawg")]
        if let Some(amnezia_settings) = &self.amnezia_settings {
            writeln!(f)?;
            writeln!(f, "{amnezia_settings}")?;
        }

        for peer in &self.peers {
            writeln!(f)?;
            writeln!(f, "{peer}")?;
        }

        fmt::Result::Ok(())
    }
}
