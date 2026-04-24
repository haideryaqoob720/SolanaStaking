use anchor_lang::prelude::*;
// #[cfg(feature = "no-entrypoint")]
use std::fmt::Display;

#[derive(
    Clone, Copy, Debug, Default, AnchorSerialize, AnchorDeserialize, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Fee {
    pub basis_points: u32,
}

impl Display for Fee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // use integer division to avoid including f64 libs
        write!(
            f,
            "{}.{:0>2}%",
            self.basis_points / 100,
            self.basis_points % 100
        )
    }
}

impl Fee{
    pub const fn from_basis_points(basis_points: u32) -> Self {
        Self { basis_points }
    }
}