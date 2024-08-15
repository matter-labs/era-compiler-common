//!
//! The hash type.
//!

use std::str::FromStr;

///
/// The hash type.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Type {
    /// Do not include bytecode hash.
    #[serde(rename = "none")]
    None,
    /// The default keccak256 hash.
    #[serde(rename = "keccak256")]
    Keccak256,
}

impl FromStr for Type {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "none" => Ok(Self::None),
            "keccak256" => Ok(Self::Keccak256),
            string => anyhow::bail!("unknown bytecode hash mode: `{string}`"),
        }
    }
}
