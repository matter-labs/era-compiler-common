//!
//! Hashing utilities.
//!

pub mod r#type;

use base58::FromBase58;
use sha3::digest::FixedOutput;
use sha3::Digest;

///
/// Hash types supported so far.
///
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Hash {
    /// The `keccak256` hash.
    Keccak256 {
        /// The byte representation.
        bytes: [u8; crate::BYTE_LENGTH_FIELD],
        /// The string representation.
        string: String,
    },
    /// The `ipfs` hash.
    Ipfs {
        /// The byte representation.
        #[serde(with = "serde_arrays")]
        bytes: [u8; 2 + crate::BYTE_LENGTH_FIELD],
        /// The string representation.
        string: String,
    },
}

impl Hash {
    ///
    /// Computes the `keccak256` hash for `preimage`.
    ///
    pub fn keccak256(preimage: &[u8]) -> Self {
        let bytes = sha3::Keccak256::digest(preimage).into();
        let string = format!("0x{}", hex::encode(bytes));
        Self::Keccak256 { bytes, string }
    }

    ///
    /// Computes the `keccak256` hash for an array of `preimages`.
    ///
    pub fn keccak256_multiple<R: AsRef<[u8]>>(preimages: &[R]) -> Self {
        let mut hasher = sha3::Keccak256::new();
        for preimage in preimages.iter() {
            hasher.update(preimage);
        }
        let bytes: [u8; crate::BYTE_LENGTH_FIELD] = hasher.finalize_fixed().into();
        let string = format!("0x{}", hex::encode(bytes));
        Self::Keccak256 { bytes, string }
    }

    ///
    /// Computes the `ipfs` hash for `preimage`.
    ///
    pub fn ipfs(preimage: &[u8]) -> Self {
        let hasher = ipfs_hasher::IpfsHasher::default();
        let string = hasher.compute(preimage);
        let bytes = string
            .from_base58()
            .expect("Base58 conversion is always valid")
            .try_into()
            .expect("The size is always correct");
        Self::Ipfs { bytes, string }
    }

    ///
    /// Formats a hash array to a hexadecimal string.
    ///
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Keccak256 { bytes, .. } => bytes.as_slice(),
            Self::Ipfs { bytes, .. } => bytes.as_slice(),
        }
    }

    ///
    /// Converts the hash into a CBOR-encoded byte array.
    ///
    pub fn as_cbor_bytes(&self) -> Vec<u8> {
        let data = match self {
            Self::Ipfs { bytes, .. } => bytes.as_slice(),
            Self::Keccak256 { .. } => panic!("Only IPFS hash can be CBOR-encoded"),
        };

        let mut cbor = Vec::with_capacity(64);
        cbor.extend(hex::decode("a1646970667358").expect("Always valid"));
        cbor.push(data.len() as u8);
        cbor.extend_from_slice(data);
        cbor.extend((cbor.len() as u16).to_be_bytes());
        cbor
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Keccak256 { string, .. } => write!(f, "{string}"),
            Self::Ipfs { string, .. } => write!(f, "{string}"),
        }
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

    #[test]
    fn keccak256_multiple() {
        assert_eq!(
            super::Hash::keccak256_multiple(&[
                "zksync".as_bytes(),
                "the".as_bytes(),
                "best".as_bytes()
            ])
            .to_string(),
            "0x30277e6e189b3fa474437d451ccbb2409c3b67fda53c6ad5df3f8f0f3873ff6b"
        );
    }

    #[test]
    fn ipfs() {
        assert_eq!(
            super::Hash::ipfs("zksync".as_bytes()).to_string(),
            "QmZNRNrU4GknvaB3kMuH2tMjw2ji9fCmeVA9R6yPboiF4J"
        );
    }

    #[test]
    fn ipfs_hex() {
        assert_eq!(
            hex::encode(super::Hash::ipfs("zksync".as_bytes()).as_bytes()),
            "1220a3e4a4b11362b17e7294afb3048e97a6cb024209f236d7388ef55476192413bf"
        );
    }

    #[test]
    fn ipfs_cbor() {
        assert_eq!(
            hex::encode(super::Hash::ipfs("zksync".as_bytes()).as_cbor_bytes()),
            "a1646970667358221220a3e4a4b11362b17e7294afb3048e97a6cb024209f236d7388ef55476192413bf002a"
        );
    }
}
