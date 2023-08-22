use crate::types::User;
use serde::{Deserialize, Serialize};

pub type SingUpResult = Result<User, SingUpError>;

#[derive(Serialize, Deserialize)]
pub enum SingUpError {
    NoUser,
    WrongPassword,
    NoUsernameHeader,
    NoPasswordHeader,
    NoHeaders,
    UserIsNotActivated,
}

pub type RegisterResult = Result<User, Vec<SingUpError>>;

#[derive(Serialize, Deserialize)]
pub enum RegistrationError {
    InvalidSchoolNumber,
    InvalidClass,
    InvalidUsername,
    InvalidPassword,
    InvalidEmail,
    InvalidLocation,
    UnExistingEmail,
    UsernameExistsInDb,
    EmailExistsInDB,
}

#[derive(Serialize, Deserialize)]
pub enum VerfyEmailError {
    EmailAlreadyVerfied,
    WrongVerfyKey,
}
