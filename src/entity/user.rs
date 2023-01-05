#[cfg(feature = "sea-orm")]
use sea_orm::{prelude::*, ConnectionTrait, DatabaseTransaction, FromQueryResult};
use serde::{Deserialize, Serialize};
pub type Id = i64;
pub type Role = u8;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sea-orm", derive(DeriveEntityModel))]
#[cfg_attr(feature = "sea-orm", sea_orm(table_name = "users"))]
pub struct Model {
    #[cfg_attr(feature = "sea-orm", sea_orm(primary_key, auto_increment = true))]
    id: Id,
    name: String,
}
#[repr(transparent)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(feature = "sea-orm", derive(FromQueryResult))]
#[serde(transparent)]
pub struct IdOnly {
    inner: Id,
}
impl IdOnly {
    pub fn into_inner(&self) -> Id {
        self.inner
    }
}

impl std::convert::From<Id> for IdOnly {
    fn from(i: Id) -> Self {
        Self { inner: i }
    }
}

impl std::default::Default for Model {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "sea-orm", derive(DeriveRelation, EnumIter))]
pub enum Relation {
    #[cfg_attr(feature = "sea-orm", sea_orm(has_many = "crate::bank_account::Entity"))]
    BankAccounts,
    #[cfg_attr(feature = "sea-orm", sea_orm(has_many = "crate::bill::Entity"))]
    Bills,
    #[cfg_attr(feature = "sea-orm", sea_orm(has_many = "crate::subscription::Entity"))]
    Subscriptions,
    #[cfg_attr(feature = "sea-orm", sea_orm(has_many = "crate::transaction::Entity"))]
    Transactions,
    #[cfg_attr(feature = "sea-orm", sea_orm(has_many = "crate::user_passwords::Entity"))]
    Passwords
}
#[cfg(feature = "sea-orm")]
impl Related<crate::bank_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BankAccounts.def()
    }
}

impl Model {
    pub fn id(&self) -> Id {
        self.id
    }
    pub fn name<'a>(&'a self) -> &'a str {
        self.name.as_str()
    }
}
#[cfg(feature = "sea-orm")]
impl ActiveModelBehavior for ActiveModel {}

#[cfg(feature = "sea-orm")]
pub async fn delete_with_related(id: Id, txn: &DatabaseTransaction) -> Result<Id, sea_orm::DbErr> {
    // use sea_orm::QueryTrait;

    crate::transaction::Entity::delete_many()
        .filter(crate::transaction::Column::UserId.eq(id))
        .exec(txn)
        .await?;
    crate::subscription_entries::Entity::delete_many()
        // BUG: THIS WILL NOT WORK AS EXPECTED
        .filter(crate::subscription_entries::Column::Id.eq(id))
        .exec(txn)
        .await?;
    crate::subscription::Entity::delete_many()
        .filter(crate::subscription::Column::UserId.eq(id))
        .exec(txn)
        .await?;
    crate::bill::Entity::delete_many()
        .filter(crate::bill::Column::UserId.eq(id))
        .exec(txn)
        .await?;
    crate::user_passwords::Entity::delete_many()
    .filter(crate::user_passwords::Column::UserId.eq(id))
    .exec(txn).await?;

    Entity::delete_by_id(id).exec(txn).await?;
    Ok(id)
}

#[cfg(feature = "sea-orm")]
pub async fn try_delete(id: Id, conn: &impl ConnectionTrait) -> Result<Id, sea_orm::DbErr> {
    Entity::delete_by_id(id).exec(conn).await?;
    Ok(id)
}
