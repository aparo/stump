use chrono::Local;
use sea_orm::{ConnectionTrait, DatabaseBackend, DbErr, Statement};
use sea_orm_migration::SchemaManager;
use std::path::PathBuf;

pub async fn dump_data(
	manager: &SchemaManager<'static>,
	path: PathBuf,
	tables: Vec<String>,
) -> Result<PathBuf, DbErr> {
	let now = Local::now();
	let folder_name = now.format("%Y%m%d%H%M%S").to_string();
	let dump_dir = path.join(folder_name);

	tokio::fs::create_dir_all(&dump_dir)
		.await
		.map_err(|e| DbErr::Custom(e.to_string()))?;

	let db = manager.get_connection();
	for table in tables {
		let stmt = Statement::from_string(
			DatabaseBackend::Sqlite,
			format!("PRAGMA table_info(\"{}\")", table),
		);
		let cols_res = db.query_all(stmt).await?;
		let mut cols = Vec::new();
		for row in cols_res {
			let name: String = row.try_get("", "name")?;
			cols.push(name);
		}

		if cols.is_empty() {
			continue;
		}

		let obj_args = cols
			.iter()
			.map(|col| format!("'{}', \"{}\"", col, col))
			.collect::<Vec<_>>()
			.join(", ");

		let query = format!(
            "SELECT COALESCE(json_group_array(json_object({})), '[]') AS json_data FROM \"{}\"",
            obj_args, table
        );

		let res = db
			.query_one(Statement::from_string(DatabaseBackend::Sqlite, query))
			.await?;

		let json_data: String = if let Some(row) = res {
			row.try_get("", "json_data")?
		} else {
			"[]".to_string()
		};

		let file_path = dump_dir.join(format!("{}.json", table));
		tokio::fs::write(&file_path, json_data)
			.await
			.map_err(|e| DbErr::Custom(e.to_string()))?;
	}

	Ok(dump_dir)
}

pub async fn restore_data(
	manager: &SchemaManager<'static>,
	path: PathBuf,
	tables: Vec<String>,
) -> Result<(), DbErr> {
	let db = manager.get_connection();
	for table in tables {
		let file_path = path.join(format!("{}.json", table));
		if !file_path.exists() {
			continue;
		}

		let json_data = tokio::fs::read_to_string(&file_path)
			.await
			.map_err(|e| DbErr::Custom(e.to_string()))?;

		let stmt = Statement::from_string(
			DatabaseBackend::Sqlite,
			format!("PRAGMA table_info(\"{}\")", table),
		);
		let cols_res = db.query_all(stmt).await?;
		let mut cols = Vec::new();
		for row in cols_res {
			let name: String = row.try_get("", "name")?;
			cols.push(name);
		}

		if cols.is_empty() || json_data.trim() == "[]" || json_data.trim().is_empty() {
			continue;
		}

		let col_names = cols
			.iter()
			.map(|col| format!("\"{}\"", col))
			.collect::<Vec<_>>()
			.join(", ");
		let col_extracts = cols
			.iter()
			.map(|col| format!("json_extract(value, '$.{}')", col))
			.collect::<Vec<_>>()
			.join(", ");

		let query = format!(
			"INSERT INTO \"{}\" ({}) SELECT {} FROM json_each(?1)",
			table, col_names, col_extracts
		);

		let stmt = Statement::from_sql_and_values(
			DatabaseBackend::Sqlite,
			query,
			vec![json_data.into()],
		);

		db.execute(stmt).await?;
	}

	Ok(())
}
