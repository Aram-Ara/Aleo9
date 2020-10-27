use crate::authentication::Signature;
use setup_utils::calculate_hash;

/// An dummy authentication for contributions.
pub struct Dummy;

impl Signature for Dummy {
    /// Returns the name of the signature scheme.
    fn name(&self) -> String {
        Self::warning();
        "DummySignatureScheme".to_string()
    }

    /// Returns `true` if the signature scheme is safe for use in production.
    fn is_secure(&self) -> bool {
        Self::warning();
        false
    }

    /// Signs the given message using the given secret key,
    /// and returns the signature as a string.
    fn sign(&self, _secret_key: &str, message: &str) -> String {
        Self::warning();
        hex::encode(calculate_hash(message.as_bytes()))
    }

    /// Verifies the given signature for the given message and public key,
    /// and returns `true` if the signature is valid.
    fn verify(&self, _public_key: &str, message: &str, signature: &str) -> bool {
        Self::warning();
        signature == &hex::encode(calculate_hash(message.as_bytes()))
    }
}

impl Dummy {
    /// Outputs an explicit message that the signature scheme should not be used.
    fn warning() {
        #[cfg(not(test))]
        {
            tracing::trace!("A DUMMY SIGNATURE SCHEME METHOD IS CURRENTLY BEING USED");
            tracing::warn!("DUMMY SIGNATURE SCHEME IS NOT SAFE FOR USE IN PRODUCTION");
            tracing::error!("ATTEMPTING TO USE INSECURE SIGNATURE SCHEME IN CEREMONY");
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_dummy_signature() {
        let public_key = "public";
        let secret_key = "secret";
        let message = "message";

        let dummy = Dummy;

        let signature = dummy.sign(secret_key, message);
        assert_eq!(64, hex::decode(&signature).unwrap().len());
        assert_eq!(signature, dummy.sign(secret_key, message));

        let is_valid = dummy.verify(public_key, message, &signature);
        assert!(is_valid);
    }
}
