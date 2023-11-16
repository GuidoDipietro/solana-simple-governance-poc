import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MockProgram } from "../target/types/mock_program";
import { Governance } from "../target/types/governance";
import { assert } from "chai";

describe("mock-program", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const mockProgram = anchor.workspace.MockProgram as Program<MockProgram>;
  const governance = anchor.workspace.Governance as Program<Governance>;

  const deployer = provider.wallet.publicKey;

  it("Creates GlobalState", async () => {
    await mockProgram.methods
      .initGlobalState(14)
      .accounts({ admin: deployer })
      .rpc();

    let globalState = (await mockProgram.account.globalState.all())[0].account;

    assert.equal(globalState.admin.toBase58(), deployer.toBase58());
    assert.equal(globalState.value, 14);
  });

  it("Can edit GlobalState as admin", async () => {
    let [governancePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("governance_pda")],
      governance.programId
    );

    await mockProgram.methods
      .editGlobalState(27)
      .accounts({
        admin: deployer,
        governancePda,
      })
      .rpc();

    let globalState = (await mockProgram.account.globalState.all())[0].account;

    assert.equal(globalState.admin.toBase58(), deployer.toBase58());
    assert.equal(globalState.value, 27);
  });

  it("Can edit GlobalState from Governance", async () => {
    let [mockProgramGlobalState] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("global_state")],
      mockProgram.programId
    );

    await governance.methods
      .editMockState(49)
      .accounts({
        mockProgramGlobalState,
        mockProgram: mockProgram.programId,
      })
      .rpc();

    let globalState = (await mockProgram.account.globalState.all())[0].account;

    assert.equal(globalState.admin.toBase58(), deployer.toBase58());
    assert.equal(globalState.value, 49);
  });

  it("Can't edit GlobalState if we use a different MockProgram PDA", async () => {
    let [anotherPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("another_pda")],
      governance.programId
    );

    try {
      await mockProgram.methods
        .editGlobalState(27)
        .accounts({
          admin: deployer,
          governancePda: anotherPda,
        })
        .rpc();

      assert.fail("MUST FAIL");
    } catch (e) {
      assert.equal(e.error?.errorCode?.code, "ConstraintSeeds");
    }
  });
});
