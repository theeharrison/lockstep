import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Lockstep } from "../target/types/lockstep";
import { assert } from "chai";

describe("lockstep", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Lockstep as Program<Lockstep>;
  const client = provider.wallet;
  const freelancer = anchor.web3.Keypair.generate();

  it("Creates an escrow and deposits funds!", async () => {
    const amount = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL); // 1 SOL

    // 1. Find the Escrow Account address (PDA)
    const [escrowPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), client.publicKey.toBuffer()],
      program.programId
    );

    // 2. Initialize the Escrow
    await program.methods
      .initializeEscrow(amount, freelancer.publicKey)
      .accounts({
        client: client.publicKey,
        escrow: escrowPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
      
    console.log("✅ Escrow initialized!");

    // 3. Deposit funds into the Escrow
    await program.methods
      .deposit(amount)
      .accounts({
        client: client.publicKey,
        escrow: escrowPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("✅ Funds deposited!");

    // 4. Check the balance of the Escrow account
    const balance = await provider.connection.getBalance(escrowPda);
    console.log(`Escrow Balance: ${balance / anchor.web3.LAMPORTS_PER_SOL} SOL`);
    
    // Verify it has AT LEAST the 1 SOL (plus the rent)
    assert.isTrue(
      balance >= amount.toNumber(), 
      "The escrow balance should hold the deposit amount + rent"
    );
  });
});