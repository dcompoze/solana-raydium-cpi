use anchor_lang::prelude::*;

declare_id!("7vQYvV51BsKTZErQuX22K2nF2F9YsY1baQSBR956UpU5");

#[program]
pub mod raydium_cpi {
    use super::*;

    pub fn add_liquidity(ctx: Context<AddLiquidity>) -> Result<()> {
        Ok(())
    }

    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddLiquidity {}

#[derive(Accounts)]
pub struct RemoveLiquidity {}
