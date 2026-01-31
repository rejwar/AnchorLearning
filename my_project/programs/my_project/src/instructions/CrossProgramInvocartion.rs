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

// #[account(
//     init,
//     payer = admin,
//     mint::decimals = 6,
//     mint::authority  = admin,
//     mint::freeae_authority = admin
// )]

// pub mint_account: Account<'info , Mint>,

// #[account(
//     token::mint = mint_account,
//     token::authority  = user,
//     token::token_program = token_2022_program
// )]

// pub user_token_account: Account<'info , TokenAccount>,

// #[account(
//     init,
//     payer = user,
//     mint::decimals = 9,
//     mint::authority = user,

//     extensions::close_authority::authority = user
// )]

// pub mint_account: Account<'info , Mint>,

// #[account(
//     init,
//     payer = admin,
//     mint::decimals = 9,
//     mint::authority = admin,
//     extensions::permanent_delegate::delegate = admin
// )]

// pub mint_account:Account<'info, Mint>,

// #[account(
//     init,
//     payer = user ,
//     mint::decimals = 9,
//     mint::authority = user,

//     extensions::transfer_hook::authority = user,
//     extensions::transfer_hook::program_id = hook_program
// )]

// pub mint_account: Account<'info , Mint>

// #[account(
//     init,
//     payer = payer,
//     mint::decimals = 0,
//     mint::authority = authority,

//     extensions::group_pointer::authority = authority,
//     extensions::group_pointer::group_address = group_account
// )]

// pub mint_account: Account<'info , Mint>,

// #[account(
//     init,
//     payer = payer,
//     mint::decimals = 0,
//     mint::authority = authority,

//     extensions::group_member_pointer::authority = authority,
//     extensions::group_member_pointer::member_address = group_member_pointer
// )]

// pub mint_account: Account<'info < Mint>,
