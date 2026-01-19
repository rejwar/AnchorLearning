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

// #[error_code]
// pub enum AuthError {
//     #[msg("Authorized access ")]
//     Unauthorized,

//     #[msg("Sniper is missing ")]
//     MissingSigner,
// }

// pub fn set_data(ctx: Context<SetData>, data: MyAccount) -> Result<()> {
//     if data.data == 100 {
//         return err!(MyError::DataTooLarge);
//     }

//     ctx.accounts.my_account.set_inner(data);
//     Ok(())
// }

// #[error_code]

// pub enum MyError {
//     #[msg("MyAccount may only hold data below 100")]
//     DataTooLarge,
// }

// pub fn set_data(ctx: Context<SetData>, data: MyAccount) -> Result<()> {
//     require!(data.data < 100, MyError::DataTooLarge);

//     ctx.accounts.my_account.set_inner(data);
//     Ok(())
// }

// #[event]

// pub struct ValueUpdated {
//     pub user: Pubkey,
//     pub old_value: u64,
//     pub new_value: u64,
// }

// emit!(ValueUpdated {
//     user: ctx.accounts.signer.key(),
//     old_value,
//     new_value,
// })

// use anchor_lang::prelude::*;
// declare_id!("1111111111111111111111111111111");

// #[program]

// pub mod emit_example {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize> , value: u64) -> Result <()> {
//         ctx.accounts.data_account.value = value;

//         emit!(Initialize {
//             user: ctx.accounts.signer.key(),
//             value,
//         });

//         Ok(())
//     }

//     pub fn update(ctx.Context<Updata> , new_value: u64 ) -> Result<()> {
//         let old = ctx.accounts.data_account.value;
//         ctx.accounts.data_account.value = new_value;

//         emit!(ValueUpdated {
//             user: ctx.accounts.signer.key(),
//             old_value: old,
//             new_value,
//         });

//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     #[account(init , payer = signer , space = 8 +8)]
//     pub data_account: Account<'info , DataAccount>,

//     #[account(mut)]
//     pub signer: Signer<'info>,
//     pub system_program: Program<'info , System>,
// }

// #[derive(Accounts)]
// pub struct Updata<'info> {
//     #[account(mut)]
//     pub data_account: Account<'info , DataAccount>,
//     pub signer: Signer<'info>,
// }

// #[account]

// pub struct DataAccount {
//     pub value : u64 ,
// }

// #[event]
// pub struct Initialize {
//     pub user: Pubkey,
//     pub value: u64,
// }

// #[event]

// pub struct ValueUpdated {
//     pub user: Pubkey,
//     pub old_value: u64,
//     pub new_value: u64,
// }

// #[event]

// pub struct ValueUpdated {
//     pub user: Pubkey,
//     pub value: u64,
// }

// emit!(ValueChanged {
//     user: user.key(),
//     value: 10,
// })

// use anchor_lang::prelude::*;
// declare_id!("11111111111111111111111111111111111111");

// #[program]

// pub mod emit_demo {
//     use super::*;

//     pub fn update(ctx.Context<Updata> , value: u64) -> Result<()> {
//         emit!(ValueChanged {
//             user: ctx.accounts.user.key(),
//             value,
//         });
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Updata<'info>{
//     pub user: Signer<'info>,
// }

// #[event]
// pub struct ValueChanged {
//     pub user: Pubkey,
//     pub value: u64,
// }

// #[account]
// pub struct Bigdata {
//     pub value: [u64; 100],
// }

// use anchor_lang::prelude::*;

// #[account(zero_copy)]
// #[repr(packed)]

// pub struct Counter {
//     pub value: u64,
// }
//zero copy
// #[account]
// pub struct Counter {
//     pub value: u64,
// }

// #[account(zero_copy)]
// #[repr(packed)]

// pub struct Counter {
//     pub value: u64,
// }

// #[derive(Accounts)]

// pub struct Increment<'info> {
//     #[account(mut)]
//     pub counter: AccountLoader<'info, Counter>,
// }

// #[derive(Accounts)]

// pub fn increment(ctx.Context<Increment> -> Result<()>) {
//     let mut counter = ctx.accounts.counter.load_mut()?;
//     counter.value +=1;
//     Ok(())
// }

// #[account(zero_copy)]
// #[repr(packed)]

// pub struct Orderbook {
//     pub orders: [u64; 1024],
// }

// #[account(zero_copy)]

// pub struct Data {
//     pub data: [u8; 10232],
// }

// #[derive(Coppy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
// #[repr(C)]

// struct Data {
//     // --snap--
// }

// #[derive(Account)]
// pub struct InstructionAccount<'info> {
//     pub zero_copy_account: AccountLoader<'info , Data>
// }
// #[derive(Account)]
// pub struct InstructionAccount<'info> {
//     pub zero_copy_account: AccountLoader<'info, Data>,
// }

// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     #[account (
//         init,
//         space = 8 + 10232,
//         payer = payer ,
//     )]
//     pub data_account: AccountLoader<'info, Data>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//     let account = &mut ctx.accounts.data_account.load_init()?;
//     account.data = [1; 10232];
//     Ok(())
// }

// #[derive(Accounts)]

// pub struct Updata<'info> {
//     #[account(mut)]
//     pub data_account: AccountLoader<'info, data>,
// }

// pub fn update(ctx: Context<update>) -> Result<()> {}

// let account = &mut ctx.accounts.data_account.load_mut()?;

// account.data = [2; 10232]
