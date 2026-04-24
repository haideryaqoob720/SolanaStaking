use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

use instructions::*;

declare_id!("DU46a8X7ZQUYf9TmKahdZoghajjhX3BKRP1XQdCyQxb3");

#[program]
pub mod solana_staking {
    use super::*;

    // &ctx.bumps,
    pub fn create_stake_pool(
        ctx: Context<StakePool>,
        token: Pubkey,
        apr: u16,
        min_stake_amount: u64,
        max_stake_amount: u64,
        is_flexible: bool,
        stake_time: u64,
        cool_down_time: u64,
    ) -> Result<()> {
        ctx.accounts.create(
            token,
            apr,
            min_stake_amount,
            max_stake_amount,
            is_flexible,
            stake_time,
            cool_down_time,
        )
    }

    pub fn update_stake_pool(
        ctx: Context<StakePool>,
        apr: u16,
        min_stake_amount: u64,
        max_stake_amount: u64,
        stake_time: u64,
        cool_down_time: u64,
    ) -> Result<()> {
        ctx.accounts.update_rules(
            apr,
            min_stake_amount,
            max_stake_amount,
            stake_time,
            cool_down_time,
        )
    }

    pub fn deposit_rewards(ctx: Context<DepositRewards>, amount: u64) -> Result<()> {
        // return deposit_rewards::deposit_rewards(ctx, amount);
        ctx.accounts.deposit(amount)
    }

    pub fn stake_tokens(ctx: Context<StakeTokens>, amount: u64) -> Result<()> {
        // return stake_tokens::stake_tokens(ctx, amount);
        ctx.accounts.stake(amount)
    }

    pub fn unstake_tokens(ctx: Context<UnstakeTokens>, bump: u8) -> Result<()> {
        // return unstake_tokens::unstake_tokens(ctx, bump);
        ctx.accounts.unstake(bump)
    }
}
