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
/// assert_eq!(as_ipnet!("fd00::/24"), "fd00::/24".parse().unwrap());
/// # }
/// ```
#[macro_export]
macro_rules! as_ipnet {
    ($x:expr) => {
        $x.parse::<::ipnet::IpNet>().unwrap()
    };
}
