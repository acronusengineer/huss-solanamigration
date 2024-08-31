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
    console.log(`https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
    console.log("transactionhash", txHash);
    await program.provider.connection.getSignatureStatus(txHash);
  });

  it("Transfer Ownership", async () => {
    const signer = provider.wallet.payer;
    // Test transownership function. First create forwarder pda
    const [forwarderPDA, _] = solanaWeb3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("forwarder"),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .createForwarder(provider.wallet.publicKey)
      .signers([signer])
      .rpc();
    expect(
      (await program.account.forwarder.fetch(forwarderPDA)).owner
    ).to.equal(provider.wallet.publicKey);

    const toTransfer = new solanaWeb3.Keypair();
    const mint = await createMint(
      provider.connection,
      signer,
      signer.publicKey,
      null,
      0
    );
    const toTransferAccount = await createAssociatedTokenAccount(
      provider.connection,
      signer,
      mint,
      toTransfer.publicKey
    );
    // And transfer its ownership to other
    await program.methods
      .transferOwnership(toTransferAccount)
      .accounts({ authorizedAddress: provider.wallet.publicKey })
      .signers([signer])
      .rpc();
    expect(
      (await program.account.forwarder.fetch(forwarderPDA)).owner
    ).to.equal(toTransferAccount);
    console.log(`Transfer ownership from ${provider.wallet.publicKey} to ${toTransferAccount} successfully`);
    // const tx = await program.methods.transferOwnership(randomAta).rpc();
    // console.log("Your transaction signature", tx);
  });
});
