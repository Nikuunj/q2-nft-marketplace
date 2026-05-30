use anchor_lang::prelude::*;
use mpl_core::{instructions::TransferV1CpiBuilder, ID as MPL_CORE_ID};

use crate::state::Listing};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    /// CHECK: Is this asset account validate during cpi transfer by mpl-core
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,

    /// CHECK: Is this collection account validate during cpi transfer by mpl-core
    #[account(mut)]
    pub collection: Option<UncheckedAccount<'info>>,


    #[account(
        init, 
        payer = maker,
        seeds = [b"listing", asset.key().as_ref()],
        space = Listing::INIT_SPACE + Listing::DISCRIMINATOR.len(),
        bump
    )]
    pub listing: Account<'info, Listing>,

    /// CHECK: Is this mpl core program validate during cpi transfer by mpl-core
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> List<'info> {
    pub fn create_list(&mut self, price: u64, bumps: ListBumps) -> Result<()> {
        self.listing.set_inner(Listing {
            maker: self.maker.key(),
            asset: self.asset.key(),
            price,
            bump: bumps.listing
        });

        TransferV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(self.collection.as_ref().map(|c | c.as_ref()))
            .payer(&self.maker.to_account_info())
            .authority(Some(&self.maker.to_account_info()))
            .new_owner(&self.listing.to_account_info())
            .system_program(Some(&self.system_program.to_account_info()))
            .invoke()?;

        Ok(())
    }
}
