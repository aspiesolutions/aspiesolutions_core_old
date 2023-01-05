// use chrono::{Datelike, Duration, Timelike};
#[cfg(feature = "sea-orm")]
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

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

use crate::currency::Currency;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sea-orm", derive(DeriveEntityModel))]
#[cfg_attr(feature = "sea-orm", sea_orm(table_name = "subscription_entry"))]
pub struct Model {
    #[cfg_attr(feature = "sea-orm", sea_orm(primary_key, auto_increment = true))]
    id: Id,
    subscription_id: crate::subscription::Id,
    date: chrono::DateTime<chrono::Utc>,
    frequency: Frequency,
    frequency_value: i64,
    amount: rust_decimal::Decimal,
    currency: Currency,
    transaction_id: Option<crate::transaction::Id>,
    cancelled: bool,
}

#[repr(i8)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sea-orm", derive(DeriveActiveEnum, EnumIter))]
#[cfg_attr(feature = "sea-orm", sea_orm(rs_type = "i8", db_type = "TinyInteger"))]
pub enum Frequency {
    Hourly = -128,
    Daily = -127,
    Monthly = -126,
    Weekly = -125,
    Yearly = -124,
}
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "sea-orm", derive(DeriveRelation, EnumIter))]
pub enum Relation {
    #[cfg_attr(feature = "sea-orm", sea_orm(has_many = "crate::transaction::Entity"))]
    Transaction,
    #[cfg_attr(
        feature = "sea-orm",
        sea_orm(
            belongs_to = "crate::subscription::Entity",
            from = "Column::SubscriptionId",
            to = "crate::subscription::Column::Id"
        )
    )]
    Subscription,
}
#[cfg(feature = "sea-orm")]
impl Related<crate::transaction::Entity> for Entity {
    fn to() -> RelationDef {
        crate::subscription_entry_transactions::Relation::Entry.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            crate::subscription_entry_transactions::Relation::Transaction
                .def()
                .rev(),
        )
    }
}
#[cfg(feature = "sea-orm")]
impl Related<crate::subscription::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Subscription.def()
    }
}
#[cfg(feature = "sea-orm")]
impl ActiveModelBehavior for ActiveModel {}
