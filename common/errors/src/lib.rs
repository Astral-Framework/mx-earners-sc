#![no_std]

pub static ERROR_PAUSED: &[u8] = b"Paused";
pub static ERROR_NOT_PAUSED: &[u8] = b"Not paused";

pub static ERROR_NOT_ADMIN: &[u8] = b"Only admin allowed";

pub static ERROR_PERCENTAGE_IS_ZERO: &[u8] = b"Percentage must be greater than 0";
pub static ERROR_PERCENTAGE_ABOVE_MAX: &[u8] = b"Percentage is above maximum";

pub static ERROR_ZERO_AMOUNT: &[u8] = b"Amount must be greater than zero";