use super::currency::Currency;
#[cfg(feature = "sea-orm")]
use sea_orm::prelude::*;
// a bill, or invoice is a planned, one-time charge
// for all unplanned activity, refer to transactions

pub type Id = i64;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "sea-orm", derive(DeriveEntityModel))]
#[cfg_attr(feature = "sea-orm", sea_orm(table_name = "bills"))]
pub struct Model {
    #[cfg_attr(feature = "sea-orm", sea_orm(primary_key, auto_increment = true))]
    id: Id,
    subscription_id: Option<super::subscription::Id>,
    user_id: Option<super::user::Id>,
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
            belongs_to = "super::user::Entity",
            from = "Column::UserId",
            to = "super::user::Column::Id"
        )
    )]
    User,
}
#[cfg(feature = "sea-orm")]
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
#[cfg(feature = "sea-orm")]
impl ActiveModelBehavior for ActiveModel {}
