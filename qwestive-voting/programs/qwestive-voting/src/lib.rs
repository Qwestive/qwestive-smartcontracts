use anchor_lang::prelude::*;
use crate::error::ErrorCode;
use crate::state::*;
use crate::validator::Validator;

//use anchor_metaplex::Metadata;
//use crate::metaplex_token_metadata::*;
//use metaplex_token_metadata::*;
use mpl_token_metadata::state::Metadata;
//use processor::Processor;

declare_id!("6bQUnk1ZFgdaj5tEBTkijkztPnB1Kcg5raYf3CqBqio");

pub mod error;
pub mod state;
pub mod validator;

#[program]
mod qwestive_voting {

    use super::*;

    // Initialize a community voting account
    pub fn initialize_voting(
        ctx: Context<InitializeVoting>,
        _community_account_bump: u8, 
        community_name: String,
        minimum_tokens: u64) -> ProgramResult {
        let community_vote_account = &mut ctx.accounts.community_vote_account;
        //let mint = &mut ctx.accounts.mint.to_account_info();
        let token_account = &mut ctx.accounts.token_account;
        let user = &mut ctx.accounts.user.to_account_info();

        // Make sure the user is the owner of the token account
        if token_account.owner != *user.key {
            // Log a formatted message, use with caution can be expensive
            msg!("{} key: {:?}", "token mint", token_account.mint);
            //msg!("{} key: {:?}", "mint", *mint.key);
            msg!("{} key: {:?}", "token", token_account.owner);
            msg!("{} key: {:?}", "user", *user.to_account_info().key);
            return Err(ErrorCode::UnauthorizedTokenHolder.into());
        }

        if community_name.chars().count() > 80 {
            return Err(ErrorCode::CommunityNameIsTooLong.into());
        }

        // Make sure the token account has the desired mint, this may not be needed since the 
        // if token_account.mint != *mint.key {
        //     return Err(ErrorCode::InvalidTokenAccount.into());
        // }

        // Check that the token account has more than 0 tokens to initiate a community
        if token_account.amount <= 0 {
            return Err(ErrorCode::InsufficientTokenBalance.into());
        }

        community_vote_account.is_nft = false;  // If the community is initialized with this instruction then it is a token community
        community_vote_account.community_name = community_name;
        community_vote_account.total_proposal_count = 0;
        community_vote_account.minimum_token_count = minimum_tokens;
        community_vote_account.key = token_account.mint;
        Ok(())
    }

    // Initialize an NFT community voting account
    pub fn initialize_nft_voting(
        ctx: Context<InitializeNFTVoting>,
        _community_account_bump: u8) -> ProgramResult {
        //community_name: String) -> ProgramResult {

        let community_vote_account = &mut ctx.accounts.community_vote_account;
        let mint = &mut ctx.accounts.nft_mint_account.to_account_info();
        let token_account = &mut ctx.accounts.token_account;
        let user = &mut ctx.accounts.user.to_account_info();
        let nft_metadata_account = &ctx.accounts.nft_metadata_account;
        //let nft_mint_account_pubkey = ctx.accounts.nft_mint_account.key();
        let token_metadata_program = &ctx.accounts.token_metadata_program;

        // &mut self,
        // token_account: TokenAccount,
        // token_metadata_program: AccountInfo,
        // mint_key: &Pubkey,
        // user_key: &Pubkey, 
        // nft_metadata_account: AccountInfo,

        // // Make sure the user is the owner of the NFT token account
        // if token_account.owner != *user.key {
        //     return Err(ErrorCode::UnauthorizedTokenHolder.into());
        // }

        // // The community name should not exceed 80 characters
        // if community_name.chars().count() > 80 {
        //     return Err(ErrorCode::CommunityNameIsTooLong.into());
        // }

        // // Make sure the token account has the same mint as the mint account
        // if token_account.mint != *mint.key {
        //      return Err(ErrorCode::InvalidTokenAccount.into());
        // }

        // // Check that the amount of the token must be 1 for NFT
        // if token_account.amount == 1 {
        //     return Err(ErrorCode::InsufficientTokenBalance.into());
        // }

        // //************* VERIFY NFT METADATA ************************//
        // // Get the seed for the NFT PDA
        // let metadata_seed = &[
        //     "metadata".as_bytes(), 
        //     ctx.accounts.token_metadata_program.key.as_ref(),
        //     nft_mint_account_pubkey.as_ref(),
        // ];

        // // Get the metdata PDA from the blockchain
        // let (metadata_derived_key, _bump_seed) = Pubkey::find_program_address (
        //     metadata_seed, 
        //     ctx.accounts.token_metadata_program.key
        // );

        // // Make sure the derived PDA key matches the current meta data account key provided
        // // If it doesn't match the metadata account provided was not created from the 
        // // metdadata program
        // if metadata_derived_key != nft_metadata_account.key() {
        //     return Err(ErrorCode::MetadataAccountMismatch.into());
        // }

        // // Make sure the metadata account is initialized
        // if ctx.accounts.nft_metadata_account.data_is_empty() {
        //     return Err(ErrorCode::MetadataAccountNotInitialized.into());
        // }

        // Get the metadata account struct so we can access its values
        let metadata_full_account =
            &mut Metadata::from_account_info(&ctx.accounts.nft_metadata_account)?;

        let full_metadata_clone = metadata_full_account.clone();
        // Use the key from the metadata since this is initializing the voting account it will use the
        // meta data creator key provided by the account any instructions after initializing must have a matching creator
        // account as the one provided
        
        let creator_key = &full_metadata_clone.data.creators.as_ref().unwrap()[0].address;
        // Comment out this is not needed since it is assumed the creator key that is passed 
        // will be used to create the NFT community
        // Make sure the creator key matches the expected
        // if(full_metadata_clone.data.creators.asref().unwrap()[0].address != creator_key)
        // {
        //     return Err(ErrorCode::CreatorKeyMismatch.into());
        // }

        
        Validator::verify_nft(
            token_metadata_program,
            token_account,
            user.key,
            mint.key,
            creator_key,
            nft_metadata_account)?;
 
        // Make sure the account has been verified
        // if full_metadata_clone.data.creators.as_ref().unwrap()[0].verified {
        //     return Err(ErrorCode::CreatorKeyIsNotVerified.into());
        // }

        // if full_metadata_clone.data.symbol.chars().count() > 80 {
        //     return Err(ErrorCode::CommunityNameIsTooLong.into());
        // }

        // Initialize Community
        community_vote_account.is_nft = true;
        community_vote_account.community_name = full_metadata_clone.data.symbol;
        community_vote_account.total_proposal_count = 0;
        community_vote_account.minimum_token_count = 1;
        community_vote_account.key = full_metadata_clone.data.creators.as_ref().unwrap()[0].address;
        Ok(())
    }

    // create a new proposal
    pub fn add_proposal(
        ctx: Context<AddProposal>,
        _community_account_bump: u8,
        proposal_account_bump: u8,
        proposal_id: u64,
        title: String,
        description: String,
        minimum_token_count: u64,
        vote_system_type: u8,
        threshold: u64,
        voting_type: u8,
        total_candidates: u64,
        voting_end_timestamp: u128,
        finalize_vote_end_timestamp: u128,
    ) -> ProgramResult {
        let community_vote_account = &mut ctx.accounts.community_vote_account;
        let proposal = &mut ctx.accounts.proposal;
        let token_account = &mut ctx.accounts.token_account;
        let user = &mut ctx.accounts.user.to_account_info();
        let current_timestamp = Clock::get()?.unix_timestamp as u128;
        
        // Anchor inherently checks and makes sure owner is the program id so no need to do this
        // if proposal.owner != anchor.program_id {
        //     return Err(ErrorCode::UnauthorizedAccount.into());
        // }

        // Anchor inherently checks and makes sure owner is the program id so no need to do this
        // if vote_account.owner != anchor.program_id {
        //     return Err(ErrorCode::UnauthorizedAccount.into());
        // }

        Validator::verify_token_account(
            token_account,
            user.key, 
            &community_vote_account.key,
            community_vote_account.minimum_token_count,
        )?; 

        if title.chars().count() > 80 {
            return Err(ErrorCode::TitleIsTooLong.into());
        }

        if description.chars().count() > 1024 {
            return Err(ErrorCode::DescriptionIsTooLong.into());
        }

        if current_timestamp > voting_end_timestamp {
            return Err(ErrorCode::VotingEndTimestampTooSmall.into());
        }

        if current_timestamp > voting_end_timestamp {
            return Err(ErrorCode::FinalizeTimestampTooSmall.into());
        }

        if voting_end_timestamp > finalize_vote_end_timestamp{
            // return error if proposal has ended
            return Err(ErrorCode::FinalizeTimestampTooSmall.into());
        }

        // Defines the voting type, 0 - 1 vote per token account, 1 - weighted, 2 - quadratic (not supported yet)
        if vote_system_type >= 2 {
            return Err(ErrorCode::InvalidVotingSystemType.into());
        }

        // Defines the voting type, 0 - yes or no votes, 1- multiple candidates
        if voting_type > 1 {
            return Err(ErrorCode::InvalidVotingType.into());
        }

        proposal.id = proposal_id;
        proposal.owner = *user.key;
        proposal.title = title;
        proposal.description = description;
        proposal.key = community_vote_account.key;
        proposal.minimum_token_count = minimum_token_count;
        proposal.vote_system_type = vote_system_type;
        proposal.threshold = threshold;
        proposal.voting_type = voting_type;
        proposal.total_candidates = total_candidates;
        //proposal.vote_options: AccountLoader<'info, VoteCandidates>, // Used for multiple choice proposals
        proposal.vote_yes =  0;
        proposal.vote_no = 0;
        proposal.total_votes = 0;
        proposal.total_tally = 0;
        proposal.tally_started = false;
        proposal.voting_finalized = false;
        proposal.created_timestamp = Clock::get()?.unix_timestamp ;
        proposal.voting_end_timestamp = voting_end_timestamp;
        proposal.finalize_vote_end_timestamp = finalize_vote_end_timestamp;
        proposal.proposal_tally_mismatch = false;
        proposal.bump = proposal_account_bump;

        // increment total proposal count
        community_vote_account.total_proposal_count += 1;
        Ok(())
    }

    // create a new proposal
    pub fn add_nft_proposal(
        ctx: Context<AddNFTProposal>,
        _community_account_bump: u8,
        proposal_account_bump: u8,
        proposal_id: u64,
        title: String,
        description: String,
        minimum_token_count: u64,
        vote_system_type: u8,
        threshold: u64,
        voting_type: u8,
        total_candidates: u64,
        voting_end_timestamp: u128,
        finalize_vote_end_timestamp: u128,
    ) -> ProgramResult {
        let community_vote_account = &mut ctx.accounts.community_vote_account;
        let proposal = &mut ctx.accounts.proposal;
        let token_account = &mut ctx.accounts.token_account;
        let user = &mut ctx.accounts.user.to_account_info();
        let current_timestamp = Clock::get()?.unix_timestamp as u128;
        // Accounts needed to verify NFT
        let mint = &mut ctx.accounts.nft_mint_account.to_account_info();
        let nft_metadata_account = &ctx.accounts.nft_metadata_account;
        let token_metadata_program = &ctx.accounts.token_metadata_program;

        // Anchor inherently checks and makes sure owner is the program id so no need to do this
        // if proposal.owner != anchor.program_id {
        //     return Err(ErrorCode::UnauthorizedAccount.into());
        // }

        // Anchor inherently checks and makes sure owner is the program id so no need to do this
        // if vote_account.owner != anchor.program_id {
        //     return Err(ErrorCode::UnauthorizedAccount.into());
        // }

        // Make sure the user is the owner of the token account
        // if token_account.owner != *user.key {
        //     // Log a formatted message, use with caution can be expensive
        //     msg!("{} key: {:?}", "token", token_account.owner);
        //     msg!("{} key: {:?}", "user", *user.to_account_info().key);
        //     return Err(ErrorCode::UnauthorizedTokenHolder.into());
        // }

        // Get the metadata account struct so we can access its values
        let metadata_full_account =
            &mut Metadata::from_account_info(&ctx.accounts.nft_metadata_account)?;

        let full_metadata_clone = metadata_full_account.clone();

        // Use the key from the metadata account and verify that it is the correct creator
        let creator_key = &full_metadata_clone.data.creators.as_ref().unwrap()[0].address;
        Validator::verify_nft(
           token_metadata_program,
           token_account,
           user.key,
           mint.key,
           creator_key,
           nft_metadata_account)?;

        // Make sure the creator key matches the community key
        if *creator_key != community_vote_account.key  {
            return Err(ErrorCode::CreatorKeyMismatch.into());
        }

        // Make sure the NFT collection name matches the community name
        if full_metadata_clone.data.symbol != community_vote_account.community_name  {
            return Err(ErrorCode::CollectionNameMismatch.into());
        }

        // Check that the token balance meets the minimum required balance specified by the community
        if token_account.amount < community_vote_account.minimum_token_count {
            return Err(ErrorCode::InsufficientTokenBalance.into());
        }

        if title.chars().count() > 80 {
            return Err(ErrorCode::TitleIsTooLong.into());
        }

        if description.chars().count() > 1024 {
            return Err(ErrorCode::DescriptionIsTooLong.into());
        }

        if current_timestamp > voting_end_timestamp {
            return Err(ErrorCode::VotingEndTimestampTooSmall.into());
        }

        if current_timestamp > voting_end_timestamp {
            return Err(ErrorCode::FinalizeTimestampTooSmall.into());
        }

        if voting_end_timestamp > finalize_vote_end_timestamp{
            // return error if proposal has ended
            return Err(ErrorCode::FinalizeTimestampTooSmall.into());
        }

        // Defines the voting type, 0 - 1 vote per token account, 1 - weighted, 2 - quadratic (not supported yet)
        if vote_system_type >= 2 {
            return Err(ErrorCode::InvalidVotingSystemType.into());
        }

        // Defines the voting type, 0 - yes or no votes, 1- multiple candidates
        if voting_type > 1 {
            return Err(ErrorCode::InvalidVotingType.into());
        }

        proposal.id = proposal_id;
        proposal.owner = *user.key;
        proposal.title = title;
        proposal.description = description;
        proposal.key = *creator_key;  // The creator key is used to gate the token// community_vote_account.key;
        proposal.minimum_token_count = minimum_token_count;
        proposal.vote_system_type = vote_system_type;
        proposal.threshold = threshold;
        proposal.voting_type = voting_type;
        proposal.total_candidates = total_candidates;
        //proposal.vote_options: AccountLoader<'info, VoteCandidates>, // Used for multiple choice proposals
        proposal.vote_yes =  0;
        proposal.vote_no = 0;
        proposal.total_votes = 0;
        proposal.total_tally = 0;
        proposal.tally_started = false;
        proposal.voting_finalized = false;
        proposal.created_timestamp = Clock::get()?.unix_timestamp ;
        proposal.voting_end_timestamp = voting_end_timestamp;
        proposal.finalize_vote_end_timestamp = finalize_vote_end_timestamp;
        proposal.proposal_tally_mismatch = false;
        proposal.bump = proposal_account_bump;

        // increment total proposal count
        community_vote_account.total_proposal_count += 1;
        Ok(())
    }

    // vote on a proposal
    pub fn vote_for_proposal(
        ctx: Context<VoteForProposal>,
        _community_account_bump: u8,
        vote_account_bump: u8,
        proposal_id: u64,
        vote_bool: bool,
        candidate: u64,
    ) -> ProgramResult {
        let community_vote_account = &mut ctx.accounts.community_vote_account;
        let proposal = &mut ctx.accounts.proposal;
        let vote_account = &mut ctx.accounts.vote;
        let user = &mut ctx.accounts.user.to_account_info();
        let token_account = &mut ctx.accounts.token_account;
        let current_timestamp = Clock::get()?.unix_timestamp as u128;

        // Anchor inherently checks and makes sure owner is the program id so no need to do this
        // if proposal.owner != anchor.program_id {
        //     return Err(ErrorCode::UnauthorizedAccount.into());
        // }

        // Anchor inherently checks and makes sure owner is the program id so no need to do this
        // if vote_account.owner != anchor.program_id {
        //     return Err(ErrorCode::UnauthorizedAccount.into());
        // }

        Validator::verify_token_account(
            token_account,
            user.key, 
            &community_vote_account.key,
            community_vote_account.minimum_token_count,
        )?; 

        // Make sure the token account and proposal mint match
        if token_account.mint != proposal.key {
            return Err(ErrorCode::InvalidTokenAccount.into());
        }

        // Check that the token balance meets the minimum required balance for voting specified by proposal
        if token_account.amount < proposal.minimum_token_count {
            return Err(ErrorCode::InsufficientTokensToVote.into());
        }

        if current_timestamp > proposal.voting_end_timestamp {
            // return error if proposal has ended
            return Err(ErrorCode::ProposalHasEnded.into());
        }

        if current_timestamp > proposal.finalize_vote_end_timestamp {
            // return error if proposal has finalized
            return Err(ErrorCode::ProposalVotingFinalized.into());
        }

        // Check that the desired proposal is the same as the proposal account
        if proposal_id != proposal.id {
            return Err(ErrorCode::ProposalIDMismatch.into());
        }

        // Make sure a tally has not started, if it has no new votes can be made
        if proposal.tally_started == true {
            return Err(ErrorCode::TallyHasStarted.into());
        }

        // Candidate vote is outside of the allowed proposal options
        if proposal.voting_type == 1 && candidate > proposal.total_candidates {
            return Err(ErrorCode::CandidateIndexOutOfBounds.into());
        }

        if proposal.voting_type == 1 && vote_bool == true {
            return Err(ErrorCode::InvalidVoteParameter.into());
        }

        vote_account.proposal_id = proposal_id;
        vote_account.voter = *user.key;
        // This flag is always set to false when a vote is made, when a tally is made the qwestive-voting will set this to true
        vote_account.tally_completed = false;
        vote_account.key = token_account.mint;
        vote_account.created_timestamp =  Clock::get()?.unix_timestamp;
        vote_account.bump =  vote_account_bump;
        vote_account.voter_weight = token_account.amount;
        // Always counts the total votes per account
        proposal.total_votes += 1;

        // If candidate is 0 then using yes no vote
        if proposal.voting_type == 0 {
            vote_account.vote_bool = vote_bool;
            // Candidate is always 0 if voting type 0 is selected
            vote_account.candidate = 0;
            // coresponding vote count base on `vote`
            if vote_bool {
                if proposal.vote_system_type == 0 {
                    proposal.vote_yes += 1;
                }
                else if proposal.vote_system_type == 1 {
                    proposal.vote_yes += token_account.amount;
                }
                
            } else {
                if proposal.vote_system_type == 0 {
                    proposal.vote_no += 1;
                }
                else if proposal.vote_system_type == 1 {
                    proposal.vote_no += token_account.amount;
                }
            }
        }
        // Voting type 1 is candidate
        else if proposal.voting_type == 1 {
            proposal.vote_yes = 0;
            proposal.vote_no = 0;
            vote_account.candidate = candidate;
            // vote bool is always false if candidate is chosen
            vote_account.vote_bool = false;
        }

        Ok(())
    }

        // vote on a proposal
    pub fn vote_for_nft_proposal(
        ctx: Context<VoteForNFTProposal>,
        _community_account_bump: u8,
        vote_account_bump: u8,
        proposal_id: u64,
        vote_bool: bool,
        candidate: u64,
    ) -> ProgramResult {
        let community_vote_account = &mut ctx.accounts.community_vote_account;
        let proposal = &mut ctx.accounts.proposal;
        let vote_account = &mut ctx.accounts.vote;
        let user = &mut ctx.accounts.user.to_account_info();
        let token_account = &mut ctx.accounts.token_account;
        let current_timestamp = Clock::get()?.unix_timestamp as u128;
        // Accounts needed to verify NFT
        let mint = &mut ctx.accounts.nft_mint_account.to_account_info();
        let nft_metadata_account = &ctx.accounts.nft_metadata_account;
        let token_metadata_program = &ctx.accounts.token_metadata_program;

        // Get the metadata account struct so we can access its values
        let metadata_full_account =
            &mut Metadata::from_account_info(&ctx.accounts.nft_metadata_account)?;

        let full_metadata_clone = metadata_full_account.clone();

        // Use the key from the metadata account and verify that it is the correct creator
        let creator_key = &full_metadata_clone.data.creators.as_ref().unwrap()[0].address;
        Validator::verify_nft(
           token_metadata_program,
           token_account,
           user.key,
           mint.key,
           creator_key,
           nft_metadata_account)?;

        // Make sure the creator key matches the community key
        if *creator_key != community_vote_account.key  {
            return Err(ErrorCode::CreatorKeyMismatch.into());
        }

        // Make sure the NFT collection name matches the community name
        if full_metadata_clone.data.symbol != community_vote_account.community_name  {
            return Err(ErrorCode::CollectionNameMismatch.into());
        }

        // Check that the token balance meets the minimum required balance specified by the community
        // Gated token check
        if token_account.amount < community_vote_account.minimum_token_count {
            return Err(ErrorCode::InsufficientTokenBalance.into());
        }

        // Make sure the creator key and proposal key match
        if *creator_key != proposal.key {
            return Err(ErrorCode::CreatorKeyMismatch.into());
        }

        // Check that the token balance meets the minimum required balance for voting specified by proposal
        if token_account.amount < proposal.minimum_token_count {
            return Err(ErrorCode::InsufficientTokensToVote.into());
        }

        if current_timestamp > proposal.voting_end_timestamp {
            // return error if proposal has ended
            return Err(ErrorCode::ProposalHasEnded.into());
        }

        if current_timestamp > proposal.finalize_vote_end_timestamp {
            // return error if proposal has finalized
            return Err(ErrorCode::ProposalVotingFinalized.into());
        }

        // Check that the desired proposal is the same as the proposal account
        if proposal_id != proposal.id {
            return Err(ErrorCode::ProposalIDMismatch.into());
        }

        // Make sure a tally has not started, if it has no new votes can be made
        if proposal.tally_started == true {
            return Err(ErrorCode::TallyHasStarted.into());
        }

        // Candidate vote is outside of the allowed proposal options
        if proposal.voting_type == 1 && candidate > proposal.total_candidates {
            return Err(ErrorCode::CandidateIndexOutOfBounds.into());
        }

        if proposal.voting_type == 1 && vote_bool == true {
            return Err(ErrorCode::InvalidVoteParameter.into());
        }

        vote_account.proposal_id = proposal_id;
        vote_account.voter = *user.key;
        // This flag is always set to false when a vote is made, when a tally is made the qwestive-voting will set this to true
        vote_account.tally_completed = false;
        vote_account.key = token_account.mint;
        vote_account.created_timestamp =  Clock::get()?.unix_timestamp;
        vote_account.bump =  vote_account_bump;
        vote_account.voter_weight = token_account.amount;
        // Always counts the total votes per account
        proposal.total_votes += 1;

        // If candidate is 0 then using yes no vote
        if proposal.voting_type == 0 {
            vote_account.vote_bool = vote_bool;
            // Candidate is always 0 if voting type 0 is selected
            vote_account.candidate = 0;
            // coresponding vote count base on `vote`
            if vote_bool {
                if proposal.vote_system_type == 0 {
                    proposal.vote_yes += 1;
                }
                else if proposal.vote_system_type == 1 {
                    proposal.vote_yes += token_account.amount;
                }
                
            } else {
                if proposal.vote_system_type == 0 {
                    proposal.vote_no += 1;
                }
                else if proposal.vote_system_type == 1 {
                    proposal.vote_no += token_account.amount;
                }
            }
        }
        // Voting type 1 is candidate
        else if proposal.voting_type == 1 {
            proposal.vote_yes = 0;
            proposal.vote_no = 0;
            vote_account.candidate = candidate;
            // vote bool is always false if candidate is chosen
            vote_account.vote_bool = false;
        }

        Ok(())
    }

    // Begin tally process
    pub fn begin_tally(
        ctx: Context<BeginTally>,
        proposal_id: u64,
    ) -> ProgramResult {
        let proposal = &mut ctx.accounts.proposal;
        let user = &mut ctx.accounts.user.to_account_info();
        let token_account = &mut ctx.accounts.token_account;
        let current_timestamp = Clock::get()?.unix_timestamp as u128;

        // Anchor inherently checks and makes sure owner is the program id so no need to do this
        // if proposal.owner != anchor.program_id {
        //     return Err(ErrorCode::UnauthorizedAccount.into());
        // }

        Validator::verify_token_account(
            token_account,
            user.key, 
            &proposal.key,
            proposal.minimum_token_count,
        )?; 

        // Make sure the voting session has ended before tally can begin
        if current_timestamp < proposal.voting_end_timestamp {
            msg!("Current time {}", current_timestamp);
            msg!("Proposal Created Time {}" , proposal.created_timestamp);   
            msg!("Vote End Time {}" , proposal.voting_end_timestamp);
            msg!("Finalize End Time {}" , proposal.finalize_vote_end_timestamp);
            return Err(ErrorCode::VotingTimeHasNotEnded.into());
        }

        // Make sure it has not exceeded the finalize voting time stamp
        if current_timestamp > proposal.finalize_vote_end_timestamp {
            return Err(ErrorCode::ProposalVotingFinalized.into());
        }

        // Check that the desired proposal is the same as the proposal account
        if proposal_id != proposal.id {
            return Err(ErrorCode::ProposalIDMismatch.into());
        }

        // Proposal tally cannot be started if it has already been started
        if proposal.tally_started == true {
            return Err(ErrorCode::ProposalTallyAlreadyStarted.into());
        }

        // Begin tally after all checks have been made
        proposal.tally_started = true;
        // Reset the vote counts to begin tallying votes
        proposal.vote_yes = 0;
        proposal.vote_no = 0;
        proposal.total_tally = 0;

        Ok(())
    }

    // Begin tally process
    pub fn begin_nft_tally(
        ctx: Context<BeginNFTTally>,
        _community_account_bump: u8,
        proposal_id: u64,
    ) -> ProgramResult {
        let community_vote_account = &mut ctx.accounts.community_vote_account;
        let proposal = &mut ctx.accounts.proposal;
        let user = &mut ctx.accounts.user.to_account_info();
        let token_account = &mut ctx.accounts.token_account;
        let current_timestamp = Clock::get()?.unix_timestamp as u128;

        // Anchor inherently checks and makes sure owner is the program id so no need to do this
        // if proposal.owner != anchor.program_id {
        //     return Err(ErrorCode::UnauthorizedAccount.into());
        // }

        // Accounts needed to verify NFT
        let mint = &mut ctx.accounts.nft_mint_account.to_account_info();
        let nft_metadata_account = &ctx.accounts.nft_metadata_account;
        let token_metadata_program = &ctx.accounts.token_metadata_program;

        // Get the metadata account struct so we can access its values
        let metadata_full_account =
            &mut Metadata::from_account_info(&ctx.accounts.nft_metadata_account)?;

        let full_metadata_clone = metadata_full_account.clone();

        // Use the key from the metadata account and verify that it is the correct creator
        let creator_key = &full_metadata_clone.data.creators.as_ref().unwrap()[0].address;
        Validator::verify_nft(
        token_metadata_program,
        token_account,
        user.key,
        mint.key,
        creator_key,
        nft_metadata_account)?;

        // Make sure the NFT collection name matches the community name
        if full_metadata_clone.data.symbol != community_vote_account.community_name  {
            return Err(ErrorCode::CollectionNameMismatch.into());
        }

        // Make creator key matches and proposal key
        if *creator_key != proposal.key {
            return Err(ErrorCode::CreatorKeyMismatch.into());
        }

        // Check that the token balance meets the minimum required balance for voting specified by proposal
        if token_account.amount < proposal.minimum_token_count {
            return Err(ErrorCode::InsufficientTokensToVote.into());
        }

        // Make sure the voting session has ended before tally can begin
        if current_timestamp < proposal.voting_end_timestamp {
            msg!("Current time {}", current_timestamp);
            msg!("Proposal Created Time {}" , proposal.created_timestamp);   
            msg!("Vote End Time {}" , proposal.voting_end_timestamp);
            msg!("Finalize End Time {}" , proposal.finalize_vote_end_timestamp);
            return Err(ErrorCode::VotingTimeHasNotEnded.into());
        }

        // Make sure it has not exceeded the finalize voting time stamp
        if current_timestamp > proposal.finalize_vote_end_timestamp {
            return Err(ErrorCode::ProposalVotingFinalized.into());
        }

        // Check that the desired proposal is the same as the proposal account
        if proposal_id != proposal.id {
            return Err(ErrorCode::ProposalIDMismatch.into());
        }

        // Proposal tally cannot be started if it has already been started
        if proposal.tally_started == true {
            return Err(ErrorCode::ProposalTallyAlreadyStarted.into());
        }

        // Begin tally after all checks have been made
        proposal.tally_started = true;
        // Reset the vote counts to begin tallying votes
        proposal.vote_yes = 0;
        proposal.vote_no = 0;
        proposal.total_tally = 0;

        Ok(())
    }

    pub fn tally_vote(
        ctx: Context<TallyVote>,
        _vote_account_bump: u8,
        proposal_id: u64,
    ) -> ProgramResult {
        let vote_account = &mut ctx.accounts.vote;
        let proposal = &mut ctx.accounts.proposal;
        let user = &mut ctx.accounts.user.to_account_info();
        let token_account = &mut ctx.accounts.token_account;
        let current_timestamp = Clock::get()?.unix_timestamp as u128;

        // Anchor inherently checks and makes sure owner is the program id so no need to do this
        // if proposal.owner != anchor.program_id {
        //     return Err(ErrorCode::UnauthorizedAccount.into());
        // }
        
        Validator::verify_token_account(
            token_account,
            user.key, 
            &proposal.key,
            proposal.minimum_token_count,
        )?; 

        // Make sure the voting session has ended before tally can begin
        if current_timestamp < proposal.voting_end_timestamp {
            // return error if proposal has ended
            return Err(ErrorCode::VotingTimeHasNotEnded.into());
        }

        // Make sure it has not exceeded the finalize voting time stamp
        if current_timestamp > proposal.finalize_vote_end_timestamp {
            // return error if proposal has finalized
            return Err(ErrorCode::ProposalVotingFinalized.into());
        }

        // Check that the desired proposal is the same as the proposal account
        if proposal_id != proposal.id {
            return Err(ErrorCode::ProposalIDMismatch.into());
        }

        // Check that the desired proposal is the same as the vote account proposal
        if proposal_id != vote_account.proposal_id {
            return Err(ErrorCode::ProposalIDMismatch.into());
        }    
        
        // Cant not begin tallying until proposal tally has started
        if proposal.tally_started == false {
            return Err(ErrorCode::TallyHasNotStarted.into());
        }

        // If the vote has already been tallied then it cannot be tallied again
        if vote_account.tally_completed == true {
            return Err(ErrorCode::VoteAccountAlreadyTallied.into());
        }
        
        // Count the total tally
        proposal.total_tally += 1;
        vote_account.voter_weight = token_account.amount;

        // Update the voting count
        if proposal.voting_type == 0 {
            // Candidate is always 0 if voting type 0 is selected
            vote_account.candidate = 0;
            // Update the vote count based on the latest token info
            if vote_account.vote_bool {
                if proposal.vote_system_type == 0 {
                    proposal.vote_yes += 1;
                }
                else if proposal.vote_system_type == 1 {
                    proposal.vote_yes += token_account.amount;
                }            
            }
            else {
                if proposal.vote_system_type == 0 {
                    proposal.vote_no += 1;
                }
                else if proposal.vote_system_type == 1 {
                    proposal.vote_no += token_account.amount;
                }
            }
        }
        // Voting type 1 is candidate TODO Determine how to do multiple choice
        else if proposal.voting_type == 1 {
            proposal.vote_no = 0;
            proposal.vote_yes = 0;
            // vote bool is always false if candidate is chosen
            vote_account.vote_bool = false;
        }

        vote_account.tally_completed = true;
        Ok(())
    }

    pub fn tally_nft_vote(
        ctx: Context<TallyNFTVote>,
        _community_account_bump: u8,
        _vote_account_bump: u8,
        proposal_id: u64,
    ) -> ProgramResult {
        let community_vote_account = &mut ctx.accounts.community_vote_account;
        let vote_account = &mut ctx.accounts.vote;
        let proposal = &mut ctx.accounts.proposal;
        let user = &mut ctx.accounts.user.to_account_info();
        let token_account = &mut ctx.accounts.token_account;
        let current_timestamp = Clock::get()?.unix_timestamp as u128;

        // Anchor inherently checks and makes sure owner is the program id so no need to do this
        // if proposal.owner != anchor.program_id {
        //     return Err(ErrorCode::UnauthorizedAccount.into());
        // }
        
        // Accounts needed to verify NFT
        let mint = &mut ctx.accounts.nft_mint_account.to_account_info();
        let nft_metadata_account = &ctx.accounts.nft_metadata_account;
        let token_metadata_program = &ctx.accounts.token_metadata_program;

        // Get the metadata account struct so we can access its values
        let metadata_full_account =
            &mut Metadata::from_account_info(&ctx.accounts.nft_metadata_account)?;

        let full_metadata_clone = metadata_full_account.clone();

        // Use the key from the metadata account and verify that it is the correct creator
        let creator_key = &full_metadata_clone.data.creators.as_ref().unwrap()[0].address;
        Validator::verify_nft(
        token_metadata_program,
        token_account,
        user.key,
        mint.key,
        creator_key,
        nft_metadata_account)?;

        // Make sure the creator key matches the community key
        if *creator_key != community_vote_account.key  {
            return Err(ErrorCode::CreatorKeyMismatch.into());
        }

        // Make sure the NFT collection name matches the community name
        if full_metadata_clone.data.symbol != community_vote_account.community_name  {
            return Err(ErrorCode::CollectionNameMismatch.into());
        }

        // Make creator key matches and proposal key
        if *creator_key != proposal.key {
            return Err(ErrorCode::CreatorKeyMismatch.into());
        }

        // Check that the token balance meets the minimum required balance for voting specified by proposal
        if token_account.amount < proposal.minimum_token_count {
            return Err(ErrorCode::InsufficientTokensToVote.into());
        }

        // Make sure the voting session has ended before tally can begin
        if current_timestamp < proposal.voting_end_timestamp {
            // return error if proposal has ended
            return Err(ErrorCode::VotingTimeHasNotEnded.into());
        }

        // Make sure it has not exceeded the finalize voting time stamp
        if current_timestamp > proposal.finalize_vote_end_timestamp {
            // return error if proposal has finalized
            return Err(ErrorCode::ProposalVotingFinalized.into());
        }

        // Check that the desired proposal is the same as the proposal account
        if proposal_id != proposal.id {
            return Err(ErrorCode::ProposalIDMismatch.into());
        }

        // Check that the desired proposal is the same as the vote account proposal
        if proposal_id != vote_account.proposal_id {
            return Err(ErrorCode::ProposalIDMismatch.into());
        }    
        
        // Cant not begin tallying until proposal tally has started
        if proposal.tally_started == false {
            return Err(ErrorCode::TallyHasNotStarted.into());
        }

        // If the vote has already been tallied then it cannot be tallied again
        if vote_account.tally_completed == true {
            return Err(ErrorCode::VoteAccountAlreadyTallied.into());
        }
        
        // Count the total tally
        proposal.total_tally += 1;
        vote_account.voter_weight = token_account.amount;

        // Update the voting count
        if proposal.voting_type == 0 {
            // Candidate is always 0 if voting type 0 is selected
            vote_account.candidate = 0;
            // Update the vote count based on the latest token info
            if vote_account.vote_bool {
                if proposal.vote_system_type == 0 {
                    proposal.vote_yes += 1;
                }
                else if proposal.vote_system_type == 1 {
                    proposal.vote_yes += token_account.amount;
                }            
            }
            else {
                if proposal.vote_system_type == 0 {
                    proposal.vote_no += 1;
                }
                else if proposal.vote_system_type == 1 {
                    proposal.vote_no += token_account.amount;
                }
            }
        }
        // Voting type 1 is candidate TODO Determine how to do multiple choice
        else if proposal.voting_type == 1 {
            proposal.vote_no = 0;
            proposal.vote_yes = 0;
            // vote bool is always false if candidate is chosen
            vote_account.vote_bool = false;
        }

        vote_account.tally_completed = true;
        Ok(())
    }

    pub fn finalize_vote(
        ctx: Context<FinalizeVote>,
        proposal_id: u64,
    ) -> ProgramResult {
        let proposal = &mut ctx.accounts.proposal;
        let user = &mut ctx.accounts.user.to_account_info();
        let token_account = &mut ctx.accounts.token_account;
        let current_timestamp = Clock::get()?.unix_timestamp as u128;

        Validator::verify_token_account(
            token_account,
            user.key, 
            &proposal.key,
            proposal.minimum_token_count,
        )?; 

        // Make sure the voting session has ended before finalizing vote
        if current_timestamp < proposal.voting_end_timestamp {
            // return error if proposal has ended
            return Err(ErrorCode::VotingTimeHasNotEnded.into());
        }

        // Make sure finalized is not called before the finalze voting end time period
        if current_timestamp < proposal.finalize_vote_end_timestamp {
            // return error if proposal has finalized
            return Err(ErrorCode::FinalizeVoteTimeHasNotEnded.into());
        }

        // Check that the desired proposal is the same as the proposal account
        if proposal_id != proposal.id {
            return Err(ErrorCode::ProposalIDMismatch.into());
        }
        
        if proposal.voting_finalized == true {
            return Err(ErrorCode::VoteAccountAlreadyTallied.into());
        }

        // This performs a user check to make sure that the number of vote accounts matches the total tally
        if proposal.total_votes != proposal.total_tally {
            proposal.proposal_tally_mismatch = true;
        }

        proposal.voting_finalized = true;
        Ok(())
    }
    pub fn finalize_nft_vote(
        ctx: Context<FinalizeNFTVote>,
        _community_account_bump: u8,
        proposal_id: u64,
    ) -> ProgramResult {
        let community_vote_account = &mut ctx.accounts.community_vote_account;
        let proposal = &mut ctx.accounts.proposal;
        let user = &mut ctx.accounts.user.to_account_info();
        let token_account = &mut ctx.accounts.token_account;
        let current_timestamp = Clock::get()?.unix_timestamp as u128;
        // Accounts needed to verify NFT
        let mint = &mut ctx.accounts.nft_mint_account.to_account_info();
        let nft_metadata_account = &ctx.accounts.nft_metadata_account;
        let token_metadata_program = &ctx.accounts.token_metadata_program;

        // Get the metadata account struct so we can access its values
        let metadata_full_account =
            &mut Metadata::from_account_info(&ctx.accounts.nft_metadata_account)?;

        let full_metadata_clone = metadata_full_account.clone();

        // Use the key from the metadata account and verify that it is the correct creator
        let creator_key = &full_metadata_clone.data.creators.as_ref().unwrap()[0].address;
        Validator::verify_nft(
        token_metadata_program,
        token_account,
        user.key,
        mint.key,
        creator_key,
        nft_metadata_account)?;

        // Make sure the creator key matches the community key
        if *creator_key != community_vote_account.key  {
            return Err(ErrorCode::CreatorKeyMismatch.into());
        }

        // Make sure the NFT collection name matches the community name
        if full_metadata_clone.data.symbol != community_vote_account.community_name  {
            return Err(ErrorCode::CollectionNameMismatch.into());
        }

        // Make creator key matches and proposal key
        if *creator_key != proposal.key {
            return Err(ErrorCode::CreatorKeyMismatch.into());
        }

        // Check that the token balance meets the minimum required balance for voting specified by proposal
        if token_account.amount < proposal.minimum_token_count {
            return Err(ErrorCode::InsufficientTokensToVote.into());
        }

        // Make sure the voting session has ended before finalizing vote
        if current_timestamp < proposal.voting_end_timestamp {
            // return error if proposal has ended
            return Err(ErrorCode::VotingTimeHasNotEnded.into());
        }

        // Make sure finalized is not called before the finalze voting end time period
        if current_timestamp < proposal.finalize_vote_end_timestamp {
            // return error if proposal has finalized
            return Err(ErrorCode::FinalizeVoteTimeHasNotEnded.into());
        }

        // Check that the desired proposal is the same as the proposal account
        if proposal_id != proposal.id {
            return Err(ErrorCode::ProposalIDMismatch.into());
        }
        
        if proposal.voting_finalized == true {
            return Err(ErrorCode::VoteAccountAlreadyTallied.into());
        }

        // This performs a user check to make sure that the number of vote accounts matches the total tally
        if proposal.total_votes != proposal.total_tally {
            proposal.proposal_tally_mismatch = true;
        }

        proposal.voting_finalized = true;
        Ok(())
    }
}
