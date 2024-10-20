use sha2::Digest;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Calculates a hash of a bytes slice.
pub fn hash(data: &[u8]) -> CryptoHash {
    CryptoHash::hash_bytes(data)
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, derive_more::AsRef, derive_more::AsMut)]
#[as_ref(forward)]
#[as_mut(forward)]
pub struct CryptoHash(pub [u8; 32]);

impl CryptoHash {
    pub const LENGTH: usize = 32;

    pub const fn new() -> Self {
        Self([0; Self::LENGTH])
    }

    /// Calculates hash of given bytes.
    pub fn hash_bytes(bytes: &[u8]) -> CryptoHash {
        CryptoHash(sha2::Sha256::digest(bytes).into())
    }

    /// Converts hash into base58-encoded string and passes it to given visitor.
    ///
    /// The conversion is performed without any memory allocation.  The visitor
    /// is given a reference to a string stored on stack.  Returns whatever the
    /// visitor returns.
    fn to_base58_impl<Out>(self, visitor: impl FnOnce(&str) -> Out) -> Out {
        // base58-encoded string is at most 1.4 times longer than the binary
        // sequence.  We’re serialising 32 bytes so ⌈32 * 1.4⌉ = 45 should be
        // enough.
        let mut buffer = [0u8; 45];
        let len = bs58::encode(self).into(&mut buffer[..]).unwrap();
        let value = std::str::from_utf8(&buffer[..len]).unwrap();
        visitor(value)
    }
}

impl Default for CryptoHash {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for CryptoHash {
    fn fmt(&self, fmtr: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, fmtr)
    }
}

impl Hash for CryptoHash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.as_ref());
    }
}

impl fmt::Display for CryptoHash {
    fn fmt(&self, fmtr: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_base58_impl(|encoded| fmtr.write_str(encoded))
    }
}
