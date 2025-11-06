use crate::{errors::AssureFundError, state::fund::*};
use anchor_lang::prelude::*;

pub fn support_project(
    ctx: Context<SupportProject>,
    fund_amount: u16,
    project_id: Pubkey,
) -> Result<()> {
    

    Ok(())
}

#[derive(Accounts)]
#[instruction(fund_amount: u16)]

pub struct SupportProject<'info> {
    #[account(mut)]
    pub fund_authority: Signer<'info>,
}
