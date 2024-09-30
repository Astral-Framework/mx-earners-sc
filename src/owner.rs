multiversx_sc::imports!();
multiversx_sc::derive_imports!();

type EarnerType<M> = MultiValue3<ManagedAddress<M>, ManagedBuffer<M>, u64>;

pub const MAX_PERCENTAGE: u64 = 10000; // 100%

#[multiversx_sc::module]
pub trait OwnerModule {
    // ========================= Endpoints =========================

    /// Sets a new earner with a name and a percentage.
    #[only_owner]
    #[endpoint(setEarner)]
    fn set_earner(&self, address: ManagedAddress, name: ManagedBuffer, percentage: u64) {
        require!(
            percentage < MAX_PERCENTAGE,
            "Wrong percentage. Must be below 100% (10000).",
        );

        let old_percentage = self.earner_percentage(&address).get();
        let new_sum = self.earners_total_percentage().get() + percentage - old_percentage;

        require!(
            new_sum <= MAX_PERCENTAGE,
            "Wrong percentage. New total would exceed max limit"
        );

        // Storage
        self.earners_total_percentage().set(new_sum);
        self.earner_name(&address).set(name.clone());
        self.earner_percentage(&address).set(percentage);
        self.earners().insert(address.clone());

        self.set_earner_event(&address, &name, percentage);
    }

    /// Removes an earner using the address.
    #[only_owner]
    #[endpoint(removeEarner)]
    fn remove_earner(&self, address: ManagedAddress) {
        self.earners_total_percentage()
            .update(|s| *s -= self.earner_percentage(&address).get());

        // Storage
        self.earners().swap_remove(&address);
        self.earner_name(&address).clear();
        self.earner_percentage(&address).clear();

        self.remove_earner_event(&address);
    }

    // ========================= Views =========================

    /// Returns the list of earners with their info (address, name, percentage).
    #[view(getEarnersInfo)]
    fn get_earners_info(&self) -> MultiValueEncoded<EarnerType<Self::Api>> {
        let mut earners: MultiValueEncoded<EarnerType<Self::Api>> = MultiValueEncoded::new();

        for earner in self.earners().iter() {
            earners.push(EarnerType::from((
                earner.clone(),
                self.earner_name(&earner).get(),
                self.earner_percentage(&earner).get(),
            )));
        }
        return earners;
    }

    // ========================= Storage =========================

    #[view(earners)]
    #[storage_mapper("earners")]
    fn earners(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(earnerName)]
    #[storage_mapper("earner_name")]
    fn earner_name(&self, address: &ManagedAddress) -> SingleValueMapper<ManagedBuffer>;

    #[view(earnerPercentage)]
    #[storage_mapper("earner_percentage")]
    fn earner_percentage(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(earnersTotalPercentage)]
    #[storage_mapper("earners_total_percentage")]
    fn earners_total_percentage(&self) -> SingleValueMapper<u64>;

    // ========================= Events =========================

    #[event("set_earner")]
    fn set_earner_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] name: &ManagedBuffer,
        #[indexed] percentage: u64,
    );

    #[event("remove_earner")]
    fn remove_earner_event(&self, #[indexed] address: &ManagedAddress);
}
