use anchor_lang::prelude::*;
use anchor_spl::associated_token;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{transfer_checked, TransferChecked};
use anchor_spl::token_interface::{Mint, TokenAccount};
// use anchor_spl::{token_2022, token_interface};

use crate::constants::{STAKE_SEED, USER_SEED};
use crate::errors::StakingErrors;
use crate::states::stake_account::Update;
use crate::states::user_info::UpdateUser;
use crate::states::{StakeAccount, UserInfo};

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(mut)]
    pub token: InterfaceAccount<'info, Mint>,

    /// CHECK
    #[account(mut)]
    pub admin: AccountInfo<'info>,

    #[account(mut)]
    pub buyer: Signer<'info>,
    // = stake_info.bump
    #[account(
        mut,
        seeds = [STAKE_SEED],
        bump 
    )]
    pub stake_info: Account<'info, StakeAccount>,

    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = stake_info,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = buyer.key(),
        associated_token::token_program = token_program
    )]
    pub buyer_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = buyer,
        space = UserInfo::INIT_SPACE,
        seeds = [USER_SEED, buyer.key().as_ref()],
        bump
    )]
    pub user_info: Account<'info, UserInfo>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}

impl<'info> StakeTokens<'info> {
    pub fn stake(&mut self, amount: u64) -> Result<()> {
        let cur_timestamp = u64::try_from(Clock::get()?.unix_timestamp).unwrap();
        if !self.stake_info.is_live {
            return Err(StakingErrors::InActive.into());
        } else if self.stake_info.min_stake_amount > amount {
            return Err(StakingErrors::MinAmount.into());
        } else if self.stake_info.max_stake_amount < amount
            || self.user_info.staked_amount + amount > self.stake_info.max_stake_amount
        {
            return Err(StakingErrors::MaxAmount.into());
        }
        transfer_checked(self.into_stake_context(), amount, self.token.decimals)?;
        self.stake_info.update_when_user_stakes(amount)?;
        self.user_info
            .update_when_user_stakes(amount, cur_timestamp)?;
        Ok(())
    }

    fn into_stake_context(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.buyer_ata.to_account_info(),
            to: self.vault.to_account_info(),
            mint: self.token.to_account_info(),
            authority: self.buyer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
