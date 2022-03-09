const assert = require("assert");
import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
const { SystemProgram } = anchor.web3;
import { Counter } from '../target/types/counter';

describe('counter', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();

  const program = anchor.workspace.Counter as Program<Counter>;

  const baseAccount = anchor.web3.Keypair.generate();

  it('Creates a counter', async () => {
    // Add your test here.
    await program.rpc.create({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount],
    });

    /* Fetch the account and check the value of count */
    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('Count 0: ', account.count.toString());
    assert.ok(account.count.toString() == '0');
  });

  it('Increments the counter', async () => {

    await program.rpc.increment({
      accounts: {
        baseAccount: baseAccount.publicKey,
      },
    });

    /* Fetch the account and check the value of count */
    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('Count 1: ', account.count.toString());
    assert.ok(account.count.toString() == '1');
  })
});
