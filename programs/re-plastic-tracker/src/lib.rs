pub mod processor;
pub mod product_instruction;
pub mod error;
pub mod state;

use anchor_lang::prelude::*;
use processor::*;

declare_id!("Aq6sLZDjh3MiwpWyk1J84RQT7vKZfGtsZRaNGHtQfEwv");

#[program]
pub mod re_plastic_tracker {
    use crate::processor::process_instruction;

    use super::*;

    pub fn add_or_update_product(
        ctx: Context<AddOrUpdateProduct>,
        variant: u8,
        re_plastic_pct: f32,
        serial_num: String,
        ingridient_manufacturer_key: String,
        ingridient_serial_num: String,
        purchaser_key: String
    ) -> Result<()>{
        process_instruction(ctx, variant, re_plastic_pct, serial_num, ingridient_manufacturer_key, ingridient_serial_num, purchaser_key)
    }

}
