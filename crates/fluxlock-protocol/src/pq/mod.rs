use pqcrypto_dilithium::dilithium2;
use pqcrypto_traits::sign::{PublicKey, SecretKey, DetachedSignature};

/// 🔐 Generate PQ keypair
pub fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
    let (pk, sk) = dilithium2::keypair();
    (pk.as_bytes().to_vec(), sk.as_bytes().to_vec())
}

/// 🔐 Sign message
pub fn sign(message: &[u8], secret_key: &[u8]) -> Vec<u8> {
    let sk = dilithium2::SecretKey::from_bytes(secret_key).unwrap();
    let sig = dilithium2::detached_sign(message, &sk);
    sig.as_bytes().to_vec()
}

/// 🔐 Verify signature
pub fn verify(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    let pk = match dilithium2::PublicKey::from_bytes(public_key) {
        Ok(pk) => pk,
        Err(_) => return false,
    };

    let sig = match dilithium2::DetachedSignature::from_bytes(signature) {
        Ok(sig) => sig,
        Err(_) => return false,
    };

    dilithium2::verify_detached_signature(&sig, message, &pk).is_ok()
}