import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
// import { Migration } from "../target/types/migration";
import { Forwarder } from "../target/types/forwarder";

describe("migration", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Migration as Program<Forwarder>;

  it("Is initialized!", async () => {
    // Test initialize function.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Can flush SPL tokens", async () => {
    // Test the flush_spl_tokens function
    const amount = new anchor.BN(100); // Wrap the number in a BN array
    const tx = await program.methods.flushSplTokens(amount).rpc();
    console.log("Flushed tokens transaction signature:", tx);
  });
});
