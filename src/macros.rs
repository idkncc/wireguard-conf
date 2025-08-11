/// Get expression as [`ipnet::Ipv4Net`]
///
/// Shorthand for `expr.parse::<Ipv4Net>().unwrap()`
///
/// # Examples
///
/// ```
/// use wireguard_conf::as_ipnet;
/// use ipnet::Ipv4Net;
///
/// assert_eq!(as_ipnet!("1.2.3.4/32"), "1.2.3.4/32".parse().unwrap())
/// ```
#[macro_export]
macro_rules! as_ipnet {
    ($x:expr) => {
        $x.parse::<::ipnet::Ipv4Net>().unwrap()
    };
}
