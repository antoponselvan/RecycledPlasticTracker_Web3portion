
use anchor_lang::{prelude::*};

pub enum ProductInstruction {
    AddProduct{
        re_plastic_pct:f32,
        serial_num: String,
        ingridient_manufacturer_key: String,
        ingridient_serial_num: String,
        purchaser_key: String
    },
    UpdateProduct{
        re_plastic_pct:f32,
        serial_num: String,
        ingridient_manufacturer_key: String,
        ingridient_serial_num: String,
        purchaser_key: String
    }
}

impl ProductInstruction {
    pub fn unpack(variant:u8, re_plastic_pct:f32, serial_num:String, ingridient_manufacturer_key:String,ingridient_serial_num:String, purchaser_key:String) -> Result<Self> {
        Ok(match variant {
            0 => Self::AddProduct {
                re_plastic_pct: re_plastic_pct,
                serial_num: serial_num,
                ingridient_manufacturer_key: ingridient_manufacturer_key,
                ingridient_serial_num: ingridient_serial_num,
                purchaser_key: purchaser_key
            },
            1 => Self::UpdateProduct {
                re_plastic_pct: re_plastic_pct,
                serial_num: serial_num,
                ingridient_manufacturer_key: ingridient_manufacturer_key,
                ingridient_serial_num: ingridient_serial_num,
                purchaser_key: purchaser_key
            },
            _ => return Err(ProgramError::InvalidInstructionData.into())
        })
    }
}