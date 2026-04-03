use sea_orm::{
	sea_query::{Condition, Query, SqliteQueryBuilder},
	EntityTrait, QuerySelect, QueryTrait,
};
use uuid::Uuid;

use crate::entity::user::AuthUser;

pub fn condition_to_string(condition: &Condition) -> String {
	Query::select()
		.cond_where(condition.clone())
		.to_string(SqliteQueryBuilder)
}

pub fn select_no_cols_to_string<EntityType: EntityTrait>(
	select: sea_orm::Select<EntityType>,
) -> String {
	select
		.select_only()
		.into_query()
		.to_string(SqliteQueryBuilder)
}

pub fn default_user_id() -> Uuid {
	Uuid::parse_str("0ad39398-ce6a-4bcc-b044-719163a07c53").unwrap()
}

pub fn get_default_user() -> AuthUser {
	AuthUser {
		id: default_user_id(),
		username: "test".to_string(),
		avatar_path: None,
		avatar_url: None,
		is_server_owner: true,
		is_locked: false,
		permissions: vec![],
		age_restriction: None,
		preferences: None,
	}
}
