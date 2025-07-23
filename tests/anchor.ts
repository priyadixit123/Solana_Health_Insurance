i  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaHealthInsurance as anchor.Program<SolanaHealthInsurance>;
  
mport BN from "bn.js";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * anchor from "@coral-xyz/anchor";
import { Program } from "@corla-xyz/anchor";
import { SolanaHealthInsurence } from "../target/types/solana_health_insurance";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddress, mintTo, createAssociatedTokenAddress,}
from "@solana/spl-token";
import assert from "assert";
import type { SolanaHealthInsurance } from "../target/types/solana_health_insurance";

describe ("Solana health Insureance Claim Verification", ()=>{
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaHealthInsurance as Program < SolanaHealthInsurence>;

  let claimId = new anchor.BN(1);
  let treatmentHash = new Uint8Array(64).fill(1);

  const patient = provider.wallet;
  const hospital = Keypair.generate();
  const insurer = Keypair.generate();
  const claim = PublicKey.findProgramAddressSync(
    [Buffer.from("claim"). patient.publicKey.toBuffer(), claimId.toArrayLike(Buffer, "le", 8)],
    program.programId
  )[0];

  let usdcMint: PublicKey;
  let insurerTokenAccount: PublicKey;
  let patientTokenAccount: PublicKey;

  it("Airdrops SQl to hospital and insurer", async()=>{
    const tx1 = await provider.connection.requestAirdrop(hospital.publicKey,2e9);
    const tx2 =await provider.connection.requestAirdrop(insurer.publicKey, 2e9);
    await provider.connection.confirmTransaction(tx1);
    await provider.connection.confirmTransaction(tx2);
  });

  it("Creates USDT Mint nd token Accounts", async () =>{
    usdcMint  await createMint(
    provider.connection,
    insurer,
    insurer.publicKey,
    null,
    6
    );

    patientTokenAccount = await getAssociatedTokenAddress (usdcMint, patient, patient.publicKey);
     await createAssociatedTokenAddress (
      provider.connection,
      patient.payer,
      usdcMint,
      patient.publicKey
     );

     await mintTo(
      provider.connection,
      insurer,
      usdcMint,
      insurerTokenAccount,
      insurer,
      1_000_000_000
     );
  });

  it("Submits a claim ", async()=>{
    await program.methods
    .submitClaim(claimId,[...treatmentHash])
    .accounts ({
      claim,
      patient: patient.publicKey,
      hospital: hospital.publicKey,
      insurer: insurer.publicKey,
      SystemProgram: SystemProgram.programId,

    })
    .signers([])
    .rpc();

    const account = await program.account.claimAccount.fetch(claim);
    assert.strictEqual(account.claimStatus.Submitted,{});
    assert.ok(account.patient.equals(patient.publicKey));
  });

  it("Hospital verifies the claim", async()=> {
    await program.methods
    .hospitalVerifyClaim()
    .accounts ({
      claim,
      hospital : hospital.publicKey,
    })
    .signers([hospital])
    .rpc();

    const account = await program.account.claimAccount.fetch(claim);
    assert.strictEqual(account.claimStatus.verified, {});
  });

  it("Insurer approves and transfers funds ", async() =>{
    await program.methods 
    .insurerApproveAndReleaseFunds(new anchor.BN(100_000_000))
    .accounts({
    insurer: insurer.publicKey,
    insurerToken: insurerTokenAccount,
    patientToken: patientTokenAccount,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .signers([insurer])
  .rpc();

  const account = await program.account.claimAccount.fetch(claim);
  assert.strictEqual(account.claimStatus.Approved, {});
 });
});