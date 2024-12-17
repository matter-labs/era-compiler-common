//!
//! Binary object format.
//!

use std::str::FromStr;

///
/// Binary object format.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ObjectFormat {
    /// ELF object format.
    ELF,
    /// Raw binary data.
    Raw,
}

impl FromStr for ObjectFormat {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "elf" => Ok(Self::ELF),
            "raw" => Ok(Self::Raw),
            _ => anyhow::bail!(
                "Unknown object format: {value}. Supported formats: {}",
                vec![Self::ELF, Self::Raw]
                    .into_iter()
                    .map(|format| format.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

impl std::fmt::Display for ObjectFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ELF => write!(f, "elf"),
            Self::Raw => write!(f, "raw"),
        }
    }
}
