// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

use crate::error::{Error, ErrorKind};

pub const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
pub struct Salt([u8; CREDENTIAL_LEN]);
impl Salt {
    pub fn new() -> Self {
        Self([0u8; CREDENTIAL_LEN])
    }

    pub fn from_string(s: &str) -> Result<Self, Error> {
        let bytes = s.as_bytes();
        if bytes.len() == CREDENTIAL_LEN {
            let mut salt = Self::new();
            for (index, byte) in bytes.iter().enumerate() {
                salt.0[index] = *byte;
            }
            Ok(salt)
        } else {
            Err(Error::from_string(
                ErrorKind::RingError,
                format!("Invalid hash: {}", s),
            ))
        }
    }

    pub fn hex(&self) -> String {
        HEXUPPER.encode(&self.0)
    }
}

pub type PasswordHash = Salt;

pub fn new_salt() -> Result<Salt, Error> {
    let rng = rand::SystemRandom::new();
    let mut salt = Salt::new();
    rng.fill(&mut salt.0)?;
    Ok(salt)
}

pub fn encrypt(password: &str, salt: &Salt) -> PasswordHash {
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let mut pbkdf2_hash = PasswordHash::new();
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt.0,
        password.as_bytes(),
        &mut pbkdf2_hash.0,
    );
    pbkdf2_hash
}

pub fn verify(password: &str, hash: &PasswordHash, salt: &Salt) -> Result<(), Error> {
    let n_iter = NonZeroU32::new(100_000).unwrap();

    pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt.0,
        password.as_bytes(),
        &hash.0,
    )
    .map_err(Into::into)
}
