use anchor_lang::prelude::*;

pub mod errors;
pub mod utils;
pub mod instructions;
pub mod state;
pub mod consts;

pub use instructions::initialize::InitialCurveConfiguration;
pub use instructions::create_pool::CreateLiquidityPool;
pub use instructions::add_liquidity::AddLiquidity;
pub use instructions::remove_liquidity::RemoveLiquidity;
pub use instructions::buy::Buy;
pub use instructions::sell::Sell;

declare_id!("9yFGjEq3TEdgtQSzQaZQBg1xuCbj5hDshPxo1xy3E4d8");

#[program]
pub mod pump {
    use super::*;

    pub fn initialize(ctx: Context<InitialCurveConfiguration>, fee: f64) -> Result<()> {
        instructions::initialize(ctx, fee)
    }

    pub fn create_pool(ctx: Context<CreateLiquidityPool>) -> Result<()> {
        instructions::create_pool(ctx)
    }

    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
    ) -> Result<()> {
        instructions::add_liquidity(ctx)
    }

    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>, bump: u8) -> Result<()> {
        instructions::remove_liquidity(ctx, bump)
    }

    pub fn buy(ctx: Context<Buy>, amount: u64) -> Result<()> {
        instructions::buy(ctx, amount)
    }

    pub fn sell(ctx: Context<Sell>, amount: u64, bump: u8) -> Result<()> {
        instructions::sell(ctx, amount, bump)
    }
}