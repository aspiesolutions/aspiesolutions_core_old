// use chrono::{Datelike, Duration, Timelike};
#[cfg(feature = "sea-orm")]
use sea_orm::prelude::*;
// a subscription could be thought of as a  bill that ocurrs more than once on a regular basis

// possible strategies are
// every N hours starting at time
// every N days at time
// every N months on Day of Month
// every N weeks on Day of week
// every N years on Month/day

// a subscription is a planned, recurring charge billed on a regular schedule

// this table holds the name, user, and other non_repeating data

pub type Id = i32;

use super::currency::Currency;
#[derive(Debug, PartialEq)]
#[cfg_attr(feature="clone", derive(Clone))]
#[cfg_attr(feature="serde", derive(serde::Serialize,serde::Deserialize))]
#[cfg_attr(feature = "sea-orm", derive(DeriveEntityModel))]
#[cfg_attr(feature = "sea-orm", sea_orm(table_name = "subscription_entry"))]
pub struct Model {
    #[cfg_attr(feature = "sea-orm", sea_orm(primary_key, auto_increment = true))]
    id: Id,
    subscription_id: super::subscription::Id,
    date: chrono::DateTime<chrono::Utc>,
    frequency: Frequency,
    frequency_value: i64,
    amount: rust_decimal::Decimal,
    currency: Currency,
    transaction_id: Option<super::transaction::Id>,
    cancelled: bool,
}

#[repr(i8)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature="clone", derive(Clone))]
#[cfg_attr(feature="serde", derive(serde::Serialize,serde::Deserialize))]
#[cfg_attr(feature = "sea-orm", derive(DeriveActiveEnum, EnumIter))]
#[cfg_attr(feature = "sea-orm", sea_orm(rs_type = "i8", db_type = "TinyInteger"))]
pub enum Frequency {
    Hourly = -128,
    Daily = -127,
    Monthly = -126,
    Weekly = -125,
    Yearly = -124,
}
#[derive(Copy, Debug)]
#[cfg_attr(feature="clone", derive(Clone))]
#[cfg_attr(feature = "sea-orm", derive(DeriveRelation, EnumIter))]
pub enum Relation {
    #[cfg_attr(feature = "sea-orm", sea_orm(has_many = "super::transaction::Entity"))]
    Transaction,
    #[cfg_attr(
        feature = "sea-orm",
        sea_orm(
            belongs_to = "super::subscription::Entity",
            from = "Column::SubscriptionId",
            to = "super::subscription::Column::Id"
        )
    )]
    Subscription,
}
#[cfg(feature = "sea-orm")]
impl Related<super::transaction::Entity> for Entity {
    fn to() -> RelationDef {
        super::subscription_entry_transactions::Relation::Entry.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::subscription_entry_transactions::Relation::Transaction
                .def()
                .rev(),
        )
    }
}
#[cfg(feature = "sea-orm")]
impl Related<super::subscription::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Subscription.def()
    }
}
#[cfg(feature = "sea-orm")]
impl ActiveModelBehavior for ActiveModel {}
