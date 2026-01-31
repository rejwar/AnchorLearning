// use anchor_lang::prelude::*;

// declare_id!("fadjfasdjfkjadshfklajsdhf524380958723490");

// declare_id!(lever);
// use kever::cpi::accounts::SwitchPower;
// use lever::accounts::PowerStatus;
// use lever::cpi::switch_power;
// use lever::program::Lever;

// #[program]
// pub mod hand {
//     use super::*;

//     pub fn pull_lever(ctx: Context<PullLever>, name: String) -> Result<()> {
//         let cpi_ctx = CpiContext::new(
//             ctx.accounts.lever_program.to_account_info(),
//             SwitchPower {
//                 power: ctx.accounts.power.to_account_info(),
//             },
//         );

//         switch_power(cpt_ctx, name)?;
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct PullLever<'info> {
//     #[account(mut)]
//     pub power: Account<'info, PowerStatus>,
//     pub lever_program: Program<'info, Lever>,
// }


#[account(
    init,
    payer = admin,
    mint::decimals = 6,
    mint::authority  = admin,
    mint::freeae_authority = admin
)]

pub mint_account: Account<'info , Mint>,