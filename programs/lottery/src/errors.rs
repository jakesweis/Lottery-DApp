use anchor_lang::prelude::*;

#[error_code]
pub enum LotteryError {
    #[msg("Winner already exists")]
    WinnerAlreadyExists,
    #[msg("No tickets in lottery")]
    NoTickets,
    #[msg("Winner has not been chosen")]
    WinnerNotChosen,
    #[msg("Invalid winner")]
    InvalidWinner,
    #[msg("Minimum number of Likes Reached")]
    MinLikesReached,
    #[msg("Minimum number of Dislikes Reached")]
    MinDislikesReached,
    #[msg("Prize has already been claimed")]
    AlreadyClaimed,
}