use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
}; 
use std::str::FromStr;
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn create_event(
    ctx: Context<Event>,
    name: String,
    day: i64,
    finalize: i64,
    tickets: u64,
    amount: u64
) -> Result<()> {
    let pubkey: Pubkey = Pubkey::from_str("98EgyyxzsehNpNy8yTpcGfTxRmxxVJnu2RHwSvF5nn6i").unwrap();
    let (system_account_pda, _bump): (Pubkey, u8) = Pubkey::find_program_address(&[pubkey.as_ref()], ctx.program_id);
    let (_event_account_pda, bump): (Pubkey, u8) = Pubkey::find_program_address(&[ctx.accounts.system_account.history_events.to_be_bytes().as_ref()], ctx.program_id);
    require_keys_eq!(ctx.accounts.system_account.key(), system_account_pda, ErrorCode::AuthorityError);
    require!(name.chars().count() < 51, ErrorCode::TooLong);
    let system_account: &mut Account<SystemAccount> = &mut ctx.accounts.system_account;
    let event_account: &mut Account<EventAccount> = &mut ctx.accounts.event_account;
    event_account.authority = ctx.accounts.signer.key();
    event_account.bump_original = bump;
    event_account.name = name;
    event_account.event_number = system_account.history_events;
    event_account.day = day;
    event_account.finalize = finalize;
    event_account.tickets = tickets;
    event_account.tickets_sold = 0;
    event_account.amount = amount;
    system_account.history_events += 1;
    system_account.active_events += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct Event<'info> {
    #[account(mut, seeds = [system_account.authority.key().as_ref()], bump = system_account.bump_original)]
    pub system_account: Account<'info, SystemAccount>,
    #[account(init, seeds = [system_account.history_events.to_be_bytes().as_ref()], bump, payer = signer, space = 143)]
    pub event_account: Account<'info, EventAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}