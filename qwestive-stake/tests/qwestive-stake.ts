import * as anchor from '@project-serum/anchor';
import { Program, BN, IdlAccounts } from '@project-serum/anchor';
import { PublicKey, Keypair, SystemProgram } from '@project-serum/web3.js';
import { TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
import * as assert from "assert";
import { QwestiveStake } from '../target/types/qwestive_stake';

describe('qwestive-stake', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.QwestiveStake as Program<QwestiveStake>;


  // // it('Is initialized!', async () => {
  // //   // Add your test here.
  // //   const tx = await program.rpc.initialize({});
  // //   console.log("Your transaction signature", tx);
  // // });
  it('can initialize an account', async () => {
    // Before sending the transaction to the blockchain.
    const creator = anchor.web3.Keypair.generate();
    const mind_id = anchor.web3.Keypair.generate();
    const signature = await program.provider.connection.requestAirdrop(creator.publicKey, 1000000000);
    await program.provider.connection.confirmTransaction(signature);

    await program.rpc.initializeCommunity(mind_id.publicKey,
    {
        accounts: {
            // Accounts here...
            communityAccount: creator.publicKey,
            initializer: creator.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: anchor
          },
        // Key pairs of signers here...
        signers: [creator],
        instructions: []
    });

    const [sandboxPda, sandboxBump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from('community_seed')], program.programId);
    const [sandboxPda2, sandboxBump2] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from('community__seed2')], program.programId);

    // Fetch the account details of the created tweet.
    const communityAccount = await program.account.communityAccount.fetch(creator.publicKey);
    
    // Ensure it has the right data.
    assert.equal(communityAccount.initializer.toBase58(), creator.publicKey.toBase58());
    assert.equal(communityAccount.mintId.toBase58(), mind_id.publicKey.toBase58());
    assert.equal(communityAccount.freezeAuthority.toBase58(), creator.publicKey.toBase58());
    assert.equal(communityAccount.stakePda.toBase58(), sandboxPda.toBase58());
    assert.notEqual(communityAccount.stakePda.toBase58(), sandboxPda2.toBase58());
    assert.ok(!communityAccount.freezeEnabled);
    assert.ok(communityAccount.timeStamp);
    //   console.log("Your transaction signature", tx);
  });
});
