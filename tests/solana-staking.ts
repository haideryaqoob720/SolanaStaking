import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaStaking } from "../target/types/solana_staking";
import {
  clusterApiUrl,
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  LAMPORTS_PER_SOL,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import type { Constants } from "../target/types/constants";
const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const testWallet = provider.wallet;
const program = anchor.workspace.SolanaStaking as Program<SolanaStaking>;
const programId = program.programId;
const solConfig = {
  programId: programId,
  connection: connection,
  provider: provider,
  tokenAddress: new anchor.web3.PublicKey(
    "HkE7tyLsiDyUeQpswvBqZeuQkqZSa3sj8JNi4n4Q62eK"
  ),
  STAKE_SEED: "STAKE_SEED",
  admin: new PublicKey("FQMQ2Damu3FfcM439nJ8n2hDmNr1g4DJfTfYBtDPnKJY")
};
describe("solana-staking", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Constants as anchor.Program<Constants>;
  
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const getStakePDA = async () => {
    return await PublicKey.findProgramAddressSync(
      [Buffer.from(solConfig.STAKE_SEED)],
      program.programId
    );
  };
  const getUserPda = () =>{
    
  }
  // it("Stake pool created!", async () => {
  //   const [stakePda] = await getStakePDA();
  //   let mint = solConfig.tokenAddress;
  //   let apr = new anchor.BN(100);
  //   let min = new anchor.BN(10);
  //   let max = new anchor.BN(100000);
  //   let isFlexible = true;
  //   let sxTime = new anchor.BN(86400);
  //   let coolDown = new anchor.BN(60);
  //   const tx = await program.methods
  //     .createStakePool(mint, apr, min, max, isFlexible, sxTime, coolDown)
  //     .accounts({
  //       stakeInfo: stakePda,
  //       authority: testWallet.publicKey,
  //       systemProgram: SystemProgram.programId,
  //     })
  //     .rpc();
  //   console.log("Created stake vault", tx);
  // });

  // it("Stake tokens", async () => {
  //   const [stakePda] = await getStakePDA();
  //   let amount: any = JSON.stringify(1000 * LAMPORTS_PER_SOL);
  //   amount = new anchor.BN(amount);
  //   const fromAta = getAssociatedTokenAddressSync(
  //     solConfig.tokenAddress,
  //     testWallet.publicKey,
  //     true,
  //     TOKEN_2022_PROGRAM_ID
  //   );
  //   const stakeVault = getAssociatedTokenAddressSync(
  //     solConfig.tokenAddress,
  //     stakePda,
  //     true,
  //     TOKEN_2022_PROGRAM_ID
  //   );
  //   const context = {
  //     token: solConfig.tokenAddress,
  //     admin: testWallet,
  //     fromAta: fromAta,
  //     stakeVault: stakeVault,
  //     stakeInfo: stakePda,
  //     SystemProgram: SystemProgram.programId,
  //     rent: SYSVAR_RENT_PUBKEY,
  //     tokenProgram: TOKEN_2022_PROGRAM_ID,
  //     associatredTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //   };
  //   const tx = await program.methods
  //     .depositRewards(amount)
  //     .accounts(context)
  //     .rpc();
  //   console.log(`deposoted 1000 tokens link: ${tx}`);
  // });

  it("Profit calculation", async() =>{
    // const data = await 
    const [stakeinfo, bump] = await getStakePDA()
    const userInfo = 
  const vault = getAssociatedTokenAddressSync(solConfig.tokenAddress, stakeinfo, true, TOKEN_2022_PROGRAM_ID)
  const buyerAta = getAssociatedTokenAddressSync(solConfig.tokenAddress, testWallet.publicKey, true, TOKEN_2022_PROGRAM_ID)
    const tx = program.methods.unstakeTokens(new anchor.BN(bump)).accounts({
      token: solConfig.tokenAddress,
      stakeVault: vault,
      buyerAta: buyerAta,
      admin: solConfig.admin,
      buyer: testWallet.publicKey,
      stakeInfo: stakeinfo,
      userInfo: 
    })
  })
});
