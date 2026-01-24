use anchor_lang::prelude::*;
use anchor_lang::token_interface::{Mint , Token2022};

#[derive(Accounts)]
pub struct CheckCloseAuthority<'info>{
    #[account(
        extensions::close_Authority::authority = expectd_authority,
        mint::token_program = token_program,
    )]

    pub mint:: InterfaceAccount<'info , Mint>,
    pub expected_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token2022>,
}