use anchor_lang::prelude::*;

pub use instructions::InitialCurveConfiguration;
pub use instructions::CreateLiquidityPool;
pub use instructions::AddLiquidity;
pub use instructions::RemoveLiquidity;
pub use instructions::Buy;
pub use instructions::Sell;

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