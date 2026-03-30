use sea_orm::DatabaseBackend;
use sea_orm::{ConnectionTrait, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const SQLITE_TEXT_UUID_COLUMNS: [(&str, &str, bool); 86] = [
	("users", "id", false),
	("libraries", "id", false),
	("series", "id", false),
	("series", "library_id", false),
	("media", "series_id", false),
	("media", "id", false),
	("jobs", "id", false),
	("age_restrictions", "user_id", true),
	("api_keys", "user_id", true),
	("book_club_book_suggestion_likes", "liked_by_id", false),
	("book_club_book_suggestion_likes", "suggestion_id", false),
	("book_club_book_suggestions", "id", false),
	("book_club_book_suggestions", "resolved_by_id", false),
	("book_club_book_suggestions", "suggested_by_id", false),
	("book_club_book_suggestions", "book_id", false),
	("book_club_book_suggestions", "book_club_id", false),
	("book_club_books", "id", false),
	("book_club_books", "book_club_id", false),
	("book_club_books", "book_entity_id", false),
	("book_club_discussion_message_reactions", "id", false),
	("book_club_discussion_message_reactions", "member_id", false),
	(
		"book_club_discussion_message_reactions",
		"message_id",
		false,
	),
	("book_club_discussion_message", "id", false),
	("book_club_discussion_message", "parent_message_id", false),
	("book_club_discussion_message", "reply_to_message_id", false),
	("book_club_discussion_message", "discussion_id", false),
	("book_club_discussion_message", "book_club_id", false),
	("book_club_discussions", "id", false),
	("book_club_discussions", "book_club_book_id", false),
	("book_club_discussions", "book_club_id", false),
	("book_club_invitations", "id", false),
	("book_club_invitations", "book_club_id", false),
	("book_club_invitations", "user_id", false),
	("book_club_member_favorite_books", "id", false),
	("book_club_member_favorite_books", "member_id", false),
	("book_club_member_favorite_books", "book_id", false),
	("book_club_members", "id", false),
	("book_club_members", "user_id", false),
	("book_club_members", "book_club_id", false),
	("book_clubs", "id", false),
	("bookmarks", "id", false),
	("bookmarks", "media_id", false),
	("bookmarks", "user_id", false),
	("custom_emojis", "created_by_id", false),
	("emailer_send_records", "sent_by_user_id", false),
	("favorite_libraries", "library_id", false),
	("favorite_libraries", "user_id", false),
	("scheduled_job_libraries", "library_id", false),
	("favorite_media", "user_id", false),
	("favorite_media", "media_id", false),
	("favorite_series", "user_id", false),
	("favorite_series", "series_id", false),
	("finished_reading_sessions", "user_id", false),
	("finished_reading_sessions", "media_id", false),
	("last_library_visits", "user_id", false),
	("last_library_visits", "library_id", false),
	("library_configs", "library_id", false),
	("library_exclusions", "user_id", false),
	("library_exclusions", "library_id", false),
	("library_scan_records", "library_id", false),
	("library_tags", "library_id", false),
	("logs", "job_id", false),
	("media_analysis", "media_id", false),
	("media_annotations", "id", false),
	("media_annotations", "media_id", false),
	("media_annotations", "user_id", false),
	("media_metadata", "media_id", false),
	("media_tags", "media_id", false),
	("reading_sessions", "media_id", false),
	("reading_sessions", "user_id", false),
	("reading_sessions", "device_id", false),
	("refresh_tokens", "user_id", false),
	("series_metadata", "series_id", false),
	("series_tags", "series_id", false),
	("sessions", "user_id", false),
	("smart_list_access_rules", "user_id", false),
	("smart_list_access_rules", "smart_list_id", false),
	("smart_list_views", "list_id", false),
	("smart_lists", "id", false),
	("smart_lists", "creator_id", false),
	("user_login_activity", "user_id", false),
	("user_preferences", "user_id", false),
	("reviews", "user_id", false),
	("reviews", "media_id", false),
	("book_club_discussion_message", "member_id", false),
	("library_scan_records", "job_id", false),
];

fn sqlite_quote_ident(ident: &str) -> String {
	format!("\"{}\"", ident.replace('"', "\"\""))
}

#[derive(Debug, Clone)]
struct SqliteColumnInfo {
	name: String,
	type_name: String,
	not_null: bool,
	default_value: Option<String>,
	pk_position: i64,
}

async fn sqlite_table_columns(
	manager: &SchemaManager<'_>,
	table_name: &str,
) -> Result<Vec<SqliteColumnInfo>, DbErr> {
	let query = format!("PRAGMA table_info({})", sqlite_quote_ident(table_name));
	let stmt = Statement::from_string(DatabaseBackend::Sqlite, query);
	let rows = manager.get_connection().query_all(stmt).await?;

	let mut cols = Vec::with_capacity(rows.len());
	for row in rows {
		let name: String = row.try_get("", "name")?;
		let type_name: String = row.try_get("", "type")?;
		let not_null: i64 = row.try_get("", "notnull")?;
		let default_value: Option<String> = row.try_get("", "dflt_value")?;
		let pk_position: i64 = row.try_get("", "pk")?;

		cols.push(SqliteColumnInfo {
			name,
			type_name,
			not_null: not_null == 1,
			default_value,
			pk_position,
		});
	}

	Ok(cols)
}

async fn sqlite_set_id_primary_key(
	manager: &SchemaManager<'_>,
	table_name: &str,
	id_type: &str,
	has_primary_key: bool,
) -> Result<(), DbErr> {
	let cols = sqlite_table_columns(manager, table_name).await?;
	if cols.is_empty() {
		return Ok(());
	}

	let tmp_table_name = format!("{table_name}__pk_swap_tmp");
	let quoted_table = sqlite_quote_ident(table_name);
	let quoted_tmp_table = sqlite_quote_ident(&tmp_table_name);

	let mut col_defs = Vec::with_capacity(cols.len());
	let mut col_names = Vec::with_capacity(cols.len());
	for col in &cols {
		let quoted_col = sqlite_quote_ident(&col.name);
		col_names.push(quoted_col.clone());

		let type_name = if col.name == "id" {
			id_type.to_string()
		} else {
			col.type_name.clone()
		};

		let mut def = format!("{quoted_col} {type_name}");
		if col.not_null {
			def.push_str(" NOT NULL");
		}
		if let Some(default) = &col.default_value {
			def.push_str(&format!(" DEFAULT {default}"));
		}

		if col.name == "id" {
			if has_primary_key {
				def.push_str(" PRIMARY KEY");
			}
		} else if col.pk_position > 0 {
			def.push_str(" PRIMARY KEY");
		}

		col_defs.push(def);
	}

	let sql = format!(
		r#"
			PRAGMA foreign_keys=OFF;
			ALTER TABLE {table} RENAME TO {tmp_table};
			CREATE TABLE {table} ({column_defs});
			INSERT INTO {table} ({columns}) SELECT {columns} FROM {tmp_table};
			DROP TABLE {tmp_table};
			PRAGMA foreign_keys=ON;
		"#,
		table = quoted_table,
		tmp_table = quoted_tmp_table,
		column_defs = col_defs.join(", "),
		columns = col_names.join(", "),
	);

	manager.get_connection().execute_unprepared(&sql).await?;

	Ok(())
}

async fn sqlite_alter_column_type(
	manager: &SchemaManager<'_>,
	table_name: &str,
	column_name: &str,
	unique: bool,
) -> Result<(), DbErr> {
	let quoted_table = sqlite_quote_ident(table_name);
	let quoted_column = sqlite_quote_ident(column_name);
	let temp_column_name = format!("{column_name}__tmp_uuid");
	let quoted_temp_column = sqlite_quote_ident(&temp_column_name);

	// Clean up any leftover temp column from previous failed attempts
	let cols = sqlite_table_columns(manager, table_name).await?;
	if cols.iter().any(|c| c.name == temp_column_name) {
		let drop_sql = format!(
			"ALTER TABLE {table} DROP COLUMN {temp_col}",
			table = quoted_table,
			temp_col = quoted_temp_column
		);
		let _ = manager.get_connection().execute_unprepared(&drop_sql).await;
	}

	if column_name == "id" {
		sqlite_set_id_primary_key(manager, table_name, "UUID", false).await?;
	}

	if unique {
		// we drop unique indexes before the alter and re-create them on the new column since SQLite doesn't support altering column types that are part of a unique index
		let index_name = format!("sqlite_autoindex_{table_name}_1");
		let drop_sql = format!(
			"DROP INDEX IF EXISTS {index}",
			index = sqlite_quote_ident(&index_name)
		);
		let _ = manager.get_connection().execute_unprepared(&drop_sql).await;
	}

	let sql = format!(
		r#"
			PRAGMA foreign_keys=OFF;
			ALTER TABLE {table} ADD COLUMN {temp_column} UUID;
			UPDATE {table}
			SET {temp_column} = CASE
				WHEN {column} IS NULL THEN NULL
				ELSE lower({column})
			END;
			ALTER TABLE {table} DROP COLUMN {column};
			ALTER TABLE {table} RENAME COLUMN {temp_column} TO {column};
			PRAGMA foreign_keys=ON;
		"#,
		table = quoted_table,
		column = quoted_column,
		temp_column = quoted_temp_column,
	);

	manager.get_connection().execute_unprepared(&sql).await?;

	if column_name == "id" {
		sqlite_set_id_primary_key(manager, table_name, "UUID", true).await?;
	}
	if unique {
		// we add the unique index back after the alter
		let index_name = format!("sqlite_autoindex_{table_name}_1");
		let create_sql = format!(
			"CREATE UNIQUE INDEX {index} ON {table} ({column})",
			index = sqlite_quote_ident(&index_name),
			table = quoted_table,
			column = quoted_column
		);
		let _ = manager
			.get_connection()
			.execute_unprepared(&create_sql)
			.await;
	}

	Ok(())
}

/// An alternative to [`sqlite_alter_column_type`] that uses a full table copy approach:
/// creates a new table with the updated column type, copies all data (converting the target
/// column with `lower(CAST(... AS TEXT))`), drops the old table, then renames the new table.
/// This is safer for tables with complex constraints or triggers.
#[allow(dead_code)]
async fn sqlite_alter_column_type_full_copy(
	manager: &SchemaManager<'_>,
	table_name: &str,
	column_name: &str,
	unique: bool,
) -> Result<(), DbErr> {
	let cols = sqlite_table_columns(manager, table_name).await?;
	if cols.is_empty() {
		return Ok(());
	}

	let tmp_table_name = format!("{table_name}__col_copy_tmp");
	let quoted_table = sqlite_quote_ident(table_name);
	let quoted_tmp_table = sqlite_quote_ident(&tmp_table_name);

	let mut col_defs: Vec<String> = Vec::with_capacity(cols.len());
	let mut col_names: Vec<String> = Vec::with_capacity(cols.len());
	let mut select_exprs: Vec<String> = Vec::with_capacity(cols.len());

	for col in &cols {
		let quoted_col = sqlite_quote_ident(&col.name);
		col_names.push(quoted_col.clone());

		let type_name = if col.name == column_name {
			"UUID".to_string()
		} else {
			col.type_name.clone()
		};

		let mut def = format!("{quoted_col} {type_name}");
		if col.not_null {
			def.push_str(" NOT NULL");
		}
		if let Some(default) = &col.default_value {
			def.push_str(&format!(" DEFAULT {default}"));
		}
		if col.pk_position > 0 {
			def.push_str(" PRIMARY KEY");
		}
		if unique && col.name == column_name {
			def.push_str(" UNIQUE");
		}
		col_defs.push(def);

		// Convert the target column value; leave all others as-is
		if col.name == column_name {
			select_exprs.push(format!(
				"CASE WHEN {col} IS NULL THEN NULL ELSE lower(CAST({col} AS TEXT)) END",
				col = quoted_col,
			));
		} else {
			select_exprs.push(quoted_col.clone());
		}
	}

	let sql = format!(
		r#"
			PRAGMA foreign_keys=OFF;
			CREATE TABLE {tmp_table} ({column_defs});
			INSERT INTO {tmp_table} ({columns}) SELECT {select_exprs} FROM {table};
			DROP TABLE {table};
			ALTER TABLE {tmp_table} RENAME TO {table};
			PRAGMA foreign_keys=ON;
		"#,
		tmp_table = quoted_tmp_table,
		column_defs = col_defs.join(", "),
		columns = col_names.join(", "),
		select_exprs = select_exprs.join(", "),
		table = quoted_table,
	);

	manager.get_connection().execute_unprepared(&sql).await?;

	Ok(())
}

async fn sqlite_revert_column_type(
	manager: &SchemaManager<'_>,
	table_name: &str,
	column_name: &str,
	unique: bool,
) -> Result<(), DbErr> {
	let quoted_table = sqlite_quote_ident(table_name);
	let quoted_column = sqlite_quote_ident(column_name);
	let temp_column_name = format!("{column_name}__tmp_text");
	let quoted_temp_column = sqlite_quote_ident(&temp_column_name);

	// Clean up any leftover temp column from previous failed attempts
	let cols = sqlite_table_columns(manager, table_name).await?;
	if cols.iter().any(|c| c.name == temp_column_name) {
		let drop_sql = format!(
			"ALTER TABLE {table} DROP COLUMN {temp_col}",
			table = quoted_table,
			temp_col = quoted_temp_column
		);
		let _ = manager.get_connection().execute_unprepared(&drop_sql).await;
	}

	if column_name == "id" {
		sqlite_set_id_primary_key(manager, table_name, "TEXT", false).await?;
	}

	if unique {
		// we drop unique indexes before the alter and re-create them on the new column since SQLite doesn't support altering column types that are part of a unique index
		let index_name = format!("sqlite_autoindex_{table_name}_1");
		let drop_sql = format!(
			"DROP INDEX IF EXISTS {index}",
			index = sqlite_quote_ident(&index_name)
		);
		let _ = manager.get_connection().execute_unprepared(&drop_sql).await;
	}

	let sql = format!(
		r#"
			PRAGMA foreign_keys=OFF;
			ALTER TABLE {table} ADD COLUMN {temp_column} TEXT;
			UPDATE {table}
			SET {temp_column} = CASE
				WHEN {column} IS NULL THEN NULL
				ELSE CAST({column} AS TEXT)
			END;
			ALTER TABLE {table} DROP COLUMN {column};
			ALTER TABLE {table} RENAME COLUMN {temp_column} TO {column};
			PRAGMA foreign_keys=ON;
		"#,
		table = quoted_table,
		column = quoted_column,
		temp_column = quoted_temp_column,
	);

	manager.get_connection().execute_unprepared(&sql).await?;

	if column_name == "id" {
		sqlite_set_id_primary_key(manager, table_name, "TEXT", true).await?;
	}

	if unique {
		// we add the unique index back after the alter
		let index_name = format!("sqlite_autoindex_{table_name}_1");
		let create_sql = format!(
			"CREATE UNIQUE INDEX {index} ON {table} ({column})",
			index = sqlite_quote_ident(&index_name),
			table = quoted_table,
			column = quoted_column
		);
		let _ = manager
			.get_connection()
			.execute_unprepared(&create_sql)
			.await;
	}
	Ok(())
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		if manager.get_database_backend() == DatabaseBackend::Sqlite {
			for (table_name, column_name, unique) in SQLITE_TEXT_UUID_COLUMNS {
				if unique {
					sqlite_alter_column_type_full_copy(
						manager,
						table_name,
						column_name,
						unique,
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
			for (table_name, column_name, unique) in SQLITE_TEXT_UUID_COLUMNS {
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
			"#;
			manager.get_connection().execute_unprepared(sql).await?;
		}
		return Ok(());
	}
}
