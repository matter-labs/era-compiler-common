//!
//! The contract code segment.
//!

///
/// The contract code segment.
///
/// On EraVM, the segments do not represent any entities in the final bytecode, but this separation is present
/// in IRs used for lowering.
///
/// On EVM, the segments represent deploy and runtime code segments without changes.
///
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub enum CodeSegment {
    /// The deploy code segment.
    Deploy,
    /// The runtime code segment.
    Runtime,
}

impl std::fmt::Display for CodeSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Deploy => write!(f, "deploy"),
            Self::Runtime => write!(f, "runtime"),
        }
    }
}
