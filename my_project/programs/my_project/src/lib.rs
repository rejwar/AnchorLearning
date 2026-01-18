//

// #[error_code]

// pub enum MyError {
//     #[msg("Your age is under 18")]
//     underage,
//     #[msg("There  is no balance in your account ")]
//     insufficientFunds,
// }

// pub fn custom_instruction(ctx: Context<CustomInstruction>) -> Result<()> {
//     Ok(())
// }

// pub type Result<T> = std::result::Result<T, error::Error>;

// use other_program::cpi::accounts::Increments;
// use other_program::cpi::increment;

// let cpi_ctx = CpiContext::new(
//     ctx.accounts.other_program.to_account_info(),
//     Increment {
//         counter: ctx.accounts.counter.to_account_info(),
//         authority: ctx.accounts.user.to_account_info(),
//     },

// );

// // increment(cpi_ctx)?;

// pub fn my_instructions(ctx: Context<MyContext>) -> Result<()> {
//     Ok(())
// }

// pub fn withdraw(ctx: Context<withdraw>, amount: u64) -> Result<()> {
//     require!(
//         ctx.accounts.vault.balance >= amount,
//         MyError::insufficientFunds
//     );

//     Ok(())
// }

// #[error_code]

// pub enum MyError {
//     #[msg("My Custom error message ")]
//     MyCustomError,
//     #[msg(" My second custom error message")]
//     MySecondCustomError,
// }

// #[error_code]

// pub enum MyError {
//     #[msg("Custom Error message")]
//     MyCustomError,
//     #[msg("My second custom error message")]
//     MySecondCustomError,
// }
// use anchor_lang::*;

// #[program]

// pub mod my_program {
//     use super::*;

//     pub fn do_something(ctx: Context<ctx>, value: u64) -> Result<()> {
//         if value == 0 {
//             return err!(MyError::MyCustomError);
//         }

//         Ok(())
//     }
// }

// #[error_code]
// pub enum MyError {
//     MissingAuthority,
// }
