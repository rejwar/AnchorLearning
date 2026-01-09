pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

use anchor_lang::prelude::*;

declare_id!("1WFDytM6NBBXRoHuBxAXLycbP8gwL1ehPzQAZ8Esoub");

#[program]
pub mod my_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}
