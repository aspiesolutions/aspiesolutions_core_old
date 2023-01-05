#[cfg(feature = "sea-orm")]
use sea_orm::prelude::*;

// mapping table linking subscription_entries to many transactions
pub type Id = i64;
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "sea-orm", derive(DeriveEntityModel))]
#[cfg_attr(
    feature = "sea-orm",
    sea_orm(table_name = "subscription_entry_transactions")
)]
pub struct Model {
    #[cfg_attr(feature = "sea-orm", sea_orm(primary_key, auto_increment = true))]
    id: Id,
    entry_id: super::subscription_entries::Id,
    transaction_id: super::transaction::Id,
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "sea-orm", derive(DeriveRelation, EnumIter))]

pub enum Relation {
    #[cfg_attr(
        feature = "sea-orm",
        sea_orm(
            belongs_to = "super::subscription_entries::Entity",
            from = "Column::EntryId",
            to = "super::subscription_entries::Column::Id"
        )
    )]
    Entry,
    #[cfg_attr(
        feature = "sea-orm",
        sea_orm(
            belongs_to = "super::transaction::Entity",
            from = "Column::TransactionId",
            to = "super::transaction::Column::Id"
        )
    )]
    Transaction,
}
#[cfg(feature = "sea-orm")]
impl ActiveModelBehavior for ActiveModel {}
