use wireguard_conf::prelude::*;

#[test]
pub fn private_key() {
    let private_key = PrivateKey::random();
    let private_key_str = private_key.to_string();

    assert_eq!(
        PrivateKey::try_from(private_key_str.as_str()),
        Ok(private_key.clone())
    );
    assert_eq!(
        PrivateKey::try_from(private_key_str.to_string()),
        Ok(private_key.clone())
    );
    assert_eq!(
        PrivateKey::try_from(private_key_str.to_string()),
        Ok(private_key.clone())
    );

    assert_eq!(
        PrivateKey::try_from("ThisWillBeErrored"),
        Err(WireguardError::InvalidPrivateKey)
    );
}

#[test]
pub fn public_key() {
    let private_key = PrivateKey::random();
    let public_key = PublicKey::from(&private_key);
    let public_key_str = public_key.to_string();

    assert_eq!(
        PublicKey::try_from(public_key_str.as_str()),
        Ok(public_key.clone())
    );
    assert_eq!(
        PublicKey::try_from(public_key_str.to_string()),
        Ok(public_key.clone())
    );
}

#[test]
pub fn preshared_key() {
    let preshared_key = PresharedKey::random();
    let preshared_key_str = preshared_key.to_string();

    assert_eq!(
        PresharedKey::try_from(preshared_key_str.as_str()),
        Ok(preshared_key.clone())
    );
    assert_eq!(
        PresharedKey::try_from(preshared_key_str.to_string()),
        Ok(preshared_key.clone())
    );
}
