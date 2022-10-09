use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey,
}; 
use std::str::FromStr;

declare_id!("DvrXsDmz9cBLbLsCun9a37ojkCaxG1hxSAJsg36yttu7");

#[program]
pub mod ticket_system {
    use super::*;

    pub fn create_system_account(
        ctx: Context<TicketSystem>,
    ) -> Result<()> {
        let pubkey: Pubkey = Pubkey::from_str("98EgyyxzsehNpNy8yTpcGfTxRmxxVJnu2RHwSvF5nn6i").unwrap();
        require_keys_eq!(ctx.accounts.signer.key(), pubkey, ErrorCode::AuthorityError);
        let (_system_account_pda, bump): (Pubkey, u8) = Pubkey::find_program_address(&[ctx.accounts.signer.key().as_ref()], &pubkey);
        let system_account: &mut Account<SystemAccount> = &mut ctx.accounts.system_account;
        system_account.authority = ctx.accounts.signer.key();
        system_account.bump_original = bump;
        system_account.events = 1;
        Ok(())
    }
    pub fn create_event(
        ctx: Context<Event>,
    ) -> Result<()> {
        let pubkey: Pubkey = Pubkey::from_str("98EgyyxzsehNpNy8yTpcGfTxRmxxVJnu2RHwSvF5nn6i").unwrap();
        //require_keys_eq!(ctx.accounts.signer.key(), pubkey, ErrorCode::AuthorityError);
        let (_event_account_pda, bump): (Pubkey, u8) = Pubkey::find_program_address(&[ctx.accounts.seed.key().as_ref()], &pubkey);
        let system_account: &mut Account<SystemAccount> = &mut ctx.accounts.system_account;
        let event_account: &mut Account<EventAccount> = &mut ctx.accounts.event_account;
        event_account.authority = ctx.accounts.signer.key();
        event_account.bump_original = bump;
        system_account.events += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TicketSystem<'info> {
    #[account(init, seeds = [signer.key().as_ref()], bump, payer = signer, space = 49)]
    pub system_account: Account<'info, SystemAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Event<'info> {
    #[account(mut, seeds = [system_account.authority.key().as_ref()], bump = system_account.bump_original)]
    pub system_account: Account<'info, SystemAccount>,
    #[account(init, seeds = [seed.key().as_ref()], bump, payer = signer, space = 73)]
    pub event_account: Account<'info, EventAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub seed: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SystemAccount {
    pub authority: Pubkey, 
    pub bump_original: u8,
    pub events: u64
}

#[account]
pub struct EventAccount {
    pub authority: Pubkey, 
    pub bump_original: u8,
    pub dynamic_seed: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("")]AuthorityError,
}
