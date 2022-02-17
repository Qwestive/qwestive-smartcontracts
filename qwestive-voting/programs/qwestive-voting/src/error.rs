use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode  {
	#[msg("No Proposal at this index")]
	ProposalIndexOutOfBounds,
	#[msg("You have already voted for this proposal")]
	YouAlreadyVotedForThisProposal,
	#[msg("Title is too long. maximum: 80 character")]
	TitleIsTooLong,
	#[msg("Description is too long. maximum: 1024 character")]
	DescriptionIsTooLong,
    #[msg("The weight voting type is not a valid option")]
	InvalidVotingType,
	#[msg("Proposal deadline is past")]
	ProposalHasEnded,
	#[msg("Insufficient number of tokens to be included in voting session")]
	InsufficientTokensToVote,
    #[msg("This voting account has already been tallied")]
	VoteAccountAlreadyTallied,
    #[msg("The timing window for when the tally has ended")]
    TallyHasEnded,
    #[msg("This proposal has already been finalized and no changes can be made")]
	ProposalIsFinalized,
    #[msg("The number of candidates has exceeded the allowed amount for vote options")]
	TooManyCandidates,
}