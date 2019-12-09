//! This crate provides wrappers around pure rust implementations of the crypto
//! primitives used in `noise-protocol`.
//
//! The underlying implementations are:
//
//! * [`x25519-dalek`](https://crates.io/crates/x25519-dalek)
//! * [`chacha20poly1305`](https://crates.io/crates/chacha20poly1305)
//! * [`aes-gcm`](https://crates.io/crates/aes-gcm)
//! * [`sha2`](https://crates.io/crates/sha2)
//! * [`blake2`](https://crates.io/crates/blake2)
//!
//! Warning relayed from `chacha20poly1305` and `aes-gcm`: they have not yet
//! received any formal cryptographic and security reviews. *USE AT YOUR OWN RISK*.

#![no_std]

#[cfg(any(feature = "use-chacha20poly1305", feature = "use-aes-256-gcm",))]
use aead::{Aead, NewAead};
#[cfg(any(feature = "use-blake2", feature = "use-sha2",))]
use digest::Digest;
use noise_protocol::*;
#[cfg(feature = "use-x25519")]
use x25519_dalek::{PublicKey, StaticSecret};

#[cfg(feature = "use-x25519")]
pub enum X25519 {}

#[cfg(feature = "use-x25519")]
impl DH for X25519 {
    type Key = [u8; 32];
    type Pubkey = [u8; 32];
    type Output = [u8; 32];

    fn name() -> &'static str {
        "25519"
    }

    fn genkey() -> Self::Key {
        // This does not work with the latest version of rand:
        //
        // StaticSecret::new(&mut OsRng)
        //
        // Because x25519-dalek is using an older version of rand_core.

        let mut k = [0u8; 32];
        getrandom::getrandom(&mut k).expect("getrandom failed");
        k[0] &= 248;
        k[31] &= 127;
        k[31] |= 64;
        k
    }

    fn pubkey(k: &Self::Key) -> Self::Pubkey {
        let static_secret = StaticSecret::from(*k);
        *PublicKey::from(&static_secret).as_bytes()
    }

    fn dh(k: &Self::Key, pk: &Self::Pubkey) -> Result<Self::Output, ()> {
        let k = StaticSecret::from(*k);
        let pk = PublicKey::from(*pk);
        Ok(*k.diffie_hellman(&pk).as_bytes())
    }
}

#[cfg(feature = "use-chacha20poly1305")]
pub enum ChaCha20Poly1305 {}

#[cfg(feature = "use-chacha20poly1305")]
impl Cipher for ChaCha20Poly1305 {
    fn name() -> &'static str {
        "ChaChaPoly"
    }

    type Key = [u8; 32];

    fn encrypt(k: &Self::Key, nonce: u64, ad: &[u8], plaintext: &[u8], out: &mut [u8]) {
        assert!(plaintext.len().checked_add(16) == Some(out.len()));

        let mut full_nonce = [0u8; 12];
        full_nonce[4..].copy_from_slice(&nonce.to_le_bytes());

        let (in_out, tag_out) = out.split_at_mut(plaintext.len());
        in_out.copy_from_slice(plaintext);

        let tag = chacha20poly1305::ChaCha20Poly1305::new((*k).into())
            .encrypt_in_place_detached(&full_nonce.into(), ad, in_out)
            .unwrap();

        tag_out.copy_from_slice(tag.as_ref())
    }

    fn decrypt(
        k: &Self::Key,
        nonce: u64,
        ad: &[u8],
        ciphertext: &[u8],
        out: &mut [u8],
    ) -> Result<(), ()> {
        assert!(ciphertext.len().checked_sub(16) == Some(out.len()));

        let mut full_nonce = [0u8; 12];
        full_nonce[4..].copy_from_slice(&nonce.to_le_bytes());

        out.copy_from_slice(&ciphertext[..out.len()]);
        let tag = &ciphertext[out.len()..];

        chacha20poly1305::ChaCha20Poly1305::new((*k).into())
            .decrypt_in_place_detached(&full_nonce.into(), ad, out, tag.into())
            .map_err(|_| ())
    }
}

#[cfg(feature = "use-aes-256-gcm")]
pub enum Aes256Gcm {}

#[cfg(feature = "use-aes-256-gcm")]
impl Cipher for Aes256Gcm {
    fn name() -> &'static str {
        "AESGCM"
    }

    type Key = [u8; 32];

    fn encrypt(k: &Self::Key, nonce: u64, ad: &[u8], plaintext: &[u8], out: &mut [u8]) {
        assert!(plaintext.len().checked_add(16) == Some(out.len()));

        let mut full_nonce = [0u8; 12];
        full_nonce[4..].copy_from_slice(&nonce.to_be_bytes());

        let (in_out, tag_out) = out.split_at_mut(plaintext.len());
        in_out.copy_from_slice(plaintext);

        let tag = aes_gcm::Aes256Gcm::new((*k).into())
            .encrypt_in_place_detached(&full_nonce.into(), ad, in_out)
            .unwrap();

        tag_out.copy_from_slice(tag.as_ref())
    }

    fn decrypt(
        k: &Self::Key,
        nonce: u64,
        ad: &[u8],
        ciphertext: &[u8],
        out: &mut [u8],
    ) -> Result<(), ()> {
        assert!(ciphertext.len().checked_sub(16) == Some(out.len()));

        let mut full_nonce = [0u8; 12];
        full_nonce[4..].copy_from_slice(&nonce.to_be_bytes());

        out.copy_from_slice(&ciphertext[..out.len()]);
        let tag = &ciphertext[out.len()..];

        aes_gcm::Aes256Gcm::new((*k).into())
            .decrypt_in_place_detached(&full_nonce.into(), ad, out, tag.into())
            .map_err(|_| ())
    }
}

#[cfg(feature = "use-sha2")]
#[derive(Default, Clone)]
pub struct Sha256(sha2::Sha256);

#[cfg(feature = "use-sha2")]
impl Hash for Sha256 {
    fn name() -> &'static str {
        "SHA256"
    }

    type Block = [u8; 64];
    type Output = [u8; 32];

    fn input(&mut self, data: &[u8]) {
        self.0.input(data);
    }

    fn result(&mut self) -> Self::Output {
        self.0.clone().result().into()
    }
}

#[cfg(feature = "use-sha2")]
#[derive(Default, Clone)]
pub struct Sha512(sha2::Sha512);

#[cfg(feature = "use-sha2")]
impl Hash for Sha512 {
    fn name() -> &'static str {
        "SHA512"
    }

    type Block = [u8; 128];
    type Output = [u8; 64];

    fn input(&mut self, data: &[u8]) {
        self.0.input(data);
    }

    fn result(&mut self) -> Self::Output {
        Self::Output::from_slice(self.0.clone().result().as_ref())
    }
}

#[cfg(feature = "use-blake2")]
#[derive(Default, Clone)]
pub struct Blake2s(blake2::Blake2s);

#[cfg(feature = "use-blake2")]
impl Hash for Blake2s {
    fn name() -> &'static str {
        "BLAKE2s"
    }

    type Block = [u8; 64];
    type Output = [u8; 32];

    fn input(&mut self, data: &[u8]) {
        self.0.input(data);
    }

    fn result(&mut self) -> Self::Output {
        self.0.clone().result().into()
    }
}

#[cfg(feature = "use-blake2")]
#[derive(Default, Clone)]
pub struct Blake2b(blake2::Blake2b);

#[cfg(feature = "use-blake2")]
impl Hash for Blake2b {
    fn name() -> &'static str {
        "BLAKE2b"
    }

    type Block = [u8; 128];
    type Output = [u8; 64];

    fn input(&mut self, data: &[u8]) {
        self.0.input(data);
    }

    fn result(&mut self) -> Self::Output {
        Self::Output::from_slice(self.0.clone().result().as_ref())
    }
}
