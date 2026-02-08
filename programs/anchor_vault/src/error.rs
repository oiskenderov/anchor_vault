use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
#[msg("Insuficients funds")]
    InsufficientFunds,

    #[msg("Unexepected error")]
    Unexepected,
}