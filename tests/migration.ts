import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Forwarder } from "../target/types/forwarder";
import {
  createMint,
  createAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import * as solanaWeb3 from "@solana/web3.js";
// import { assert } from "chai";

describe("migration", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.forwarder as Program<Forwarder>;

  it("Initialized", async () => {
    // Test initialize function.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Can_flush_SPL_tokens", async () => {
    // Test the flush_spl_tokens function
    // Generate the keypairs for the new account
    // const program = anchor.workspace;
    const signer = anchor.web3.Keypair.generate();
    const fromKp = provider.wallet;
    const toKp = new solanaWeb3.Keypair();
    const toKp1 = new solanaWeb3.Keypair();

    // Create a new mint and initialize it
    const mintKp = new solanaWeb3.Keypair();
    const mint = await createMint(
      provider.connection,
      signer,
      fromKp.publicKey,
      null,
      0
    );

    // Assiociated token accounts for the new accounts
    const fromAta = await createAssociatedTokenAccount(
      provider.connection,
      signer,
      mint,
      fromKp.publicKey
    );
    const toAta = await createAssociatedTokenAccount(
      provider.connection,
      signer,
      mint,
      toKp.publicKey
    );
    const toAta1 = await createAssociatedTokenAccount(
      provider.connection,
      signer,
      mint,
      toKp1.publicKey
    );

    // Mint SPL tokens to the "from" assiociated token account
    const mintAmmount = 10000;
    await mintTo(
      provider.connection,
      signer,
      mint,
      fromAta,
      fromKp.publicKey,
      mintAmmount
    );

    // Send transaction
    const amount = new anchor.BN(1000); // Wrap the number in a BN array
    const accounts = [
      { pubkey: toAta, isSigner: false, isWritable: true },
      { pubkey: toAta1, isSigner: false, isWritable: true },
    ];
    const txHash = await program.methods
      .flushSplTokens(amount)
      .accounts({
        from: fromKp.publicKey,
        fromAta: fromAta,
      })
      .remainingAccounts(accounts)
      .signers([signer])
      .rpc();
    console.log(`https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
    await program.provider.connection.getSignatureStatus(txHash);

  });
});
