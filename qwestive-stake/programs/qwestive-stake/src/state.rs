use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, SetAuthority};

#[derive(Accounts)]
#[instruction(freeze_authority: Pubkey)]
pub struct InitializeCommunity<'info> {
    #[account(mut)] // Account is mutuable since it will be responsible for paying rent, will need to mutate the amont of money in the account
    pub initializer: Signer<'info>,
    #[account(init, payer = initializer, space = CommunityAccount::LEN)]  // Anchor initialization, rent payer, and storage size needed
    pub community_account: Account<'info, CommunityAccount>,
    #[account(mut)]  
    pub mint_account: AccountInfo<'info>,
    #[account(address = system_program::ID)]   // Makes sure the address really is the address of the system_program
    pub system_program: AccountInfo<'info>,
    #[account(address = token::ID)]    // Makes sure the address really is the address of the token_program
    pub token_program: Program<'info, Token>,
}

// The account used to store the community staking info
#[account]
pub struct CommunityAccount {
    // The public key of the community initializer
    pub initializer: Pubkey,
    // The public key of the mint for the creator token
    pub mint: Pubkey,
    // The PDA account holding the staked USD tokens
    pub community_pda: Pubkey,
    // The seed for the community pda
    pub pda_seed: String,
    // The current balance of the staked account
    pub account_balance: u64,
    // The authority to freeze the account
    pub freeze_authority: Pubkey,
    // The boolean flag to indicate if an account is currently freezed
    pub freeze_enabled: bool,
    // The time stamp of when the account was created
    pub time_stamp: i64
}

// Constants definining the size of the account
const DISCRIMINATOR_LENGTH: usize = 8;
const INITIALIZER_PUBLIC_KEY_LENGTH: usize = 32;
const MINT_AUTHORITY_PUBLIC_KEY_LENGTH: usize = 32;
const COMMUNITY_PDA_PUBLIC_KEY_LENGTH: usize = 32;
const PDA_SEED_PREFIX_LENGTH: usize = 4;
const PDA_SEED_LENGTH: usize = 80;
const ACCOUNT_BALANCE_LENGTH: usize = 8;
const FREEZE_AUTHORITY_PUBLIC_KEY_LENGTH: usize = 32;
const FREEZE_ENABLED_FLAG_LENGTH: usize = 4;
const TIMESTAMP_LENGTH: usize = 8;

impl CommunityAccount {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + INITIALIZER_PUBLIC_KEY_LENGTH // Initializer public key
        + MINT_AUTHORITY_PUBLIC_KEY_LENGTH // Mint ID of token
        + COMMUNITY_PDA_PUBLIC_KEY_LENGTH // PDA Stake account public key
        + PDA_SEED_PREFIX_LENGTH // 4 byte prefix of string length
        + PDA_SEED_LENGTH // 20 character string of pda seed
        + ACCOUNT_BALANCE_LENGTH // Current USDC Balance.
        + FREEZE_AUTHORITY_PUBLIC_KEY_LENGTH // Freeze authorit public key
        + FREEZE_ENABLED_FLAG_LENGTH // Flag for freezing account.
        + TIMESTAMP_LENGTH; // Time Stamp.
}

impl<'info> From<&mut InitializeCommunity<'info>>
    for CpiContext<'_, '_, '_, 'info, SetAuthority<'info>>
{
    fn from(accounts: &mut InitializeCommunity<'info>) -> Self {
        let cpi_accounts = SetAuthority {
            account_or_mint: accounts
                .mint_account
                .to_account_info()
                .clone(),
            current_authority: accounts.initializer.to_account_info().clone(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}