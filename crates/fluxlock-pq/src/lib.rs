use std::ffi::{c_char, c_void};

#[repr(C)]
struct OqsSig {
    method_name: *const c_char,
    alg_version: *const c_char,
    claimed_nist_level: u8,
    euf_cma: bool,

    length_public_key: usize,
    length_secret_key: usize,
    length_signature: usize,

    keypair: *const c_void,
    sign: *const c_void,
    verify: *const c_void,
}

#[link(name = "oqs")]
extern "C" {
    fn OQS_SIG_new(method_name: *const c_char) -> *mut OqsSig;
    fn OQS_SIG_free(sig: *mut OqsSig);

    fn OQS_SIG_keypair(
        sig: *mut OqsSig,
        public_key: *mut u8,
        secret_key: *mut u8,
    ) -> i32;

    fn OQS_SIG_sign(
        sig: *mut OqsSig,
        signature: *mut u8,
        signature_len: *mut usize,
        message: *const u8,
        message_len: usize,
        secret_key: *const u8,
    ) -> i32;

    fn OQS_SIG_verify(
        sig: *mut OqsSig,
        message: *const u8,
        message_len: usize,
        signature: *const u8,
        signature_len: usize,
        public_key: *const u8,
    ) -> i32;
}

const DILITHIUM3: &[u8] = b"Dilithium3\0";

fn new_sig() -> *mut OqsSig {
    unsafe {
        let sig = OQS_SIG_new(DILITHIUM3.as_ptr() as *const c_char);
        if sig.is_null() {
            panic!("Failed to create OQS_SIG for dilithium_3");
        }
        sig
    }
}

pub fn dilithium_keypair() -> (Vec<u8>, Vec<u8>) {
    unsafe {
        let sig = new_sig();

        let pub_len = (*sig).length_public_key;
        let sec_len = (*sig).length_secret_key;

        let mut pubkey = vec![0u8; pub_len];
        let mut secret = vec![0u8; sec_len];

        let result = OQS_SIG_keypair(sig, pubkey.as_mut_ptr(), secret.as_mut_ptr());
        if result != 0 {
            OQS_SIG_free(sig);
            panic!("Dilithium keypair failed");
        }

        OQS_SIG_free(sig);
        (pubkey, secret)
    }
}

pub fn dilithium_sign(secret: &[u8], message: &[u8]) -> Vec<u8> {
    unsafe {
        let sig = new_sig();

        let sig_len_max = (*sig).length_signature;
        let mut signature = vec![0u8; sig_len_max];
        let mut sig_len: usize = 0;

        let result = OQS_SIG_sign(
            sig,
            signature.as_mut_ptr(),
            &mut sig_len,
            message.as_ptr(),
            message.len(),
            secret.as_ptr(),
        );

        if result != 0 {
            OQS_SIG_free(sig);
            panic!("Dilithium sign failed");
        }

        OQS_SIG_free(sig);

        signature.truncate(sig_len);
        signature
    }
}

pub fn dilithium_verify(public: &[u8], message: &[u8], signature: &[u8]) -> bool {
    unsafe {
        let sig = new_sig();

        let result = OQS_SIG_verify(
            sig,
            message.as_ptr(),
            message.len(),
            signature.as_ptr(),
            signature.len(),
            public.as_ptr(),
        );

        OQS_SIG_free(sig);

        result == 0
    }
}
