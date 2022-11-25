use anchor_lang::{
    prelude::*,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey
}; 
use std::str::FromStr;
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn take_a_ticket(
    ctx: Context<Ticket>
) -> Result<()> {
    require!(ctx.accounts.event_account.tickets_sold < ctx.accounts.event_account.tickets, ErrorCode::AmountTicketsError);
    require_keys_eq!(ctx.accounts.event_authority.key(), ctx.accounts.event_account.authority.key(), ErrorCode::AuthorityError);
    let (_ticket_account_pda, bump): (Pubkey, u8) = Pubkey::find_program_address(&[ctx.accounts.system_account.history_events.to_be_bytes().as_ref()], ctx.program_id);
    anchor_lang::solana_program::program::invoke(
        &system_instruction::transfer(&ctx.accounts.from.key(), &ctx.accounts.event_authority.key(), ctx.accounts.event_account.amount),
        &[ctx.accounts.from.to_account_info(), ctx.accounts.event_authority.to_account_info().clone()],).expect("Error");
    let event_account: &mut Account<EventAccount> = &mut ctx.accounts.event_account;
    let ticket_account: &mut Account<TicketAccount> = &mut ctx.accounts.ticket_account;
    let system_account: &mut Account<SystemAccount> = &mut ctx.accounts.system_account;
    ticket_account.authority = ctx.accounts.signer.key(); 
    ticket_account.bump_original = bump;
    system_account.history_tickets_sold += 1;
    event_account.tickets_sold += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct Ticket<'info> {
    #[account(mut, seeds = [event_account.event_number.to_be_bytes().as_ref()], bump = event_account.bump_original)]
    pub event_account: Account<'info, EventAccount>,
    #[account(init, seeds = [event_account.tickets_sold.to_be_bytes().as_ref(), event_account.key().as_ref()], bump, payer = signer, space = 41)]
    pub ticket_account: Account<'info, TicketAccount>,
    #[account(mut, seeds = [system_account.authority.key().as_ref()], bump = system_account.bump_original)]
    pub system_account: Account<'info, SystemAccount>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub event_authority: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}