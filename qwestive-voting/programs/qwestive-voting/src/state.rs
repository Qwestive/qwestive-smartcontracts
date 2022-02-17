use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeVoting<'info> {
    // base account which holds all proposals
    #[account(init, payer = user, space = 1000)] // TODO: calculate correct size for it
    pub community_vote_account: Account<'info, CommunityVoteAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(proposal_account_bump: u8, proposal_id: u64)]
pub struct AddProposal<'info> {
    #[account(mut)]
    pub community_vote_account: Account<'info, CommunityVoteAccount>,

    #[account(init, seeds = [b"proposal_account".as_ref(), proposal_id.to_le_bytes().as_ref()], bump =  proposal_account_bump, payer = user, space = Proposal::LEN)]
    pub proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(vote_account_bump: u8, proposal_id: u64)]
pub struct VoteForProposal<'info> {
    #[account(mut, seeds = [b"proposal_account".as_ref(), proposal_id.to_le_bytes().as_ref()], bump = proposal.bump)]
    pub proposal: Account<'info, Proposal>,

    #[account(init, seeds = [b"vote_account".as_ref(), proposal_id.to_le_bytes().as_ref(), user.key.as_ref()] , bump = vote_account_bump, payer = user, space = Vote::LEN)]
    pub vote: Account<'info, Vote>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CommunityVoteAccount {
    pub total_proposal_count: u64,
}

#[account]
pub struct Proposal {
    pub id: u64, // unique id for each proposal
    pub owner: Pubkey,
    pub created_timestamp: i64,
    pub title: String,
    pub description: String,
    pub vote_yes: u64,
    pub vote_no: u64,
    pub bump: u8,
    pub end_time_stamp: u128,
}

#[account]
pub struct Vote {
    pub proposal_id: u64,  // Unique proposal id
    pub vote_bool: bool,   // Use this vote type for yes or no votes
    //pub vote_choice: u8,   // Use this for a multiple choice vote, a 0 value indicates vote bool is used
    pub voter: Pubkey, // The voters token account key
    //pub mint: Pubkey, // Mint key of the gated token
    //pub tally_completed: bool,  // Flag indicating the vote has been tallied
    pub created_timestamp: i64,
    pub bump: u8,
}

// #[account]
// pub struct VoteCandidates{
//     pub id: u64, // unique proposal id this vote option belongs to
//     pub voting_options: [VoteItem; 20], // Array storing the vote options
// }

// #[account]
// pub struct VoteItem {
//     pub candidate: u8, // The candidate vote number
//     pub vote_count: u64, // The total vote count of this option
// }

const U64_LEN: usize = 8;
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBKEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_PROPOSAL_TITLE_LENGTH: usize = 80 * STRING_LENGTH_PREFIX;
const MAX_PROPOSAL_DESCRIPTION_LENGTH: usize = 1024 * STRING_LENGTH_PREFIX;
const VOTE_COUNT_LENGTH: usize = U64_LEN;
const BUMP_LENGTH: usize = 1;
const BOOL_LENGTH: usize = 1;

impl Proposal {
    const LEN: usize = DISCRIMINATOR_LENGTH 
    + U64_LEN // id
    + PUBKEY_LENGTH // Author
    + TIMESTAMP_LENGTH // Timestamp
    + MAX_PROPOSAL_TITLE_LENGTH // Title 
    + MAX_PROPOSAL_DESCRIPTION_LENGTH // Description 
    + VOTE_COUNT_LENGTH // vote yes count
    + VOTE_COUNT_LENGTH // vote no count
    + BUMP_LENGTH;
}

impl Vote {
    const LEN: usize = DISCRIMINATOR_LENGTH 
    + U64_LEN // proposal id
    + BOOL_LENGTH // vote
    + PUBKEY_LENGTH // Author
    + TIMESTAMP_LENGTH
    + BUMP_LENGTH;
}