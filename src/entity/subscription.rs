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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sea-orm", derive(DeriveEntityModel))]
#[cfg_attr(feature = "sea-orm", sea_orm(table_name = "subscriptions"))]
pub struct Model {
    #[cfg_attr(feature = "sea-orm", sea_orm(primary_key, auto_increment = true))]
    id: Id,
    date: chrono::DateTime<chrono::Utc>,
    user_id: crate::user::Id,
}
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "sea-orm", derive(DeriveRelation, EnumIter))]

pub enum Relation {
    #[cfg_attr(
        feature = "sea-orm",
        sea_orm(has_many = "crate::subscription_entries::Entity")
    )]
    Entry,
    #[cfg_attr(
        feature = "sea-orm",
        sea_orm(
            belongs_to = "crate::user::Entity",
            from = "Column::UserId",
            to = "crate::user::Column::Id"
        )
    )]
    User,
}

// impl Related<crate::subscription::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Entry.def()
//     }
// }
#[cfg(feature = "sea-orm")]
impl Related<crate::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
#[cfg(feature = "sea-orm")]
impl ActiveModelBehavior for ActiveModel {}
