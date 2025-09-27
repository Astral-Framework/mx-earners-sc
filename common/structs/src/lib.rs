#![no_std]

use multiversx_sc::imports::*;

pub type Address<M> = ManagedAddress<M>;
pub type Percentage = u64;
pub type Earner<M> = MultiValue2<Address<M>, Percentage>;
