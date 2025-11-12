use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

use super::{PresharedKey, PrivateKey, PublicKey};

impl Serialize for PrivateKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for PrivateKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PrivateKeyVisitor;

        impl<'de> de::Visitor<'de> for PrivateKeyVisitor {
            type Value = PrivateKey;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an private key")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                PrivateKey::try_from(s)
                    .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(s), &self))
            }
        }

        deserializer.deserialize_str(PrivateKeyVisitor)
    }
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PublicKeyVisitor;

        impl<'de> de::Visitor<'de> for PublicKeyVisitor {
            type Value = PublicKey;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an public key")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                PublicKey::try_from(s)
                    .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(s), &self))
            }
        }

        deserializer.deserialize_str(PublicKeyVisitor)
    }
}

impl Serialize for PresharedKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for PresharedKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PresharedKeyVisitor;

        impl<'de> de::Visitor<'de> for PresharedKeyVisitor {
            type Value = PresharedKey;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an preshared key")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                PresharedKey::try_from(s)
                    .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(s), &self))
            }
        }

        deserializer.deserialize_str(PresharedKeyVisitor)
    }
}
