use anchor_lang::prelude::*;
use crate::{states::*, errors::LotteryError};

pub fn pick_winner(ctx: Context<PickWinnerContext>, _lottery_id: u32) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;

    if lottery.winner_id.is_some() {
        return err!(LotteryError::WinnerAlreadyExists);
    }

    if lottery.last_ticket_id == 0 {
        return err!(LotteryError::NoTickets);
    }

    // let clock = Clock::get()?;
    // let random_number = ((u64::from_le_bytes(
    //     <[u8; 8]>::try_from(&hash(&clock.unix_timestamp.to_be_bytes()).to_bytes()[..8])
    //     .unwrap(),
    // ) * clock.slot)
    //     % u32::MAX as u64) as u32;

    // I tried everything to get the above code to work,
    // and I could not get it to even complete. I sadly
    // have to abandon this and just use the last ticket ID.
    // I also looked at using other VRFs.
    let winner_id = lottery.last_ticket_id;
    lottery.winner_id = Some(winner_id);

    msg!("Winner id: {}", winner_id);

    Ok(())
}

#[derive(Accounts)]
#[instruction(lottery_id: u32)]
pub struct PickWinnerContext<'info> {
    #[account(
        mut,
        seeds = [
            LOTTERY_SEED.as_bytes(), 
            &lottery_id.to_le_bytes(),
        ],
    bump,
    has_one = authority)]
    pub lottery: Account<'info, Lottery>,
    pub authority: Signer<'info>,
}