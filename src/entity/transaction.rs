#[cfg(feature = "sea-orm")]
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

use crate::currency::Currency;
// a transaction represents what happens in a users account
// transactions represent the flow of money to/from in/out of the system

// a deposit is represented as an addition of money into the system tied to an account number

// a withdrawal is represented as a subtraction of money out of the system tied to an account number

// a transfer of money (or payment to an external / internal account) is represented as a withdrawal of money
// from one account, followed by a deposit of money into another account

// all transactions are required to be linked to an account number

// it is common for banks to hold a transaction for at least two days before the final status is known

// db_type = "Numberic"
pub type Id = i64;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "sea-orm", derive(DeriveEntityModel))]
#[cfg_attr(feature = "sea-orm", sea_orm(table_name = "transactions"))]
pub struct Model {
    #[cfg_attr(feature = "sea-orm", sea_orm(primary_key, auto_increment = true))]
    id: Id,
    /// what text is associated with the transaction
    text: String,
    amount: rust_decimal::Decimal,
    currency: Currency,
    bank_account: crate::bank_account::Id,
    // created and finalized account for the period of time that banks "hold" the transcation
    user_id: crate::user::Id,
    created: chrono::DateTime<chrono::Utc>,
    finalized: Option<chrono::DateTime<chrono::Utc>>,
    // TODO: what method was used to pay for the transaction
    vendor: Vendor,
    is_buisness: bool,
    bill: Option<crate::bill::Id>,
    subscription_entry: Option<crate::subscription::Id>,
    reciept: Option<String>,
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "sea-orm", derive(EnumIter, DeriveRelation))]
pub enum Relation {
    #[cfg_attr(
        feature = "sea-orm",
        sea_orm(
            belongs_to = "crate::bank_account::Entity",
            from = "Column::BankAccount",
            to = "crate::bank_account::Column::Id"
        )
    )]
    BankAccount,
    #[cfg_attr(
        feature = "sea-orm",
        sea_orm(
            belongs_to = "crate::bill::Entity",
            from = "Column::Bill",
            to = "crate::bill::Column::Id"
        )
    )]
    Bills,
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

impl Model {
    pub fn id(&self) -> Id {
        self.id
    }
    pub fn text<'a>(&'a self) -> &'a str {
        self.text.as_str()
    }
    pub fn amount<'a>(&'a self) -> &'a rust_decimal::Decimal {
        &self.amount
    }
    pub fn currency(&self) -> Currency {
        self.currency.clone()
    }
    pub fn bank_account(&self) -> crate::bank_account::Id {
        self.bank_account
    }
    pub fn user_id(&self) -> crate::user::Id {
        self.user_id
    }
    pub fn created<'a>(&'a self) -> &'a chrono::DateTime<chrono::Utc> {
        &self.created
    }
    pub fn finalized<'a>(&'a self) -> Option<&'a chrono::DateTime<chrono::Utc>> {
        self.finalized.as_ref()
    }
    pub fn vendor(&self) -> Vendor {
        self.vendor.clone()
    }
    pub fn is_buisness(&self) -> bool {
        self.is_buisness
    }
    pub fn bill(&self) -> Option<crate::bill::Id> {
        self.bill.clone()
    }
}
#[cfg(feature = "sea-orm")]
impl Related<crate::subscription_entries::Entity> for Entity {
    fn to() -> RelationDef {
        crate::subscription_entry_transactions::Relation::Transaction.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            crate::subscription_entry_transactions::Relation::Entry
                .def()
                .rev(),
        )
    }
}
#[repr(i16)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sea-orm", derive(DeriveActiveEnum, EnumIter))]
#[cfg_attr(
    feature = "sea-orm",
    sea_orm(rs_type = "i16", db_type = "SmallInteger")
)]
pub enum Vendor {
    DoorDash = -32_768,
    Uber = -32_767,
    Lyft = -32_766,
    Roadie = -32_765,
    Ford = -32_764,
    FordMotorCredit = -32_763,
    // LAST
    Other = 32_767,
}
impl Vendor {
    pub fn to_name(&self) -> &str {
        match self {
            &Self::DoorDash => "DoorDash",
            &Self::Uber => "Uber",
            &Self::Lyft => "Lyft",
            &Self::Roadie => "Roadie",
            &Self::Ford => "Ford",
            &Self::FordMotorCredit => "Ford Motor Credit",
            &Self::Other=> "Other"
        }
    }
}

#[cfg(feature = "sea-orm")]
impl Related<crate::bank_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BankAccount.def()
    }
}
#[cfg(feature = "sea-orm")]
impl Related<crate::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
#[cfg(feature = "sea-orm")]
impl ActiveModelBehavior for ActiveModel {}
