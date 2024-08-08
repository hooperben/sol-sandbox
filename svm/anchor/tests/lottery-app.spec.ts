import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { LotteryApp } from '../target/types/lottery_app';

describe('lottery-app', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.LotteryApp as Program<LotteryApp>;

  it('should run the program', () => {
    const proof = Buffer.from([1, 2, 3, 4]);

    // Example public inputs as an array of arrays of 32 bytes
    const publicInputs = [Array(32).fill(0), Array(32).fill(1)];

    program.methods
      .verifyProof(proof, publicInputs)
      .rpc()
      .then((result) => console.log(result));
    // console.log(tester);
  });
});
