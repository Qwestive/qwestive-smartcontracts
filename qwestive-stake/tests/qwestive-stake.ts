import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { QwestiveStake } from '../target/types/qwestive_stake';

describe('qwestive-stake', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.QwestiveStake as Program<QwestiveStake>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
