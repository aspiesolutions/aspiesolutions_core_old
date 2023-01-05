#[cfg(feature = "sea-orm")]
use sea_orm::prelude::*;

pub type Repr = i8;

#[repr(i8)]
#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "sea-orm", derive(DeriveActiveEnum, EnumIter))]
#[cfg_attr(feature = "sea-orm", sea_orm(rs_type = "i8", db_type = "SmallInteger"))]
pub enum Currency {
    USD = -128,
}
impl Currency {
    pub fn to_symbol(&self) -> &str {
        match self {
            Self::USD => "$",
        }
    }
    pub fn to_iso_currency_code(&self) -> &str {
        match self {
            Self::USD => "USD",
        }
    }
}
