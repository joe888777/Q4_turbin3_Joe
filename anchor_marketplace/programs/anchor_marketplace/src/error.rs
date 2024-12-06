use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("the length of given name for the market place should be 0 ~ 32.")]
    NameTooLong,
}
