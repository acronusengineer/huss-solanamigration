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
import { expect } from "chai";
// import { assert } from "chai";
// 552HXJshauLuvCCfAzYHLCWqJftdmgz71wpRiZip7798bkCffSvzHLUREFzSAG5AictcoJ3M4DyY7itoXrDFwnCk

describe("forwarder", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.forwarder as Program<Forwarder>;

  it("Initialized", async () => {
    const signer = provider.wallet.payer;
    // const signer = provider.wallet;
    // Test initialize function.
    const tx = await program.methods
      .initialize()
      .accounts({
        user: provider.wallet.publicKey,
        authorizedAddress: provider.wallet.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Can_flush_SPL_tokens", async () => {
    // Test the flush_spl_tokens function
    // Generate the keypairs for the new account
    const signer = provider.wallet.payer;
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
    console.log(program.programId);
    // Assiociated token accounts for the new accounts
    const fromAta = await createAssociatedTokenAccount(
      provider.connection,
      signer,
      mint,
      fromKp.publicKey
    );

    // const fromAtaBalance = await provider.connection;
    console.log("fromAta--->", fromAta);
    const toAta = await createAssociatedTokenAccount(
      provider.connection,
      signer,
      mint,
      toKp.publicKey
    );

    console.log("toAta--->", toAta);
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
        authorizedAddress: fromKp.publicKey,
        toAta: toAta,
      })
      .remainingAccounts(accounts)
      .signers([signer])
      .rpc();
    const fromAtaBalance = await provider.connection.getTokenAccountBalance(
      fromAta
    );
    const toAtaBalance = await provider.connection.getTokenAccountBalance(
      toAta
    );
    console.log(`https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
    console.log(`fromAta's amount--->`, fromAtaBalance);
    console.log(`toAta's amount--->`, toAtaBalance);
    console.log("transactionhash", txHash);
    await program.provider.connection.getSignatureStatus(txHash);
  });

  it("Create Forwarder & Transfer Ownership", async () => {
    const signer = provider.wallet.payer;

    // Create forwarder PDA.
    const [forwarderPDA, _] = solanaWeb3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("forwarder"),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    );

    // Create forwarder first, must be done before transferring ownership
    await program.methods
      .createForwarder(provider.wallet.publicKey)
      .accounts({
        user: signer.publicKey,
        authorizedAddress: signer.publicKey,
        // systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([signer])
      .rpc();

    try {
      const forwarderAccount = await program.account.forwarder.fetch(
        forwarderPDA
      );
      expect(forwarderAccount.owner.toString()).to.equal(
        provider.wallet.publicKey.toString()
      );
      console.log("Success creating forwarder");
    } catch (error) {
      console.error("Error fetching forwarder account:", error);
    }

    const newOwner = solanaWeb3.Keypair.generate();

    try {
      await program.methods
        .transferOwnership(newOwner.publicKey)
        .accounts({
          authorizedAddress: provider.wallet.publicKey,
          forwarder: forwarderPDA, // Include the forwarder PDA here
        })
        .signers([signer]) // signer should be the wallet's payer
        .rpc();

      console.log(
        `Ownership transferred from ${provider.wallet.publicKey} to ${newOwner.publicKey} successfully`
      );

      // Verify the new owner
      const updatedForwarderAccount = await program.account.forwarder.fetch(
        forwarderPDA
      );
      expect(updatedForwarderAccount.owner.toString()).to.equal(
        newOwner.publicKey.toString()
      );
    } catch (error) {
      console.error("Error during ownership transfer:", error);
    }
  });
});
