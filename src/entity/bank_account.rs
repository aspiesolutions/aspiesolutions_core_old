#[cfg(feature = "sea-orm")]
use sea_orm::prelude::*;
pub type Id = i32;

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "sea-orm", derive(DeriveEntityModel))]
#[cfg_attr(feature = "sea-orm", sea_orm(table_name = "bank_accounts"))]
pub struct Model {
    #[cfg_attr(feature = "sea-orm", sea_orm(primary_key, auto_increment = true))]
    id: Id,
    user_id: super::user::Id,
    bank_name: String,
    nick_name: Option<String>,
    #[cfg_attr(feature = "sea-orm", sea_orm(unique))]
    account_number: String,
    account_type: AccountType,
}
#[repr(i8)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "sea-orm", derive(EnumIter, DeriveActiveEnum))]
#[cfg_attr(feature = "sea-orm", sea_orm(rs_type = "i8", db_type = "TinyInteger"))]
pub enum AccountType {
    Checking = -128,
    Savings = -127,
}
#[cfg(not(feature = "sea-orm"))]
impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Self::Checking => write!(f, "Checking"),
            &Self::Savings => write!(f, "Savings"),
        }
    }
}

// const ACCOUNT_TYPE_CHECKING: AccountType = AccountType::Checking;
// const ACCOUNT_TYPE_SAVINGS: AccountType = AccountType::Savings;

impl std::str::FromStr for AccountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-128" | "Checking" | "checking" => Ok(AccountType::Checking),
            "-127" | "Savings" | "savings" => Ok(AccountType::Savings),
            _ => Err(()),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(feature = "sea-orm", derive(DeriveRelation, EnumIter))]
pub enum Relation {
    #[cfg_attr(feature = "sea-orm", sea_orm(has_many = "super::transaction::Entity"))]
    Transactions,
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
