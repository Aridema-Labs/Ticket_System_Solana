import * as anchor from "@project-serum/anchor";
import { TicketSystem } from "../target/types/ticket_system";
import { Event } from "../Accounts"
import  * as time from "../unix_time";

describe("Checking event", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.TicketSystem as anchor.Program<TicketSystem>;

  it("Check it", async () => {
    const Account = await program.account.eventAccount.fetch(Event);
    console.log("----------------------------------------------")
    console.log("PDA: ", Event.toBase58())
    console.log("----------------------------------------------")
    console.log("Authority:", Account.authority.toBase58())
    console.log("----------------------------------------------")
    console.log("Bump:", Account.bumpOriginal.toString())
    console.log("----------------------------------------------")
    console.log("Event number:", Account.eventNumber.toString())
    console.log("----------------------------------------------")
    console.log("Name event:", Account.name.toString())
    console.log("----------------------------------------------")
    console.log("Day:", time.timeConverter(Account.day.toNumber()))
    console.log("----------------------------------------------")
    console.log("Finalize:", time.timeConverter(Account.finalize.toNumber()))
    console.log("----------------------------------------------")
    console.log("Tickets for sale:", Account.tickets.toString())
    console.log("----------------------------------------------")
    console.log("Tickets sold:", Account.ticketsSold.toString())
    console.log("----------------------------------------------")
    console.log("Ticket value:", Account.amount.toString())
    console.log("----------------------------------------------")
  });
});