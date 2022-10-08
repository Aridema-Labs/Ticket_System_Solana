import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TicketSystem } from "../target/types/ticket_system";

describe("ticket_system", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TicketSystem as Program<TicketSystem>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
