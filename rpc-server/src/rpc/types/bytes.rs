
use std::fmt;
use rustc_hex::{ToHex, FromHex};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{Error, Visitor};

/// Wrapper structure around vector of bytes.
#[derive(PartialEq, Eq, Default, Hash, Clone)]
pub struct Bytes(pub Vec<u8>);

impl Bytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        Bytes(bytes)
    }

    pub fn to_vec(self) -> Vec<u8> {self.0}
}

impl From<Vec<u8>> for Bytes {
    fn from(v: Vec<u8>) -> Self {
        Bytes(v)
    }
}

impl Into<Vec<u8>> for Bytes {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.0.as_slice().to_hex::<String>();
        write!(f,"{}",s.as_str())
    }
}

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.0.as_slice().to_hex::<String>();
        write!(f,"{}",s.as_str())
    }
}


impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut serialized = "0x".to_owned();
        serialized.push_str(self.0.to_hex::<String>().as_ref());
        serializer.serialize_str(serialized.as_ref())
    }
}

impl<'a> Deserialize<'a> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Bytes, D::Error>
        where D: Deserializer<'a> {
        deserializer.deserialize_any(BytesVisitor)
    }
}

struct BytesVisitor;

impl<'a> Visitor<'a> for BytesVisitor {
    type Value = Bytes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a 0x-prefixed, hex-encoded vector of bytes")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: Error {
        if value.len() >= 2 && value.starts_with("0x") && value.len() & 1 == 0 {
            Ok(Bytes::new(FromHex::from_hex(&value[2..]).map_err(|e| Error::custom(format!("Invalid hex: {}", e)))?))
        } else {
            Err(Error::custom("Invalid bytes format. Expected a 0x-prefixed hex string with even length"))
        }
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> where E: Error {
        self.visit_str(value.as_ref())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rustc_hex::FromHex;

    #[test]
    fn test_bytes_serialize() {
        let bytes = Bytes("0123456789abcdef".from_hex().unwrap());
        let serialized = serde_json::to_string(&bytes).unwrap();
        assert_eq!(serialized, r#""0x0123456789abcdef""#);
    }
}