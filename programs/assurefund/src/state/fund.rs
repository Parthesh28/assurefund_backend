use anchor_lang::prelude::*;

pub const FUND: &[u8] = b"ASSUREFUND_PROJECT_FUND";

#[account]
#[derive(InitSpace)]
pub struct Fund{
    pub fund_authority: Pubkey,
    pub project_id: Pubkey,
    #[max_len(32)]
    pub amount: u16,
    pub bump: u8
}