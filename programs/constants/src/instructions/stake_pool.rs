use anchor_lang::prelude::*;
// use anchor_spl::token_interface::Mint;
// use anchor_spl::

use crate::constants::STAKE_SEED;
use crate::states::stake_account::Update;
use crate::states::StakeAccount;

#[derive(Accounts)]
pub struct StakePool<'info> {
    #[account(
        init_if_needed,
        seeds = [STAKE_SEED],
        bump,
        payer = authority,
        space = StakeAccount::INIT_SPACE
    )]
    pub stake_info: Account<'info, StakeAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> StakePool<'info> {
    pub fn create(
        &mut self,
        token: Pubkey,
        apr: u16,
        min_stake_amount: u64,
        max_stake_amount: u64,
        is_flexible: bool,
        stake_time: u64,
        cool_down_time: u64,
    ) -> Result<()> {
        self.stake_info.set_inner(StakeAccount {
            token: token,
            apr: apr,
            total_deposit_for_reward: 0,
            total_amount_staked: 0,
            amount_of_users: 0,
            total_distributed_reward: 0,
            total_premature_user_unstake: 0,
            min_stake_amount: min_stake_amount,
            max_stake_amount: max_stake_amount,
            authority: self.authority.key(),
            is_live: true,
            is_flexible: is_flexible,
            stake_time: stake_time,
            cool_down_time: cool_down_time,
        });
        Ok(())
    }

    pub fn update_rules(
        &mut self,
        apr: u16,
        min_stake_amount: u64,
        max_stake_amount: u64,
        stake_time: u64,
        cool_down_time: u64,
    ) -> Result<()> {
        self.stake_info.update_stake_rules(
            apr,
            min_stake_amount,
            max_stake_amount,
            stake_time,
            cool_down_time,
        )?;
        Ok(())
    }
}
