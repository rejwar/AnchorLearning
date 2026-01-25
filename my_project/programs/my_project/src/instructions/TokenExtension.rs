//

#[derive(Accounts)]
pub struct InitializeGroupPointer<'info> {
    #[account(
        mut,
        extensions::group_pointer::authority = group_auth,
        extensions::group_pointer::group_address = group_acc,
        mint::token_program = token_programm,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    pub group_auth: AccountInfo<'info>,
    pub group_acc: AccountInfo<'info>,
    pub token_progmram: Program<'info, Token2022>,
}
