#![allow(unexpected_cfgs, deprecated, ambiguous_glob_reexports)]

use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("5FNsdhUK4Qn9HLkrkpR8bi916vistHfi5kihUk8As26M");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initliaze(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.initialize(name, fee, ctx.bumps)
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_list(price, ctx.bumps)
    }

    pub fn buy(ctx: Context<Buy>) -> Result<()> {
        ctx.accounts.send_sol()?;
        ctx.accounts.receive_nft()?;
        ctx.accounts.receive_rewards()
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.refund()
    }

    pub fn maker_offer(ctx: Context<MakeOffer>, price: u64) -> Result<()> {
        ctx.accounts.make_offer(price, ctx.bumps)
    }
    pub fn accept_offer(ctx: Context<AcceptOffer>) -> Result<()> {
        ctx.accounts.send_sol()?;
        ctx.accounts.receive_nft()?;
        ctx.accounts.receive_rewards()
    }

    pub fn close_offer(ctx: Context<CloseOffer>) -> Result<()> {
        ctx.accounts.close_offer()
    }

    pub fn withdraw_fee(ctx: Context<WithdrawFee>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw_fee(amount)
    }

    pub fn reject_offer(ctx: Context<RejectOffer>) -> Result<()> {
        ctx.accounts.reject_offer()
    }
}
