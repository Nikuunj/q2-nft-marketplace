use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::state::{Listing, Offer};

#[derive(Accounts)]
pub struct RejectOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    /// CHECK:
    #[account(mut)]
    pub offer_maker: UncheckedAccount<'info>,

    #[account(
        mut,
        close = offer_maker,
        seeds = [b"offer", offer.listing.as_ref(), offer.offer_maker.as_ref()],
        bump = offer.bump,
        has_one = offer_maker,
        has_one = listing,
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        mut,
        seeds = [b"offer_vault", offer.key().as_ref()],
        bump = offer.vault_bump
    )]
    pub offer_vault: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"listing", listing.asset.as_ref()],
        bump = listing.bump,
        has_one = maker,
    )]
    pub listing: Account<'info, Listing>,
    pub system_program: Program<'info, System>,
}

impl<'info> RejectOffer<'info> {
    pub fn reject_offer(&mut self) -> Result<()> {
        let offer_key = self.offer.key();
        let signers_seeds: &[&[&[u8]]] =
            &[&[b"offer_vault", offer_key.as_ref(), &[self.offer.vault_bump]]];

        transfer(
            CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.offer_vault.to_account_info(),
                    to: self.offer_maker.to_account_info(),
                },
                signers_seeds,
            ),
            self.offer_vault.lamports(),
        )
    }
}
