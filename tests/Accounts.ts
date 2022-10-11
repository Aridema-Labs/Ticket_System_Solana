import * as anchor from "@project-serum/anchor";
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
export const wallet = provider.wallet as anchor.Wallet;

export const System = new anchor.web3.PublicKey(
    "8dzqZaEE3QTatnXF72fJRbNHnyh7QqXyFTsbKnFXcYEX"
);
export const Event = new anchor.web3.PublicKey(
    "FsUkKfCouLEd9KR2vRvNCBpFHnqRcYqJi37C8GeincrF"
);
