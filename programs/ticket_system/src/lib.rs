use anchor_lang::prelude::*;
use instructions::*;
use crate::errors::ErrorCode;


pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("3ShEbfCKTDmdX8NSX5kmG9H5Cms4G4HYT8ZbYKhPYdxW");

#[program]
pub mod ticket_system {
    use super::*;

    pub fn create_system_account(
        ctx: Context<TicketSystem>,
    ) -> Result<()> {
        instructions::create_system_account::create_system_account(ctx)
    }
    pub fn create_event(
        ctx: Context<Event>,
        name: String,
        day: i64,
        finalize: i64,
        tickets: u64,
        amount: u64
    ) -> Result<()> {
        instructions::create_event::create_event(
            ctx,
            name,
            day,
            finalize,
            tickets,
            amount
        )
    }
    pub fn take_a_ticket(
        ctx: Context<Ticket>
    ) -> Result<()> {
        instructions::take_a_ticket::take_a_ticket(ctx)
    }
}