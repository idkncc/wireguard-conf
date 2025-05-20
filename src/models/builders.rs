use either::Either;
use ipnet::Ipv4Net;

use crate::prelude::*;

/// Builder, that used for creating [`Interface`]s.
///
/// # Examples
///
/// ```
/// use wireguard_conf::prelude::*;
///
/// let server_private_key = PrivateKey::random();
///
/// let interface = InterfaceBuilder::new()
///     .address("10.0.0.1/24".parse().unwrap())
///     .listen_port(6969)
///     .private_key(server_private_key.clone())
///     .set_dns(vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()])
///     .endpoint("vpn.example.com".to_string())
///     // .add_peer(some_peer)
///     .build();
///
/// assert_eq!(interface.address, "10.0.0.1/24".parse().unwrap());
/// assert_eq!(interface.listen_port, Some(6969));
/// assert_eq!(interface.private_key, server_private_key);
/// assert_eq!(interface.dns, vec!["8.8.8.8", "8.8.4.4"]);
/// assert_eq!(interface.endpoint, Some("vpn.example.com".to_string()));
/// ```
#[must_use]
#[derive(Default)]
pub struct InterfaceBuilder {
    address: Ipv4Net,
    listen_port: Option<u16>,
    private_key: Option<PrivateKey>,
    dns: Vec<String>,
    endpoint: Option<String>,

    table: Option<Table>,
    mtu: Option<usize>,

    peers: Vec<Peer>,

    #[cfg(feature = "amneziawg")]
    #[cfg_attr(docsrs, doc(cfg(feature = "amneziawg")))]
    amnezia_settings: Option<AmneziaSettings>,

    pre_up: Vec<String>,
    pre_down: Vec<String>,
    post_up: Vec<String>,
    post_down: Vec<String>,
}

impl InterfaceBuilder {
    pub fn new() -> InterfaceBuilder {
        InterfaceBuilder::default()
    }

    /// Set the address.
    ///
    /// [Wireguard Docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#address)
    pub fn address(mut self, address: Ipv4Net) -> Self {
        self.address = address;
        self
    }

    /// Set the listen port.
    ///
    /// [Wireguard Docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#listenport)
    pub fn listen_port(mut self, listen_port: u16) -> Self {
        self.listen_port = Some(listen_port);
        self
    }

    /// Set the private key.
    ///
    /// [Wireguard Docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#privatekey)
    pub fn private_key(mut self, private_key: PrivateKey) -> Self {
        self.private_key = Some(private_key);
        self
    }

    /// Set the DNS servers array.
    ///
    /// [Wireguard Docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#dns-2)
    pub fn set_dns(mut self, dns: Vec<String>) -> Self {
        self.dns = dns;
        self
    }

    /// Add DNS server.
    ///
    /// [Wireguard Docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#dns-2)
    pub fn add_dns(mut self, dns: String) -> Self {
        self.dns.push(dns);
        self
    }

    /// Set the endpoint.
    ///
    /// # Note
    ///
    /// - `[Interface]` section will have `# Name = <endpoint>` comment at the top.
    /// - Exported [`Peer`] (via [`Interface::to_peer`]) will have this endpoint.
    ///
    /// [Wireguard Docs for `# Name`](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#-name-1);
    /// [Wireguard Docs for endpoint](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#endpoint)
    pub fn endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    /// Set routing table. See [`Table`] for special values.
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#table)
    pub fn set_table(mut self, value: Table) -> Self {
        self.table = Some(value);
        self
    }

    /// Set Maximum Transmission Unit (MTU, aka packet/frame size).
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#mtu)
    pub fn set_mtu(mut self, value: usize) -> Self {
        self.mtu = Some(value);
        self
    }

    /// Set the Peers array.
    ///
    /// [Wireguard Docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#peer)
    pub fn set_peers(mut self, peers: Vec<Peer>) -> Self {
        self.peers = peers;
        self
    }

    /// Add peer.
    ///
    /// [Wireguard Docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#peer)
    pub fn add_peer(mut self, peer: Peer) -> Self {
        self.peers.push(peer);
        self
    }

    /// Sets AmneziaWG obfuscation values.
    ///
    /// [AmneziaWG Docs](https://github.com/amnezia-vpn/amneziawg-linux-kernel-module?tab=readme-ov-file#configuration)
    #[cfg(feature = "amneziawg")]
    #[cfg_attr(docsrs, doc(cfg(feature = "amneziawg")))]
    pub fn amnezia_settings(mut self, amnezia_settings: AmneziaSettings) -> Self {
        self.amnezia_settings = Some(amnezia_settings);
        self
    }

    // TODO: refactor with macros

    /// Set commands, that will be executed before the interface is brought up.
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs#preup)
    pub fn set_pre_up(mut self, snippets: Vec<String>) -> Self {
        self.pre_up = snippets;
        self
    }

    /// Add command, that will be executed before the interface is brought up.
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs#$docs)
    pub fn add_pre_up(mut self, snippet: String) -> Self {
        self.pre_up.push(snippet);
        self
    }

    /// Set commands, that will be executed before the interface is brought down.
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs#predown)
    pub fn set_pre_down(mut self, snippets: Vec<String>) -> Self {
        self.pre_down = snippets;
        self
    }

    /// Add command, that will be executed before the interface is brought down.
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs#predown)
    pub fn add_pre_down(mut self, snippet: String) -> Self {
        self.pre_down.push(snippet);
        self
    }

    /// Set commands, that will be executed after the interface is brought up.
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs#postup)
    pub fn set_post_up(mut self, snippets: Vec<String>) -> Self {
        self.post_up = snippets;
        self
    }

    /// Add command, that will be executed after the interface is brought up.
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs#postup)
    pub fn add_post_up(mut self, snippet: String) -> Self {
        self.post_up.push(snippet);
        self
    }

    /// Set commands, that will be executed after the interface is brought down.
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs#postdown)
    pub fn set_post_down(mut self, snippets: Vec<String>) -> Self {
        self.post_down = snippets;
        self
    }

    /// Add command, that will be executed after the interface is brought down.
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs#postdown)
    pub fn add_post_down(mut self, snippet: String) -> Self {
        self.post_down.push(snippet);
        self
    }

    /// Creates [`Interface`].
    pub fn build(self) -> Interface {
        Interface {
            address: self.address,
            listen_port: self.listen_port,
            private_key: self.private_key.unwrap_or_else(PrivateKey::random),
            dns: self.dns,

            #[cfg(feature = "amneziawg")]
            amnezia_settings: self.amnezia_settings,

            endpoint: self.endpoint,

            table: self.table,
            mtu: self.mtu,

            peers: self.peers,

            pre_up: self.pre_up,
            pre_down: self.pre_down,
            post_up: self.post_up,
            post_down: self.post_down,
        }
    }
}

/// Builder, that used for creating [`Peer`]s.
///
/// # Examples
///
/// ```
/// use wireguard_conf::prelude::*;
/// use either::Either;
///
/// let client_private_key = PrivateKey::random();
///
/// let peer = PeerBuilder::new()
///     .endpoint("public.client.example.com".to_string())
///     .add_allowed_ip("10.0.0.2/32".parse().unwrap())
///     .private_key(client_private_key.clone())
///     // you can provide public key, instead of private_key.
///     // but you can't generate `Interface` out of `Peer`:
///     //  .public_key(client_public_key)
///     .build();
///
/// assert_eq!(peer.endpoint, Some("public.client.example.com".to_string()));
/// assert_eq!(peer.allowed_ips, vec!["10.0.0.2/32".parse().unwrap()]);
/// assert_eq!(peer.key, Either::Left(client_private_key));
/// ```
#[must_use]
#[derive(Default)]
pub struct PeerBuilder {
    endpoint: Option<String>,
    allowed_ips: Vec<Ipv4Net>,
    key: Option<Either<PrivateKey, PublicKey>>,

    #[cfg(feature = "amneziawg")]
    #[cfg_attr(docsrs, doc(cfg(feature = "amneziawg")))]
    amnezia_settings: Option<AmneziaSettings>,
}

impl PeerBuilder {
    pub fn new() -> PeerBuilder {
        PeerBuilder::default()
    }

    /// Sets endpoint.
    ///
    /// [Wireguard Docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#endpoint)
    pub fn endpoint(mut self, endpoint: String) -> PeerBuilder {
        self.endpoint = Some(endpoint);
        self
    }

    /// Sets Allowed IPs array.
    ///
    /// [Wireguard Docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#allowedips)
    pub fn set_allowed_ips(mut self, ip: Vec<Ipv4Net>) -> PeerBuilder {
        self.allowed_ips = ip;
        self
    }

    /// Adds allowed IP.
    ///
    /// [Wireguard Docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#allowedips)
    pub fn add_allowed_ip(mut self, ip: Ipv4Net) -> PeerBuilder {
        self.allowed_ips.push(ip);
        self
    }

    /// Sets private key.
    ///
    /// # Note
    ///
    /// If you set private key (instead of public key), you can generate [`Interface`] from [`Peer`] (see [`Peer::to_interface()`]).
    ///
    /// [Wireguard Docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#privatekey)
    pub fn private_key(mut self, private_key: PrivateKey) -> PeerBuilder {
        self.key = Some(Either::Left(private_key));
        self
    }

    /// Sets public key.
    ///
    /// [Wireguard Docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#publickey)
    pub fn public_key(mut self, public_key: PublicKey) -> PeerBuilder {
        self.key = Some(Either::Right(public_key));
        self
    }

    /// Sets AmneziaWG obfuscation values.
    ///
    /// [AmneziaWG Docs](https://github.com/amnezia-vpn/amneziawg-linux-kernel-module?tab=readme-ov-file#configuration)
    #[cfg(feature = "amneziawg")]
    #[cfg_attr(docsrs, doc(cfg(feature = "amneziawg")))]
    pub fn amnezia_settings(mut self, amnezia_settings: AmneziaSettings) -> Self {
        self.amnezia_settings = Some(amnezia_settings);
        self
    }

    /// Creates [`Peer`].
    pub fn build(self) -> Peer {
        let key = self
            .key
            .unwrap_or_else(|| Either::Left(PrivateKey::random()));

        Peer {
            endpoint: self.endpoint,
            allowed_ips: self.allowed_ips,
            key,

            #[cfg(feature = "amneziawg")]
            amnezia_settings: self.amnezia_settings,
        }
    }
}
