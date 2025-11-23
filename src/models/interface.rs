use derive_builder::Builder;
use either::Either;
use ipnet::IpNet;
use itertools::Itertools as _;

use std::fmt;
use std::net::Ipv4Addr;
use std::{convert::Infallible, net::IpAddr};

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::prelude::*;

/// Controls the routing table to which routes are added.
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub enum Table {
    /// Routing table
    RoutingTable(usize),

    /// Disables the creation of routes altogether
    Off,

    /// Adds routes to the default table and enables special handling of default routes.
    #[default]
    Auto,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Table::RoutingTable(n) => write!(f, "{n}"),
            Table::Off => write!(f, "off"),
            Table::Auto => write!(f, "auto"),
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for Table {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Table::RoutingTable(n) => serializer.serialize_u64(*n as u64),
            Table::Off => serializer.serialize_str("off"),
            Table::Auto => serializer.serialize_str("auto"),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Table {
    fn deserialize<D>(deserializer: D) -> Result<Table, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TableVisitor;
        impl de::Visitor<'_> for TableVisitor {
            type Value = Table;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an routing table value (number, off or auto)")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "off" => Ok(Table::Off),
                    "auto" => Ok(Table::Auto),
                    _ => Err(E::invalid_value(de::Unexpected::Str(value), &self)),
                }
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Table::RoutingTable(
                    usize::try_from(value).map_err(E::custom)?,
                ))
            }
        }

        deserializer.deserialize_any(TableVisitor)
    }
}

/// Struct, that represents complete configuration (contains both `[Interface]` and `[Peer]`
/// sections).
///
/// Use [`InterfaceBuilder`] to create interface.
///
/// [Wireguard docs](https://github.com/pirate/wireguard-docs#interface)
#[must_use]
#[derive(Clone, Debug, PartialEq, Builder)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[builder(build_fn(private, name = "fallible_build", error = "Infallible"))]
pub struct Interface {
    /// Interface's address.
    ///
    /// `/32` and `/128` IP networks will be generated as regular ips (f.e. `1.2.3.4/32` -> `1.2.3.4`)
    ///
    /// You can also use [`InterfaceBuilder::add_network()`] to add a single network and
    /// [`InterfaceBuilder::add_address()`] to add a single address.
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs#address)
    #[builder(
        setter(into),
        default = "vec![IpNet::new_assert(Ipv4Addr::UNSPECIFIED.into(), 0)]"
    )]
    pub address: Vec<IpNet>,

    /// Port to listen for incoming VPN connections.
    ///
    /// [Wireguard conf](https://github.com/pirate/wireguard-docs#listenport)
    #[builder(setter(strip_option), default)]
    pub listen_port: Option<u16>,

    /// Node's private key.
    ///
    /// [Wireguard conf](https://github.com/pirate/wireguard-docs#privatekey)
    #[builder(default = "PrivateKey::random()")]
    pub private_key: PrivateKey,

    /// The DNS servers to announce to VPN clients via DHCP.
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs#dns-2)
    #[builder(setter(into, strip_option), default)]
    pub dns: Vec<String>,

    /// Endpoint.
    ///
    /// - `[Interface]` section will have `# Name = <endpoint>` comment at the top.
    /// - Exported [`Peer`] (via [`Interface::to_peer`]) will have this endpoint.
    ///
    /// [Wireguard Docs for `# Name`](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#-name-1);
    /// [Wireguard Docs for endpoint](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#endpoint)
    #[builder(setter(into, strip_option), default)]
    pub endpoint: Option<String>,

    /// Routing table to use for the WireGuard routes.
    ///
    /// See [`Table`] for special values.
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#table)
    #[builder(setter(strip_option), default)]
    pub table: Option<Table>,

    /// Maximum Transmission Unit (MTU, aka packet/frame size) to use when connecting to the peer.
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs?tab=readme-ov-file#mtu)
    #[builder(setter(strip_option), default)]
    pub mtu: Option<usize>,

    /// AmneziaWG obfuscation values.
    ///
    /// [AmneziaWG Docs](https://github.com/amnezia-vpn/amneziawg-linux-kernel-module?tab=readme-ov-file#configuration)
    #[cfg(feature = "amneziawg")]
    #[cfg_attr(docsrs, doc(cfg(feature = "amneziawg")))]
    #[builder(setter(strip_option), default)]
    pub amnezia_settings: Option<AmneziaSettings>,

    /// Commands, that will be executed before the interface is brought up
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs#preup)
    #[builder(setter(into), default)]
    pub pre_up: Vec<String>,

    /// Commands, that will be executed before the interface is brought down
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs#predown)
    #[builder(setter(into), default)]
    pub pre_down: Vec<String>,

    /// Commands, that will be executed after the interface is brought up
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs#postup)
    #[builder(setter(into), default)]
    pub post_up: Vec<String>,

    /// Commands, that will be executed after the interface is brought down
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs#postdown)
    #[builder(setter(into), default)]
    pub post_down: Vec<String>,

    /// Peers.
    ///
    /// Create them using [`PeerBuilder`] or [`Interface::to_peer`] method.
    ///
    /// [Wireguard docs](https://github.com/pirate/wireguard-docs#peer)
    #[builder(setter(into), default)]
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
    ///     .peers([server.to_peer()]) // convert `Interface` to `Peer` using `.to_peer()` method.
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
            allowed_ips: self.address.clone(),
            key: Either::Left(self.private_key.clone()),
            preshared_key: None,
            persistent_keepalive: 0,
        }
    }
}

impl Interface {
    /// Create new `InterfaceBuilder`. Alias for `InterfaceBuilder::new()`.
    ///
    /// ```rust
    /// # use wireguard_conf::prelude::*;
    /// # use wireguard_conf::as_ipnet;
    /// #
    /// let interface = Interface::builder()
    ///     .address([as_ipnet!("10.0.0.1/24")])
    ///     // <snip>
    ///     .build();
    /// ```
    #[must_use]
    pub fn builder() -> InterfaceBuilder {
        InterfaceBuilder::default()
    }
}

impl InterfaceBuilder {
    /// Create new `InterfaceBuilder`.
    ///
    /// ```rust
    /// # use wireguard_conf::prelude::*;
    /// # use wireguard_conf::as_ipnet;
    /// #
    /// let interface = InterfaceBuilder::new()
    ///     .address([as_ipnet!("10.0.0.1/24")])
    ///     // <snip>
    ///     .build();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds IP Network to `Address = ...` field.
    ///
    /// `value` is [`Into<IpNet>`], which means that it can be either [`ipnet::IpNet`] or [`std::net::IpAddr`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use wireguard_conf::{as_ipnet, prelude::*};
    ///
    /// let interface = InterfaceBuilder::new()
    ///     .add_network(as_ipnet!("1.2.3.4/16"))
    ///     .add_network(as_ipnet!("fd00:DEAD:BEEF::1/48"))
    ///     .build();
    ///
    /// assert_eq!(
    ///     interface.address,
    ///     vec![
    ///         as_ipnet!("1.2.3.4/16"),
    ///         as_ipnet!("fd00:DEAD:BEEF::1/48")
    ///     ]
    /// );
    /// ```
    pub fn add_network<T: Into<IpNet>>(&mut self, value: T) -> &mut Self {
        if self.address.is_none() {
            self.address = Some(Vec::with_capacity(1));
        }

        self.address
            .as_mut()
            .unwrap_or_else(|| unreachable!())
            .push(value.into());
        self
    }

    /// Adds IP address to `Address = ...` field.
    ///
    /// `value` is [`Into<IpAddr>`], which means that it can be either [`std::net::Ipv4Addr`] or [`std::net::Ipv6Addr`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use wireguard_conf::{as_ipaddr, as_ipnet, prelude::*};
    ///
    /// let interface = InterfaceBuilder::new()
    ///     .add_address(as_ipaddr!("1.2.3.4"))
    ///     .add_address(as_ipaddr!("fd00::1"))
    ///     .build();
    ///
    /// // /32 and /128 are added automatically
    /// assert_eq!(
    ///     interface.address,
    ///     vec![
    ///         as_ipnet!("1.2.3.4/32"),
    ///         as_ipnet!("fd00::1/128"),
    ///     ]
    /// );
    /// ```
    pub fn add_address<T: Into<IpAddr>>(&mut self, value: T) -> &mut Self {
        if self.address.is_none() {
            self.address = Some(Vec::with_capacity(1));
        }

        let ip_addr = value.into();
        let ip_net = if ip_addr.is_ipv4() {
            IpNet::new_assert(ip_addr, 32) // 1.2.3.4/32
        } else {
            IpNet::new_assert(ip_addr, 128) // fd00::1/128
        };

        self.address
            .as_mut()
            .unwrap_or_else(|| unreachable!())
            .push(ip_net);
        self
    }

    /// Builds an `Interface`.
    pub fn build(&self) -> Interface {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl fmt::Display for Interface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[Interface]")?;
        if let Some(endpoint) = &self.endpoint {
            writeln!(f, "# Name = {endpoint}")?;
        }
        writeln!(
            f,
            "Address = {}",
            self.address
                .iter()
                .map(ToString::to_string)
                .map(|addr| {
                    if addr.ends_with("/32") {
                        addr.trim_end_matches("/32").to_owned()
                    } else if addr.ends_with("/128") {
                        addr.trim_end_matches("/128").to_owned()
                    } else {
                        addr
                    }
                })
                .join(",")
        )?;
        if let Some(listen_port) = self.listen_port {
            writeln!(f, "ListenPort = {listen_port}")?;
        }
        writeln!(f, "PrivateKey = {}", self.private_key)?;
        if !self.dns.is_empty() {
            writeln!(f, "DNS = {}", self.dns.join(","))?;
        }
        if let Some(table) = &self.table {
            writeln!(f, "Table = {table}")?;
        }
        if let Some(mtu) = &self.mtu {
            writeln!(f, "MTU = {mtu}")?;
        }

        if !self.pre_up.is_empty() {
            writeln!(f)?;
            for snippet in &self.pre_up {
                writeln!(f, "PreUp = {snippet}")?;
            }
        }
        if !self.pre_down.is_empty() {
            writeln!(f)?;
            for snippet in &self.pre_down {
                writeln!(f, "PreDown = {snippet}")?;
            }
        }
        if !self.post_up.is_empty() {
            writeln!(f)?;
            for snippet in &self.post_up {
                writeln!(f, "PostUp = {snippet}")?;
            }
        }
        if !self.post_down.is_empty() {
            writeln!(f)?;
            for snippet in &self.post_down {
                writeln!(f, "PostDown = {snippet}")?;
            }
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
