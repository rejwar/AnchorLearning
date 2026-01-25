// #[derive(Accounts)]
// pub struct Updata<'info> {
//     pub counter: Account<'info, Counter>,
// }

// #[derive(Accounts)]
// pub struct Update<'info> {
//     #[account(mut)]
//     pub counter: Account<'info, Counter>,
// }

// #[derive(Accounts)]
// pub struct Update<'info> {
//     #[account(mut)]
//     pub counter: Account<'info, Counter>,
// }

// #[derive(Accounts)]
// pub struct Update<'info> {
//     #[account(mut)]
//     pub counter: Account<'info, Counter>,
// }
// #[derive(Accoutns)]
// pub struct Update<'info> {
//     #[account(mut)]
//     pub counter: Account<'info, Counter>,
// }

// #[derive(Accounts)]
// pub struct Update<'info> {
//     #[account(mut)]
//     pub counter: Account<'info, Counter>,
// }

// pub struct Example<'info> {
//     pub payer: Signer<'info>,
//     pub authority: Signer<'info>,
// }

// pub struct Example<'info> {
//     pub payer: Signer<'info>,
//     pub authority: Signer<'info>,
// }

// payer == authority
// #[derive(Accounts)]

// pub struct Example<'info> {
//     #[account(dup)]
//     pub payer: Signer<'info>,
//     pub authority: Signer<'info>,
// }

// #[account(
//     init,
//     payer = user,
//     space = 8 + 32,
// )]

// pub profile: Account<'info , Profile>,

// #[account(
//     init,
//     seeds = [b"profile" , authority.key().as_ref()],
//     bump,
//     payer = authority,
//     space = 8+ 64
// )]

// pub profile: Account<'info , Profile>,

// #[account(
//     init_if_needed,
//     payer = <target_account>
// )]

// [#account
// init_if_needed,
// payer = <target_account>,
// space = <num_byte>
// ])

// seeds = [
//     b"user"
//     authority.key().as_ref()
// ]
// seeds = [
//     b"user",
//     authority.key().as_ref()
// ]
// Pubkey::create_program_address(seeds + bump)

// #[account]
// pub struct Vault {
//     pub target: Pubkey,
//     pub balance: u64,
// }

// #[derive(Accounts)]

// pub struct UseVault<'info> {
//     #[account (
//         mut,
//         has_one = target
//     )]
//     pub valut: Account<'info, Vault>,
//     pub target: AccountInfo<'info>,
// }

// #[derive(Accounts)]
// pub struct Example<'info> {
//     #[account(address = authority.key())]
//     pub signer_account: AccountInfo<'info>,

//     pub authority: Signer<'info>,
// }

// #[account(address = system_program::ID)]
// pub system_programa: Program<'info , System>,

// #[account(
//     mut,
//     token::mint = mint
// )]

// pub user_token: Account<'info , TokenAccount>;
// pub mint: Account<'info, Mint>;

// #[account(
//     mut,
//     token::authority = user
// )]

// pub user_token: Account <'info, TokenAccount>;
// pub user: Signer<'info>

// #[account(
//     token::program = token_program
// )]

// pub user_token:Account<'info , TokenAccount>;
// pub token_program: Program<'info , Token>;

// #[account(
//     mut,
//     token::mint = mint,
//     token::authority = user,
//     token::program = token_program
// )]

// pub user_token:Account<'info , TokenAccount>

// #[account(
//     token::program = token_program
// )]

// pub mint:Account<'info , Mint>

// #[derive(
//     token::program = token_program
// )]

// pub mint: Account<'info , Mint>

// #[derive(Account)]
// pub struct Deposit<'info> {
//     pub user: Signer<'info>,

//     #[account(
//         mut,
//         token::mint = mint,
//         token::authority = user
//     )]

//     pub user_token:: Account<'info , TokenAccount>,

//     #[account(
//         mut,
//         token::mint = mint
//     )]

//     pub vault_token = Account<'info, TokenAccount>,

//     pub mint: Account<'info , Mint>,

//     pub token_program: Program<'info , Token>
// }

// #[account(
//     mut,
//     associated_token::mint = mint
// )]

// pub user_ata: Account<'info, TokenAccount>;
// pub mint: Account<'info, Mint>;

// #[account(
//     init_if_needed,
//     payer = user,
//     associated_token::mint = user,
//     *::token_program = token_program
// )]

// pub user_ata: Account<'info , TokenAccount>;

// #[account(
//     mut,
//     token::mint,
//     token::authority = user,
//     *::token_program = token_program
// )]

// pub user_token: Account<'info , TokenAccount>;

// #[derive(Accounts)]
// pub struct InitAta<'info> {
//     pub user: Signer<'inf
// }

// #[account(
//     init_if_needed,
//     payer = user,
//     associated_token::mint = mint,
//     associated_token::authority = user,
//     associated_token::authority = user,
//     *::token_program = token_program
// )]

// pub user_ata: Account<'info, TokenAccount>,

// pub mint: Account<'info , TokenAccount>,
// pub token_program: Program<'info, Toek

// #[derive(Accounts)]
// pub struct MintExtensionToken<'info> {
//     #[account(
//         mut,
//         extensions::token_program = token_2022_program,

//     )]
//     pub mint: InterfaceAccount<'info, Mint>,
//     pub token_2022_program: Program<'info, Token2022>,
// }
