elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait CustomRewardsModule:
    config::ConfigModule
    + token_send::TokenSendModule
    + farm_token::FarmTokenModule
    + rewards::RewardsModule
{
    #[only_owner]
    #[payable("*")]
    #[endpoint(depositRewards)]
    fn deposit_rewards(&self, #[payment_token] payment_token: TokenIdentifier) -> SCResult<()> {
        let reward_token_id = self.reward_token_id().get();
        require!(payment_token == reward_token_id, "Invalid token");
        Ok(())
    }

    fn calculate_and_increase_per_block_rewards(&self) -> BigUint {
        let current_block_nonce = self.blockchain().get_block_nonce();
        let last_reward_nonce = self.last_reward_block_nonce().get();

        if current_block_nonce > last_reward_nonce {
            let to_mint = core::cmp::min(
                self.calculate_per_block_rewards(current_block_nonce, last_reward_nonce),
                self.reward_reserve().get(),
            );

            // rewards are not minted, but rather deposited by the owner
            // so we don't neeed to actually mint them

            if to_mint == 0u64 {
                // at this point, all rewards have been generated
                // so we should stop calculating them
                self.produce_rewards_enabled().set(&false);
            }

            self.last_reward_block_nonce().set(&current_block_nonce);
            to_mint
        } else {
            BigUint::zero()
        }
    }

    fn generate_aggregated_rewards(&self) {
        let total_reward = self.calculate_and_increase_per_block_rewards();
        if total_reward > 0 {
            self.increase_reward_reserve(&total_reward);
            self.update_reward_per_share(&total_reward);
        }
    }

    #[endpoint]
    fn end_produce_rewards(&self) -> SCResult<()> {
        self.require_permissions()?;
        self.generate_aggregated_rewards();
        self.produce_rewards_enabled().set(&false);
        Ok(())
    }

    #[endpoint(setPerBlockRewardAmount)]
    fn set_per_block_rewards(&self, per_block_amount: BigUint) -> SCResult<()> {
        self.require_permissions()?;
        require!(per_block_amount != 0, "Amount cannot be zero");
        self.generate_aggregated_rewards();
        self.per_block_reward_amount().set(&per_block_amount);
        Ok(())
    }
}
