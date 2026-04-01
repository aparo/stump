use sea_orm::DatabaseBackend;
use sea_orm::{ConnectionTrait, Statement};
use sea_orm_migration::prelude::*;

pub fn sqlite_quote_ident(ident: &str) -> String {
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

pub async fn sqlite_set_id_primary_key(
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

/// Rebuilds the given table from its current schema (via `PRAGMA table_info`) without
/// any FOREIGN KEY clauses, effectively dropping all FK constraints on that table.
/// This is necessary before column type migrations that would otherwise violate FK
/// consistency checks in SQLite.
pub async fn sqlite_drop_foreign_keys(
	manager: &SchemaManager<'_>,
	table_name: &str,
) -> Result<(), DbErr> {
	let cols = sqlite_table_columns(manager, table_name).await?;
	if cols.is_empty() {
		return Ok(());
	}

	let tmp_table_name = format!("{table_name}__drop_fk_tmp");
	let quoted_table = sqlite_quote_ident(table_name);
	let quoted_tmp_table = sqlite_quote_ident(&tmp_table_name);

	let mut col_defs: Vec<String> = Vec::with_capacity(cols.len());
	let mut col_names: Vec<String> = Vec::with_capacity(cols.len());
	for col in &cols {
		let quoted_col = sqlite_quote_ident(&col.name);
		col_names.push(quoted_col.clone());

		let mut def = format!("{quoted_col} {}", col.type_name);
		if col.not_null {
			def.push_str(" NOT NULL");
		}
		if let Some(default) = &col.default_value {
			def.push_str(&format!(" DEFAULT {default}"));
		}
		if col.pk_position > 0 {
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

pub async fn sqlite_alter_column_type(
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
		sqlite_set_id_primary_key(manager, table_name, "uuid_text", false).await?;
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
			ALTER TABLE {table} ADD COLUMN {temp_column} uuid_text;
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
		sqlite_set_id_primary_key(manager, table_name, "uuid_text", true).await?;
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
pub async fn sqlite_alter_column_type_full_copy(
	manager: &SchemaManager<'_>,
	table_name: &str,
	column_name: &str,
	unique: bool,
	skip_pk: bool,
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
			"uuid_text".to_string()
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
		if !skip_pk && col.pk_position > 0 {
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

pub async fn sqlite_revert_column_type(
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
		sqlite_set_id_primary_key(manager, table_name, "uuid_text", false).await?;
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
		sqlite_set_id_primary_key(manager, table_name, "uuid_text", true).await?;
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

pub async fn sqlite_drop_indexes(
	manager: &SchemaManager<'_>,
	table_name: &str,
	index_name: &str,
) -> Result<(), DbErr> {
	let quoted_table = sqlite_quote_ident(table_name);
	let quoted_index = sqlite_quote_ident(index_name);

	let check_sql = format!(
		r#"
			SELECT 1
			FROM sqlite_master
			WHERE type = 'index'
				AND name = {index_name}
				AND tbl_name = {table_name}
			LIMIT 1
		"#,
		index_name = quoted_index,
		table_name = quoted_table,
	);
	let check_stmt = Statement::from_string(DatabaseBackend::Sqlite, check_sql);
	let rows = manager.get_connection().query_all(check_stmt).await?;

	if rows.is_empty() {
		return Ok(());
	}

	let drop_sql = format!("DROP INDEX IF EXISTS {index}", index = quoted_index);
	manager
		.get_connection()
		.execute_unprepared(&drop_sql)
		.await?;

	Ok(())
}
