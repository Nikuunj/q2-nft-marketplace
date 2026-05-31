use anchor_lang::prelude::*;

use crate::state::{Listing, Offer};

#[derive(Accounts)]
pub struct CloseOffer<'info> {
    #[account(mut)]
    pub offer_maker: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub maker: UncheckedAccount<'info>,

    #[account(
        mut,
        close = offer_maker,
        seeds = [b"offer", offer.listing.as_ref(), offer.offer_maker.as_ref()],
        bump = offer.bump,
        has_one = offer_maker,
        has_one = listing
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        mut,
        seeds = [b"listing", listing.asset.as_ref()],
        bump = listing.bump,
        has_one = maker,
    )]
    pub listing: Account<'info, Listing>,

    pub system_program: Program<'info, System>,
}


impl<'info> CloseOffer<'info> {
    pub fn close_offer(&mut self) -> Result<()> {
        self.listing.solded = false;
        Ok(())
    }
}
