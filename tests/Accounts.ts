import * as anchor from "@project-serum/anchor";
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
export const wallet = provider.wallet as anchor.Wallet;

export const System = new anchor.web3.PublicKey(
    "2r8mPD65rzbjdXCxX1jgaZoKbQKJRHRWAzFQCgpcis3E"
);
