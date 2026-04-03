use crate::sqlite_extra::{
	sqlite_alter_column_type, sqlite_alter_column_type_full_copy,
	sqlite_drop_foreign_keys, sqlite_revert_column_type,
};
use sea_orm::ConnectionTrait;
use sea_orm::DatabaseBackend;
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

const SQLITE_TEXT_UUID_COLUMNS: [(&str, &str, bool, bool); 86] = [
	("users", "id", false, false),
	("libraries", "id", false, false),
	("series", "id", false, false),
	("series", "library_id", false, false),
	("media", "series_id", false, false),
	("media", "id", false, false),
	("jobs", "id", false, false),
	("age_restrictions", "user_id", true, false),
	("api_keys", "user_id", true, false),
	(
		"book_club_book_suggestion_likes",
		"liked_by_id",
		false,
		false,
	),
	(
		"book_club_book_suggestion_likes",
		"suggestion_id",
		false,
		false,
	),
	("book_club_book_suggestions", "id", false, false),
	("book_club_book_suggestions", "resolved_by_id", false, false),
	(
		"book_club_book_suggestions",
		"suggested_by_id",
		false,
		false,
	),
	("book_club_book_suggestions", "book_id", false, false),
	("book_club_book_suggestions", "book_club_id", false, false),
	("book_club_books", "id", false, false),
	("book_club_books", "book_club_id", false, false),
	("book_club_books", "book_entity_id", false, false),
	("book_club_discussion_message_reactions", "id", false, false),
	(
		"book_club_discussion_message_reactions",
		"member_id",
		false,
		false,
	),
	(
		"book_club_discussion_message_reactions",
		"message_id",
		false,
		false,
	),
	("book_club_discussion_message", "id", false, false),
	(
		"book_club_discussion_message",
		"parent_message_id",
		false,
		false,
	),
	(
		"book_club_discussion_message",
		"reply_to_message_id",
		false,
		false,
	),
	(
		"book_club_discussion_message",
		"discussion_id",
		false,
		false,
	),
	("book_club_discussion_message", "book_club_id", false, false),
	("book_club_discussions", "id", false, false),
	("book_club_discussions", "book_club_book_id", false, false),
	("book_club_discussions", "book_club_id", false, false),
	("book_club_invitations", "id", false, false),
	("book_club_invitations", "book_club_id", false, false),
	("book_club_invitations", "user_id", false, false),
	("book_club_member_favorite_books", "id", false, false),
	("book_club_member_favorite_books", "member_id", false, false),
	("book_club_member_favorite_books", "book_id", false, false),
	("book_club_members", "id", false, false),
	("book_club_members", "user_id", false, false),
	("book_club_members", "book_club_id", false, false),
	("book_clubs", "id", false, false),
	("bookmarks", "id", false, false),
	("bookmarks", "media_id", false, false),
	("bookmarks", "user_id", false, false),
	("custom_emojis", "created_by_id", false, false),
	("emailer_send_records", "sent_by_user_id", false, false),
	("favorite_libraries", "library_id", true, true),
	("favorite_libraries", "user_id", true, true),
	("scheduled_job_libraries", "library_id", false, false),
	("favorite_media", "user_id", true, true),
	("favorite_media", "media_id", true, true),
	("favorite_series", "user_id", true, true),
	("favorite_series", "series_id", true, true),
	("finished_reading_sessions", "user_id", true, true),
	("finished_reading_sessions", "media_id", true, true),
	("last_library_visits", "user_id", false, false),
	("last_library_visits", "library_id", false, false),
	("library_configs", "library_id", false, false),
	("library_exclusions", "user_id", true, true),
	("library_exclusions", "library_id", true, true),
	("library_scan_records", "library_id", false, false),
	("library_tags", "library_id", false, false),
	("logs", "job_id", false, false),
	("media_analysis", "media_id", true, true),
	("media_annotations", "id", false, false),
	("media_annotations", "media_id", false, false),
	("media_annotations", "user_id", false, false),
	("media_metadata", "media_id", true, true),
	("media_tags", "media_id", true, true),
	("reading_sessions", "media_id", false, false),
	("reading_sessions", "user_id", false, false),
	("reading_sessions", "device_id", false, false),
	("refresh_tokens", "user_id", false, false),
	("series_metadata", "series_id", true, true),
	("series_tags", "series_id", true, true),
	("sessions", "user_id", true, true),
	("smart_list_access_rules", "user_id", true, true),
	("smart_list_access_rules", "smart_list_id", false, false),
	("smart_list_views", "list_id", true, true),
	("smart_lists", "id", true, true),
	("smart_lists", "creator_id", false, false),
	("user_login_activity", "user_id", true, true),
	("user_preferences", "user_id", true, true),
	("reviews", "user_id", true, true),
	("reviews", "media_id", false, false),
	("book_club_discussion_message", "member_id", false, false),
	("library_scan_records", "job_id", false, false),
];

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		if manager.get_database_backend() == DatabaseBackend::Sqlite {
			// Drop FK_book_club_book_suggestion_likes_book_club_book_suggestions so that
			// the suggestion_id column can be converted to UUID without FK violations.
			sqlite_drop_foreign_keys(manager, "book_club_discussion_message").await?;
			sqlite_drop_foreign_keys(manager, "book_club_book_suggestion_likes").await?;
			sqlite_drop_foreign_keys(manager, "scheduled_job_libraries").await?;
			sqlite_drop_foreign_keys(manager, "library_tags").await?;
			sqlite_drop_foreign_keys(manager, "library_scan_records").await?;
			sqlite_drop_foreign_keys(manager, "custom_emojis").await?;
			sqlite_drop_foreign_keys(manager, "logs").await?;
			sqlite_drop_foreign_keys(manager, "media").await?;
			sqlite_drop_foreign_keys(manager, "emailer_send_records").await?;
			sqlite_drop_foreign_keys(manager, "last_library_visits").await?;
			sqlite_drop_foreign_keys(manager, "reading_sessions").await?;
			sqlite_drop_foreign_keys(manager, "refresh_tokens").await?;

			for (table_name, column_name, unique, skip_pk) in SQLITE_TEXT_UUID_COLUMNS {
				if unique {
					sqlite_alter_column_type_full_copy(
						manager,
						table_name,
						column_name,
						unique,
						skip_pk,
					)
					.await?;
				} else {
					sqlite_alter_column_type(manager, table_name, column_name, unique)
						.await?;
				}
			}

			return Ok(());
		}
		if manager.get_database_backend() == DatabaseBackend::Postgres {
			let sql = r#"
				ALTER TABLE "book_club_book_suggestion_likes" DROP CONSTRAINT IF EXISTS "FK_book_club_book_suggestion_likes_book_club_book_suggestions";
				ALTER TABLE "finished_reading_sessions" DROP CONSTRAINT IF EXISTS "fk-finished_reading_sessions-device";
				ALTER TABLE "users" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "libraries" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "series" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "series" ALTER COLUMN "library_id" TYPE UUID USING "library_id"::uuid;
				ALTER TABLE "media" ALTER COLUMN "series_id" TYPE UUID USING "series_id"::uuid;
				ALTER TABLE "media" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "jobs" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "age_restrictions" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "api_keys" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "book_club_book_suggestion_likes" ALTER COLUMN "liked_by_id" TYPE UUID USING "liked_by_id"::uuid;
				ALTER TABLE "book_club_book_suggestion_likes" ALTER COLUMN "suggestion_id" TYPE UUID USING "suggestion_id"::uuid;
				ALTER TABLE "book_club_book_suggestions" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "book_club_book_suggestions" ALTER COLUMN "resolved_by_id" TYPE UUID USING "resolved_by_id"::uuid;
				ALTER TABLE "book_club_book_suggestions" ALTER COLUMN "suggested_by_id" TYPE UUID USING "suggested_by_id"::uuid;
				ALTER TABLE "book_club_book_suggestions" ALTER COLUMN "book_id" TYPE UUID USING "book_id"::uuid;
				ALTER TABLE "book_club_book_suggestions" ALTER COLUMN "book_club_id" TYPE UUID USING "book_club_id"::uuid;
				ALTER TABLE "book_club_books" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "book_club_books" ALTER COLUMN "book_club_id" TYPE UUID USING "book_club_id"::uuid;
				ALTER TABLE "book_club_books" ALTER COLUMN "book_entity_id" TYPE UUID USING "book_entity_id"::uuid;
				ALTER TABLE "book_club_discussion_message_reactions" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "book_club_discussion_message_reactions" ALTER COLUMN "member_id" TYPE UUID USING "member_id"::uuid;
				ALTER TABLE "book_club_discussion_message_reactions" ALTER COLUMN "message_id" TYPE UUID USING "message_id"::uuid;
				ALTER TABLE "book_club_discussion_message" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "book_club_discussion_message" ALTER COLUMN "parent_message_id" TYPE UUID USING "parent_message_id"::uuid;
				ALTER TABLE "book_club_discussion_message" ALTER COLUMN "reply_to_message_id" TYPE UUID USING "reply_to_message_id"::uuid;
				ALTER TABLE "book_club_discussion_message" ALTER COLUMN "discussion_id" TYPE UUID USING "discussion_id"::uuid;
				ALTER TABLE "book_club_discussion_message" ALTER COLUMN "book_club_id" TYPE UUID USING "book_club_id"::uuid;
				ALTER TABLE "book_club_discussions" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "book_club_discussions" ALTER COLUMN "book_club_book_id" TYPE UUID USING "book_club_book_id"::uuid;
				ALTER TABLE "book_club_discussions" ALTER COLUMN "book_club_id" TYPE UUID USING "book_club_id"::uuid;
				ALTER TABLE "book_club_invitations" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "book_club_invitations" ALTER COLUMN "book_club_id" TYPE UUID USING "book_club_id"::uuid;
				ALTER TABLE "book_club_invitations" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "book_club_member_favorite_books" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "book_club_member_favorite_books" ALTER COLUMN "member_id" TYPE UUID USING "member_id"::uuid;
				ALTER TABLE "book_club_member_favorite_books" ALTER COLUMN "book_id" TYPE UUID USING "book_id"::uuid;
				ALTER TABLE "book_club_members" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "book_club_members" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "book_club_members" ALTER COLUMN "book_club_id" TYPE UUID USING "book_club_id"::uuid;
				ALTER TABLE "book_clubs" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "bookmarks" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "bookmarks" ALTER COLUMN "media_id" TYPE UUID USING "media_id"::uuid;
				ALTER TABLE "bookmarks" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "custom_emojis" ALTER COLUMN "created_by_id" TYPE UUID USING "created_by_id"::uuid;
				ALTER TABLE "emailer_send_records" ALTER COLUMN "sent_by_user_id" TYPE UUID USING "sent_by_user_id"::uuid;
				ALTER TABLE "favorite_libraries" ALTER COLUMN "library_id" TYPE UUID USING "library_id"::uuid;
				ALTER TABLE "favorite_libraries" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "scheduled_job_libraries" ALTER COLUMN "library_id" TYPE UUID USING "library_id"::uuid;
				ALTER TABLE "favorite_media" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "favorite_media" ALTER COLUMN "media_id" TYPE UUID USING "media_id"::uuid;
				ALTER TABLE "favorite_series" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "favorite_series" ALTER COLUMN "series_id" TYPE UUID USING "series_id"::uuid;
				ALTER TABLE "finished_reading_sessions" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "finished_reading_sessions" ALTER COLUMN "media_id" TYPE UUID USING "media_id"::uuid;
				ALTER TABLE "last_library_visits" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "last_library_visits" ALTER COLUMN "library_id" TYPE UUID USING "library_id"::uuid;
				ALTER TABLE "library_configs" ALTER COLUMN "library_id" TYPE UUID USING "library_id"::uuid;
				ALTER TABLE "library_exclusions" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "library_exclusions" ALTER COLUMN "library_id" TYPE UUID USING "library_id"::uuid;
				ALTER TABLE "library_scan_records" ALTER COLUMN "library_id" TYPE UUID USING "library_id"::uuid;
				ALTER TABLE "library_tags" ALTER COLUMN "library_id" TYPE UUID USING "library_id"::uuid;
				ALTER TABLE "logs" ALTER COLUMN "job_id" TYPE UUID USING "job_id"::uuid;
				ALTER TABLE "media_analysis" ALTER COLUMN "media_id" TYPE UUID USING "media_id"::uuid;
				ALTER TABLE "media_annotations" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "media_annotations" ALTER COLUMN "media_id" TYPE UUID USING "media_id"::uuid;
				ALTER TABLE "media_annotations" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "media_metadata" ALTER COLUMN "media_id" TYPE UUID USING "media_id"::uuid;
				ALTER TABLE "media_tags" ALTER COLUMN "media_id" TYPE UUID USING "media_id"::uuid;
				ALTER TABLE "reading_sessions" ALTER COLUMN "media_id" TYPE UUID USING "media_id"::uuid;
				ALTER TABLE "reading_sessions" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "reading_sessions" ALTER COLUMN "device_id" TYPE UUID USING "device_id"::uuid;
				ALTER TABLE "refresh_tokens" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "registered_reading_devices" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "series_metadata" ALTER COLUMN "series_id" TYPE UUID USING "series_id"::uuid;
				ALTER TABLE "series_tags" ALTER COLUMN "series_id" TYPE UUID USING "series_id"::uuid;
				ALTER TABLE "sessions" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "smart_list_access_rules" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "smart_list_access_rules" ALTER COLUMN "smart_list_id" TYPE UUID USING "smart_list_id"::uuid;
				ALTER TABLE "smart_list_views" ALTER COLUMN "list_id" TYPE UUID USING "list_id"::uuid;
				ALTER TABLE "smart_lists" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "smart_lists" ALTER COLUMN "creator_id" TYPE UUID USING "creator_id"::uuid;
				ALTER TABLE "user_login_activity" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "user_preferences" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "reviews" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "reviews" ALTER COLUMN "media_id" TYPE UUID USING "media_id"::uuid;
				ALTER TABLE "book_club_discussion_message" ALTER COLUMN "member_id" TYPE UUID USING "member_id"::uuid;
				ALTER TABLE "library_scan_records" ALTER COLUMN "job_id" TYPE UUID USING "job_id"::uuid;
			"#;
			manager.get_connection().execute_unprepared(sql).await?;
		}
		return Ok(());
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		if manager.get_database_backend() == DatabaseBackend::Sqlite {
			for (table_name, column_name, unique, _) in SQLITE_TEXT_UUID_COLUMNS {
				sqlite_revert_column_type(manager, table_name, column_name, unique)
					.await?;
			}

			return Ok(());
		}
		if manager.get_database_backend() == DatabaseBackend::Postgres {
			let sql = r#"
				ALTER TABLE "age_restrictions" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "api_keys" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "book_club_book_suggestion_likes" ALTER COLUMN "liked_by_id" TYPE TEXT;
				ALTER TABLE "book_club_book_suggestion_likes" ALTER COLUMN "suggestion_id" TYPE TEXT;
				ALTER TABLE "book_club_book_suggestions" ALTER COLUMN "id" TYPE TEXT;
				ALTER TABLE "book_club_book_suggestions" ALTER COLUMN "suggested_by_id" TYPE TEXT;
				ALTER TABLE "book_club_book_suggestions" ALTER COLUMN "book_id" TYPE TEXT;
				ALTER TABLE "book_club_book_suggestions" ALTER COLUMN "book_club_id" TYPE TEXT;
				ALTER TABLE "book_club_books" ALTER COLUMN "id" TYPE TEXT;
				ALTER TABLE "book_club_books" ALTER COLUMN "book_club_id" TYPE TEXT;
				ALTER TABLE "book_club_books" ALTER COLUMN "book_entity_id" TYPE TEXT;
				ALTER TABLE "book_club_discussion_message_reactions" ALTER COLUMN "id" TYPE TEXT;
				ALTER TABLE "book_club_discussion_message_reactions" ALTER COLUMN "member_id" TYPE TEXT;
				ALTER TABLE "book_club_discussion_message_reactions" ALTER COLUMN "message_id" TYPE TEXT;
				ALTER TABLE "book_club_discussion_message" ALTER COLUMN "id" TYPE TEXT;
				ALTER TABLE "book_club_discussion_message" ALTER COLUMN "parent_message_id" TYPE TEXT;
				ALTER TABLE "book_club_discussion_message" ALTER COLUMN "reply_to_message_id" TYPE TEXT;
				ALTER TABLE "book_club_discussion_message" ALTER COLUMN "discussion_id" TYPE TEXT;
				ALTER TABLE "book_club_discussion_message" ALTER COLUMN "book_club_id" TYPE TEXT;
				ALTER TABLE "book_club_discussions" ALTER COLUMN "id" TYPE TEXT;
				ALTER TABLE "book_club_discussions" ALTER COLUMN "book_club_book_id" TYPE TEXT;
				ALTER TABLE "book_club_discussions" ALTER COLUMN "book_club_id" TYPE TEXT;
				ALTER TABLE "book_club_invitations" ALTER COLUMN "id" TYPE TEXT;
				ALTER TABLE "book_club_invitations" ALTER COLUMN "book_club_id" TYPE TEXT;
				ALTER TABLE "book_club_invitations" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "book_club_member_favorite_books" ALTER COLUMN "id" TYPE TEXT;
				ALTER TABLE "book_club_member_favorite_books" ALTER COLUMN "member_id" TYPE TEXT;
				ALTER TABLE "book_club_member_favorite_books" ALTER COLUMN "book_id" TYPE TEXT;
				ALTER TABLE "book_club_members" ALTER COLUMN "id" TYPE TEXT;
				ALTER TABLE "book_club_members" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "book_club_members" ALTER COLUMN "book_club_id" TYPE TEXT;
				ALTER TABLE "book_clubs" ALTER COLUMN "id" TYPE TEXT;
				ALTER TABLE "bookmarks" ALTER COLUMN "id" TYPE TEXT;
				ALTER TABLE "bookmarks" ALTER COLUMN "media_id" TYPE TEXT;
				ALTER TABLE "bookmarks" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "custom_emojis" ALTER COLUMN "created_by_id" TYPE TEXT;
				ALTER TABLE "emailer_send_records" ALTER COLUMN "sent_by_user_id" TYPE TEXT;
				ALTER TABLE "favorite_libraries" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "favorite_libraries" ALTER COLUMN "library_id" TYPE TEXT;
				ALTER TABLE "favorite_media" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "favorite_media" ALTER COLUMN "media_id" TYPE TEXT;
				ALTER TABLE "favorite_series" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "favorite_series" ALTER COLUMN "series_id" TYPE TEXT;
				ALTER TABLE "finished_reading_sessions" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "finished_reading_sessions" ALTER COLUMN "media_id" TYPE TEXT;
				ALTER TABLE "jobs" ALTER COLUMN "id" TYPE TEXT;
				ALTER TABLE "last_library_visits" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "last_library_visits" ALTER COLUMN "library_id" TYPE TEXT;
				ALTER TABLE "library_configs" ALTER COLUMN "library_id" TYPE TEXT;
				ALTER TABLE "library_exclusions" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "library_exclusions" ALTER COLUMN "library_id" TYPE TEXT;
				ALTER TABLE "library_scan_records" ALTER COLUMN "library_id" TYPE TEXT;
				ALTER TABLE "library_tags" ALTER COLUMN "library_id" TYPE TEXT;
				ALTER TABLE "libraries" ALTER COLUMN "id" TYPE TEXT;
				ALTER TABLE "logs" ALTER COLUMN "job_id" TYPE TEXT;
				ALTER TABLE "media_analysis" ALTER COLUMN "media_id" TYPE TEXT;
				ALTER TABLE "media_annotations" ALTER COLUMN "id" TYPE TEXT;
				ALTER TABLE "media_annotations" ALTER COLUMN "media_id" TYPE TEXT;
				ALTER TABLE "media_annotations" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "media_metadata" ALTER COLUMN "media_id" TYPE TEXT;
				ALTER TABLE "media_tags" ALTER COLUMN "media_id" TYPE TEXT;
				ALTER TABLE "media" ALTER COLUMN "id" TYPE TEXT;
				ALTER TABLE "media" ALTER COLUMN "series_id" TYPE TEXT;
				ALTER TABLE "reading_sessions" ALTER COLUMN "media_id" TYPE TEXT;
				ALTER TABLE "reading_sessions" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "reading_sessions" ALTER COLUMN "device_id" TYPE TEXT;
				ALTER TABLE "refresh_tokens" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "series_metadata" ALTER COLUMN "series_id" TYPE TEXT;
				ALTER TABLE "series_tags" ALTER COLUMN "series_id" TYPE TEXT;
				ALTER TABLE "series" ALTER COLUMN "id" TYPE TEXT;
				ALTER TABLE "series" ALTER COLUMN "library_id" TYPE TEXT;
				ALTER TABLE "sessions" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "smart_list_access_rules" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "smart_list_access_rules" ALTER COLUMN "smart_list_id" TYPE TEXT;
				ALTER TABLE "smart_list_views" ALTER COLUMN "list_id" TYPE TEXT;
				ALTER TABLE "smart_lists" ALTER COLUMN "id" TYPE TEXT;
				ALTER TABLE "smart_lists" ALTER COLUMN "creator_id" TYPE TEXT;
				ALTER TABLE "user_login_activity" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "user_preferences" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "users" ALTER COLUMN "id" TYPE TEXT;
				ALTER TABLE "reviews" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "reviews" ALTER COLUMN "media_id" TYPE TEXT;
				ALTER TABLE "book_club_discussion_message" ALTER COLUMN "member_id" TYPE TEXT;
				ALTER TABLE "library_scan_records" ALTER COLUMN "job_id" TYPE TEXT;
				ALTER TABLE "book_club_book_suggestion_likes" ADD CONSTRAINT "FK_book_club_book_suggestion_likes_book_club_book_suggestions" FOREIGN KEY ("suggestion_id") REFERENCES "book_club_book_suggestions"("id");
			"#;
			manager.get_connection().execute_unprepared(sql).await?;
		}
		return Ok(());
	}
}
