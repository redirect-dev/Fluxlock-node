use std::ffi::c_void;

#[link(name = "oqs")]
extern "C" {
    fn OQS_SIG_new(method_name: *const u8) -> *mut c_void;
    fn OQS_SIG_free(sig: *mut c_void);

    fn OQS_SIG_keypair(
        sig: *mut c_void,
        public_key: *mut u8,
        secret_key: *mut u8,
    ) -> i32;

    fn OQS_SIG_sign(
        sig: *mut c_void,
        signature: *mut u8,
        signature_len: *mut usize,
        message: *const u8,
        message_len: usize,
        secret_key: *const u8,
    ) -> i32;
}

// IMPORTANT:
// liboqs 0.9.x uses "Dilithium3" (capital D, no underscore)
const DILITHIUM3: &[u8] = b"Dilithium3\0";

pub fn dilithium_keypair() -> (Vec<u8>, Vec<u8>) {
    unsafe {
        let sig = OQS_SIG_new(DILITHIUM3.as_ptr());
        assert!(!sig.is_null(), "OQS_SIG_new returned NULL");

        // Fixed sizes for Dilithium3 in liboqs 0.9.x
        let mut pubkey = vec![0u8; 1952];
        let mut secret = vec![0u8; 4000];

        let result = OQS_SIG_keypair(sig, pubkey.as_mut_ptr(), secret.as_mut_ptr());
        assert_eq!(result, 0, "Dilithium keypair generation failed");

        OQS_SIG_free(sig);

        (pubkey, secret)
    }
}

pub fn dilithium_sign(secret: &[u8], message: &[u8]) -> Vec<u8> {
    unsafe {
        let sig = OQS_SIG_new(DILITHIUM3.as_ptr());
        assert!(!sig.is_null(), "OQS_SIG_new returned NULL");

        let mut signature = vec![0u8; 3293];
        let mut sig_len: usize = 0;

        let result = OQS_SIG_sign(
            sig,
            signature.as_mut_ptr(),
            &mut sig_len,
            message.as_ptr(),
            message.len(),
            secret.as_ptr(),
        );

        assert_eq!(result, 0, "Dilithium signing failed");

        OQS_SIG_free(sig);

        signature.truncate(sig_len);
        signature
    }
}
