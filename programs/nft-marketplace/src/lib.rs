use anchor_lang::prelude::*;

declare_id!("BY1bqJ3LAi54jWk6xo5WF5Nxc6tXf75mpBDyzZgVZsCQ");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
