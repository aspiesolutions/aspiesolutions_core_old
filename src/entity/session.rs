#[cfg(feature = "sea-orm")]
use sea_orm::prelude::*;
// represents all data needed to represent a logged in user/etc
pub type Id = i32;
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "sea-orm", derive(DeriveEntityModel))]
#[cfg_attr(feature = "sea-orm", sea_orm(table_name = "sessions"))]
#[derive(Debug, PartialEq)]
pub struct Model {
    #[cfg_attr(feature = "sea-orm", sea_orm(primary_key, auto_increment = true))]
    id: Id,
    #[cfg_attr(feature = "sea-orm", sea_orm(unique = true))]
    uuid: uuid::Uuid,
    user_id: super::user::Id,
}
#[derive(Debug)]
#[cfg_attr(feature = "clone", derive(Clone))]
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
