use core::fmt;

use base64::prelude::*;
use x25519_dalek::{PublicKey as XPublicKey, StaticSecret};

use crate::WireguardError;

/// Private key
///
/// Wrapper around [`x25519_dalek::StaticSecret`] with some traits.
///
/// # Implements
///
/// - Implements [`fmt::Display`] for exporting key in Wireguard's format.
/// - Implements [`fmt::Debug`].
#[derive(Clone)]
pub struct PrivateKey {
    secret: StaticSecret,
}

impl PrivateKey {
    /// Generate new random [`PrivateKey`]
    #[must_use]
    pub fn random() -> PrivateKey {
        Self {
            secret: StaticSecret::random(),
        }
    }
}

impl fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PrivateKey")
            .field("secret", &self.to_string())
            .finish()
    }
}

impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", BASE64_STANDARD.encode(self.secret.to_bytes()))
    }
}

impl PartialEq for PrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.secret.as_bytes() == other.secret.as_bytes()
    }
}

impl TryFrom<String> for PrivateKey {
    type Error = WireguardError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bytes: [u8; 32] = BASE64_STANDARD
            .decode(value)
            .map_err(|_| WireguardError::InvalidPrivateKey)?
            .try_into()
            .map_err(|_| WireguardError::InvalidPrivateKey)?;

        Ok(Self {
            secret: StaticSecret::from(bytes),
        })
    }
}

/// Public key.
///
/// Wrapper around [`x25519_dalek::PublicKey`] with some traits.
///
/// # Implements
///
/// - Implements [`fmt::Display`] for exporting key in Wireguard's format.
/// - Implements [`fmt::Debug`].
/// - Implements [`From<&PrivateKey>`] for converting [`PrivateKey`] to [`PublicKey`].
#[derive(Clone, PartialEq)]
pub struct PublicKey {
    key: XPublicKey,
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PublicKey")
            .field("key", &self.to_string())
            .finish()
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", BASE64_STANDARD.encode(self.key.to_bytes()))
    }
}

impl TryFrom<String> for PublicKey {
    type Error = WireguardError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bytes: [u8; 32] = BASE64_STANDARD
            .decode(value)
            .map_err(|_| WireguardError::InvalidPublicKey)?
            .try_into()
            .map_err(|_| WireguardError::InvalidPublicKey)?;

        Ok(Self {
            key: XPublicKey::from(bytes),
        })
    }
}

impl From<&PrivateKey> for PublicKey {
    fn from(value: &PrivateKey) -> Self {
        Self {
            key: XPublicKey::from(&value.secret),
        }
    }
}
