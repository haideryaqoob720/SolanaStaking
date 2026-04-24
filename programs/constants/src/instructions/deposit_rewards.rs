use anchor_lang::prelude::*;
use anchor_spl::token_2022::TransferChecked;
use anchor_spl::token_interface::{Mint, TokenAccount};
use anchor_spl::{associated_token, token_2022};

use crate::constants::STAKE_SEED;
use crate::states::stake_account::Update;
use crate::states::StakeAccount;


#[derive(Accounts)]
pub struct DepositRewards<'info> {
    #[account(mut)]
    pub token: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = admin.key(),
        associated_token::token_program = token_program
    )]
    pub from_ata: InterfaceAccount<'info, TokenAccount>,
    // owned by the depositor
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = token,
        associated_token::authority = stake_info,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    // owned by stake_info. In order to carrry out any transactions regarding the vault, the stake info pda must be used that owns the vault.
    #[account(
        mut,
        seeds = [STAKE_SEED],
        bump 
    )]
    pub stake_info: Account<'info, StakeAccount>,
    //pda
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, token_2022::Token2022>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}

impl<'info> DepositRewards<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        token_2022::transfer_checked(self.into_deposit_context(), amount, self.token.decimals)?;
        self.stake_info.update_deposit(amount)?;
        Ok(())
    }

    fn into_deposit_context(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.from_ata.to_account_info(),
            to: self.vault.to_account_info(),
            mint: self.token.to_account_info(),
            authority: self.admin.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
