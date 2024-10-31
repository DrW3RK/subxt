// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! An ecdsa keypair implementation.
use codec::Encode;

use crate::crypto::{seed_from_entropy, DeriveJunction, SecretUri};
use core::{fmt::Display, str::FromStr};
use hex::FromHex;
use polkadot_sdk::sp_crypto_hashing;
use secp256k1::{ecdsa::RecoverableSignature, Message, Secp256k1, SecretKey};
use secrecy::ExposeSecret;

const SECRET_KEY_LENGTH: usize = 32;

/// Seed bytes used to generate a key pair.
pub type SecretKeyBytes = [u8; SECRET_KEY_LENGTH];

/// A signature generated by [`Keypair::sign()`]. These bytes are equivalent
/// to a Substrate `MultiSignature::Ecdsa(bytes)`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Signature(pub [u8; 65]);

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// The (compressed) public key for an [`Keypair`] key pair.
#[derive(Debug, Clone)]
pub struct PublicKey(pub [u8; 33]);

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// An ecdsa keypair implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keypair(pub secp256k1::Keypair);

impl Keypair {
    /// Create an ecdsa keypair from a [`SecretUri`]. See the [`SecretUri`] docs for more.
    ///
    /// # Example
    ///
    /// ```rust
    /// use subxt_signer::{ SecretUri, ecdsa::Keypair };
    /// use std::str::FromStr;
    ///
    /// let uri = SecretUri::from_str("//Alice").unwrap();
    /// let keypair = Keypair::from_uri(&uri).unwrap();
    ///
    /// keypair.sign(b"Hello world!");
    /// ```
    pub fn from_uri(uri: &SecretUri) -> Result<Self, Error> {
        let SecretUri {
            junctions,
            phrase,
            password,
        } = uri;

        // If the phrase is hex, convert bytes directly into a seed, ignoring password.
        // Else, parse the phrase string taking the password into account. This is
        // the same approach taken in sp_core::crypto::Pair::from_string_with_seed.
        let key = if let Some(hex_str) = phrase.expose_secret().strip_prefix("0x") {
            let seed = SecretKeyBytes::from_hex(hex_str)?;
            Self::from_secret_key(seed)?
        } else {
            let phrase = bip39::Mnemonic::from_str(phrase.expose_secret())?;
            let pass_str = password.as_ref().map(|p| p.expose_secret());
            Self::from_phrase(&phrase, pass_str)?
        };

        // Now, use any "junctions" to derive a new key from this root key.
        key.derive(junctions.iter().copied())
    }

    /// Create an ecdsa keypair from a BIP-39 mnemonic phrase and optional password.
    ///
    /// # Example
    ///
    /// ```rust
    /// use subxt_signer::{ bip39::Mnemonic, ecdsa::Keypair };
    ///
    /// let phrase = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
    /// let mnemonic = Mnemonic::parse(phrase).unwrap();
    /// let keypair = Keypair::from_phrase(&mnemonic, None).unwrap();
    ///
    /// keypair.sign(b"Hello world!");
    /// ```
    pub fn from_phrase(mnemonic: &bip39::Mnemonic, password: Option<&str>) -> Result<Self, Error> {
        let (arr, len) = mnemonic.to_entropy_array();
        let big_seed =
            seed_from_entropy(&arr[0..len], password.unwrap_or("")).ok_or(Error::InvalidSeed)?;

        let secret_key_bytes: SecretKeyBytes = big_seed[..SECRET_KEY_LENGTH]
            .try_into()
            .expect("should be valid Seed");

        Self::from_secret_key(secret_key_bytes)
    }

    /// Turn a 32 byte seed into a keypair.
    ///
    /// # Warning
    ///
    /// This will only be secure if the seed is secure!
    pub fn from_secret_key(secret_key: SecretKeyBytes) -> Result<Self, Error> {
        let secret = SecretKey::from_slice(&secret_key).map_err(|_| Error::InvalidSeed)?;
        Ok(Self(secp256k1::Keypair::from_secret_key(
            &Secp256k1::signing_only(),
            &secret,
        )))
    }

    /// Derive a child key from this one given a series of junctions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use subxt_signer::{ bip39::Mnemonic, ecdsa::Keypair, DeriveJunction };
    ///
    /// let phrase = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
    /// let mnemonic = Mnemonic::parse(phrase).unwrap();
    /// let keypair = Keypair::from_phrase(&mnemonic, None).unwrap();
    ///
    /// // Equivalent to the URI path '//Alice//stash':
    /// let new_keypair = keypair.derive([
    ///     DeriveJunction::hard("Alice"),
    ///     DeriveJunction::hard("stash")
    /// ]);
    /// ```
    pub fn derive<Js: IntoIterator<Item = DeriveJunction>>(
        &self,
        junctions: Js,
    ) -> Result<Self, Error> {
        let mut acc = self.0.secret_key().clone().secret_bytes();
        for junction in junctions {
            match junction {
                DeriveJunction::Soft(_) => return Err(Error::SoftJunction),
                DeriveJunction::Hard(junction_bytes) => {
                    acc = ("Secp256k1HDKD", acc, junction_bytes)
                        .using_encoded(sp_crypto_hashing::blake2_256)
                }
            }
        }
        Self::from_secret_key(acc)
    }

    /// Obtain the [`PublicKey`] part of this key pair, which can be used in calls to [`verify()`].
    /// or otherwise converted into an address. In case of ECDSA, the public key bytes are not
    /// equivalent to a Substrate `AccountId32`. They have to be hashed to obtain `AccountId32`.
    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.public_key().serialize())
    }

    /// Obtain the [`SecretKey`] part of this key pair. This should be kept secret.
    pub fn secret_key(&self) -> SecretKeyBytes {
        *self.0.secret_key().as_ref()
    }

    /// Sign some message. These bytes can be used directly in a Substrate `MultiSignature::Ecdsa(..)`.
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.sign_prehashed(&sp_crypto_hashing::blake2_256(message))
    }

    /// Signs a pre-hashed message.
    pub fn sign_prehashed(&self, message_hash: &[u8; 32]) -> Signature {
        let wrapped = Message::from_digest_slice(message_hash).expect("Message is 32 bytes; qed");
        Signature(internal::sign(&self.0.secret_key(), &wrapped))
    }
}

/// Verify that some signature for a message was created by the owner of the [`PublicKey`].
///
/// ```rust
/// use subxt_signer::{ bip39::Mnemonic, ecdsa };
///
/// let keypair = ecdsa::dev::alice();
/// let message = b"Hello!";
///
/// let signature = keypair.sign(message);
/// let public_key = keypair.public_key();
/// assert!(ecdsa::verify(&signature, message, &public_key));
/// ```
pub fn verify<M: AsRef<[u8]>>(sig: &Signature, message: M, pubkey: &PublicKey) -> bool {
    let message_hash = sp_crypto_hashing::blake2_256(message.as_ref());
    let wrapped = Message::from_digest_slice(&message_hash).expect("Message is 32 bytes; qed");

    internal::verify(&sig.0, &wrapped, pubkey)
}

pub(crate) mod internal {
    use super::*;

    pub fn sign(secret_key: &secp256k1::SecretKey, message: &Message) -> [u8; 65] {
        let recsig: RecoverableSignature =
            Secp256k1::signing_only().sign_ecdsa_recoverable(message, secret_key);
        let (recid, sig): (_, [u8; 64]) = recsig.serialize_compact();
        let mut signature_bytes: [u8; 65] = [0; 65];
        signature_bytes[..64].copy_from_slice(&sig);
        signature_bytes[64] = (i32::from(recid) & 0xFF) as u8;
        signature_bytes
    }

    pub fn verify(sig: &[u8; 65], message: &Message, pubkey: &PublicKey) -> bool {
        let Ok(signature) = secp256k1::ecdsa::Signature::from_compact(&sig[..64]) else {
            return false;
        };
        let Ok(public) = secp256k1::PublicKey::from_slice(&pubkey.0) else {
            return false;
        };

        Secp256k1::verification_only()
            .verify_ecdsa(message, &signature, &public)
            .is_ok()
    }
}

/// An error handed back if creating a keypair fails.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Invalid seed.
    InvalidSeed,
    /// Invalid seed.
    SoftJunction,
    /// Invalid phrase.
    Phrase(bip39::Error),
    /// Invalid hex.
    Hex(hex::FromHexError),
}
impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::InvalidSeed => write!(f, "Invalid seed (was it the wrong length?)"),
            Error::SoftJunction => write!(f, "Invalid seed for ECDSA, contained soft junction"),
            Error::Phrase(e) => write!(f, "Cannot parse phrase: {e}"),
            Error::Hex(e) => write!(f, "Cannot parse hex string: {e}"),
        }
    }
}

impl_from!(bip39::Error => Error::Phrase);
impl_from!(hex::FromHexError => Error::Hex);

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Dev accounts, helpful for testing but not to be used in production,
/// since the secret keys are known.
pub mod dev {
    use super::*;

    once_static_cloned! {
        /// Equivalent to `{DEV_PHRASE}//Alice`.
        pub fn alice() -> Keypair {
            Keypair::from_uri(&SecretUri::from_str("//Alice").unwrap()).unwrap()
        }
        /// Equivalent to `{DEV_PHRASE}//Bob`.
        pub fn bob() -> Keypair {
            Keypair::from_uri(&SecretUri::from_str("//Bob").unwrap()).unwrap()
        }
        /// Equivalent to `{DEV_PHRASE}//Charlie`.
        pub fn charlie() -> Keypair {
            Keypair::from_uri(&SecretUri::from_str("//Charlie").unwrap()).unwrap()
        }
        /// Equivalent to `{DEV_PHRASE}//Dave`.
        pub fn dave() -> Keypair {
            Keypair::from_uri(&SecretUri::from_str("//Dave").unwrap()).unwrap()
        }
        /// Equivalent to `{DEV_PHRASE}//Eve`.
        pub fn eve() -> Keypair {
            Keypair::from_uri(&SecretUri::from_str("//Eve").unwrap()).unwrap()
        }
        /// Equivalent to `{DEV_PHRASE}//Ferdie`.
        pub fn ferdie() -> Keypair {
            Keypair::from_uri(&SecretUri::from_str("//Ferdie").unwrap()).unwrap()
        }
        /// Equivalent to `{DEV_PHRASE}//One`.
        pub fn one() -> Keypair {
            Keypair::from_uri(&SecretUri::from_str("//One").unwrap()).unwrap()
        }
        /// Equivalent to `{DEV_PHRASE}//Two`.
        pub fn two() -> Keypair {
            Keypair::from_uri(&SecretUri::from_str("//Two").unwrap()).unwrap()
        }
    }
}

// Make `Keypair` usable to sign transactions in Subxt. This is optional so that
// `subxt-signer` can be used entirely independently of Subxt.
#[cfg(feature = "subxt")]
mod subxt_compat {
    use super::*;

    use subxt_core::config::Config;
    use subxt_core::tx::signer::Signer as SignerT;
    use subxt_core::utils::{AccountId32, MultiAddress, MultiSignature};

    impl From<Signature> for MultiSignature {
        fn from(value: Signature) -> Self {
            MultiSignature::Ecdsa(value.0)
        }
    }

    impl From<PublicKey> for AccountId32 {
        fn from(value: PublicKey) -> Self {
            value.to_account_id()
        }
    }

    impl<T> From<PublicKey> for MultiAddress<AccountId32, T> {
        fn from(value: PublicKey) -> Self {
            value.to_address()
        }
    }

    impl PublicKey {
        /// A shortcut to obtain an [`AccountId32`] from a [`PublicKey`].
        /// We often want this type, and using this method avoids any
        /// ambiguous type resolution issues.
        pub fn to_account_id(self) -> AccountId32 {
            AccountId32(sp_crypto_hashing::blake2_256(&self.0))
        }
        /// A shortcut to obtain a [`MultiAddress`] from a [`PublicKey`].
        /// We often want this type, and using this method avoids any
        /// ambiguous type resolution issues.
        pub fn to_address<T>(self) -> MultiAddress<AccountId32, T> {
            MultiAddress::Id(self.to_account_id())
        }
    }

    impl<T: Config> SignerT<T> for Keypair
    where
        T::AccountId: From<PublicKey>,
        T::Address: From<PublicKey>,
        T::Signature: From<Signature>,
    {
        fn account_id(&self) -> T::AccountId {
            self.public_key().into()
        }

        fn address(&self) -> T::Address {
            self.public_key().into()
        }

        fn sign(&self, signer_payload: &[u8]) -> T::Signature {
            self.sign(signer_payload).into()
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    use polkadot_sdk::sp_core::{self, crypto::Pair as _, ecdsa::Pair as SpPair};

    #[test]
    fn check_from_phrase_matches() {
        for _ in 0..20 {
            let (sp_pair, phrase, _seed) = SpPair::generate_with_phrase(None);
            let phrase = bip39::Mnemonic::parse(phrase).expect("valid phrase expected");
            let pair = Keypair::from_phrase(&phrase, None).expect("should be valid");

            assert_eq!(sp_pair.public().0, pair.public_key().0);
        }
    }

    #[test]
    fn check_from_phrase_with_password_matches() {
        for _ in 0..20 {
            let (sp_pair, phrase, _seed) = SpPair::generate_with_phrase(Some("Testing"));
            let phrase = bip39::Mnemonic::parse(phrase).expect("valid phrase expected");
            let pair = Keypair::from_phrase(&phrase, Some("Testing")).expect("should be valid");

            assert_eq!(sp_pair.public().0, pair.public_key().0);
        }
    }

    #[test]
    fn check_from_secret_uri_matches() {
        // Some derive junctions to check that the logic there aligns:
        let uri_paths = ["//bar", "//0001", "//1", "//0001", "//foo//bar//wibble"];

        for i in 0..2 {
            for path in &uri_paths {
                // Build an sp_core::Pair that includes a phrase, path and password:
                let password = format!("Testing{i}");
                let (_sp_pair, phrase, _seed) = SpPair::generate_with_phrase(Some(&password));
                let uri = format!("{phrase}{path}///{password}");
                let sp_pair = SpPair::from_string(&uri, None).expect("should be valid");

                // Now build a local Keypair using the equivalent API:
                let uri = SecretUri::from_str(&uri).expect("should be valid secret URI");
                let pair = Keypair::from_uri(&uri).expect("should be valid");

                // They should match:
                assert_eq!(sp_pair.public().0, pair.public_key().0);
            }
        }
    }

    #[test]
    fn check_derive_errs_with_soft_junction() {
        let uri_paths = ["/bar", "/1", "//foo//bar/wibble"];
        for path in &uri_paths {
            let (_sp_pair, phrase, _seed) = SpPair::generate_with_phrase(None);
            let uri = format!("{phrase}{path}");
            let uri = SecretUri::from_str(&uri).expect("should be valid secret URI");
            let result = Keypair::from_uri(&uri);
            assert_eq!(result.err(), Some(Error::SoftJunction));
        }
    }

    #[test]
    fn check_signing_and_verifying_matches() {
        use sp_core::ecdsa::Signature as SpSignature;

        for _ in 0..20 {
            let (sp_pair, phrase, _seed) = SpPair::generate_with_phrase(Some("Testing"));
            let phrase = bip39::Mnemonic::parse(phrase).expect("valid phrase expected");
            let pair = Keypair::from_phrase(&phrase, Some("Testing")).expect("should be valid");

            let message = b"Hello world";
            let sp_sig = sp_pair.sign(message).0;
            let sig: [u8; 65] = pair.sign(message).0;

            assert!(SpPair::verify(
                &SpSignature::from(sig),
                message,
                &sp_pair.public(),
            ));
            assert!(verify(&Signature(sp_sig), message, &pair.public_key()));
        }
    }

    #[test]
    fn check_hex_uris() {
        // Hex URIs seem to ignore the password on sp_core and here. Check that this is consistent.
        let uri_str =
            "0x1122334455667788112233445566778811223344556677881122334455667788///SomePassword";

        let uri = SecretUri::from_str(uri_str).expect("should be valid");
        let pair = Keypair::from_uri(&uri).expect("should be valid");
        let sp_pair = SpPair::from_string(uri_str, None).expect("should be valid");

        assert_eq!(pair.public_key().0, sp_pair.public().0);
    }
}
