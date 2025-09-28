use constants::MAX_PERCENTAGE;
use errors::{ERROR_PERCENTAGE_ABOVE_MAX, ERROR_PERCENTAGE_IS_ZERO, ERROR_ZERO_AMOUNT};
use multiversx_sc::imports::*;
use structs::Earner;

/// Smart Contract module that offers fee distribution to earners.
///
/// It provides:
/// * an endpoint to distribute the fees to the earners based on their percentage
/// * an endpoint to set earners and their percentages
/// * an endpoint to remove earners
/// * a view to get the list of earners and their percentages
/// * internal logic to ensure the total percentage of all earners does not exceed 100%
#[multiversx_sc::module]
pub trait EarnersModule: crate::admins::AdminsModule + crate::pause::PauseModule {
    // === Endpoints ===

    #[payable("*")]
    #[endpoint(distribute)]
    fn distribute(&self) {
        let (token, amount) = self.call_value().egld_or_single_fungible_esdt();
        require!(amount > 0, ERROR_ZERO_AMOUNT);

        let mut distributed_amount = BigUint::zero();
        for earner in self.earners().iter() {
            let earner_percentage = self.earner_percentage(&earner).get();
            let earner_amount = &amount * earner_percentage / MAX_PERCENTAGE;

            if earner_amount > 0 {
                self.send().direct(&earner, &token, 0, &earner_amount);
                distributed_amount += &earner_amount;

                self.distribute_fee_event(&earner, &token, &earner_amount);
            }
        }

        let remainder = &amount - &distributed_amount;
        if remainder > 0 {
            self.send().direct(
                &self.blockchain().get_owner_address(),
                &token,
                0,
                &remainder,
            );
        }
    }

    #[endpoint(setEarners)]
    fn set_earner(&self, earners: MultiValueEncoded<Earner<Self::Api>>) {
        self.require_is_admin(&self.blockchain().get_caller());

        let mut total_to_add = 0;
        let mut total_to_remove = 0;

        for earner in earners {
            let (address, percentage) = earner.into_tuple();

            require!(percentage > 0, ERROR_PERCENTAGE_IS_ZERO);
            require!(percentage <= MAX_PERCENTAGE, ERROR_PERCENTAGE_ABOVE_MAX);

            total_to_add += percentage;
            total_to_remove += self.earner_percentage(&address).get();

            self.set_earner_event(&address, percentage);

            self.earner_percentage(&address).set(percentage);
            self.earners().insert(address);
        }

        require!(
            self.earners_total_percentage() + total_to_add - total_to_remove <= MAX_PERCENTAGE,
            ERROR_PERCENTAGE_ABOVE_MAX
        );
    }

    #[endpoint(removeEarners)]
    fn remove_earner(&self, addresses: MultiValueEncoded<ManagedAddress>) {
        self.require_is_admin(&self.blockchain().get_caller());

        self.remove_earners_event(&addresses);

        for address in addresses {
            self.earner_percentage(&address).clear();
            self.earners().swap_remove(&address);
        }
    }

    // === Views ===

    #[view(getEarners)]
    fn get_earners_info(&self) -> MultiValueEncoded<Earner<Self::Api>> {
        let mut earners: MultiValueEncoded<Earner<Self::Api>> = MultiValueEncoded::new();

        for earner in self.earners().iter() {
            let percntage = self.earner_percentage(&earner).get();

            earners.push(Earner::from((earner, percntage)));
        }

        return earners;
    }

    // === Private ===

    fn earners_total_percentage(&self) -> u64 {
        let mut total_percentage: u64 = 0;

        for earner in self.earners().iter() {
            total_percentage += self.earner_percentage(&earner).get();
        }

        total_percentage
    }

    // === Storage ===

    #[storage_mapper("earners")]
    fn earners(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[storage_mapper("earner_percentage")]
    fn earner_percentage(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;

    // === Events ===

    #[event("feeDistributed")]
    fn distribute_fee_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] token: &EgldOrEsdtTokenIdentifier,
        #[indexed] amount: &BigUint,
    );

    #[event("earnerSet")]
    fn set_earner_event(&self, #[indexed] address: &ManagedAddress, #[indexed] percentage: u64);

    #[event("earnersRemoved")]
    fn remove_earners_event(&self, #[indexed] addresses: &MultiValueEncoded<ManagedAddress>);
}
