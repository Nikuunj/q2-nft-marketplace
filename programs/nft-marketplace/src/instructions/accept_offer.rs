use anchor_lang::prelude::*;

use crate::state::{Listing, Offer};

#[derive(Accounts)]
pub struct AcceptOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    /// CHECK:
    pub offer_maker: UncheckedAccount<'info>,

    #[account(
        mut,
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

impl<'info> AcceptOffer<'info> {
    pub fn accept_offer(&mut self) -> Result<()> {
        self.offer.accepted = true;
        self.listing.solded = true;
        Ok(())
    }
}
