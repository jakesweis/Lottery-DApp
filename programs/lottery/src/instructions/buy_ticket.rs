use anchor_lang::{prelude::*, solana_program::{program::invoke, system_instruction::transfer}};
use crate::errors::LotteryError;
use crate::states::*;

pub fn buy_ticket(ctx: Context<BuyTicketContext>, lottery_id: u32) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;
    let ticket = &mut ctx.accounts.ticket;
    let buyer = &mut ctx.accounts.buyer;

    if lottery.winner_id.is_some() {
        return err!(LotteryError::WinnerAlreadyExists);
    }

    invoke(
        &transfer(
            &buyer.key(),
            &lottery.key(),
            lottery.ticket_price,
        ),
        &[
            buyer.to_account_info(),
            lottery.to_account_info(),
            ctx.accounts.system_program.to_account_info()
        ],
    )?;

    lottery.last_ticket_id += 1;
    ticket.id = lottery.last_ticket_id;
    ticket.lottery_id = lottery_id;
    ticket.authority = buyer.key();

    msg!("Ticket id: {}", ticket.id);
    msg!("Ticket authority: {}", ticket.authority);

    Ok(())
}

#[derive(Accounts)]
#[instruction(lottery_id: u32)]
pub struct BuyTicketContext<'info> {
    #[account(
        mut,
        seeds = [
            LOTTERY_SEED.as_bytes(),
            &lottery_id.to_le_bytes()
            ],
        bump)]
    pub lottery: Account<'info, Lottery>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(
        init,
        payer = buyer,
        space = 4 + 4 + 32 + 8,
        seeds = [
            TICKET_SEED.as_bytes(),
            lottery.key().as_ref(),
            &(lottery.last_ticket_id + 1).to_le_bytes()
            ],
        bump)]
    pub ticket: Account<'info, Ticket>,
    pub system_program: Program<'info, System>,
}