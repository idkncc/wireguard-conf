use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use super::{PresharedKey, PrivateKey, PublicKey};

impl Serialize for PrivateKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_string())
        } else {
            serializer.serialize_bytes(self.as_bytes())
        }
    }
}

impl<'de> Deserialize<'de> for PrivateKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let data = String::deserialize(deserializer)?;

            PrivateKey::try_from(data.as_str())
                .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(&data), &"a private key"))
        } else {
            let bytes = <[u8; 32]>::deserialize(deserializer)?;

            Ok(PrivateKey::from(bytes))
        }
    }
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_string())
        } else {
            serializer.serialize_bytes(self.as_bytes())
        }
    }
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let data = String::deserialize(deserializer)?;

            PublicKey::try_from(data.as_str())
                .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(&data), &"a public key"))
        } else {
            let bytes = <[u8; 32]>::deserialize(deserializer)?;

            Ok(PublicKey::from(bytes))
        }
    }
}

impl Serialize for PresharedKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_string())
        } else {
            serializer.serialize_bytes(self.as_bytes())
        }
    }
}

impl<'de> Deserialize<'de> for PresharedKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let data = String::deserialize(deserializer)?;

            PresharedKey::try_from(data.as_str()).map_err(|_| {
                de::Error::invalid_value(de::Unexpected::Str(&data), &"a preshared key")
            })
        } else {
            let bytes = <[u8; 32]>::deserialize(deserializer)?;

            Ok(PresharedKey::from(bytes))
        }
    }
}
