use sea_orm::{
    prelude::*, ActiveModelTrait, ActiveValue, ConnectionTrait, QueryOrder, TransactionError,
    TransactionTrait,
};
// use crate::error::Error;
use crate::entity;
use crate::forms::{CreateUserForm, UserSearchForm};
use crate::{Paged, SearchUserResult};
use chrono::Utc;
pub async fn search_users(
    database: &DatabaseConnection,
    query: &UserSearchForm,
) -> SearchUserResult {
    let mut stmt = entity::user::Entity::find();
    if let Some(query_user) = query.user() {
        if let Some(query_user_name_contains) = query_user.name().contains() {
            stmt = stmt.filter(entity::user::Column::Name.contains(query_user_name_contains))
        }
    }
    if query.user().is_some() {}
    let limit = query
        .limit()
        .map(|limit| *limit as u64)
        .unwrap_or_else(|| u16::MAX as u64);
    stmt = stmt.order_by_asc(entity::user::Column::Id);
    if let Some(user) = query.user() {
        if let Some(user_name_equals) = user.name().equals() {
            stmt = stmt.filter(entity::user::Column::Name.eq(user_name_equals))
        }
    }
    let paginator: sea_orm::Paginator<'_, _, _> = stmt.paginate(database, limit);
    let query_page = query.page().map(|page| *page as u64).unwrap_or(1_u64);
    let data = match paginator.fetch_page(query_page).await {
        Ok(page) => page,
        Err(e) => return Err(e.into()),
    };
    let num_pages_and_items = match paginator.num_items_and_pages().await {
        Ok(num_pages_and_items) => num_pages_and_items,
        Err(e) => return Err(e.into()),
    };
    // paginator.cur_page isnt useful unless you hold it in memory.
    // the returned value is only modified when you call next_page()
    let cur_page = query_page;
    let paged: Paged<_> = Paged::new(
        num_pages_and_items.number_of_items,
        num_pages_and_items.number_of_pages,
        cur_page,
        data,
    );
    Ok(paged)
}

pub async fn create_user(
    database: &(impl ConnectionTrait + TransactionTrait),
    form: CreateUserForm,
) -> Result<entity::user::Model, DbErr> {
    let mut active_model: entity::user::ActiveModel =
        <entity::user::ActiveModel as ActiveModelTrait>::default();
    active_model.name = ActiveValue::Set(form.data().name().to_string());
    entity::user::Entity::insert(active_model)
        .exec_with_returning(database)
        .await
}
pub async fn create_password_for_user(
    database: &(impl ConnectionTrait + TransactionTrait),
    user_id: entity::user::Id,
    hashed_password: String,
) -> Result<entity::user_passwords::Model, DbErr> {
    let mut active_model: entity::user_passwords::ActiveModel =
        <entity::user_passwords::ActiveModel as ActiveModelTrait>::default();
    active_model.date_time_created = ActiveValue::Set(Utc::now());
    active_model.hash = ActiveValue::Set(hashed_password);
    active_model.user_id = ActiveValue::Set(user_id);
    entity::user_passwords::Entity::insert(active_model)
        .exec_with_returning(database)
        .await
}

pub async fn create_user_and_password(
    database: &DatabaseConnection,
    form: CreateUserForm,
    hashed_password: String,
) -> Result<(entity::user::Model, entity::user_passwords::Model), TransactionError<DbErr>> {
    database
        .transaction::<_, (entity::user::Model, entity::user_passwords::Model), sea_orm::DbErr>(
            |txn| {
                Box::pin(async move {
                    // hello
                    let user: entity::user::Model = create_user(txn, form.clone()).await?;
                    let user_password: entity::user_passwords::Model =
                        create_password_for_user(txn, user.id(), hashed_password).await?;
                    Ok((user, user_password))
                })
            },
        )
        .await
}
