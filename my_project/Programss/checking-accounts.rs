use anchor_lang::prelude::*;
declare_id!("adhsjflakdhjflkajsdhf254389052093454235");

#[program]
pub mod checking_account_program {
    use super::*;

    pub fn check_account(_ctx: Context<CheckingAccount>) -> Result<()> {
        OK(())
    }
}

#[derive(Account)]
pub struct CheckingAccount<'info> {
    payer: Signer<'info>,

    #[account(mut)]
    account_to_create: UncheckedAccount<'info>,

    #[account(
        mut,
        owner = id()
    )]
    account_to_create: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
}
