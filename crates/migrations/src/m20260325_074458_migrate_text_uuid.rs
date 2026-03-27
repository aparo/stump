use crate::m20250807_202824_init::*;
use crate::m20251116_000000_book_club_enhancements::*;
use crate::m20251118_183043_media_analysis::*;
use crate::m20260116_000000_rewrite_media_annotations::*;
use crate::m20260118_204601_add_bookmark_created_at::Bookmarks;
use sea_orm::DatabaseBackend;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		if manager.get_database_backend() == DatabaseBackend::Sqlite {
			// only run this migration for sqlite, as other databases already use timestamp with timezone
			return Ok(());
		}
		if manager.get_database_backend() == DatabaseBackend::Postgres {
			let sql = r#"
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
				ALTER TABLE "book_club_book_suggestions" ALTER COLUMN "suggested_by_id" TYPE UUID USING "suggested_by_id"::uuid;
				ALTER TABLE "book_club_book_suggestions" ALTER COLUMN "book_id" TYPE UUID USING "book_id"::uuid;
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
				ALTER TABLE "custom_emojis" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "custom_emojis" ALTER COLUMN "created_by_id" TYPE UUID USING "created_by_id"::uuid;
				ALTER TABLE "emailer_send_records" ALTER COLUMN "sent_by_user_id" TYPE UUID USING "sent_by_user_id"::uuid;
				ALTER TABLE "favorite_libraries" ALTER COLUMN "library_id" TYPE UUID USING "library_id"::uuid;
				ALTER TABLE "favorite_libraries" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
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
				ALTER TABLE "series_metadata" ALTER COLUMN "series_id" TYPE UUID USING "series_id"::uuid;
				ALTER TABLE "series_tags" ALTER COLUMN "series_id" TYPE UUID USING "series_id"::uuid;
				ALTER TABLE "sessions" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "smart_list_access_rules" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "smart_list_access_rules" ALTER COLUMN "smart_list_id" TYPE UUID USING "smart_list_id"::uuid;
				ALTER TABLE "smart_list_views" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "smart_list_views" ALTER COLUMN "list_id" TYPE UUID USING "list_id"::uuid;
				ALTER TABLE "smart_lists" ALTER COLUMN "id" TYPE UUID USING "id"::uuid;
				ALTER TABLE "smart_lists" ALTER COLUMN "creator_id" TYPE UUID USING "creator_id"::uuid;
				ALTER TABLE "user_login_activity" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
				ALTER TABLE "user_preferences" ALTER COLUMN "user_id" TYPE UUID USING "user_id"::uuid;
			"#;
			manager.get_connection().execute_unprepared(sql).await?;
		}
		return Ok(());
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		if manager.get_database_backend() == DatabaseBackend::Sqlite {
			// only run this migration for sqlite, as other databases already use timestamp with timezone
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
				ALTER TABLE "custom_emojis" ALTER COLUMN "id" TYPE TEXT;
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
				ALTER TABLE "smart_list_views" ALTER COLUMN "id" TYPE TEXT;
				ALTER TABLE "smart_list_views" ALTER COLUMN "list_id" TYPE TEXT;
				ALTER TABLE "smart_lists" ALTER COLUMN "id" TYPE TEXT;
				ALTER TABLE "smart_lists" ALTER COLUMN "creator_id" TYPE TEXT;
				ALTER TABLE "user_login_activity" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "user_preferences" ALTER COLUMN "user_id" TYPE TEXT;
				ALTER TABLE "users" ALTER COLUMN "id" TYPE TEXT;
			"#;
			manager.get_connection().execute_unprepared(sql).await?;
		}
		return Ok(());
	}
}
