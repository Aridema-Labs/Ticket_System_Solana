use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
}; 
use std::str::FromStr;
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn create_system_account(
    ctx: Context<TicketSystem>,
) -> Result<()> {
    let pubkey: Pubkey = Pubkey::from_str("98EgyyxzsehNpNy8yTpcGfTxRmxxVJnu2RHwSvF5nn6i").unwrap();
    require_keys_eq!(ctx.accounts.signer.key(), pubkey, ErrorCode::AuthorityError);
    let (_system_account_pda, bump): (Pubkey, u8) = Pubkey::find_program_address(&[pubkey.as_ref()], ctx.program_id);
    let system_account: &mut Account<SystemAccount> = &mut ctx.accounts.system_account;
    system_account.authority = ctx.accounts.signer.key();
    system_account.bump_original = bump;
    system_account.history_events = 1;
    system_account.active_events = 0;
    system_account.history_tickets_sold = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct TicketSystem<'info> {
    #[account(init, seeds = [signer.key().as_ref()], bump, payer = signer, space = 65)]
    pub system_account: Account<'info, SystemAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}