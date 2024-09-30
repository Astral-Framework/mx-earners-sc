use crate::owner::MAX_PERCENTAGE;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait PublicModule: crate::owner::OwnerModule {
    /// Sends the fees to the earners based on their percentage.
    #[payable("*")]
    #[endpoint(sendFees)]
    fn send_fees(&self) {
        let (token, nonce, amount) = self.call_value().egld_or_single_esdt().into_tuple();

        let total_percentage = BigUint::from(self.earners_total_percentage().get());
        require!(
            total_percentage == MAX_PERCENTAGE,
            "Total percentage must be 100%."
        );

        for earner in self.earners().iter() {
            let earner_amount =
                &amount * &BigUint::from(self.earner_percentage(&earner).get()) / &total_percentage;

            if earner_amount > 0 {
                self.send().direct(&earner, &token, nonce, &earner_amount);
            }
        }

        let left_balance = self.blockchain().get_sc_balance(&token, nonce);
        if left_balance > 0 {
            self.send().direct(
                &self.blockchain().get_owner_address(),
                &token,
                nonce,
                &left_balance,
            );
        }
    }
}
