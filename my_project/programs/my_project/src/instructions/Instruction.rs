// [[test.genesis]]
// address = "..."
// program = "..."
// upgradeable =  "..."

// [[test.genesis]]
// address = "jadshfkajsdhflkahhrqwoherjioweq24352346"

// program = "swap.so"
// upgradeable = true

// pub fn emergency_upgrade(ctx: Context<AdminOnly>) -> Result<()> {
//     require!(
//         ctx.accounts.admin.key() == ctx.accounts.upgrade_authority.key(),
//         CustomError::Unauthorized
//     );
//     Ok(())
// }


const anchor = require("@anchor-lang/core");

module.exports = async function (provider) {
    anchor.setProvider(provider);
}