// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor';
import { Cluster, PublicKey } from '@solana/web3.js';
import LotteryAppIDL from '../target/idl/lottery_app.json';
import type { LotteryApp } from '../target/types/lottery_app';

// Re-export the generated IDL and type
export { LotteryApp, LotteryAppIDL };

// The programId is imported from the program IDL.
export const LOTTERY_APP_PROGRAM_ID = new PublicKey(LotteryAppIDL.address);

// This is a helper function to get the LotteryApp Anchor program.
export function getLotteryAppProgram(provider: AnchorProvider) {
  return new Program(LotteryAppIDL as LotteryApp, provider);
}

// This is a helper function to get the program ID for the LotteryApp program depending on the cluster.
export function getLotteryAppProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
    case 'mainnet-beta':
    default:
      return LOTTERY_APP_PROGRAM_ID;
  }
}
