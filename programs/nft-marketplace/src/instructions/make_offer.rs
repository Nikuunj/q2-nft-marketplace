// make offecr give counter offer
use anchor_lang::prelude::*;

use crate::state::{Listing, Offer};

#[derive(Accounts)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        init,
        payer = maker,
        seeds = [b"offer", listing.key().as_ref(), maker.key().as_ref()],
        space = Offer::INIT_SPACE + Offer::DISCRIMINATOR.len(),
        bump
    )]
    pub offer: Account<'info, Offer>,
    #[account(
        mut,
        seeds = [b"listing", listing.asset.as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    pub system_program: Program<'info, System>,
}

impl<'info> MakeOffer<'info> {
    pub fn make_offer(&mut self, price: u64, bumps: MakeOfferBumps) -> Result<()> {
        self.offer.set_inner(Offer {
            listing: self.listing.key(),
            offer_maker: self.maker.key(),
            price,
            accepted: false,
            bump: bumps.offer,
        });
        Ok(())
    }
}
