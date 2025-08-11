import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloWorld } from "../target/types/hello_world";
import { assert } from "chai";

describe("hello_world", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.HelloWorld as Program<HelloWorld>;
  const baseAccount = anchor.web3.Keypair.generate();

  it("Initialize account", async () => {
    await program.methods
      .initialize("Hello, Solana!")
      .accounts({
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([baseAccount])
      .rpc();

    const account = await program.account.messageAccount.fetch(
      baseAccount.publicKey
    );
    assert.equal(account.message, "Hello, Solana!");
  });

  it("Update message", async () => {
    await program.methods
      .updateMessage("Anchor is awesome!")
      .accounts({
        baseAccount: baseAccount.publicKey,
      })
      .rpc();

    const account = await program.account.messageAccount.fetch(
      baseAccount.publicKey
    );
    assert.equal(account.message, "Anchor is awesome!");
  });
});
