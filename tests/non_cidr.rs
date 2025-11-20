use indoc::formatdoc;
use wireguard_conf::{as_ipaddr, as_ipnet, prelude::*};

#[test]
fn non_cidr() {
    // you can use either `IpNet(<ipv4>/32)`/`IpNet(<ipv6>/128)` or just `IpAddr(<addr>)`
    let interface = InterfaceBuilder::new()
        .add_network(as_ipnet!("10.0.0.1/32"))
        .add_address(as_ipaddr!("1.2.3.4"))
        .add_network(as_ipnet!("fd00:1234:5678::1/128"))
        .add_address(as_ipaddr!("fd00:DEAD:BEEF::1"))
        .build();

    let interface_privatekey = interface.private_key.clone();
    let interface_conf = interface.to_string();
    assert_eq!(
        interface_conf,
        formatdoc! {"
            [Interface]
            Address = 10.0.0.1,1.2.3.4,fd00:1234:5678::1,fd00:dead:beef::1
            PrivateKey = {interface_privatekey}
        "}
    );
}
