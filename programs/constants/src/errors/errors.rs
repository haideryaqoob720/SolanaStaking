use anchor_lang::prelude::*;

#[error_code]
pub enum StakingErrors {
    #[msg("You tried to stake less than the minimum value.")]
    MinAmount,
    #[msg("You tried to stake more than the maximum allowed amount.")]
    MaxAmount,
    #[msg("Staking is currently inactive")]
    InActive,
}
