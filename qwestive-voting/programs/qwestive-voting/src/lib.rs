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
    pub fn initialize_voting(ctx: Context<InitializeVoting>) -> ProgramResult {
        let community_vote_account = &mut ctx.accounts.community_vote_account;
        community_vote_account.total_proposal_count = 0;
        Ok(())
    }

    // create a new proposal
    pub fn add_proposal(
        ctx: Context<AddProposal>,
        proposal_account_bump: u8,
        proposal_id: u64,
        title: String,
        description: String,
        end_time_stamp: u128,
    ) -> ProgramResult {
        let community_vote_account = &mut ctx.accounts.community_vote_account;
        let proposal = &mut ctx.accounts.proposal;
        let user = &mut ctx.accounts.user;

        if title.chars().count() > 80 {
            return Err(ErrorCode::TitleIsTooLong.into());
        }

        if description.chars().count() > 1024 {
            return Err(ErrorCode::DescriptionIsTooLong.into());
        }

        proposal.id = proposal_id;
        proposal.owner = *user.to_account_info().key;
        proposal.title = title;
        proposal.description = description;
        proposal.vote_yes =  0;
        proposal.vote_no = 0;
        proposal.created_timestamp = Clock::get()?.unix_timestamp;
        proposal.end_time_stamp = end_time_stamp;
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
        vote: bool,
    ) -> ProgramResult {
        let proposal = &mut ctx.accounts.proposal;
        let vote_account = &mut ctx.accounts.vote;
        let user = &mut ctx.accounts.user;

        vote_account.proposal_id = proposal_id;
        vote_account.voter = *user.to_account_info().key;
        vote_account.vote_bool = vote;
        vote_account.created_timestamp =  Clock::get()?.unix_timestamp;
        vote_account.bump =  vote_account_bump;

        if (Clock::get()?.unix_timestamp as u128) > proposal.end_time_stamp {
            // return error if proposal has ended
            return Err(ErrorCode::ProposalHasEnded.into());
        }

        // coresponding vote count base on `vote`
        if vote {
            proposal.vote_yes += 1
        } else {
            proposal.vote_no += 1
        }
        Ok(())
    }
}
