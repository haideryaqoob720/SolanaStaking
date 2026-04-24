use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserInfo {
    pub staked_amount: u64,
    pub premature_unstake: bool,
    pub stake_time: u64,
    pub unstake_time: u64,
    pub profit: u64,
}

impl Space for UserInfo {
    const INIT_SPACE: usize = 8
        + std::mem::size_of::<u64>()
        + std::mem::size_of::<bool>()
        + std::mem::size_of::<u64>()
        + std::mem::size_of::<u64>()
        + std::mem::size_of::<u64>();
}

pub trait UpdateUser {
    fn update_when_user_stakes(&mut self, amount: u64, cur_timestamp: u64) -> Result<()>;
    fn update_when_user_un_stakes(&mut self, amount: u64, cur_timestamp: u64) -> Result<()>;
}

impl UpdateUser for UserInfo {
    fn update_when_user_stakes(&mut self, amount: u64, cur_timestamp: u64) -> Result<()> {
        self.stake_time = cur_timestamp;
        self.staked_amount += amount;
        Ok(())
    }
    fn update_when_user_un_stakes(&mut self, amount: u64, cur_timestamp: u64) -> Result<()> {
        self.staked_amount = 0;
        self.profit = amount;
        self.unstake_time = cur_timestamp;
        Ok(())
    }
}
