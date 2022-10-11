use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey,
}; 
use std::str::FromStr;

declare_id!("3ShEbfCKTDmdX8NSX5kmG9H5Cms4G4HYT8ZbYKhPYdxW");

#[program]
pub mod ticket_system {
    use super::*;

    pub fn create_system_account(
        ctx: Context<TicketSystem>,
    ) -> Result<()> {
        let program: Pubkey = Pubkey::from_str("3ShEbfCKTDmdX8NSX5kmG9H5Cms4G4HYT8ZbYKhPYdxW").unwrap();
        let pubkey: Pubkey = Pubkey::from_str("98EgyyxzsehNpNy8yTpcGfTxRmxxVJnu2RHwSvF5nn6i").unwrap();
        require_keys_eq!(ctx.accounts.signer.key(), pubkey, ErrorCode::AuthorityError);
        let (_system_account_pda, bump): (Pubkey, u8) = Pubkey::find_program_address(&[pubkey.as_ref()], &program);
        let system_account: &mut Account<SystemAccount> = &mut ctx.accounts.system_account;
        system_account.authority = ctx.accounts.signer.key();
        system_account.bump_original = bump;
        system_account.history_events = 1;
        system_account.active_events = 0;
        system_account.history_tickets_sold = 0;
        Ok(())
    }
    pub fn create_event(
        ctx: Context<Event>,
        name: String,
        day: i64,
        finalize: i64,
        tickets: u64,
        amount: u64
    ) -> Result<()> {
        let pubkey: Pubkey = Pubkey::from_str("98EgyyxzsehNpNy8yTpcGfTxRmxxVJnu2RHwSvF5nn6i").unwrap();
        let program: Pubkey = Pubkey::from_str("3ShEbfCKTDmdX8NSX5kmG9H5Cms4G4HYT8ZbYKhPYdxW").unwrap();
        let (system_account_pda, _bump): (Pubkey, u8) = Pubkey::find_program_address(&[pubkey.as_ref()], &program);
        let (_event_account_pda, bump): (Pubkey, u8) = Pubkey::find_program_address(&[ctx.accounts.system_account.history_events.to_be_bytes().as_ref()], &program);
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
    pub fn take_a_ticket(
        ctx: Context<Ticket>
    ) -> Result<()> {
        let program: Pubkey = Pubkey::from_str("4c1qH9QYhMCcSXLYJVACTNu1MP4MrenoNwWH1EnpAfxB").unwrap();
        require!(ctx.accounts.event_account.tickets_sold < ctx.accounts.event_account.tickets, ErrorCode::AmountTicketsError);
        require_keys_eq!(ctx.accounts.event_authority.key(), ctx.accounts.event_account.authority.key(), ErrorCode::AuthorityError);
        let (_ticket_account_pda, bump): (Pubkey, u8) = Pubkey::find_program_address(&[ctx.accounts.system_account.history_events.to_be_bytes().as_ref()], &program);
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
}
#[derive(Accounts)]
pub struct TicketSystem<'info> {
    #[account(init, seeds = [signer.key().as_ref()], bump, payer = signer, space = 65)]
    pub system_account: Account<'info, SystemAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Event<'info> {
    #[account(mut, seeds = [system_account.authority.key().as_ref()], bump = system_account.bump_original)]
    pub system_account: Account<'info, SystemAccount>,
    #[account(init, seeds = [system_account.history_events.to_be_bytes().as_ref()], bump, payer = signer, space = 143)]
    //#[account(init, seeds = [signer.key().as_ref()], bump, payer = signer, space = 150)]
    pub event_account: Account<'info, EventAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Ticket<'info> {
    #[account(mut, seeds = [event_account.event_number.to_be_bytes().as_ref()], bump = event_account.bump_original)]
    pub event_account: Account<'info, EventAccount>,
    #[account(init, seeds = [event_account.tickets_sold.to_be_bytes().as_ref(), event_account.key().as_ref()], bump, payer = signer, space = 41)]
    pub ticket_account: Account<'info, TicketAccount>,
    #[account(mut, seeds = [system_account.authority.key().as_ref()], bump = system_account.bump_original)]
    pub system_account: Account<'info, SystemAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub event_authority: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
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
#[error_code]
pub enum ErrorCode {
    #[msg("")]AuthorityError, #[msg("")]TooLong, #[msg("")]AmountTicketsError,
}
