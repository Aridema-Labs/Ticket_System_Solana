use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("")]AuthorityError, 
    #[msg("")]TooLong, 
    #[msg("")]AmountTicketsError,
}
