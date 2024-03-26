use anchor_lang::prelude::*;
use crate::states::*;

pub fn initialize_master(_ctx: Context<InitializeMaster>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeMaster<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 4 + 8,
        seeds = [MASTER_SEED.as_bytes()],
        bump)]
    pub master: Account<'info, Master>,
    pub system_program: Program<'info, System>,
}