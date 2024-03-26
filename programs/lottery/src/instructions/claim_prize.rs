use anchor_lang::prelude::*;
use crate::{states::*, errors::LotteryError};

pub fn claim_prize(ctx: Context<ClaimPrizeContext>, _lottery_id: u32, _ticket_id: u32) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;
    let ticket = &ctx.accounts.ticket;
    let winner = &ctx.accounts.authority;

    if lottery.claimed {
        return err!(LotteryError::AlreadyClaimed);
    }

    match lottery.winner_id {
        Some(winner_id) =>{
            if winner_id != ticket.id {
                return err!(LotteryError::InvalidWinner);
            }
        }
        None => return err!(LotteryError::WinnerNotChosen),
    }

    let prize = lottery
        .ticket_price
        .checked_mul(lottery.last_ticket_id.into())
        .unwrap();

    **lottery.to_account_info().try_borrow_mut_lamports()? -= prize;
    **winner.to_account_info().try_borrow_mut_lamports()? += prize;

    lottery.claimed = true;

    msg!(
        "{} claimed {} lamports from lottery id {} with ticket id {}",
        winner.key(),
        prize,
        lottery.id,
        ticket.id,
    );

    Ok(())
} 

#[derive(Accounts)]
#[instruction(lottery_id: u32, ticket_id: u32)]
pub struct ClaimPrizeContext<'info> {
    #[account(
        mut,
        seeds = [
            LOTTERY_SEED.as_bytes(),
            &lottery_id.to_le_bytes(),
        ],
        bump)]
    pub lottery: Account<'info, Lottery>,
    #[account(
        seeds = [
            TICKET_SEED.as_bytes(),
            lottery.key().as_ref(),
            &ticket_id.to_le_bytes(),
        ],
        bump)]
    pub ticket: Account<'info, Ticket>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}