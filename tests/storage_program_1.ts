import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { StorageProgram1 } from "../target/types/storage_program_1";

describe("storage_program_1", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.StorageProgram1 as Program<StorageProgram1>;
  const dataAccount = anchor.web3.Keypair.generate()
  const provider = anchor.AnchorProvider.env()
  const owner = provider.publicKey

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(34).accounts({
      owner:owner,
      dataAccount:dataAccount.publicKey,
      systemProgram:anchor.web3.SystemProgram.programId
    }).signers([dataAccount]).rpc()

    console.log("Your transaction signature", tx);
  });
  it ("Get data from blockchain", async () => {
    const b = await provider.connection.getAccountInfo(dataAccount.publicKey)
    const data = await program.account.secondStorage.fetch(dataAccount.publicKey)
    let data_value = data.value;
    console.log("data", data_value);
  })
});
