use anchor_lang::prelude::*;
use crate::error::ErrorCode;
use crate::state::*;

declare_id!("6bQUnk1ZFgdaj5tEBTkijkztPnB1Kcg5raYf3CqBqio");

pub mod error;
pub mod state;

#[program]
mod qwestive_voting {

    use super::*;

    // setup base account for anchor voting
    pub fn initialize_voting(
        ctx: Context<InitializeVoting>, 
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

        // Make sure the token account has the desired mint
        // if token_account.mint != *mint.key {
        //     return Err(ErrorCode::InvalidTokenAccount.into());
        // }

        // Check that the token account has more than 0 tokens to initiate a community
        if token_account.amount <= 0 {
            return Err(ErrorCode::InsufficientTokenBalance.into());
        }

        community_vote_account.total_proposal_count = 0;
        community_vote_account.minimum_token_count = minimum_tokens;
        community_vote_account.mint = token_account.mint;
        Ok(())
    }

    // create a new proposal
    pub fn add_proposal(
        ctx: Context<AddProposal>,
        proposal_account_bump: u8,
        proposal_id: u64,
        title: String,
        description: String,
        minimum_token_count: u64,
        vote_system_type: u8,
        threshold: u64,
        voting_type: u8,
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

        // Make sure the user is the owner of the token account
        if token_account.owner != *user.key {
            // Log a formatted message, use with caution can be expensive
            msg!("{} key: {:?}", "token", token_account.owner);
            msg!("{} key: {:?}", "user", *user.to_account_info().key);
            return Err(ErrorCode::UnauthorizedTokenHolder.into());
        }

        // Make sure the token account holds the community mint
        if token_account.mint != community_vote_account.mint {
            return Err(ErrorCode::InvalidTokenAccount.into());
        }

        // Check that the token balance meets the minimum required balance specified by the community
        if token_account.amount <= community_vote_account.minimum_token_count {
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

        // Defines the voting type, 0 - 1 vote per token account, 1 - weighted, 2 - quadratic
        if vote_system_type > 2 {
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
        proposal.mint = community_vote_account.mint;
        proposal.minimum_token_count = minimum_token_count;
        proposal.vote_system_type = vote_system_type;
        proposal.threshold = threshold;
        proposal.voting_type = voting_type;
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
        vote_account_bump: u8,
        proposal_id: u64,
        vote_bool: bool,
        candidate: u8,
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
        
        // Make sure the user is the owner of the token account
        if token_account.owner != *user.key {
            // Log a formatted message, use with caution can be expensive
            msg!("{} key: {:?}", "token", token_account.owner);
            msg!("{} key: {:?}", "user", *user.to_account_info().key);
            return Err(ErrorCode::UnauthorizedTokenHolder.into());
        }

        // Make sure the token account holds the community mint
        if token_account.mint != community_vote_account.mint {
            return Err(ErrorCode::InvalidTokenAccount.into());
        }
        
        // Check that the token balance meets the minimum required balance specified by the community
        // Gated token check
        if token_account.amount <= community_vote_account.minimum_token_count {
            return Err(ErrorCode::InsufficientTokenBalance.into());
        }

        // Make sure the token account and proposal mint match
        if token_account.mint != proposal.mint {
            return Err(ErrorCode::InvalidTokenAccount.into());
        }

        // Check that the token balance meets the minimum required balance for voting specified by proposal
        if token_account.amount <= proposal.minimum_token_count {
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

        vote_account.proposal_id = proposal_id;
        vote_account.voter = *user.key;
        // This flag is always set to false when a vote is made, when a tally is made the qwestive-voting will set this to true
        vote_account.tally_completed = false;
        vote_account.mint = token_account.mint;
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

        // Make sure the user is the owner of the token account
        if token_account.owner != *user.key {
            // Log a formatted message, use with caution can be expensive
            msg!("{} key: {:?}", "token", token_account.owner);
            msg!("{} key: {:?}", "user", *user.to_account_info().key);
            return Err(ErrorCode::UnauthorizedTokenHolder.into());
        }

        // Make sure the token account and proposal mint match
        if token_account.mint != proposal.mint {
            return Err(ErrorCode::InvalidTokenAccount.into());
        }

        // Check that the token balance meets the minimum required balance for voting specified by proposal
        if token_account.amount <= proposal.minimum_token_count {
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
        vote_account_bump: u8,
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
        
        // Make sure the user is the owner of the token account
        if token_account.owner != *user.key {
            // Log a formatted message, use with caution can be expensive
            msg!("{} key: {:?}", "token", token_account.owner);
            msg!("{} key: {:?}", "user", *user.to_account_info().key);
            return Err(ErrorCode::UnauthorizedTokenHolder.into());
        }

        // Make sure the token account and proposal mint match
        if token_account.mint != proposal.mint {
            return Err(ErrorCode::InvalidTokenAccount.into());
        }

        // Check that the token balance meets the minimum required balance for voting specified by proposal
        if token_account.amount <= proposal.minimum_token_count {
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
            // vote_account.candidate = candidate;
            // // vote bool is always false if candidate is chosen
            // vote_account.vote_bool = false;
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

        // Make sure the user is the owner of the token account
        if token_account.owner != *user.key {
            // Log a formatted message, use with caution can be expensive
            msg!("{} key: {:?}", "token", token_account.owner);
            msg!("{} key: {:?}", "user", *user.to_account_info().key);
            return Err(ErrorCode::UnauthorizedTokenHolder.into());
        }

         // Make sure the token account and proposal mint match
         if token_account.mint != proposal.mint {
            return Err(ErrorCode::InvalidTokenAccount.into());
        }

        // Check that the token balance meets the minimum required balance for voting specified by proposal
        if token_account.amount <= proposal.minimum_token_count {
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
