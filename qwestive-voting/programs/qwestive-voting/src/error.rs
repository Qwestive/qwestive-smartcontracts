use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode  {
	#[msg("No Proposal at this index")]
	ProposalIndexOutOfBounds,
	#[msg("The requested proposal id does not match the proposal account")]
	ProposalIDMismatch,
	#[msg("There is no candidate vote with this number")]
	CandidateIndexOutOfBounds,
	#[msg("You have already voted for this proposal")]
	YouAlreadyVotedForThisProposal,
	#[msg("Title is too long. maximum: 80 character")]
	TitleIsTooLong,
	#[msg("Description is too long. maximum: 1024 character")]
	DescriptionIsTooLong,
    #[msg("The weight voting type is not a valid option")]
	InvalidVotingType,
	#[msg("The voting system type is not a valid option")]
	InvalidVotingSystemType,
	#[msg("Proposal deadline is past")]
	ProposalHasEnded,
	#[msg("Proposal voting is finalized no changes can be made")]
	ProposalVotingFinalized,
	#[msg("The voting time has not ended")]
	VotingTimeHasNotEnded,
	#[msg("The time for tallying votes has not ended wait until tally time has ended")]
	FinalizeVoteTimeHasNotEnded,
	#[msg("The timestamp of voting end must be greater than the current timestamp")]
	VotingEndTimestampTooSmall,
	#[msg("The timestamp of finalizing proposal must be greater than the voting end timestamp and current timestamp")]
	FinalizeTimestampTooSmall,
	#[msg("Insufficient number of tokens to be included in voting session")]
	InsufficientTokensToVote,
    #[msg("This voting account has already been tallied")]
	VoteAccountAlreadyTallied,
	#[msg("The proposal tally has already started")]
	ProposalTallyAlreadyStarted,
	#[msg("Vote tally has not started tally must begin before tallying vote")]
    TallyHasNotStarted,
	#[msg("Vote tally has started no new votes can be submitted")]
    TallyHasStarted,
    #[msg("The timing window for any vote tally has ended")]
    TallyHasEnded,
    #[msg("This proposal has already been finalized and no changes can be made")]
	ProposalIsFinalized,
    #[msg("The number of candidates has exceeded the allowed amount for vote options")]
	TooManyCandidates,
	#[msg("The vote parameter entered is invalid for this type of voting")]
	InvalidVoteParameter,
	#[msg("The token account is not the correct mint")]
	InvalidTokenAccount,
	#[msg("The token account balance is less than the required balance for the community")]
	InsufficientTokenBalance,
	#[msg("The user is not the owner of the token account")]
	UnauthorizedTokenHolder,
	#[msg("The account provided is not owned by the Qwestive Voting program")]
	UnauthorizedAccount,
}