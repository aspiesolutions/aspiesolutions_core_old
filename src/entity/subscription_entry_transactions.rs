#[cfg(feature = "sea-orm")]
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

// mapping table linking subscription_entries to many transactions
pub type Id = i64;
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "sea-orm", derive(DeriveEntityModel))]
#[cfg_attr(
    feature = "sea-orm",
    sea_orm(table_name = "subscription_entry_transactions")
)]
pub struct Model {
    #[cfg_attr(feature = "sea-orm", sea_orm(primary_key, auto_increment = true))]
    id: Id,
    entry_id: crate::subscription_entries::Id,
    transaction_id: crate::transaction::Id,
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "sea-orm", derive(DeriveRelation, EnumIter))]

pub enum Relation {
    #[cfg_attr(
        feature = "sea-orm",
        sea_orm(
            belongs_to = "crate::subscription_entries::Entity",
            from = "Column::EntryId",
            to = "crate::subscription_entries::Column::Id"
        )
    )]
    Entry,
    #[cfg_attr(
        feature = "sea-orm",
        sea_orm(
            belongs_to = "crate::transaction::Entity",
            from = "Column::TransactionId",
            to = "crate::transaction::Column::Id"
        )
    )]
    Transaction,
}
#[cfg(feature = "sea-orm")]
impl ActiveModelBehavior for ActiveModel {}
