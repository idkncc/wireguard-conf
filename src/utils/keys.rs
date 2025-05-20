use core::fmt;

use base64::prelude::*;
use x25519_dalek::{PublicKey as XPublicKey, StaticSecret};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::WireguardError;

/// Private key
///
/// Wrapper around [`x25519_dalek::StaticSecret`] with some traits.
///
/// # Implements
///
/// - Implements [`Zeroize`] and [`ZeroizeOnDrop`] for clearing secrets from memory.
/// - Implements [`TryFrom<&str>`] or [`TryFrom<String>`] for importing key from Base64 format.
/// - Implements [`fmt::Display`] for exporting key in Wireguard's format.
/// - Implements [`fmt::Debug`].
///
/// # Examples
///
/// ```
/// # use wireguard_conf::prelude::*;
/// # fn main() -> WireguardResult<()> {
/// // generate new random key:
/// let key = PrivateKey::random();
///
/// // import key:
/// let imported_key = PrivateKey::try_from("sJkP2oorqrq49P6Ln25MWo3X04PxhB8k+RnJJnZ4gEo=")?;
///
/// // export key via `fmt::Display` trait:
/// let exported_key = imported_key.to_string();
///
/// assert_eq!(exported_key, "sJkP2oorqrq49P6Ln25MWo3X04PxhB8k+RnJJnZ4gEo=".to_string());
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct PrivateKey(StaticSecret);

impl PrivateKey {
    /// Generate new a random [`PrivateKey`]
    #[must_use]
    pub fn random() -> PrivateKey {
        Self(StaticSecret::random())
    }
}

impl PrivateKey {
    /// View private key as byte array.
    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 32] {
        self.0.as_bytes()
    }

    /// Convert private key to a byte array.
    #[inline]
    #[must_use]
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }
}

impl fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PrivateKey")
            .field(&self.to_string())
            .finish()
    }
}

/// Exporting key as base64 for Wireguard.
impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", BASE64_STANDARD.encode(self.as_bytes()))
    }
}

impl PartialEq for PrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl TryFrom<&str> for PrivateKey {
    type Error = WireguardError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let bytes: [u8; 32] = BASE64_STANDARD
            .decode(value)
            .map_err(|_| WireguardError::InvalidPrivateKey)?
            .try_into()
            .map_err(|_| WireguardError::InvalidPrivateKey)?;

        Ok(Self(StaticSecret::from(bytes)))
    }
}

impl TryFrom<String> for PrivateKey {
    type Error = WireguardError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

/// Public key.
///
/// Wrapper around [`x25519_dalek::PublicKey`] with some traits.
///
/// # Implements
///
/// - Implements [`Zeroize`] and [`ZeroizeOnDrop`] for clearing secrets from memory.
/// - Implements [`TryFrom<&str>`] or [`TryFrom<String>`] for importing key from Base64 format.
/// - Implements [`From<&PrivateKey>`] for converting [`PrivateKey`] to [`PublicKey`].
/// - Implements [`fmt::Display`] for exporting key in Wireguard's format.
/// - Implements [`fmt::Debug`].
///
/// # Examples
///
/// ```
/// # use wireguard_conf::prelude::*;
/// # fn main() -> WireguardResult<()> {
/// // generate new random key:
/// let private_key = PrivateKey::random();         // 1. generate private key
/// let public_key = PublicKey::from(&private_key); // 2. get public key via `From<&PrivateKey>`
///
/// // import key:
/// let imported_key = PublicKey::try_from("ijxpP+2xo+s77bfbm4QZzl6OyYP7sIOTutqngQSlZBs=")?;
///
/// // export key via `fmt::Display` trait:
/// let exported_key = imported_key.to_string();
///
/// assert_eq!(exported_key, "ijxpP+2xo+s77bfbm4QZzl6OyYP7sIOTutqngQSlZBs=".to_string());
/// # Ok(())
/// # }
/// ```
#[derive(Clone, PartialEq, Zeroize, ZeroizeOnDrop)]
pub struct PublicKey(XPublicKey);

impl PublicKey {
    /// Convert this public key to a byte array.
    #[inline]
    #[must_use]
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    /// View this public key as a byte array.
    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 32] {
        self.0.as_bytes()
    }
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PublicKey").field(&self.to_string()).finish()
    }
}

/// Export key in base64 format for Wireguard.
impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", BASE64_STANDARD.encode(self.as_bytes()))
    }
}

impl TryFrom<&str> for PublicKey {
    type Error = WireguardError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let bytes: [u8; 32] = BASE64_STANDARD
            .decode(value)
            .map_err(|_| WireguardError::InvalidPublicKey)?
            .try_into()
            .map_err(|_| WireguardError::InvalidPublicKey)?;

        Ok(Self(XPublicKey::from(bytes)))
    }
}

impl TryFrom<String> for PublicKey {
    type Error = WireguardError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl From<&PrivateKey> for PublicKey {
    fn from(value: &PrivateKey) -> Self {
        Self(XPublicKey::from(&value.0))
    }
}
