//!
//! The common `keccak256` utilities.
//!

///
/// Computes the `keccak256` hash for `preimage`.
///
pub fn keccak256(preimage: &[u8]) -> [u8; crate::BYTE_LENGTH_FIELD] {
    use sha3::Digest;

    let hash_bytes = sha3::Keccak256::digest(preimage);
    hash_bytes.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn keccak256() {
        assert_eq!(
            crate::hash::format(crate::hash::keccak256("zksync".as_bytes()).as_slice()),
            "0x0238fb1ab06c28c32885f9a4842207ac480c2467df26b6c58e201679628c5a5b"
        );
    }
}
