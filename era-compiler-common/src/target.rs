//!
//! Compilation target.
//!

use std::str::FromStr;

///
/// Compilation target.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Target {
    /// The EraVM target.
    EraVM,
}

impl Target {
    ///
    /// Returns the LLVM target triple.
    ///
    pub fn triple(&self) -> &str {
        match self {
            Self::EraVM => "eravm-unknown-unknown",
        }
    }
}

impl FromStr for Target {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "eravm" => Ok(Self::EraVM),
            _ => Err(anyhow::anyhow!(
                "Unknown target `{}`. Supported targets: {}",
                string,
                vec![Self::EraVM]
                    .into_iter()
                    .map(|target| target.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )),
        }
    }
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::EraVM => write!(f, "eravm"),
        }
    }
}
