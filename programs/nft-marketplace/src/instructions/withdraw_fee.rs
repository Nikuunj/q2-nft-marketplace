use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{error::ErrorCode, state::MarketPlace};

pub struct WithdrawFee<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        has_one = admin,
        seeds = [b"maketplace", maketplace.name.as_bytes()],
        space = MarketPlace::INIT_SPACE + MarketPlace::DISCRIMINATOR.len(),
        bump = maketplace.bump
    )]
    pub maketplace: Account<'info, MarketPlace>,

    #[account(
        seeds = [b"treasury", maketplace.key().as_ref()],
        bump = maketplace.treasury_bump
    )]
    pub treasury: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawFee<'info> {
    pub fn withdraw_fee(&mut self, amount: u64) -> Result<()> {
        require!(amount <= self.treasury.lamports(), ErrorCode::AmountTooMuch);
        let seeds = &[
            b"treasury",
            self.maketplace.key().as_ref(),
            &[self.maketplace.treasury_bump],
        ];
        let signer_seeds = &[seeds];

        transfer(
            CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.treasury.to_account_info(),
                    to: self.admin.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )
    }
}
