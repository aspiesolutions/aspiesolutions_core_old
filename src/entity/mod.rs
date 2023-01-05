#![deny(warnings)]
pub mod bill;
pub mod currency;
pub mod subscription;
#[cfg(feature = "sea-orm")]
pub use sea_orm;
pub mod bank_account;
pub mod query_engine;
pub mod subscription_entries;
pub mod subscription_entry_transactions;
pub mod transaction;
pub mod user;
pub mod user_passwords;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The provided start date is greater than or equal to the end date")]
    StartDateAfterEndDate,
    #[error("The provided end date is less than or equal to the start date ")]
    EndDateBeforeStartDate,
}

// i want a way to encode an advanced query system based on sea orm
