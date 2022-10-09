import * as anchor from "@project-serum/anchor";
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
export const wallet = provider.wallet as anchor.Wallet;

export const System = new anchor.web3.PublicKey(
    "CR3zteMpR9y5D9pVBQES1XADDAjTpGF1MCNYzGFcasGj"
);
