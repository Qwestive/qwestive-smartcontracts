use anchor_lang::prelude::*;
//use anchor_lang::solana_program::system_program;
use crate::error::ErrorCode;
use crate::state::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, SetAuthority};
use spl_token::instruction::AuthorityType;

declare_id!("4UQ8dKxHjwV8QpetmqF62Xqnf17DavL3EWf6b6DbfYmz");

pub mod error;
pub mod state;

#[program]
pub mod qwestive_stake {
    use super::*;

    const COMMUNITY_PDA_SEED: &[u8] = b"community_seed";

    pub fn initialize_community(
        ctx: Context<InitializeCommunity>,
        mint: Pubkey,
        pda_seed: String,
        freeze_authority: Pubkey) -> ProgramResult {
        let community_account: &mut Account<CommunityAccount> = &mut ctx.accounts.community_account;
        let initializer: &Signer = &ctx.accounts.initializer;
        let clock: Clock = Clock::get().unwrap();
       
        // Make sure the user initializing the community is a signer
        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Make sure the seed provided is within the 20 character limit
        if pda_seed.chars().count() > 20 {
            return Err(ErrorCode::SeedTooLong.into())
        }

        // Make sure there is enough lamports to be rent exempt
        // if !ctx.accounts.rent.is_exempt(ctx.lamports(), community_account.data_len()) {
        //     return Err(EscrowError::NotRentExempt.into());
        // }

        //Create PDA to be owner of community token for all community **
        //Program Derived Addresses do not lie on the ed25519 curve and 
        //therefore have no private key associated with them.
        let (pda, _bump_seed) = Pubkey::find_program_address(&[pda_seed.as_bytes()],  ctx.program_id);
        
        // Make sure the owner of the PDA is the qwestive-take program
        // if pda.authority.key != ctx.program_id {
        //     return Err(ProgramError::InvalidAccountData);
        // }

        community_account.initializer = *initializer.key;
        community_account.mint = mint;
        community_account.community_pda = pda;
        community_account.account_balance = 0;
        community_account.freeze_authority = freeze_authority;
        community_account.freeze_enabled = false;
        community_account.time_stamp = clock.unix_timestamp;

        //Get the token program and reassign ownership of mint to 
        //Program Derived Account (PDA) 
        // Build function to create account change instruction
        token::set_authority(ctx.accounts.into(), AuthorityType::AccountOwner, Some(pda))?;

        Ok(())
    }
}


