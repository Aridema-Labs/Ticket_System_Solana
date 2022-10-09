import * as anchor from "@project-serum/anchor";
import { TicketSystem } from "../target/types/ticket_system";
import { PublicKey } from '@solana/web3.js'

describe("Initialized system account", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.TicketSystem as anchor.Program<TicketSystem>;

  it("Is initialized!", async () => {
    const [SystemAccount, _bump] = await PublicKey.findProgramAddress(
      [
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    )
    const tx = await program.methods.createSystemAccount()
    .accounts({
      systemAccount: SystemAccount,
      signer: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .rpc();
    const Account = await program.account.systemAccount.fetch(SystemAccount)
    console.log("----------------------------------------------")
    console.log("Your transaction signature", tx);
    console.log("----------------------------------------------")
    console.log("PDA: ", SystemAccount.toBase58())
    console.log("----------------------------------------------")
    console.log("Authority:", Account.authority.toBase58())
    console.log("----------------------------------------------")
    console.log("Bump:", Account.bumpOriginal.toString())
    console.log("----------------------------------------------")
    console.log("Events:", Account.events.toString())
    console.log("----------------------------------------------")
  });
});