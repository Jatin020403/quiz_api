use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Clone)]
pub struct Argon {
    pub salt: SaltString,
    pub argon: Argon2<'static>,
}

#[derive(Clone)]
pub struct Util {
    pub Argon: Argon,
}

pub fn initialise_argon() -> Argon {
    Argon {
        salt: SaltString::generate(&mut OsRng),
        argon: Argon2::default(),
    }
}

pub fn initialise() -> Util {
    let a = Util {
        Argon: initialise_argon(),
    };

    a
}
