export type QwestiveVoting = {
  "version": "0.1.0",
  "name": "qwestive_voting",
  "instructions": [
    {
      "name": "initializeVoting",
      "accounts": [
        {
          "name": "baseAccount",
          "isMut": true,
          "isSigner": true
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
      "args": []
    },
    {
      "name": "addProposal",
      "accounts": [
        {
          "name": "baseAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proposal",
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
          "name": "endTimeStamp",
          "type": "u128"
        }
      ]
    },
    {
      "name": "voteForProposal",
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
        },
        {
          "name": "vote",
          "type": "bool"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "baseAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "totalProposalCount",
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
            "name": "createdAt",
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
            "name": "voteYes",
            "type": "u64"
          },
          {
            "name": "voteNo",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "endTimeStamp",
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
            "name": "vote",
            "type": "bool"
          },
          {
            "name": "voter",
            "type": "publicKey"
          },
          {
            "name": "createdAt",
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
      "name": "YouAlreadyVotedForThisProposal",
      "msg": "You have already voted for this proposal"
    },
    {
      "code": 6002,
      "name": "TitleIsTooLong",
      "msg": "Title is too long. maximum: 80 character"
    },
    {
      "code": 6003,
      "name": "DescriptionIsTooLong",
      "msg": "Description is too long. maximum: 1024 character"
    },
    {
      "code": 6004,
      "name": "ProposalHasEnded",
      "msg": "Proposal deadline is past"
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
          "name": "baseAccount",
          "isMut": true,
          "isSigner": true
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
      "args": []
    },
    {
      "name": "addProposal",
      "accounts": [
        {
          "name": "baseAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proposal",
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
          "name": "endTimeStamp",
          "type": "u128"
        }
      ]
    },
    {
      "name": "voteForProposal",
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
        },
        {
          "name": "vote",
          "type": "bool"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "baseAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "totalProposalCount",
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
            "name": "createdAt",
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
            "name": "voteYes",
            "type": "u64"
          },
          {
            "name": "voteNo",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "endTimeStamp",
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
            "name": "vote",
            "type": "bool"
          },
          {
            "name": "voter",
            "type": "publicKey"
          },
          {
            "name": "createdAt",
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
      "name": "YouAlreadyVotedForThisProposal",
      "msg": "You have already voted for this proposal"
    },
    {
      "code": 6002,
      "name": "TitleIsTooLong",
      "msg": "Title is too long. maximum: 80 character"
    },
    {
      "code": 6003,
      "name": "DescriptionIsTooLong",
      "msg": "Description is too long. maximum: 1024 character"
    },
    {
      "code": 6004,
      "name": "ProposalHasEnded",
      "msg": "Proposal deadline is past"
    }
  ]
};
