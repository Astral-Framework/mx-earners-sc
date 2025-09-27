#![no_std]

use multiversx_sc::imports::*;

mod admins;
mod earners;
mod pause;

/// A simple Smart Contract that distributes a payment (fee) to multiple addresses.
/// - The payment can be in EGLD or in any single ESDT token.
/// - The payment is distributed to multiple addresses (called Earners) based on their percentage.
/// - The total percentage must be exactly 100%.
/// - Any leftover amount is sent back to the owner.
#[multiversx_sc::contract]
pub trait Earners: admins::AdminsModule + pause::PauseModule + earners::EarnersModule {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
