use anchor_lang::prelude::*;

#[account]
    // pub bump:u8,
#[derive(Default)]
pub struct StakeAccount {
    pub token: Pubkey,                     // the token address
    pub apr: u16,                          // the rate for rewards
    pub total_deposit_for_reward: u64, // the amount deposited by the admin. this is used to distribute rewards
    pub total_amount_staked: u64,      // the total amount that users have deposited
    pub amount_of_users: u64,          // the total number of user that have staked tokens
    pub total_distributed_reward: u64, // the amount of tokens distributed as rewards
    pub total_premature_user_unstake: u64, // the amount of users that unstaked the token before fixed time ends
    pub min_stake_amount: u64,             // the minimum amount a user can stake
    pub max_stake_amount: u64,             // the max amount a user can stake
    pub authority: Pubkey,                 // the admin who has the autority to mint tokens
    pub is_live: bool,                     // admin allows user to stake tokens
    pub is_flexible: bool,                 // can the users stake or unstake any time
    pub stake_time: u64, // the amount of time the token needs to be staked if the stake pool is not flexible
    pub cool_down_time: u64, // if this time is greater than the current time than staking is disabled. if it is zero than staking is enabled.
}

        // + std::mem::size_of::<u8>()
impl Space for StakeAccount {
    // const DISCRIMINATOR: usize = 8;
    const INIT_SPACE: usize = 8
        + std::mem::size_of::<Pubkey>()
        + std::mem::size_of::<u16>()
        + std::mem::size_of::<u64>()
        + std::mem::size_of::<u64>()
        + std::mem::size_of::<u64>()
        + std::mem::size_of::<u64>()
        + std::mem::size_of::<u64>()
        + std::mem::size_of::<u64>()
        + std::mem::size_of::<u64>()
        + std::mem::size_of::<Pubkey>()
        + std::mem::size_of::<bool>()
        + std::mem::size_of::<bool>()
        + std::mem::size_of::<u64>()
        + std::mem::size_of::<u64>();
}

pub trait Update {
    fn update_stake_rules(
        &mut self,
        apr: u16,
        min_stake_amount: u64,
        max_stake_amount: u64,
        stake_time: u64,
        cool_down_time: u64,
    ) -> Result<()>;

    fn update_when_user_stakes(&mut self, amount: u64) -> Result<()>;

    fn update_deposit(&mut self, amount: u64) -> Result<()>;

    fn update_when_user_un_stakes(&mut self, amount: u64, reward: u64) -> Result<()>;
}

impl Update for StakeAccount {
    fn update_stake_rules(
        &mut self,
        apr: u16,
        min_stake_amount: u64,
        max_stake_amount: u64,
        stake_time: u64,
        cool_down_time: u64,
    ) -> Result<()> {
        self.apr = apr;
        self.min_stake_amount = min_stake_amount;
        self.max_stake_amount = max_stake_amount;
        self.stake_time = stake_time;
        self.cool_down_time = cool_down_time;
        Ok(())
    }

    fn update_deposit(&mut self, amount: u64) -> Result<()> {
        self.total_deposit_for_reward += amount;
        Ok(())
    }

    fn update_when_user_stakes(&mut self, amount: u64) -> Result<()> {
        self.amount_of_users += 1;
        self.total_amount_staked += amount;
        Ok(())
    }

    fn update_when_user_un_stakes(&mut self, amount: u64, reward: u64) -> Result<()> {
        self.amount_of_users -= 1;
        self.total_amount_staked -= amount;
        self.total_distributed_reward += reward;
        Ok(())
    }
}
