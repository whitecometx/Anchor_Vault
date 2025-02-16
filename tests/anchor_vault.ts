import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import { PublicKey, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect } from "chai";
import { randomBytes } from "crypto";
import { BN } from "bn.js";

describe("anchor_vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorVault as Program<AnchorVault>;
  const user = provider.wallet.publicKey;

  let statePDA: PublicKey;
  let vaultPDA: PublicKey;
  let stateBump: number;
  let vaultBump: number;

  before(async () => {
    [statePDA, stateBump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("state"), user.toBuffer()],
      program.programId
    );

    [vaultPDA, vaultBump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), statePDA.toBuffer()],
      program.programId
    );
  });

  it("Initializes the vault", async () => {
    await program.methods
      .initialize()
      .accountsPartial({
        user,
        state: statePDA,
        vault: vaultPDA,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const stateAccount = await program.account.vaultState.fetch(statePDA);
    expect(stateAccount.stateBump).to.equal(stateBump);
    expect(stateAccount.vaultBump).to.equal(vaultBump);
  });

  it("Deposits funds into the vault", async () => {
    const depositAmount = new anchor.BN(LAMPORTS_PER_SOL * 2);
    const initialBalance = await provider.connection.getBalance(vaultPDA);

    await program.methods
      .deposit(depositAmount)
      .accountsPartial({
        user,
        state: statePDA,
        vault: vaultPDA,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const finalBalance = await provider.connection.getBalance(vaultPDA);
    expect(finalBalance - initialBalance).to.equal(depositAmount.toNumber());
  });

  it("Withdraws funds from the vault", async () => {
    const withdrawAmount = new anchor.BN(LAMPORTS_PER_SOL / 4);
    const initialUserBalance = await provider.connection.getBalance(user);
    const initialVaultBalance = await provider.connection.getBalance(vaultPDA);

    await program.methods
      .withdraw(withdrawAmount)
      .accountsPartial({
        user,
        state: statePDA,
        vault: vaultPDA,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const finalUserBalance = await provider.connection.getBalance(user);
    const finalVaultBalance = await provider.connection.getBalance(vaultPDA);

    expect(finalUserBalance - initialUserBalance).to.be.closeTo(
      withdrawAmount.toNumber(),
      100000 // Allow for a small difference due to transaction fees
    );
    expect(initialVaultBalance - finalVaultBalance).to.equal(withdrawAmount.toNumber());
  });

  it("Closes the vault", async () => {
    const initialUserBalance = await provider.connection.getBalance(user);
    const initialVaultBalance = await provider.connection.getBalance(vaultPDA);

    await program.methods
      .close()
      .accountsPartial({
        user,
        state: statePDA,
        vault: vaultPDA,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const finalUserBalance = await provider.connection.getBalance(user);
    const finalVaultBalance = await provider.connection.getBalance(vaultPDA);

    expect(finalUserBalance - initialUserBalance).to.be.closeTo(
      initialVaultBalance,
      1000000 // Allow for a small difference due to transaction fees
    );
    expect(finalVaultBalance).to.equal(0);

    // Verify that the state account is closed
    try {
      await program.account.vaultState.fetch(statePDA);
      expect.fail("State account should be closed");
    } catch (error) {
      expect(error.message).to.include("Account does not exist");
    }
  });
});
