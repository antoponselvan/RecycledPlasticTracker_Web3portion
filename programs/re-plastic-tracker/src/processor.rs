
use anchor_lang::{prelude::*, solana_program::program_pack::IsInitialized};
use crate::product_instruction::ProductInstruction;
use crate::state::ProductAccountState;
use crate::error::ReviewError;

#[derive(Accounts)]
#[instruction(variant:u8, re_plastic_pct:f32, serial_num:String, ingridient_manufacturer_key:String, ingridient_serial_num:String, purchaser_key:String)]
pub  struct AddOrUpdateProduct<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        init_if_needed,
        seeds = [initializer.key.as_ref(), serial_num.as_bytes().as_ref()],
        bump,
        payer = initializer,
        space = 1000,
        constraint = 1 + 1 + 4 + (4 + serial_num.len()) + (4 + ingridient_manufacturer_key.len()) + (4 + ingridient_serial_num.len()) + (4 + purchaser_key.len()) <= 1000
        @ ReviewError::InvalidDataLength
    )]
    pub pda_account: Account<'info, ProductAccountState>,
    pub system_program: Program<'info, System>
}

pub fn add_product(
    ctx: Context<AddOrUpdateProduct>,
     re_plastic_pct: f32,
     serial_num: String,
     ingridient_manufacturer_key: String,
     ingridient_serial_num: String,
     purchaser_key: String
) -> Result<()> {
    msg!("Adding product {}", serial_num);

    // Code to confirm product doesnt exist

    // Code to confirm re_plastic_pct is within 0-100

    // Code to confirm ingridient is bought by manufacturer

    ctx.accounts.pda_account.serial_num = serial_num;
    ctx.accounts.pda_account.re_plastic_pct = re_plastic_pct;
    ctx.accounts.pda_account.ingridient_manufacturer_key = ingridient_manufacturer_key;
    ctx.accounts.pda_account.ingridient_serial_num = ingridient_serial_num;
    ctx.accounts.pda_account.purchaser_key = purchaser_key;
    ctx.accounts.pda_account.is_initialized = true;

    msg!("product registered {}", ctx.accounts.pda_account.key());
    Ok(())
}

pub fn update_product(
     ctx: Context<AddOrUpdateProduct>,
     re_plastic_pct: f32,
     serial_num: String,
     ingridient_manufacturer_key: String,
     ingridient_serial_num: String,
     purchaser_key: String
) -> Result<()> {
    msg!("Updating product {}", serial_num);

    // Code to check if product exists

    // Code to confirm re_plastic_pct is within 0-100

    // Code to confirm ingridient is bought by manufacturer

    ctx.accounts.pda_account.re_plastic_pct = re_plastic_pct;
    ctx.accounts.pda_account.ingridient_manufacturer_key = ingridient_manufacturer_key;
    ctx.accounts.pda_account.ingridient_serial_num = ingridient_serial_num;
    ctx.accounts.pda_account.purchaser_key = purchaser_key;

    msg!("product registered {}", ctx.accounts.pda_account.key());
    Ok(())
}

pub fn process_instruction(
     ctx: Context<AddOrUpdateProduct>,
     variant:u8,
     re_plastic_pct: f32,
     serial_num: String,
     ingridient_manufacturer_key: String,
     ingridient_serial_num: String,
     purchaser_key: String
) -> Result<()> {
    let instruction = ProductInstruction::unpack(variant, re_plastic_pct, serial_num, ingridient_manufacturer_key, ingridient_serial_num, purchaser_key)?;
    match instruction {
        ProductInstruction::AddProduct {re_plastic_pct, serial_num, ingridient_manufacturer_key, ingridient_serial_num, purchaser_key} => {
            add_product(ctx, re_plastic_pct, serial_num, ingridient_manufacturer_key, ingridient_serial_num, purchaser_key)
        },
        ProductInstruction::UpdateProduct {re_plastic_pct, serial_num, ingridient_manufacturer_key, ingridient_serial_num, purchaser_key} => {
            update_product(ctx, re_plastic_pct, serial_num, ingridient_manufacturer_key, ingridient_serial_num, purchaser_key)
        }
    }
}