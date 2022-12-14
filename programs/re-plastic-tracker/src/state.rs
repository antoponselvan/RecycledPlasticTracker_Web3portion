
use anchor_lang::{prelude::*, solana_program};
use solana_program::{
    program_pack::{IsInitialized}
};

#[account]
pub struct ProductAccountState {
    pub is_initialized: bool,
    pub re_plastic_pct: f32,
    pub serial_num: String,
    pub ingridient_manufacturer_key: String,
    pub ingridient_serial_num: String,
    pub purchaser_key: String
}

impl IsInitialized for ProductAccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}