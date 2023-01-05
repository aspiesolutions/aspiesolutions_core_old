use crate::currency::Currency;
#[cfg(feature = "sea-orm")]
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

// a bill, or invoice is a planned, one-time charge
// for all unplanned activity, refer to transactions

pub type Id = i64;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sea-orm", derive(DeriveEntityModel))]
#[cfg_attr(feature = "sea-orm", sea_orm(table_name = "bills"))]
pub struct Model {
    #[cfg_attr(feature = "sea-orm", sea_orm(primary_key, auto_increment = true))]
    id: Id,
    subscription_id: Option<crate::subscription::Id>,
    user_id: Option<crate::user::Id>,
    name: String,
    date: chrono::DateTime<chrono::Utc>,
    amount: rust_decimal::Decimal,
    currency: Currency,
}

impl Model {
    pub fn id(&self) -> Id {
        self.id
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "sea-orm", derive(DeriveRelation, EnumIter))]

pub enum Relation {
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
#[cfg(feature = "sea-orm")]
impl Related<crate::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
#[cfg(feature = "sea-orm")]
impl ActiveModelBehavior for ActiveModel {}
