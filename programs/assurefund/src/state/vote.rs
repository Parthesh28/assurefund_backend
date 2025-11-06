use anchor_lang::{prelude::*};

pub const VOTE: &[u8] = b"ASSUREFUND_MILESTONE_VOTE";

#[account]
#[derive(InitSpace)]
pub struct Vote{
    pub vote_authority: Pubkey,
    pub project_id: Pubkey,
    #[max_len(32)]
    pub milestone_id: Pubkey,
    pub decision: bool,
    pub bump: u8
}