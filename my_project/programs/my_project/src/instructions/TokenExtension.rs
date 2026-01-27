//

// #[derive(Accounts)]
// pub struct InitializeGroupPointer<'info> {
//     #[account(
//         mut,
//         extensions::group_pointer::authority = group_auth,
//         extensions::group_pointer::group_address = group_acc,
//         mint::token_program = token_programm,
//     )]
//     pub mint: InterfaceAccount<'info, Mint>,

//     pub group_auth: AccountInfo<'info>,
//     pub group_acc: AccountInfo<'info>,
//     pub token_progmram: Program<'info, Token2022>,
// }

// use anchor_lang::prelude::*;
// use anchor_spl::token_interface::{Mint, Token2022};

// #[derive(Accounts)]
// pub struct InitializeGroupPointer<'info> {
//     #[account(
//         mut,
//         extensions::metadata_pointer::authority = meta_auth,
//         extensions::metadata_pointer::metadata_address = mint.key(),
//         mint::token_program = token_program,
//     )]
//     pub mint: InterfaceAccount<'info, Mint>,
//     pub meta_auth: AccountInfo<'info>,
//     pub token_program: Program<info, Token2022>,
// }
