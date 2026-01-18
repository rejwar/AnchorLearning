//

#[error_code]

pub enum MyError {
    #[msg("Your age is under 18")]
    underage,
    #[msg("There  is no balance in your account ")]
    insufficientFunds,
}
