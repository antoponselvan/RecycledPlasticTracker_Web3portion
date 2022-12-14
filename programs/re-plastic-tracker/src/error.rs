

use anchor_lang::prelude::*;

#[error_code]
pub enum ReviewError{
    #[msg("Account not yet initialized")]
    UnInitializedAccount,
    #[msg("Cant link ingridient you have not bought")]
    InvalidIngridient,
    #[msg("Input exceeds max length")]
    InvalidDataLength,
    #[msg("RePlastic Pct outside 0-100 range")]
    InvalidRePlasticPct
}
