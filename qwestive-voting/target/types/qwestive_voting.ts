export type QwestiveVoting = {
  "version": "0.1.0",
  "name": "qwestive_voting",
  "instructions": [
    {
      "name": "initializeVoting",
      "accounts": [
        {
          "name": "communityVoteAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityAccountBump",
          "type": "u8"
        },
        {
          "name": "communityName",
          "type": "string"
        },
        {
          "name": "minimumTokens",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initializeNftVoting",
      "accounts": [
        {
          "name": "communityVoteAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMintAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftMetadataAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityAccountBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "addProposal",
      "accounts": [
        {
          "name": "communityVoteAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalId",
          "type": "u64"
        },
        {
          "name": "title",
          "type": "string"
        },
        {
          "name": "description",
          "type": "string"
        },
        {
          "name": "minimumTokenCount",
          "type": "u64"
        },
        {
          "name": "voteSystemType",
          "type": "u8"
        },
        {
          "name": "threshold",
          "type": "u64"
        },
        {
          "name": "votingType",
          "type": "u8"
        },
        {
          "name": "totalCandidates",
          "type": "u64"
        },
        {
          "name": "votingEndTimestamp",
          "type": "u128"
        },
        {
          "name": "finalizeVoteEndTimestamp",
          "type": "u128"
        }
      ]
    },
    {
      "name": "addNftProposal",
      "accounts": [
        {
          "name": "communityVoteAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMintAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftMetadataAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalId",
          "type": "u64"
        },
        {
          "name": "title",
          "type": "string"
        },
        {
          "name": "description",
          "type": "string"
        },
        {
          "name": "minimumTokenCount",
          "type": "u64"
        },
        {
          "name": "voteSystemType",
          "type": "u8"
        },
        {
          "name": "threshold",
          "type": "u64"
        },
        {
          "name": "votingType",
          "type": "u8"
        },
        {
          "name": "totalCandidates",
          "type": "u64"
        },
        {
          "name": "votingEndTimestamp",
          "type": "u128"
        },
        {
          "name": "finalizeVoteEndTimestamp",
          "type": "u128"
        }
      ]
    },
    {
      "name": "voteForProposal",
      "accounts": [
        {
          "name": "communityVoteAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityAccountBump",
          "type": "u8"
        },
        {
          "name": "voteAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalId",
          "type": "u64"
        },
        {
          "name": "voteBool",
          "type": "bool"
        },
        {
          "name": "candidate",
          "type": "u64"
        }
      ]
    },
    {
      "name": "voteForNftProposal",
      "accounts": [
        {
          "name": "communityVoteAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMintAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftMetadataAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityAccountBump",
          "type": "u8"
        },
        {
          "name": "voteAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalId",
          "type": "u64"
        },
        {
          "name": "voteBool",
          "type": "bool"
        },
        {
          "name": "candidate",
          "type": "u64"
        }
      ]
    },
    {
      "name": "beginTally",
      "accounts": [
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "proposalId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "beginNftTally",
      "accounts": [
        {
          "name": "communityVoteAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMintAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftMetadataAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "tallyVote",
      "accounts": [
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "voteAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "tallyNftVote",
      "accounts": [
        {
          "name": "communityVoteAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMintAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftMetadataAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityAccountBump",
          "type": "u8"
        },
        {
          "name": "voteAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "finalizeVote",
      "accounts": [
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "proposalId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "finalizeNftVote",
      "accounts": [
        {
          "name": "communityVoteAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMintAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftMetadataAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalId",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "communityVoteAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "isNft",
            "type": "bool"
          },
          {
            "name": "key",
            "type": "publicKey"
          },
          {
            "name": "communityName",
            "type": "string"
          },
          {
            "name": "totalProposalCount",
            "type": "u64"
          },
          {
            "name": "minimumTokenCount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "proposal",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u64"
          },
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "createdTimestamp",
            "type": "i64"
          },
          {
            "name": "title",
            "type": "string"
          },
          {
            "name": "description",
            "type": "string"
          },
          {
            "name": "key",
            "type": "publicKey"
          },
          {
            "name": "minimumTokenCount",
            "type": "u64"
          },
          {
            "name": "voteSystemType",
            "type": "u8"
          },
          {
            "name": "threshold",
            "type": "u64"
          },
          {
            "name": "votingType",
            "type": "u8"
          },
          {
            "name": "totalCandidates",
            "type": "u64"
          },
          {
            "name": "voteYes",
            "type": "u64"
          },
          {
            "name": "voteNo",
            "type": "u64"
          },
          {
            "name": "totalVotes",
            "type": "u64"
          },
          {
            "name": "totalTally",
            "type": "u64"
          },
          {
            "name": "tallyStarted",
            "type": "bool"
          },
          {
            "name": "votingFinalized",
            "type": "bool"
          },
          {
            "name": "proposalTallyMismatch",
            "type": "bool"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "votingEndTimestamp",
            "type": "u128"
          },
          {
            "name": "finalizeVoteEndTimestamp",
            "type": "u128"
          }
        ]
      }
    },
    {
      "name": "vote",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposalId",
            "type": "u64"
          },
          {
            "name": "tallyCompleted",
            "type": "bool"
          },
          {
            "name": "voteBool",
            "type": "bool"
          },
          {
            "name": "candidate",
            "type": "u64"
          },
          {
            "name": "voterWeight",
            "type": "u64"
          },
          {
            "name": "voter",
            "type": "publicKey"
          },
          {
            "name": "key",
            "type": "publicKey"
          },
          {
            "name": "createdTimestamp",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "ProposalIndexOutOfBounds",
      "msg": "No Proposal at this index"
    },
    {
      "code": 6001,
      "name": "ProposalIDMismatch",
      "msg": "The requested proposal id does not match the proposal account"
    },
    {
      "code": 6002,
      "name": "CandidateIndexOutOfBounds",
      "msg": "There is no candidate vote with this number"
    },
    {
      "code": 6003,
      "name": "YouAlreadyVotedForThisProposal",
      "msg": "You have already voted for this proposal"
    },
    {
      "code": 6004,
      "name": "CommunityNameIsTooLong",
      "msg": "Community name is too long. maximum: 80 character"
    },
    {
      "code": 6005,
      "name": "TitleIsTooLong",
      "msg": "Title is too long. maximum: 80 character"
    },
    {
      "code": 6006,
      "name": "DescriptionIsTooLong",
      "msg": "Description is too long. maximum: 1024 character"
    },
    {
      "code": 6007,
      "name": "InvalidVotingType",
      "msg": "The weight voting type is not a valid option"
    },
    {
      "code": 6008,
      "name": "InvalidVotingSystemType",
      "msg": "The voting system type is not a valid option"
    },
    {
      "code": 6009,
      "name": "ProposalHasEnded",
      "msg": "Proposal deadline is past"
    },
    {
      "code": 6010,
      "name": "ProposalVotingFinalized",
      "msg": "Proposal voting is finalized no changes can be made"
    },
    {
      "code": 6011,
      "name": "VotingTimeHasNotEnded",
      "msg": "The voting time has not ended"
    },
    {
      "code": 6012,
      "name": "FinalizeVoteTimeHasNotEnded",
      "msg": "The time for tallying votes has not ended wait until tally time has ended"
    },
    {
      "code": 6013,
      "name": "VotingEndTimestampTooSmall",
      "msg": "The timestamp of voting end must be greater than the current timestamp"
    },
    {
      "code": 6014,
      "name": "FinalizeTimestampTooSmall",
      "msg": "The timestamp of finalizing proposal must be greater than the voting end timestamp and current timestamp"
    },
    {
      "code": 6015,
      "name": "InsufficientTokensToVote",
      "msg": "Insufficient number of tokens to be included in voting session"
    },
    {
      "code": 6016,
      "name": "VoteAccountAlreadyTallied",
      "msg": "This voting account has already been tallied"
    },
    {
      "code": 6017,
      "name": "ProposalTallyAlreadyStarted",
      "msg": "The proposal tally has already started"
    },
    {
      "code": 6018,
      "name": "TallyHasNotStarted",
      "msg": "Vote tally has not started tally must begin before tallying vote"
    },
    {
      "code": 6019,
      "name": "TallyHasStarted",
      "msg": "Vote tally has started no new votes can be submitted"
    },
    {
      "code": 6020,
      "name": "TallyHasEnded",
      "msg": "The timing window for any vote tally has ended"
    },
    {
      "code": 6021,
      "name": "ProposalIsFinalized",
      "msg": "This proposal has already been finalized and no changes can be made"
    },
    {
      "code": 6022,
      "name": "TooManyCandidates",
      "msg": "The number of candidates has exceeded the allowed amount for vote options"
    },
    {
      "code": 6023,
      "name": "InvalidVoteParameter",
      "msg": "The vote parameter entered is invalid for this type of voting"
    },
    {
      "code": 6024,
      "name": "InvalidTokenAccount",
      "msg": "The token account is not the correct mint"
    },
    {
      "code": 6025,
      "name": "InsufficientTokenBalance",
      "msg": "The token account balance is less than the required balance for the community"
    },
    {
      "code": 6026,
      "name": "UnauthorizedTokenHolder",
      "msg": "The user is not the owner of the token account"
    },
    {
      "code": 6027,
      "name": "UnauthorizedAccount",
      "msg": "The account provided is not owned by the Qwestive Voting program"
    },
    {
      "code": 6028,
      "name": "MetadataAccountMismatch",
      "msg": "The metadata account provided does not match the expected key"
    },
    {
      "code": 6029,
      "name": "MetadataAccountNotInitialized",
      "msg": "The metadata account is not initialized"
    },
    {
      "code": 6030,
      "name": "CreatorKeyMismatch",
      "msg": "The NFT token creator does not match the expected creator"
    },
    {
      "code": 6031,
      "name": "CreatorKeyIsNotVerified",
      "msg": "The NFT creator key is not verified"
    },
    {
      "code": 6032,
      "name": "CollectionNameMismatch",
      "msg": "The collection name does not match the expected name"
    }
  ]
};

export const IDL: QwestiveVoting = {
  "version": "0.1.0",
  "name": "qwestive_voting",
  "instructions": [
    {
      "name": "initializeVoting",
      "accounts": [
        {
          "name": "communityVoteAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityAccountBump",
          "type": "u8"
        },
        {
          "name": "communityName",
          "type": "string"
        },
        {
          "name": "minimumTokens",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initializeNftVoting",
      "accounts": [
        {
          "name": "communityVoteAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMintAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftMetadataAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityAccountBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "addProposal",
      "accounts": [
        {
          "name": "communityVoteAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalId",
          "type": "u64"
        },
        {
          "name": "title",
          "type": "string"
        },
        {
          "name": "description",
          "type": "string"
        },
        {
          "name": "minimumTokenCount",
          "type": "u64"
        },
        {
          "name": "voteSystemType",
          "type": "u8"
        },
        {
          "name": "threshold",
          "type": "u64"
        },
        {
          "name": "votingType",
          "type": "u8"
        },
        {
          "name": "totalCandidates",
          "type": "u64"
        },
        {
          "name": "votingEndTimestamp",
          "type": "u128"
        },
        {
          "name": "finalizeVoteEndTimestamp",
          "type": "u128"
        }
      ]
    },
    {
      "name": "addNftProposal",
      "accounts": [
        {
          "name": "communityVoteAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMintAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftMetadataAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalId",
          "type": "u64"
        },
        {
          "name": "title",
          "type": "string"
        },
        {
          "name": "description",
          "type": "string"
        },
        {
          "name": "minimumTokenCount",
          "type": "u64"
        },
        {
          "name": "voteSystemType",
          "type": "u8"
        },
        {
          "name": "threshold",
          "type": "u64"
        },
        {
          "name": "votingType",
          "type": "u8"
        },
        {
          "name": "totalCandidates",
          "type": "u64"
        },
        {
          "name": "votingEndTimestamp",
          "type": "u128"
        },
        {
          "name": "finalizeVoteEndTimestamp",
          "type": "u128"
        }
      ]
    },
    {
      "name": "voteForProposal",
      "accounts": [
        {
          "name": "communityVoteAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityAccountBump",
          "type": "u8"
        },
        {
          "name": "voteAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalId",
          "type": "u64"
        },
        {
          "name": "voteBool",
          "type": "bool"
        },
        {
          "name": "candidate",
          "type": "u64"
        }
      ]
    },
    {
      "name": "voteForNftProposal",
      "accounts": [
        {
          "name": "communityVoteAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMintAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftMetadataAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityAccountBump",
          "type": "u8"
        },
        {
          "name": "voteAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalId",
          "type": "u64"
        },
        {
          "name": "voteBool",
          "type": "bool"
        },
        {
          "name": "candidate",
          "type": "u64"
        }
      ]
    },
    {
      "name": "beginTally",
      "accounts": [
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "proposalId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "beginNftTally",
      "accounts": [
        {
          "name": "communityVoteAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMintAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftMetadataAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "tallyVote",
      "accounts": [
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "voteAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "tallyNftVote",
      "accounts": [
        {
          "name": "communityVoteAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMintAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftMetadataAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityAccountBump",
          "type": "u8"
        },
        {
          "name": "voteAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "finalizeVote",
      "accounts": [
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "proposalId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "finalizeNftVote",
      "accounts": [
        {
          "name": "communityVoteAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMintAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftMetadataAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityAccountBump",
          "type": "u8"
        },
        {
          "name": "proposalId",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "communityVoteAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "isNft",
            "type": "bool"
          },
          {
            "name": "key",
            "type": "publicKey"
          },
          {
            "name": "communityName",
            "type": "string"
          },
          {
            "name": "totalProposalCount",
            "type": "u64"
          },
          {
            "name": "minimumTokenCount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "proposal",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u64"
          },
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "createdTimestamp",
            "type": "i64"
          },
          {
            "name": "title",
            "type": "string"
          },
          {
            "name": "description",
            "type": "string"
          },
          {
            "name": "key",
            "type": "publicKey"
          },
          {
            "name": "minimumTokenCount",
            "type": "u64"
          },
          {
            "name": "voteSystemType",
            "type": "u8"
          },
          {
            "name": "threshold",
            "type": "u64"
          },
          {
            "name": "votingType",
            "type": "u8"
          },
          {
            "name": "totalCandidates",
            "type": "u64"
          },
          {
            "name": "voteYes",
            "type": "u64"
          },
          {
            "name": "voteNo",
            "type": "u64"
          },
          {
            "name": "totalVotes",
            "type": "u64"
          },
          {
            "name": "totalTally",
            "type": "u64"
          },
          {
            "name": "tallyStarted",
            "type": "bool"
          },
          {
            "name": "votingFinalized",
            "type": "bool"
          },
          {
            "name": "proposalTallyMismatch",
            "type": "bool"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "votingEndTimestamp",
            "type": "u128"
          },
          {
            "name": "finalizeVoteEndTimestamp",
            "type": "u128"
          }
        ]
      }
    },
    {
      "name": "vote",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposalId",
            "type": "u64"
          },
          {
            "name": "tallyCompleted",
            "type": "bool"
          },
          {
            "name": "voteBool",
            "type": "bool"
          },
          {
            "name": "candidate",
            "type": "u64"
          },
          {
            "name": "voterWeight",
            "type": "u64"
          },
          {
            "name": "voter",
            "type": "publicKey"
          },
          {
            "name": "key",
            "type": "publicKey"
          },
          {
            "name": "createdTimestamp",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "ProposalIndexOutOfBounds",
      "msg": "No Proposal at this index"
    },
    {
      "code": 6001,
      "name": "ProposalIDMismatch",
      "msg": "The requested proposal id does not match the proposal account"
    },
    {
      "code": 6002,
      "name": "CandidateIndexOutOfBounds",
      "msg": "There is no candidate vote with this number"
    },
    {
      "code": 6003,
      "name": "YouAlreadyVotedForThisProposal",
      "msg": "You have already voted for this proposal"
    },
    {
      "code": 6004,
      "name": "CommunityNameIsTooLong",
      "msg": "Community name is too long. maximum: 80 character"
    },
    {
      "code": 6005,
      "name": "TitleIsTooLong",
      "msg": "Title is too long. maximum: 80 character"
    },
    {
      "code": 6006,
      "name": "DescriptionIsTooLong",
      "msg": "Description is too long. maximum: 1024 character"
    },
    {
      "code": 6007,
      "name": "InvalidVotingType",
      "msg": "The weight voting type is not a valid option"
    },
    {
      "code": 6008,
      "name": "InvalidVotingSystemType",
      "msg": "The voting system type is not a valid option"
    },
    {
      "code": 6009,
      "name": "ProposalHasEnded",
      "msg": "Proposal deadline is past"
    },
    {
      "code": 6010,
      "name": "ProposalVotingFinalized",
      "msg": "Proposal voting is finalized no changes can be made"
    },
    {
      "code": 6011,
      "name": "VotingTimeHasNotEnded",
      "msg": "The voting time has not ended"
    },
    {
      "code": 6012,
      "name": "FinalizeVoteTimeHasNotEnded",
      "msg": "The time for tallying votes has not ended wait until tally time has ended"
    },
    {
      "code": 6013,
      "name": "VotingEndTimestampTooSmall",
      "msg": "The timestamp of voting end must be greater than the current timestamp"
    },
    {
      "code": 6014,
      "name": "FinalizeTimestampTooSmall",
      "msg": "The timestamp of finalizing proposal must be greater than the voting end timestamp and current timestamp"
    },
    {
      "code": 6015,
      "name": "InsufficientTokensToVote",
      "msg": "Insufficient number of tokens to be included in voting session"
    },
    {
      "code": 6016,
      "name": "VoteAccountAlreadyTallied",
      "msg": "This voting account has already been tallied"
    },
    {
      "code": 6017,
      "name": "ProposalTallyAlreadyStarted",
      "msg": "The proposal tally has already started"
    },
    {
      "code": 6018,
      "name": "TallyHasNotStarted",
      "msg": "Vote tally has not started tally must begin before tallying vote"
    },
    {
      "code": 6019,
      "name": "TallyHasStarted",
      "msg": "Vote tally has started no new votes can be submitted"
    },
    {
      "code": 6020,
      "name": "TallyHasEnded",
      "msg": "The timing window for any vote tally has ended"
    },
    {
      "code": 6021,
      "name": "ProposalIsFinalized",
      "msg": "This proposal has already been finalized and no changes can be made"
    },
    {
      "code": 6022,
      "name": "TooManyCandidates",
      "msg": "The number of candidates has exceeded the allowed amount for vote options"
    },
    {
      "code": 6023,
      "name": "InvalidVoteParameter",
      "msg": "The vote parameter entered is invalid for this type of voting"
    },
    {
      "code": 6024,
      "name": "InvalidTokenAccount",
      "msg": "The token account is not the correct mint"
    },
    {
      "code": 6025,
      "name": "InsufficientTokenBalance",
      "msg": "The token account balance is less than the required balance for the community"
    },
    {
      "code": 6026,
      "name": "UnauthorizedTokenHolder",
      "msg": "The user is not the owner of the token account"
    },
    {
      "code": 6027,
      "name": "UnauthorizedAccount",
      "msg": "The account provided is not owned by the Qwestive Voting program"
    },
    {
      "code": 6028,
      "name": "MetadataAccountMismatch",
      "msg": "The metadata account provided does not match the expected key"
    },
    {
      "code": 6029,
      "name": "MetadataAccountNotInitialized",
      "msg": "The metadata account is not initialized"
    },
    {
      "code": 6030,
      "name": "CreatorKeyMismatch",
      "msg": "The NFT token creator does not match the expected creator"
    },
    {
      "code": 6031,
      "name": "CreatorKeyIsNotVerified",
      "msg": "The NFT creator key is not verified"
    },
    {
      "code": 6032,
      "name": "CollectionNameMismatch",
      "msg": "The collection name does not match the expected name"
    }
  ]
};
