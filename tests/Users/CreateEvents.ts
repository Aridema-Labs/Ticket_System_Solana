import * as anchor from "@project-serum/anchor";
import { TicketSystem } from "../target/types/ticket_system";
import { PublicKey } from '@solana/web3.js'
import { System } from "../Accounts"

describe("Registering an event", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.TicketSystem as anchor.Program<TicketSystem>;

  it("Is initialized!", async () => {
    const SystemAccount = await program.account.systemAccount.fetch(System);
    const [EventAccount, _bump] = await PublicKey.findProgramAddress(
      [
        new anchor.BN(SystemAccount.events).toArrayLike(Buffer, "be", 8),
      ],
      program.programId
    )
    const tx = await program.methods.createEvent()
    .accounts({
      systemAccount: System,
      eventAccount: EventAccount,
      signer: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .rpc();
    const Account = await program.account.eventAccount.fetch(EventAccount);
    console.log("----------------------------------------------")
    console.log("Your transaction signature", tx);
    console.log("----------------------------------------------")
    console.log("PDA: ", EventAccount.toBase58())
    console.log("----------------------------------------------")
    console.log("Authority:", Account.authority.toBase58())
    console.log("----------------------------------------------")
    console.log("Bump:", Account.bumpOriginal.toString())
    console.log("----------------------------------------------")
  });
});