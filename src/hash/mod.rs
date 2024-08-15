//!
//! The hash.
//!

pub mod r#type;

///
/// The hash.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Hash {
    /// The `keccak256`` hash.
    Keccak256([u8; crate::BYTE_LENGTH_FIELD]),
    /// The `ipfs`` hash.
    Ipfs([u8; crate::BYTE_LENGTH_FIELD]),
}

impl Hash {
    ///
    /// Computes the `keccak256` hash for `preimage`.
    ///
    pub fn keccak256(preimage: &[u8]) -> Self {
        use sha3::Digest;

        let hash_bytes = sha3::Keccak256::digest(preimage);
        Self::Keccak256(hash_bytes.into())
    }

    ///
    /// Formats a hash array to a hexadecimal string.
    ///
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Keccak256(bytes) => bytes.as_slice(),
            Self::Ipfs(bytes) => bytes.as_slice(),
        }
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let bytes = match self {
            Self::Keccak256(bytes) => bytes.as_slice(),
            Self::Ipfs(bytes) => bytes.as_slice(),
        };
        write!(f, "0x{}", hex::encode(bytes))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn keccak256() {
        assert_eq!(
            super::Hash::keccak256("zksync".as_bytes()).to_string(),
            "0x0238fb1ab06c28c32885f9a4842207ac480c2467df26b6c58e201679628c5a5b"
        );
    }
}
