import * as anchor from "@project-serum/anchor";
import { TicketSystem } from "../target/types/ticket_system";
import { System } from "../Accounts"

describe("Checking event", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.TicketSystem as anchor.Program<TicketSystem>;

  it("Check it", async () => {
    const Account = await program.account.systemAccount.fetch(System);
    console.log("----------------------------------------------")
    console.log("PDA: ", System.toBase58())
    console.log("----------------------------------------------")
    console.log("Authority:", Account.authority.toBase58())
    console.log("----------------------------------------------")
    console.log("Bump:", Account.bumpOriginal.toString())
    console.log("----------------------------------------------")
    console.log("Historic events:", Account.historyEvents.toString())
    console.log("----------------------------------------------")
    console.log("Active events:", Account.activeEvents.toString())
    console.log("----------------------------------------------")
    console.log("Historic Tickets Sold:", Account.historyTicketsSold.toString())
    console.log("----------------------------------------------")
  });
});