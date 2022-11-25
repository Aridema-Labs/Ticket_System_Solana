use anchor_lang::prelude::*;

#[account]
pub struct SystemAccount {
    pub authority: Pubkey,
    pub bump_original: u8,
    pub history_events: u64, 
    pub active_events: u64,
    pub history_tickets_sold: u64
}

#[account]
pub struct EventAccount {
    pub authority: Pubkey, 
    pub bump_original: u8,
    pub amount: u64,
    pub event_number: u64,
    pub name: String,
    pub day: i64,
    pub finalize: i64,
    pub tickets: u64,
    pub tickets_sold: u64
}

#[account]
pub struct TicketAccount {
    pub authority: Pubkey,
    pub bump_original: u8
}