use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint};

#[derive(Accounts)]
#[instruction(_community_account_bump: u8)]
pub struct InitializeVoting<'info> {
    // community account which holds all proposals
    #[account(init, seeds = [b"community_account".as_ref(), token_account.mint.as_ref()], bump =  _community_account_bump, payer = user, space = CommunityVoteAccount::LEN)]
    pub community_vote_account: Account<'info, CommunityVoteAccount>,

    // The token account holding the gated token for this proposal
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_community_account_bump: u8, metadata_program_id: Pubkey)]
pub struct InitializeNFTVoting<'info> {
    // community account which holds all proposals
    #[account(init, seeds = [b"community_account".as_ref(), token_account.mint.as_ref()], bump =  _community_account_bump, payer = user, space = CommunityVoteAccount::LEN)]
    pub community_vote_account: Account<'info, CommunityVoteAccount>,

    // The NFT mint account
    pub nft_mint_account: Account<'info, Mint>,

    // The meta data of the NFT account
    pub nft_metadata_account: AccountInfo<'info>,

    // The metadata token program
    #[account(address = metadata_program_id)]
    pub token_metadata_program: AccountInfo<'info>,

    // The NFT token account holding the gated token for this proposal
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_community_account_bump: u8, proposal_account_bump: u8, proposal_id: u64)]
pub struct AddProposal<'info> {
    //#[account(mut)]
    #[account(mut, seeds = [b"community_account".as_ref(), token_account.mint.as_ref()], bump =  _community_account_bump)]
    pub community_vote_account: Account<'info, CommunityVoteAccount>,

    #[account(init, seeds = [b"proposal_account".as_ref(), proposal_id.to_le_bytes().as_ref()], bump =  proposal_account_bump, payer = user, space = Proposal::LEN)]
    pub proposal: Account<'info, Proposal>,
    
    // The token account holding the gated token for this proposal
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_community_account_bump: u8, proposal_account_bump: u8, proposal_id: u64, metadata_program_id: Pubkey)]
pub struct AddNFTProposal<'info> {
    #[account(mut, seeds = [b"community_account".as_ref(), token_account.mint.as_ref()], bump =  _community_account_bump)]
    pub community_vote_account: Account<'info, CommunityVoteAccount>,

    #[account(init, seeds = [b"proposal_account".as_ref(), proposal_id.to_le_bytes().as_ref()], bump =  proposal_account_bump, payer = user, space = Proposal::LEN)]
    pub proposal: Account<'info, Proposal>,
    
    // The NFT mint account
    pub nft_mint_account: Account<'info, Mint>,

    // The meta data of the NFT account
    pub nft_metadata_account: AccountInfo<'info>,

    // The metadata token program
    #[account(address = metadata_program_id)]
    pub token_metadata_program: AccountInfo<'info>,
  
    // The token account holding the gated token for this proposal
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_community_account_bump: u8, vote_account_bump: u8, proposal_id: u64)]
pub struct VoteForProposal<'info> {
    //#[account(mut)]
    #[account(mut, seeds = [b"community_account".as_ref(), token_account.mint.as_ref()], bump = _community_account_bump)]
    pub community_vote_account: Account<'info, CommunityVoteAccount>,

    #[account(mut, seeds = [b"proposal_account".as_ref(), proposal_id.to_le_bytes().as_ref()], bump = proposal.bump)]
    pub proposal: Account<'info, Proposal>,

    #[account(init, seeds = [b"vote_account".as_ref(), proposal_id.to_le_bytes().as_ref(), user.key.as_ref()], bump = vote_account_bump, payer = user, space = Vote::LEN)]
    pub vote: Account<'info, Vote>,

    // The token account holding the gated token for this proposal
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_community_account_bump: u8, vote_account_bump: u8, proposal_id: u64, metadata_program_id: Pubkey)]
pub struct VoteForNFTProposal<'info> {
    //#[account(mut)]
    #[account(mut, seeds = [b"community_account".as_ref(), token_account.mint.as_ref()], bump = _community_account_bump)]
    pub community_vote_account: Account<'info, CommunityVoteAccount>,

    #[account(mut, seeds = [b"proposal_account".as_ref(), proposal_id.to_le_bytes().as_ref()], bump = proposal.bump)]
    pub proposal: Account<'info, Proposal>,

    #[account(init, seeds = [b"vote_account".as_ref(), proposal_id.to_le_bytes().as_ref(), user.key.as_ref()], bump = vote_account_bump, payer = user, space = Vote::LEN)]
    pub vote: Account<'info, Vote>,
 
    // The NFT mint account
    pub nft_mint_account: Account<'info, Mint>,

    // The meta data of the NFT account
    pub nft_metadata_account: AccountInfo<'info>,

    // The metadata token program
    #[account(address = metadata_program_id)]
    pub token_metadata_program: AccountInfo<'info>,

    // The token account holding the gated token for this proposal
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct BeginTally<'info> {
    #[account(mut, seeds = [b"proposal_account".as_ref(), proposal_id.to_le_bytes().as_ref()], bump = proposal.bump)]
    pub proposal: Account<'info, Proposal>,

    // The token account holding the gated token for this proposal
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_community_account_bump: u8, proposal_id: u64, metadata_program_id: Pubkey)]
pub struct BeginNFTTally<'info> {
    #[account(mut, seeds = [b"community_account".as_ref(), token_account.mint.as_ref()], bump =  _community_account_bump)]
    pub community_vote_account: Account<'info, CommunityVoteAccount>,

    #[account(mut, seeds = [b"proposal_account".as_ref(), proposal_id.to_le_bytes().as_ref()], bump = proposal.bump)]
    pub proposal: Account<'info, Proposal>,

    // The NFT mint account
    pub nft_mint_account: Account<'info, Mint>,

    // The meta data of the NFT account
    pub nft_metadata_account: AccountInfo<'info>,

    // The metadata token program
    #[account(address = metadata_program_id)]
    pub token_metadata_program: AccountInfo<'info>,

    // The token account holding the gated token for this proposal
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_vote_account_bump: u8, proposal_id: u64)]
pub struct TallyVote<'info> {
    #[account(mut, seeds = [b"proposal_account".as_ref(), proposal_id.to_le_bytes().as_ref()], bump = proposal.bump)]
    pub proposal: Account<'info, Proposal>,

    #[account(mut, seeds = [b"vote_account".as_ref(), proposal_id.to_le_bytes().as_ref(), user.key.as_ref()], bump = _vote_account_bump)]
    pub vote: Account<'info, Vote>,

    // The token account holding the gated token for this proposal
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_community_account_bump: u8, _vote_account_bump: u8, proposal_id: u64, metadata_program_id: Pubkey)]
pub struct TallyNFTVote<'info> {
    #[account(mut, seeds = [b"community_account".as_ref(), token_account.mint.as_ref()], bump =  _community_account_bump)]
    pub community_vote_account: Account<'info, CommunityVoteAccount>,

    #[account(mut, seeds = [b"proposal_account".as_ref(), proposal_id.to_le_bytes().as_ref()], bump = proposal.bump)]
    pub proposal: Account<'info, Proposal>,

    #[account(mut, seeds = [b"vote_account".as_ref(), proposal_id.to_le_bytes().as_ref(), user.key.as_ref()], bump = _vote_account_bump)]
    pub vote: Account<'info, Vote>,

    // The NFT mint account
    pub nft_mint_account: Account<'info, Mint>,

    // The meta data of the NFT account
    pub nft_metadata_account: AccountInfo<'info>,

    // The metadata token program
    #[account(address = metadata_program_id)]
    pub token_metadata_program: AccountInfo<'info>,

    // The token account holding the gated token for this proposal
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct FinalizeVote<'info> {
    #[account(mut, seeds = [b"proposal_account".as_ref(), proposal_id.to_le_bytes().as_ref()], bump = proposal.bump)]
    pub proposal: Account<'info, Proposal>,

    // The token account holding the gated token for this proposal
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_community_account_bump: u8, proposal_id: u64, metadata_program_id: Pubkey)]
pub struct FinalizeNFTVote<'info> {
    #[account(mut, seeds = [b"community_account".as_ref(), token_account.mint.as_ref()], bump =  _community_account_bump)]
    pub community_vote_account: Account<'info, CommunityVoteAccount>,

    #[account(mut, seeds = [b"proposal_account".as_ref(), proposal_id.to_le_bytes().as_ref()], bump = proposal.bump)]
    pub proposal: Account<'info, Proposal>,

    // The NFT mint account
    pub nft_mint_account: Account<'info, Mint>,

    // The meta data of the NFT account
    pub nft_metadata_account: AccountInfo<'info>,

    // The metadata token program
    #[account(address = metadata_program_id)]
    pub token_metadata_program: AccountInfo<'info>,

    // The token account holding the gated token for this proposal
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CommunityVoteAccount {
    pub is_nft: bool,                       // Boolean to indicate whether this is an NFT
    pub key: Pubkey,                        // The Mint or Creator public key for the gated token
    pub community_name: String,             // String used to describe community or verify NFT
    pub total_proposal_count: u64,          // Total count of all proposals in the community
    pub minimum_token_count: u64,           // The minimum amount of tokens needed to be in this community, this is primarily used for sub communities or spaces
}

#[account]
pub struct Proposal {
    pub id: u64,                            // unique id for each proposal
    pub owner: Pubkey,                      // Proposal creator
    pub created_timestamp: i64,
    pub title: String,
    pub description: String,
    pub key: Pubkey,                        // Mint or Creator key of the gated token
	pub minimum_token_count: u64,           // The minimum number of tokens needed to be in this voting session
    pub vote_system_type: u8,               // Defines the voting type, 1 - 1 vote per token account, 2 - weighted, 3- quadratic
    pub threshold: u64,                     // This is a minimum vote threshold needed to approve this proposal, a 0 threshold means no vote quota is required
    pub voting_type: u8,                    // Defines the voting type, 1 - yes or no votes, 2- multiple candidates
    pub total_candidates: u64,              // Total number of candidates if using multiple candidate voting
    //pub vote_options: AccountLoader<'info, VoteCandidates>, // Used for multiple choice proposals
    pub vote_yes: u64,                      // Yes vote count used for voting type 1
    pub vote_no: u64,                       // No vote count used for voting type 1
    pub total_votes: u64,                   // Used to track the total votes made for this proposal
    pub total_tally: u64,                   // Tracks the total tally
    pub tally_started: bool,                // Flag used to indicate when voting tally has started
    pub voting_finalized: bool,             // Flag used to prevent any further voting changes
    pub proposal_tally_mismatch: bool,      // This flag is set if the proposal is deemed to have a mismatch for total votes vs total tally
    pub bump: u8,
    pub voting_end_timestamp: u128,         // The time to end the voting session
    pub finalize_vote_end_timestamp: u128,  // The time limit to finalize the voting session
}

#[account]
pub struct Vote {
    pub proposal_id: u64,                   // Unique proposal id
    pub tally_completed: bool,              // Flag indicating the vote has been tallied
    pub vote_bool: bool,                    // Use this vote type for yes or no votes
    pub candidate: u64,                     // Use this for a multiple choice vote, a 0 value indicates vote bool is used
    pub voter_weight: u64,                  // The weight of each vote, a 0 value defaults to no additional weight
    pub voter: Pubkey,                      // The voters token account key
    pub key: Pubkey,                        // Mint or Creator key of the gated token
    pub created_timestamp: i64,
    pub bump: u8,
}

// #[account]
// pub struct VoteCandidates {
//     pub id: u64, // unique proposal id this vote option belongs to
//     pub voting_options: [VoteItem; 20], // Array storing the vote options
// }

// #[account]
// pub struct VoteItem {
//     pub candidate: u8, // The candidate vote number
//     pub vote_count: u64, // The total vote count of this option
// }

// Size definitions
pub const U64_LEN: usize = 8;
pub const DISCRIMINATOR_LENGTH: usize = 8;
pub const PUBKEY_LENGTH: usize = 32;
pub const TIMESTAMP_LENGTH: usize = 8;
pub const STRING_LENGTH_PREFIX: usize = 4;
pub const MAX_COMMUNITY__DESCRIPTION_LENGTH: usize = 80 * STRING_LENGTH_PREFIX;
pub const MAX_PROPOSAL_TITLE_LENGTH: usize = 80 * STRING_LENGTH_PREFIX;
pub const MAX_PROPOSAL_DESCRIPTION_LENGTH: usize = 1024 * STRING_LENGTH_PREFIX;
pub const VOTE_COUNT_LENGTH: usize = U64_LEN;
pub const BUMP_LENGTH: usize = 1;
pub const BOOL_LENGTH: usize = 1;
pub const U8_LEN: usize = 1;
pub const END_TIMESTAMP_LENGTH: usize = 16;

impl CommunityVoteAccount {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
    + BOOL_LENGTH                       // The is NFT flag to indicate if this is an NFT community
    + PUBKEY_LENGTH                     // Mint or Creator Key
    + MAX_COMMUNITY__DESCRIPTION_LENGTH // The description that can be used to compare any NFT string
    + U64_LEN                           // Proposal count
    + U64_LEN;                          // Minimum token count
}

impl Proposal {
    pub const LEN: usize = DISCRIMINATOR_LENGTH 
    + U64_LEN                           // Proposal Id
    + PUBKEY_LENGTH                     // Owner
    + TIMESTAMP_LENGTH                  // Timestamp
    + MAX_PROPOSAL_TITLE_LENGTH         // Title 
    + MAX_PROPOSAL_DESCRIPTION_LENGTH   // Description 
    + PUBKEY_LENGTH                     // Mint or Creator
    + U64_LEN                           // minimum token
    + U8_LEN                            // vote system type
    + U64_LEN                           // total candidates to choose from
    + VOTE_COUNT_LENGTH                 // vote yes count
    + VOTE_COUNT_LENGTH                 // vote no count
    + U64_LEN                           // Vote total count
    + U64_LEN                           // Tally vote total count
    + BOOL_LENGTH                       // tally started
    + BOOL_LENGTH                       // voting finalized
    + BOOL_LENGTH                       // invalid proposal
    + BUMP_LENGTH
    + END_TIMESTAMP_LENGTH              // voting end timestamp
    + END_TIMESTAMP_LENGTH;             // finalize voting timestamp
}

impl Vote {
    pub const LEN: usize = DISCRIMINATOR_LENGTH 
    + U64_LEN                           // proposal id
    + BOOL_LENGTH                       // vote bool
    + U64_LEN                           // candidate
    + U64_LEN                           // voter weight
    + PUBKEY_LENGTH                     // Voter
    + PUBKEY_LENGTH                     // Mint or Creator
    + BOOL_LENGTH                       // Tally
    + TIMESTAMP_LENGTH                  // Created time stamp
    + BUMP_LENGTH;  
}