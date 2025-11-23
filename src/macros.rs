/// Get expression as [`ipnet::IpNet`]
///
/// Shorthand for `expr.parse::<IpNet>().unwrap()`
///
/// # Examples
///
/// ```rust
/// use wireguard_conf::as_ipnet;
/// use ipnet::IpNet;
///
/// # fn main() {
/// assert_eq!(as_ipnet!("1.2.3.4/24"), "1.2.3.4/24".parse().unwrap());
/// assert_eq!(as_ipnet!("fd00:DEAD:BEEF::1/24"), "fd00:DEAD:BEEF::1/24".parse().unwrap());
/// # }
/// ```
#[macro_export]
macro_rules! as_ipnet {
    ($x:expr) => {
        $x.parse::<$crate::ipnet::IpNet>().unwrap()
    };
}

/// Get expression as [`std::net::IpAddr`]
///
/// Shorthand for `expr.parse::<IpAddr>().unwrap()`
///
/// # Examples
///
/// ```rust
/// use wireguard_conf::as_ipaddr;
/// use std::net::IpAddr;
///
/// # fn main() {
/// assert_eq!(as_ipaddr!("1.2.3.4"), "1.2.3.4".parse::<IpAddr>().unwrap());
/// assert_eq!(as_ipaddr!("fd00:DEAD:BEEF::1"), "fd00:DEAD:BEEF::1".parse::<IpAddr>().unwrap());
/// # }
/// ```
#[macro_export]
macro_rules! as_ipaddr {
    ($x:expr) => {
        $x.parse::<::std::net::IpAddr>().unwrap()
    };
}
