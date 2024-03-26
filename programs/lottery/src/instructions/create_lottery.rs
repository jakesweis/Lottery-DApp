use anchor_lang::prelude::*;
use crate::states::*;

pub fn create_lottery(ctx: Context<CreateLotteryContext>, ticket_price: u64) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;
    let master = &mut ctx.accounts.master;

    master.last_id += 1;
    
    lottery.id = master.last_id;
    lottery.authority = ctx.accounts.authority.key();
    lottery.ticket_price = ticket_price;

    msg!("Created lottery: {}", lottery.id);
    msg!("Authory: {}", lottery.authority);
    msg!("Ticket price: {}", lottery.ticket_price);

    Ok(())
}

#[derive(Accounts)]
pub struct CreateLotteryContext<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 4 + 32 + 8 + 4 + 1 + 4 + 1 + 8,
        seeds = [
            LOTTERY_SEED.as_bytes(),
            &(master.last_id + 1).to_le_bytes()
            ],
        bump)]
    pub lottery: Account<'info, Lottery>,
    #[account(
        mut,
        seeds = [MASTER_SEED.as_bytes()],
        bump)]
    pub master: Account<'info, Master>,    
    pub system_program: Program<'info, System>,
}