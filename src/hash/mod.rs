//!
//! The common hash utilities.
//!

pub mod ipfs;
pub mod keccak256;

pub use self::keccak256::keccak256;

///
/// Formats a hash array to a hexadecimal string.
///
pub fn format(hash: &[u8]) -> String {
    format!("0x{}", hex::encode(hash))
}
