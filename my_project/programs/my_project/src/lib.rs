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

pub type Result<T> = std::result::Result<T, error::Error>;
