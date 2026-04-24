use anchor_lang::prelude::*;
// use anchor_spl::associated_token;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{transfer_checked, TransferChecked};
use anchor_spl::token_interface::{Mint, TokenAccount};
// use anchor_spl::{token_2022, token_interface};

use crate::constants::{STAKE_SEED, USER_SEED};
use crate::states::{StakeAccount, UserInfo};

use crate::states::stake_account::Update;
use crate::states::user_info::UpdateUser;

#[derive(Accounts)]
pub struct UnstakeTokens<'info> {
    #[account(mut)]
    pub token: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = stake_info,
        associated_token::token_program = token_program
    )]
    pub stake_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = buyer,
        associated_token::token_program = token_program
    )]
    pub buyer_ata: InterfaceAccount<'info, TokenAccount>,

    /// CHECK
    #[account(mut)]
    pub admin: AccountInfo<'info>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    //  = stake_info.bump
    #[account(
        mut,
        seeds = [STAKE_SEED],
        bump
    )]
    pub stake_info: Account<'info, StakeAccount>,

    #[account(
        mut,
        seeds = [USER_SEED, buyer.key().as_ref()],
        bump
    )]
    pub user_info: Account<'info, UserInfo>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> UnstakeTokens<'info> {
    pub fn unstake(&mut self, bump: u8) -> Result<()> {
        let cur_timestamp = u64::try_from(Clock::get()?.unix_timestamp).unwrap();
        let amount = self.calculate_reward(cur_timestamp);
        // let signer_seeds = &[&[STAKE_SEED, &[bump]][..]];
        // let transfer_context = self.into_unstake_context();
        transfer_checked(
            self.into_unstake_context()
                .with_signer(&[&[STAKE_SEED, &[bump]][..]]),
            amount,
            self.token.decimals,
        )?;
        let reward: u64 = amount - self.user_info.staked_amount;
        self.stake_info
            .update_when_user_un_stakes(self.user_info.staked_amount, reward)?;
        self.user_info
            .update_when_user_un_stakes(reward, cur_timestamp)?;
        Ok(())
    }

    fn calculate_reward(&self, cur_timestamp: u64) -> u64 {
        let bpc: u64 = 100;
        let days_in_year: u64 = 365;
        let one_day: u64 = 10;

        let user_info = &self.user_info;
        let stake_info = &self.stake_info;

        let stake_time: u64 = stake_info.stake_time;
        let reward_rate_per_day: u64 =
            ((stake_info.apr as u64 * bpc * 10000 as u64) / days_in_year).into();
        let reward_per_day: u64 = user_info.staked_amount * reward_rate_per_day;
        let days: u64 = ((cur_timestamp - user_info.stake_time) / one_day).into();
        let total_reward = (reward_per_day * days) / 1000000000;
        msg!("Current time: {}", cur_timestamp);
        msg!("stake time: {}", stake_time);
        msg!("apr bpc: {}", stake_info.apr as u64 * bpc);
        msg!("reward rate per day: {}", reward_rate_per_day);
        msg!("reward per day: {}", reward_per_day);
        msg!("days: {}", days);
        msg!("total reward: {}", total_reward);

        let amount = user_info.staked_amount + total_reward;
        amount
    }

    fn into_unstake_context(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        // let signer_seeds = &[&[STAKE_SEED, &[self.stake_info.bump]][..]];
        // let info_bump: u8 = bump.clone();
        let cpi_accounts = TransferChecked {
            from: self.stake_vault.to_account_info(),
            to: self.buyer_ata.to_account_info(),
            mint: self.token.to_account_info(),
            authority: self.stake_info.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
