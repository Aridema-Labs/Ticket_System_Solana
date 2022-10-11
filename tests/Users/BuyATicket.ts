import * as anchor from "@project-serum/anchor";
import { TicketSystem } from "../target/types/ticket_system";
import { PublicKey } from '@solana/web3.js'
import { Connection, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { System, Event } from "../Accounts"
import  * as time from "../unix_time";

describe("Ticket", async () => {
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.TicketSystem as anchor.Program<TicketSystem>;
  it('Ticket ...', async () => {
    const EventAccount = await program.account.eventAccount.fetch(Event);
    const [TicketAccount, _bump] = await PublicKey.findProgramAddress(
        [
          new anchor.BN(EventAccount.ticketsSold).toArrayLike(Buffer, "be", 8),
          Event.toBuffer()
        ],
        program.programId
      )
    const tx = await program.methods.takeATicket().accounts({
        eventAccount: Event,
        ticketAccount: TicketAccount,
        systemAccount: System,
        from: provider.wallet.publicKey ,
        eventAuthority: EventAccount.authority,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

      const Tk = await program.account.ticketAccount.fetch(TicketAccount);
      const connection = new Connection("https://api.devnet.solana.com");
      let balance = await connection.getBalance(provider.wallet.publicKey);

      console.log("-----------------------------------------------------")
      console.log("Tx: ", tx);
      console.log("-----------------------------------------------------")
      console.log("Your Ticket: ", TicketAccount.toString());
      console.log("-----------------------------------------------------")
      console.log("Authority: ", Tk.authority.toBase58());
      console.log("-----------------------------------------------------")
      console.log("Original bump: ", Tk.bumpOriginal.toString());
      console.log("-----------------------------------------------------")
      console.log("Tickets for sale:", EventAccount.tickets.toString())
      console.log("----------------------------------------------")
      console.log("Tickets sold:", EventAccount.ticketsSold.toString())
      console.log("----------------------------------------------")
      console.log("Ticket value:", EventAccount.amount.toString())
      console.log("-----------------------------------------------------")
      console.log("Name event:", EventAccount.name.toString())
      console.log("----------------------------------------------")
      console.log("Day:", time.timeConverter(EventAccount.day.toNumber()))
      console.log("----------------------------------------------")
      console.log("Finalize:", time.timeConverter(EventAccount.finalize.toNumber()))
      console.log("----------------------------------------------")
      console.log("Your Balance:", (balance / LAMPORTS_PER_SOL).toString(), "SOL");
      console.log("-----------------------------------------------------")
  });
});