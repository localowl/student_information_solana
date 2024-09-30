use anchor_lang::prelude::*;

#[error_code]
pub enum StudentError {
    #[msg("Name length must be max 20 characters")]
    NameLengthError,

    #[msg("Biography length must be max 50 characters")]
    BiographyLengthError,
}
