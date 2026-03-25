use chrono::{DateTime, Duration, Utc};
use models::entity::user::AuthUser;
use sea_orm::{DatabaseBackend::Sqlite, MockDatabase, ModelTrait};
use std::path::PathBuf;
use std::str::FromStr;

pub fn is_close_to_now(time: DateTime<Utc>) -> bool {
	let now = Utc::now();
	let duration = time.signed_duration_since(now);

	duration.abs() < Duration::minutes(1)
}

pub fn get_mock_db_for_model<ModelType: ModelTrait>(
	models: Vec<ModelType>,
) -> MockDatabase {
	MockDatabase::new(Sqlite).append_query_results::<ModelType, _, _>(vec![models])
}

pub fn default_user_id() -> uuid::Uuid {
	uuid::Uuid::from_str("6d53ddf7-f0d4-4918-b559-9d0a950f2d43").unwrap()
}

pub fn default_user_456_id() -> uuid::Uuid {
	uuid::Uuid::from_str("727e430c-ffcb-4761-907d-f8c8bc636550").unwrap()
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

pub fn get_test_epub_path() -> String {
	PathBuf::from(env!("CARGO_MANIFEST_DIR"))
		.join("../../core/integration-tests/data/book.epub")
		.to_string_lossy()
		.to_string()
}
