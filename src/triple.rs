use serde::{Deserialize, Deserializer, Serialize, Serializer};
use target_lexicon::Triple;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SerdeTriple(pub Triple);

impl<'de> Deserialize<'de> for SerdeTriple {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let triple = s.parse::<Triple>().map_err(serde::de::Error::custom)?;
        Ok(SerdeTriple(triple))
    }
}

impl Serialize for SerdeTriple {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}
