use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey,
}; 
use std::str::FromStr;

declare_id!("JDXSam2kCQc1eTcZxokCUqCkuBxPaMXzBf3iaVjVbcqb");

#[program]
pub mod ticket_system {
    use super::*;
    pub fn create_event(
        ctx: Context<TicketSystem>,
    ) -> Result<()> {
        let pubkey: Pubkey = Pubkey::from_str("CqtPfRjEWtqRR1XZq4EkfSUimCPxPiie7UcrWFJ2DxVV").unwrap();
        require_keys_eq!(ctx.accounts.signer.key(), pubkey, ErrorCode::AuthorityError);
        let (_bus_pda, bump): (Pubkey, u8) = Pubkey::find_program_address(&[ctx.accounts.signer.key().as_ref()], &pubkey);
        let system_account: &mut Account<SystemAccount> = &mut ctx.accounts.system_account;
        system_account.authority = ctx.accounts.signer.key();
        system_account.bump_original = bump;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TicketSystem<'info> {
    #[account(init, seeds = [signer.key().as_ref()], bump, payer = signer, space = 48)]
    pub system_account: Account<'info, SystemAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SystemAccount {
    pub authority: Pubkey, 
    pub bump_original: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("")]AuthorityError,
}
