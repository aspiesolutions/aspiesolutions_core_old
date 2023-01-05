use sea_orm::{prelude::*,QueryOrder};
use crate::forms::UserSearchForm;
use crate::{Paged,SearchUserResult};
use crate::entity::user;
pub async fn search_users(database: &DatabaseConnection,query: &UserSearchForm) -> SearchUserResult {
    let mut stmt = user::Entity::find();
    if let Some(query_user) = query.user() {
        if let Some(query_user_name_contains) = query_user.name().contains() {
            stmt = stmt.filter(user::Column::Name.contains(query_user_name_contains))
        } 
    }
    if query.user().is_some() {
    }
    let limit = query
        .limit()
        .map(|limit| *limit as u64)
        .unwrap_or_else(|| u16::MAX as u64);
    stmt = stmt.order_by_asc(user::Column::Id);
    if let Some(user) = query.user() {
        if let Some(user_name_equals) = user.name().equals() {
            stmt = stmt.filter(user::Column::Name.eq(user_name_equals))
        }
    }
    let paginator: sea_orm::Paginator<'_,_, _> = stmt.paginate(database, limit);
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