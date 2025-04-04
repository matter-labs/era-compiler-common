//!
//! EraVM metadata hash type.
//!

use std::str::FromStr;

///
/// EraVM metadata  hash type.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Type {
    /// Do not include bytecode hash.
    #[serde(rename = "none")]
    None,
    /// The `ipfs` hash.
    #[serde(rename = "ipfs")]
    IPFS,
    /// The `keccak256`` hash type.
    #[serde(rename = "keccak256")]
    Keccak256,
}

impl FromStr for Type {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "none" => Ok(Self::None),
            "ipfs" => Ok(Self::IPFS),
            "keccak256" => Ok(Self::Keccak256),
            string => anyhow::bail!("unknown bytecode hash mode: `{string}`"),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::IPFS => write!(f, "ipfs"),
            Self::Keccak256 => write!(f, "keccak256"),
        }
    }
}
