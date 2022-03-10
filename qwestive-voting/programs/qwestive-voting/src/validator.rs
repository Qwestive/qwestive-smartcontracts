
use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount};
use mpl_token_metadata::state::Metadata;
use crate::error::ErrorCode;

pub struct Validator;

impl Validator {

    pub fn verify_token_account(
        token_account: &TokenAccount, 
        user_key: &Pubkey, 
        mint_key: &Pubkey,
        token_amount: u64,
    ) -> ProgramResult {

        // Make sure the user is the owner of the token account
        if token_account.owner != *user_key {
            // Log a formatted message, use with caution can be expensive
            msg!("{} key: {:?}", "token", token_account.owner);
            msg!("{} key: {:?}", "user", *user_key);
            return Err(ErrorCode::UnauthorizedTokenHolder.into());
        }

        // Make sure the token account and mint match
        if token_account.mint != *mint_key {
            return Err(ErrorCode::InvalidTokenAccount.into());
        }

        // Check that the token balance meets the minimum required balance for voting specified by proposal
        if token_account.amount < token_amount {
            return Err(ErrorCode::InsufficientTokensToVote.into());
        }
        Ok(())
    }

    
    pub fn verify_nft(
        //&mut self,
        token_metadata_program: &AccountInfo,
        token_account: &TokenAccount,
        mint_key: &Pubkey,
        user_key: &Pubkey,
        creator_key: &Pubkey,
        nft_metadata_account: &AccountInfo,
    ) -> ProgramResult {

        // Make sure the user is the owner of the NFT token account
        if token_account.owner != *user_key {
            return Err(ErrorCode::UnauthorizedTokenHolder.into());
        }

        // Make sure the token account has the same mint as the mint account
        if token_account.mint != *mint_key {
             return Err(ErrorCode::InvalidTokenAccount.into());
        }

        // Check that the amount of the token must be 1 for NFT
        if token_account.amount == 1 {
            return Err(ErrorCode::InsufficientTokenBalance.into());
        }

        //************* VERIFY NFT METADATA ************************//
        // Get the seed for the NFT PDA
        let metadata_seed = &[
            "metadata".as_bytes(), 
            token_metadata_program.key.as_ref(),
            mint_key.as_ref(),
        ];

        // Get the metdata PDA from the blockchain
        let (metadata_derived_key, _bump_seed) = Pubkey::find_program_address (
            metadata_seed, 
            token_metadata_program.key
        );

        // Make sure the derived PDA key matches the current meta data account key provided
        // If it doesn't match the metadata account provided was not created from the 
        // metdadata program
        if metadata_derived_key != nft_metadata_account.key() {
            return Err(ErrorCode::MetadataAccountMismatch.into());
        }

        // Make sure the metadata account is initialized
        if nft_metadata_account.data_is_empty() {
            return Err(ErrorCode::MetadataAccountNotInitialized.into());
        }

        // Get the metadata account struct so we can access its values
        let metadata_full_account =
            &mut Metadata::from_account_info(&nft_metadata_account)?;

        let full_metadata_clone = metadata_full_account.clone();

        // Make sure the creator key matches the expected
        if full_metadata_clone.data.creators.as_ref().unwrap()[0].address != *creator_key {
            return Err(ErrorCode::CreatorKeyMismatch.into());
        }

        // Make sure the account has been verified
        if full_metadata_clone.data.creators.as_ref().unwrap()[0].verified {
            return Err(ErrorCode::CreatorKeyIsNotVerified.into());
        }

        // There is a limit to the string storage, check and make sure the string isn't too long
        if full_metadata_clone.data.symbol.chars().count() > 80 {
            return Err(ErrorCode::CommunityNameIsTooLong.into());
        }

        Ok(())
    }
}