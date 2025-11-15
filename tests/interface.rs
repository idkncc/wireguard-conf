use wireguard_conf::as_ipnet;
use wireguard_conf::prelude::*;

#[test]
fn empty_interface() {
    let interface = InterfaceBuilder::new().build();

    let private_key = interface.private_key.clone(); // only private key is generated

    assert_eq!(
        interface,
        Interface {
            address: as_ipnet!("0.0.0.0/0"),
            listen_port: None,
            private_key,
            dns: vec![],
            endpoint: None,
            table: None,
            mtu: None,

            #[cfg(feature = "amneziawg")]
            amnezia_settings: None,

            pre_up: vec![],
            pre_down: vec![],
            post_up: vec![],
            post_down: vec![],
            peers: vec![],
        }
    );
}

#[test]
fn address() {
    let interface = InterfaceBuilder::new()
        .address(as_ipnet!("1.2.3.4/16"))
        .build();

    assert_eq!(interface.address, as_ipnet!("1.2.3.4/16"));
}

#[test]
fn listen_port() {
    let interface = InterfaceBuilder::new().listen_port(12345).build();

    assert_eq!(interface.listen_port, Some(12345));
}

#[test]
fn private_key() {
    let private_key = PrivateKey::random();

    let interface = InterfaceBuilder::new()
        .private_key(private_key.clone())
        .build();

    assert_eq!(interface.private_key, private_key);
}

#[test]
fn dns() {
    let dns = ["1.1.1.1".to_string(), "1.0.0.1".to_string()];

    let interface = InterfaceBuilder::new().dns(&dns).build();

    assert_eq!(interface.dns, dns.to_vec());
}

#[test]
fn endpoint() {
    let endpoint = "endpoint.example.com";

    let interface = InterfaceBuilder::new().endpoint(endpoint).build();

    assert_eq!(interface.endpoint, Some(endpoint.to_string()));
}

#[test]
fn table() {
    let interface_off = InterfaceBuilder::new().table(Table::Off).build();
    let interface_auto = InterfaceBuilder::new().table(Table::Auto).build();
    let interface_routing = InterfaceBuilder::new()
        .table(Table::RoutingTable(12345))
        .build();

    assert_eq!(interface_off.table, Some(Table::Off));
    assert_eq!(interface_auto.table, Some(Table::Auto));
    assert_eq!(interface_routing.table, Some(Table::RoutingTable(12345)));
}

#[test]
fn mtu() {
    let mtu = 1420;

    let interface = InterfaceBuilder::new().mtu(1420).build();

    assert_eq!(interface.mtu, Some(mtu));
}

#[cfg(feature = "amneziawg")]
#[test]
fn amnezia_settings() {
    let amnezia_settings = AmneziaSettings::random();

    let interface = InterfaceBuilder::new()
        .amnezia_settings(amnezia_settings.clone())
        .build();

    assert_eq!(interface.amnezia_settings, Some(amnezia_settings));
}

#[test]
fn pre_up() {
    let pre_up = [
        "echo Im the pre_up script".to_string(),
        "echo hello".to_string(),
    ];

    let interface = InterfaceBuilder::new().pre_up(&pre_up).build();

    assert_eq!(interface.pre_up, pre_up.to_vec());
}

#[test]
fn pre_down() {
    let pre_down = [
        "echo Im the pre_down script".to_string(),
        "echo hello".to_string(),
    ];

    let interface = InterfaceBuilder::new().pre_down(&pre_down).build();

    assert_eq!(interface.pre_down, pre_down.to_vec());
}

#[test]
fn post_up() {
    let post_up = [
        "echo Im the post_up script".to_string(),
        "echo hello".to_string(),
    ];

    let interface = InterfaceBuilder::new().post_up(&post_up).build();

    assert_eq!(interface.post_up, post_up.to_vec());
}

#[test]
fn post_down() {
    let post_down = [
        "echo Im the post_down script".to_string(),
        "echo hello".to_string(),
    ];

    let interface = InterfaceBuilder::new().post_down(&post_down).build();

    assert_eq!(interface.post_down, post_down.to_vec());
}

#[test]
fn peers() {
    let peer1 = PeerBuilder::new().build();
    let peer2 = PeerBuilder::new().build();
    let peers = [peer1, peer2];

    let interface = InterfaceBuilder::new().peers(&peers).build();

    assert_eq!(interface.peers, peers.to_vec());
}

// TODO: amnezia feature
