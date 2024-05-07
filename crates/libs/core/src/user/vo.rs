use std::{fmt, str::FromStr};

use gnify::{error::InvalidValue, text};
use serde::Serialize;

text! {
    Username: r"^[a-z0-9][a-z0-9_]{3,63}$"
}

text! {
    Email: r"^[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?$"
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Password(String);

impl Password {
    pub fn generate(password: &str) -> Result<Password, InvalidValue> {
        use argon2::{
            password_hash::{rand_core::OsRng, SaltString},
            Argon2, PasswordHasher,
        };
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .unwrap();
        Ok(Password(hash.to_string()))
    }

    pub fn verify(&self, password: &str) -> bool {
        use argon2::{Argon2, PasswordHash, PasswordVerifier};
        let hash = PasswordHash::new(&self.0).unwrap();
        Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok()
    }
}

impl FromStr for Password {
    type Err = InvalidValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use argon2::PasswordHash;
        let s = if let Ok(s) = data_encoding::BASE64_NOPAD.decode(s.as_bytes()) {
            String::from_utf8(s).unwrap()
        } else {
            s.to_string()
        };
        if PasswordHash::new(&s).is_err() {
            return Err(InvalidValue::new("Password"));
        }
        Ok(Password(s))
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            data_encoding::BASE64_NOPAD.encode(self.0.as_bytes())
        )
    }
}
