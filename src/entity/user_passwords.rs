#[cfg(feature = "sea-orm")]
use sea_orm::prelude::*;
pub type Id = i32;
pub type UserId = crate::user::Id;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sea-orm", derive(DeriveEntityModel))]
#[cfg_attr(feature = "sea-orm", sea_orm(table_name = "user_passwords"))]
pub struct Model {
    #[cfg_attr(feature = "sea-orm", sea_orm(primary_key,auto_increment=true))]
    id:Id,
    user_id:UserId,
    date_time_created: ChronoDateTimeUtc,
    hash:String

}

impl Model {
    pub fn id(&self) -> Id {
        self.id
    }
    pub fn user_id(&self) -> UserId {
        self.user_id
    }
    pub fn date_time_created(&self) -> ChronoDateTimeUtc {
        self.date_time_created
    }
    pub fn hash(&self) -> &str {
        &self.hash
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "sea-orm", derive(DeriveRelation, EnumIter))]
pub enum Relation {
    #[cfg_attr(feature = "sea-orm", sea_orm(belongs_to = "crate::user::Entity",from="Column::UserId",to="crate::user::Column::Id"))]
    User,
}


impl Related<crate::user::Entity> for self::Entity {
    fn to() -> RelationDef {
        self::Relation::User.def()
    }
}


impl ActiveModelBehavior for ActiveModel {}