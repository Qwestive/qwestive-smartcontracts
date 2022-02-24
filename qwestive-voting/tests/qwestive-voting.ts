import { QwestiveVoting } from "../target/types/qwestive_voting";
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import assert from "assert";
import * as bs58 from "bs58";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
import * as splToken from "@solana/spl-token";
import * as serumCmn from "@project-serum/common";


describe("qwestive-voting", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.QwestiveVoting as Program<QwestiveVoting>;

  // The mint token and the voteTokenAccount
  let mintA: Token = null;
  let voter1TokenAAccount: PublicKey = null;
  let voter1TokenBAccount: PublicKey = null;
  let voter3TokenAAccount: PublicKey = null;

  let mintB: Token = null;
  let voter2TokenBAccount: PublicKey = null;
  
  // Mint Authority
  const mintAuthority = anchor.web3.Keypair.generate();
  const mintBAuthority = anchor.web3.Keypair.generate();

  // The token account
  const tokenWallet = anchor.web3.Keypair.generate();
  const tokenAmountOwned = 500;

  const secondTokenWallet = anchor.web3.Keypair.generate();

  // The Account to create.
  const communityVoteAccount = anchor.web3.Keypair.generate();
  const communityBVoteAccount = anchor.web3.Keypair.generate();

  it("Initialize token account", async () => {
    // Airdropping tokens to a payer.
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(tokenWallet.publicKey, 1 * LAMPORTS_PER_SOL),
      "confirmed"
    );

    // Airdropping tokens to a payer.
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(secondTokenWallet.publicKey, 1 * LAMPORTS_PER_SOL),
      "confirmed"
    );

    mintA = await Token.createMint(
      provider.connection,
      tokenWallet,
      mintAuthority.publicKey,
      null,
      0,
      TOKEN_PROGRAM_ID
    );

    mintB = await Token.createMint(
      provider.connection,
      secondTokenWallet,
      mintBAuthority.publicKey,
      null,
      0,
      TOKEN_PROGRAM_ID
    );

    voter1TokenAAccount = await mintA.createAccount(provider.wallet.publicKey);
    voter1TokenBAccount = await mintB.createAccount(provider.wallet.publicKey);
    voter2TokenBAccount = await mintB.createAccount(secondTokenWallet.publicKey);
    voter3TokenAAccount = await mintA.createAccount(provider.wallet.publicKey);

    await mintA.mintTo(
      voter1TokenAAccount,
      mintAuthority.publicKey,
      [mintAuthority],
      tokenAmountOwned
    );

    await mintA.mintTo(
      voter3TokenAAccount,
      mintAuthority.publicKey,
      [mintAuthority],
      1
    );

    await mintB.mintTo(
      voter1TokenBAccount,
      mintBAuthority.publicKey,
      [mintBAuthority],
      100
    );

    await mintB.mintTo(
      voter2TokenBAccount,
      mintBAuthority.publicKey,
      [mintBAuthority],
      200
    );

    let _voter1TokenAAccount = await mintA.getAccountInfo(
      voter1TokenAAccount
    );

    let _voter1TokenBAccount = await mintB.getAccountInfo(
      voter1TokenBAccount
    );

    let _voter2TokenBAccount= await mintB.getAccountInfo(
      voter2TokenBAccount
    );

    let _voter3TokenAAccount= await mintA.getAccountInfo(
      voter3TokenAAccount
    );

     assert.ok(_voter1TokenAAccount.amount.toNumber() == tokenAmountOwned);
     assert.ok(_voter1TokenBAccount.amount.toNumber() == 100);
     assert.ok(_voter2TokenBAccount.amount.toNumber() == 200);
     assert.ok(_voter3TokenAAccount.amount.toNumber() == 1);
  });

  async function findAssociatedTokenAddress(
    walletAddress: anchor.web3.PublicKey,
    tokenMintAddress: anchor.web3.PublicKey
  ): Promise<anchor.web3.PublicKey> {
    return (
      await anchor.web3.PublicKey.findProgramAddress(
        [
          walletAddress.toBuffer(),
          TOKEN_PROGRAM_ID.toBuffer(),
          tokenMintAddress.toBuffer(),
        ],
        ASSOCIATED_TOKEN_PROGRAM_ID
      )
    )[0];
  }

  const DAY_IN_UNIX = 24 * 60 * 60 * 1000;
  const HOURS_IN_UNIX = 60 * 60 * 1000;
  const MINUTES_IN_UNIX = 60 * 1000;

  const getNumberBuffer = (total: number, alloc = 8) => {
    const totalProposalAccountBuf = Buffer.alloc(alloc);
    totalProposalAccountBuf.writeUIntLE(total, 0, 6);
    return totalProposalAccountBuf;
  };

  const newUser = anchor.web3.Keypair.generate();
  before(async () => {
    const signature = await program.provider.connection.requestAirdrop(
      newUser.publicKey,
      1 * LAMPORTS_PER_SOL
    );
    await program.provider.connection.confirmTransaction(signature);
  });

  it("Initialize Community Voting", async () => {
    // Add your test here.
    const tx = await program.rpc.initializeVoting(new anchor.BN(0), 
    {
      accounts: {
        communityVoteAccount: communityVoteAccount.publicKey,
        tokenAccount: voter1TokenAAccount,
        mint: mintA.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [communityVoteAccount],
    });

    const communityA = await program.account.communityVoteAccount.fetch(
      communityVoteAccount.publicKey
    );

    const tx2 = await program.rpc.initializeVoting(new anchor.BN(1), 
    {
      accounts: {
        communityVoteAccount: communityBVoteAccount.publicKey,
        tokenAccount: voter1TokenBAccount,
        mint: mintB.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [communityBVoteAccount],
    });

    const communityB = await program.account.communityVoteAccount.fetch(
      communityBVoteAccount.publicKey
    );

    assert.equal(communityA.totalProposalCount, 0);
    assert.equal(communityA.minimumTokenCount, 0);
    assert.equal(communityA.mint.toBase58(), mintA.publicKey.toBase58());

    assert.equal(communityB.totalProposalCount, 0);
    assert.equal(communityB.minimumTokenCount, 1);
    assert.equal(communityB.mint.toBase58(), mintB.publicKey.toBase58());
  });

  it("Can add a proposal!", async () => {
    const account = await program.account.communityVoteAccount.fetch(
      communityVoteAccount.publicKey
    );
    console.log("Your account", account);
    const proposalId = getNumberBuffer(account.totalProposalCount.toNumber());
    const [proposalAccountPublicKey, accountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("proposal_account"), proposalId],
        anchor.workspace.QwestiveVoting.programId
      );

    await program.rpc.addProposal(
      accountBump,
      account.totalProposalCount,
      "Test Title",
      "Test Description",
      new anchor.BN(0), // minimum_token_count
      new anchor.BN(1), // voting system - one vote per token account
      new anchor.BN(1), // threshold - needs at least 1 vote 
      new anchor.BN(0), // voting_type - 0 - yes or not, 1 - multiple choice is not currently supported
      new anchor.BN(+new Date() + 1 * DAY_IN_UNIX), //voting_end_timestamp
      new anchor.BN(+new Date() + 2 * DAY_IN_UNIX), //finalize_vote_end_timestamp
      {
        accounts: {
          communityVoteAccount: communityVoteAccount.publicKey,
          proposal: proposalAccountPublicKey,
          tokenAccount: voter1TokenAAccount,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
      }
    );
  });

  it("Can add a second proposal!", async () => {
    let account = await program.account.communityVoteAccount.fetch(
      communityVoteAccount.publicKey
    );

    const secondProposalId = getNumberBuffer(
      account.totalProposalCount.toNumber()
    );
    const [secondProposalAccountPublicKey, secondAccountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("proposal_account"), secondProposalId],
        anchor.workspace.QwestiveVoting.programId
      );

    console.log("SECOND:", secondProposalAccountPublicKey, secondAccountBump);

    await program.rpc.addProposal(
      secondAccountBump,
      account.totalProposalCount,
      "Second Test Title",
      "Second Test Description",
      new anchor.BN(0), // minimum_token_count
      new anchor.BN(1), // voting system - one vote per token account
      new anchor.BN(2), // threshold - needs at least 1 vote 
      new anchor.BN(0), // voting_type - 0 - yes or not, 1 - multiple choice is not currently supported
      new anchor.BN(+new Date() + 1 * DAY_IN_UNIX), //voting_end_timestamp
      new anchor.BN(+new Date() + 2 * DAY_IN_UNIX), //finalize_vote_end_timestamp
      {
        accounts: {
          communityVoteAccount: communityVoteAccount.publicKey,
          proposal: secondProposalAccountPublicKey,
          tokenAccount: voter1TokenAAccount,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
      }
    );

    account = await program.account.communityVoteAccount.fetch(communityVoteAccount.publicKey);

    const proposals = await program.account.proposal.all();
    assert.ok(proposals.length === account.totalProposalCount.toNumber());
  });

  it("Can NOT add a proposal with wrong token account!", async () => {
    let account = await program.account.communityVoteAccount.fetch(
      communityVoteAccount.publicKey
    );

    const thirdProposalId = getNumberBuffer(
      account.totalProposalCount.toNumber()
    );
    const [thirdProposalAccountPublicKey, thirdAccountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("proposal_account"), thirdProposalId],
        anchor.workspace.QwestiveVoting.programId
      );

    console.log("THIRD:", thirdProposalAccountPublicKey, thirdAccountBump);
    await assert.rejects(
      async () => {
      await program.rpc.addProposal(
        thirdAccountBump,
        account.totalProposalCount,
        "Third Test Title",
        "Third Test Description",
        new anchor.BN(0), // minimum_token_count
        new anchor.BN(1), // voting system - one vote per token account
        new anchor.BN(1), // threshold - needs at least 1 vote 
        new anchor.BN(0), // voting_type - 0 - yes or not, 1 - multiple choice is not currently supported
        new anchor.BN(+new Date() + 1 * DAY_IN_UNIX), //voting_end_timestamp
        new anchor.BN(+new Date() + 2 * DAY_IN_UNIX), //finalize_vote_end_timestamp
        {
          accounts: {
            communityVoteAccount: communityVoteAccount.publicKey,
            proposal: thirdProposalAccountPublicKey,
            tokenAccount: voter1TokenBAccount,
            user: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
          },
        });
      },
      {
        name: "Error",
        // message: "301: You have already voted for this proposal",
      }
    );

    account = await program.account.communityVoteAccount.fetch(communityVoteAccount.publicKey);

    const proposals = await program.account.proposal.all();
    assert.ok(proposals.length === 2);
  });

  it("Can add a new proposal with a different token account!", async () => {
    let account = await program.account.communityVoteAccount.fetch(
      communityBVoteAccount.publicKey
    );

    console.log("Your Token B account", account);

    // Assign a new number with the next proposal id available
    const tokenBProposalId = getNumberBuffer(2);

    const [tokenBProposalAccountPublicKey, tokenBAccountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("proposal_account"), tokenBProposalId],
        anchor.workspace.QwestiveVoting.programId
      );

    console.log("Token B FIRST:", tokenBProposalAccountPublicKey, tokenBAccountBump);

    await program.rpc.addProposal(
      tokenBAccountBump,
      new anchor.BN(2),
      "Token B Test Title",
      "Token B Test Description",
      new anchor.BN(0), // minimum_token_count
      new anchor.BN(1), // voting system - one vote per token account
      new anchor.BN(1), // threshold - needs at least 1 vote 
      new anchor.BN(0), // voting_type - 0 - yes or not, 1 - multiple choice is not currently supported
      new anchor.BN(+new Date() + 1 * DAY_IN_UNIX), //voting_end_timestamp
      new anchor.BN(+new Date() + 2 * DAY_IN_UNIX), //finalize_vote_end_timestamp
      {
        accounts: {
          communityVoteAccount: communityBVoteAccount.publicKey,
          proposal: tokenBProposalAccountPublicKey,
          tokenAccount: voter1TokenBAccount,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
      }
    );

    account = await program.account.communityVoteAccount.fetch(communityBVoteAccount.publicKey);

    const proposals = await program.account.proposal.all();

    let token_a_account = await program.account.communityVoteAccount.fetch(
      communityVoteAccount.publicKey
    );

    assert.ok(proposals.length === token_a_account.totalProposalCount.toNumber() + account.totalProposalCount.toNumber());
    assert.ok(account.totalProposalCount.toNumber() == 1);
  });

  it("Can vote for a proposal!", async () => {
    const proposalId = getNumberBuffer(0);
    const [proposalAccountPublicKey] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("proposal_account"), proposalId],
        anchor.workspace.QwestiveVoting.programId
      );

    const [voteAccountPublicKey, voteBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("vote_account"),
          proposalId,
          provider.wallet.publicKey.toBuffer(),
        ],
        anchor.workspace.QwestiveVoting.programId
      );

    await program.rpc.voteForProposal(
      voteBump,                   // vote bump account
      new anchor.BN(0),           // proposal id
      true,                       // vote_bool  true - yes and false - no
      new anchor.BN(0),           // candidate = 0 to implies multiple choice is not selected
      {
        accounts: {
          communityVoteAccount: communityVoteAccount.publicKey,
          proposal: proposalAccountPublicKey,
          vote: voteAccountPublicKey,
          tokenAccount: voter1TokenAAccount,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
    });

    const vote = await program.account.vote.all();
    assert.equal(vote.length, 1);
    const account = await program.account.communityVoteAccount.fetch(
      communityVoteAccount.publicKey
    );
    assert.ok(account.totalProposalCount.toNumber() === 2);
  });

  it("Can vote for a second proposal!", async () => {
    const proposalId = getNumberBuffer(1);
    const [proposalAccountPublicKey] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("proposal_account"), proposalId],
        anchor.workspace.QwestiveVoting.programId
      );

    const [voteAccountPublicKey, voteBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("vote_account"),
          proposalId,
          provider.wallet.publicKey.toBuffer(),
        ],
        anchor.workspace.QwestiveVoting.programId
      );
    await program.rpc.voteForProposal(
      voteBump,                           // vote bump account
      new anchor.BN(1),                   // proposal id
      false,                              // vote_bool  true - yes and false - no
      new anchor.BN(0), {                 // candidate = 0 to implies multiple choice is not selected
      accounts: {
        communityVoteAccount: communityVoteAccount.publicKey,
        proposal: proposalAccountPublicKey,
        user: provider.wallet.publicKey,
        vote: voteAccountPublicKey,
        tokenAccount: voter1TokenAAccount,
        systemProgram: SystemProgram.programId,
      },
    });
    const vote = await program.account.vote.all();
    assert.equal(vote.length, 2);
  });

  it("Can not vote for a same proposal twice!", async () => {
    await assert.rejects(
      async () => {
        const proposalId = getNumberBuffer(0);
        const [proposalAccountPublicKey] =
          await anchor.web3.PublicKey.findProgramAddress(
            [Buffer.from("proposal_account"), proposalId],
            anchor.workspace.QwestiveVoting.programId
          );

        const [voteAccountPublicKey, voteBump] =
          await anchor.web3.PublicKey.findProgramAddress(
            [
              Buffer.from("vote_account"),
              proposalId,
              provider.wallet.publicKey.toBuffer(),
            ],
            anchor.workspace.QwestiveVoting.programId
          );
        await program.rpc.voteForProposal(
          voteBump, 
          new anchor.BN(0), 
          true, 
          new anchor.BN(0), {
          accounts: {
            communityVoteAccount: communityVoteAccount.publicKey,
            proposal: proposalAccountPublicKey,
            user: provider.wallet.publicKey,
            vote: voteAccountPublicKey,
            tokenAccount: voter1TokenAAccount,
            systemProgram: SystemProgram.programId,
          },
        });
      },
      {
        name: "Error",
        // message: "301: You have already voted for this proposal",
      }
    );
    const vote = await program.account.vote.all();
    assert.equal(vote.length, 2);
  });

  it("Can not vote if insufficient token count!", async () => {
    await assert.rejects(
      async () => {
        const proposalId = getNumberBuffer(1);
        const [proposalAccountPublicKey] =
          await anchor.web3.PublicKey.findProgramAddress(
            [Buffer.from("proposal_account"), proposalId],
            anchor.workspace.QwestiveVoting.programId
          );

        const [voteAccountPublicKey, voteBump] =
          await anchor.web3.PublicKey.findProgramAddress(
            [
              Buffer.from("vote_account"),
              proposalId,
              provider.wallet.publicKey.toBuffer(),
            ],
            anchor.workspace.QwestiveVoting.programId
          );
        await program.rpc.voteForProposal(
          voteBump, 
          new anchor.BN(1), 
          true, 
          new anchor.BN(0), {
          accounts: {
            communityVoteAccount: communityVoteAccount.publicKey,
            proposal: proposalAccountPublicKey,
            user: provider.wallet.publicKey,
            vote: voteAccountPublicKey,
            tokenAccount: voter3TokenAAccount,
            systemProgram: SystemProgram.programId,
          },
        });
      },
      {
        name: "Error",
        // message: "301: You have already voted for this proposal",
      }
    );
    const vote = await program.account.vote.all();
    assert.equal(vote.length, 2);
  });


  it("Can not vote for a proposal with the wrong token account!", async () => {
    await assert.rejects(
      async () => {
        const proposalId = getNumberBuffer(2);
        const [proposalAccountPublicKey] =
          await anchor.web3.PublicKey.findProgramAddress(
            [Buffer.from("proposal_account"), proposalId],
            anchor.workspace.QwestiveVoting.programId
          );

        const [voteAccountPublicKey, voteBump] =
          await anchor.web3.PublicKey.findProgramAddress(
            [
              Buffer.from("vote_account"),
              proposalId,
              provider.wallet.publicKey.toBuffer(),
            ],
            anchor.workspace.QwestiveVoting.programId
          );
        await program.rpc.voteForProposal(
          voteBump, 
          new anchor.BN(2), 
          true, 
          new anchor.BN(0), {
          accounts: {
            communityVoteAccount: communityBVoteAccount.publicKey,
            proposal: proposalAccountPublicKey,
            user: provider.wallet.publicKey,
            vote: voteAccountPublicKey,
            tokenAccount: voter1TokenAAccount,
            systemProgram: SystemProgram.programId,
          },
        });
      },
      {
        name: "Error",
        // message: "301: You have already voted for this proposal",
      }
    );
    const vote = await program.account.vote.all();
    assert.equal(vote.length, 2);
  });

  it("Can not vote for a proposal that does not exist!", async () => {
    await assert.rejects(
      async () => {
        const proposalId = getNumberBuffer(999999009);
        const [proposalAccountPublicKey] =
          await anchor.web3.PublicKey.findProgramAddress(
            [Buffer.from("proposal_account"), proposalId],
            anchor.workspace.QwestiveVoting.programId
          );

        const [voteAccountPublicKey, voteBump] =
          await anchor.web3.PublicKey.findProgramAddress(
            [
              Buffer.from("vote_account"),
              proposalId,
              provider.wallet.publicKey.toBuffer(),
            ],
            anchor.workspace.QwestiveVoting.programId
          );
        console.log(voteAccountPublicKey.toString(), voteBump);
        await program.rpc.voteForProposal(
          voteBump,
          new anchor.BN(999999009),
          true,
          new anchor.BN(0),
          {
            accounts: {
              communityVoteAccount: communityVoteAccount.publicKey,
              proposal: proposalAccountPublicKey,
              user: provider.wallet.publicKey,
              vote: voteAccountPublicKey,
              tokenAccount: voter1TokenAAccount,
              systemProgram: SystemProgram.programId,
            },
          }
        );
      },
      {
        name: "Error",
        // message: "301: You have already voted for this proposal",
      }
    );

    const vote = await program.account.vote.all();
    assert.equal(vote.length, 2);
  });
  
  
  it("Can vote for a new proposal for Token B!", async () => {
    const proposalId = getNumberBuffer(2);
    const [proposalAccountPublicKey] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("proposal_account"), proposalId],
        anchor.workspace.QwestiveVoting.programId
      );

    const [voteAccountPublicKey, voteBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("vote_account"),
          proposalId,
          provider.wallet.publicKey.toBuffer(),
        ],
        anchor.workspace.QwestiveVoting.programId
      );

    const tokenBFirstProposal = await program.account.proposal.fetch(
        proposalAccountPublicKey
      );
    assert.ok(tokenBFirstProposal.title === "Token B Test Title");

    await program.rpc.voteForProposal(
      voteBump,            // vote bump account
      new anchor.BN(2),                   // proposal id
      false,                              // vote_bool  true - yes and false - no
      new anchor.BN(0), {                 // candidate = 0 to implies multiple choice is not selected
      accounts: {
        communityVoteAccount: communityBVoteAccount.publicKey,
        proposal: proposalAccountPublicKey,
        user: provider.wallet.publicKey,
        vote: voteAccountPublicKey,
        tokenAccount: voter1TokenBAccount,
        systemProgram: SystemProgram.programId,
      },
    });
    const vote = await program.account.vote.all();
    assert.equal(vote.length, 3);
  });

  it("New User Can Vote to first proposal", async () => {
    const proposalId = getNumberBuffer(0);
    const [proposalAccountPublicKey] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("proposal_account"), proposalId],
        anchor.workspace.QwestiveVoting.programId
      );

    const firstProposal = await program.account.proposal.fetch(
      proposalAccountPublicKey
    );
    assert.ok(firstProposal.title === "Test Title");

    const [voteAccountPublicKey, voteBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("vote_account"), proposalId, newUser.publicKey.toBuffer()],
        anchor.workspace.QwestiveVoting.programId
      );

    await program.rpc.voteForProposal(
      voteBump, 
      new anchor.BN(0),
      false, 
      new anchor.BN(0), {
      accounts: {
        communityVoteAccount: communityVoteAccount.publicKey,
        proposal: proposalAccountPublicKey,
        user: newUser.publicKey,
        vote: voteAccountPublicKey,
        tokenAccount: voter1TokenAAccount,
        systemProgram: SystemProgram.programId,
      },
      signers: [newUser],
    });

    const vote = await program.account.vote.all();
    assert.equal(vote.length, 4);
  });

  it("New user can vote to second proposal", async () => {
    const secondProposalId = getNumberBuffer(1);
    const [secondProposalAccountPublicKey] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("proposal_account"), secondProposalId],
        anchor.workspace.QwestiveVoting.programId
      );

    const secondProposal = await program.account.proposal.fetch(
      secondProposalAccountPublicKey
    );

    assert.ok(secondProposal.title === "Second Test Title");
    const [secondVoteAccountPublicKey, secondVoteBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("vote_account"),
          secondProposalId,
          newUser.publicKey.toBuffer(),
        ],
        anchor.workspace.QwestiveVoting.programId
      );

    await program.rpc.voteForProposal(secondVoteBump, secondProposal.id, true, new anchor.BN(0), {
      accounts: {
        communityVoteAccount: communityVoteAccount.publicKey,
        proposal: secondProposalAccountPublicKey,
        vote: secondVoteAccountPublicKey,
        tokenAccount: voter1TokenAAccount,
        user: newUser.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [newUser],
    });

    const vote = await program.account.vote.all();
    assert.equal(vote.length, 5);
  });

  it("We can get votes for Proposals", async () => {
    const proposalOneVotes = await program.account.vote.all([
      {
        memcmp: {
          offset: 8, // Discriminator.
          bytes: bs58.encode(getNumberBuffer(0)),
        },
      },
    ]);
    assert.ok(proposalOneVotes.length === 2);
  });

  it("We can filter votes which is yes", async () => {
    const proposalOneYesVotes = await program.account.vote.all([
      {
        memcmp: {
          offset: 8, // Discriminator.
          bytes: bs58.encode(
            Buffer.concat([getNumberBuffer(0), Buffer.from([1])])
          ),
        },
      },
    ]);
    const allVotes = await program.account.vote.all();
    assert.equal(allVotes.length, 5);
    assert.equal(proposalOneYesVotes.length, 1);
    assert.ok(proposalOneYesVotes[0].account.voteBool === true);
  });

  it("We can filter votes which is no", async () => {
    const proposalOneNoVotes = await program.account.vote.all([
      {
        memcmp: {
          offset: 8, // Discriminator.
          bytes: bs58.encode(
            Buffer.concat([getNumberBuffer(1), Buffer.from([0])])
          ),
        },
      },
    ]);
    assert.equal(proposalOneNoVotes.length, 1);
    assert.ok(proposalOneNoVotes[0].account.voteBool === false);
  });

  it("Can add a weight vote proposal!", async () => {
    const account = await program.account.communityVoteAccount.fetch(
      communityVoteAccount.publicKey
    );
    console.log("Your account", account);
    const proposalId = getNumberBuffer(account.totalProposalCount.toNumber() + 1);
    const [proposalAccountPublicKey, accountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("proposal_account"), proposalId],
        anchor.workspace.QwestiveVoting.programId
      );

    await program.rpc.addProposal(
      accountBump,
      new anchor.BN(account.totalProposalCount.toNumber() + 1),
      "Weighted Vote Title",
      "Weighted Vote Description",
      new anchor.BN(0), // minimum_token_count
      new anchor.BN(1), // voting system - 0 - one vote per token account, 1 - weighted, 2 - quadratic (not supported)
      new anchor.BN(1), // threshold - needs at least 1 vote 
      new anchor.BN(0), // voting_type - 0 - yes or not, 1 - multiple choice is not currently supported
      new anchor.BN(+new Date() + 1 * DAY_IN_UNIX), //voting_end_timestamp
      new anchor.BN(+new Date() + 2 * DAY_IN_UNIX), //finalize_vote_end_timestamp
      {
        accounts: {
          communityVoteAccount: communityVoteAccount.publicKey,
          proposal: proposalAccountPublicKey,
          tokenAccount: voter1TokenAAccount,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
      }
    );
  });

  it("Can vote for weighted vote proposal", async () => {
    const account = await program.account.communityVoteAccount.fetch(
      communityVoteAccount.publicKey
    );
    const proposalId = getNumberBuffer(account.totalProposalCount.toNumber());

    const [proposalAccountPublicKey] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("proposal_account"), proposalId],
        anchor.workspace.QwestiveVoting.programId
      );

    const weightedVoteProposal = await program.account.proposal.fetch(
      proposalAccountPublicKey
    );

    assert.ok(weightedVoteProposal.title === "Weighted Vote Title");

    // Voter with 500 tokens votes true
    const [vote1AccountPublicKey, vote1Bump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("vote_account"),
          proposalId,
          newUser.publicKey.toBuffer(),
        ],
        anchor.workspace.QwestiveVoting.programId
      );

    await program.rpc.voteForProposal(vote1Bump, weightedVoteProposal.id, true, new anchor.BN(0), {
      accounts: {
        communityVoteAccount: communityVoteAccount.publicKey,
        proposal: proposalAccountPublicKey,
        vote: vote1AccountPublicKey,
        tokenAccount: voter1TokenAAccount,
        user: newUser.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [newUser],
    });

    // Voter with 1 token votes false
    const [vote2AccountPublicKey, vote2Bump] =
    await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("vote_account"),
        proposalId,
        provider.wallet.publicKey.toBuffer(),
      ],
      anchor.workspace.QwestiveVoting.programId
    );

    await program.rpc.voteForProposal(vote2Bump, weightedVoteProposal.id, false, new anchor.BN(0), {
      accounts: {
        communityVoteAccount: communityVoteAccount.publicKey,
        proposal: proposalAccountPublicKey,
        vote: vote2AccountPublicKey,
        tokenAccount: voter3TokenAAccount,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      //signers: [tokenWallet],
    });

    const vote = await program.account.vote.all();
    assert.equal(vote.length, 7);

    const updatedWeightedVoteProposal = await program.account.proposal.fetch(
      proposalAccountPublicKey
    );

    const updated1Vote = await program.account.vote.fetch(
      vote1AccountPublicKey
    );

    const updated2Vote = await program.account.vote.fetch(
      vote2AccountPublicKey
    );

    assert.ok(updatedWeightedVoteProposal.voteYes.toNumber() == tokenAmountOwned);
    assert.ok(updatedWeightedVoteProposal.voteNo.toNumber() == 1);
    assert.ok(updated1Vote.voterWeight.toNumber() == tokenAmountOwned);
    assert.ok(updated2Vote.voterWeight.toNumber() == 1);
  });

});
