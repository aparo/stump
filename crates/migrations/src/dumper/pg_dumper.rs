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
		let query = format!(
            "SELECT COALESCE(json_agg(row_to_json(t)), '[]'::json)::TEXT AS json_data FROM (SELECT * FROM \"{}\") t",
            table
        );
		let stmt = Statement::from_string(DatabaseBackend::Postgres, query);
		let res = db.query_one(stmt).await?;

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

		// Uses Postgres json_populate_recordset for lightning-fast parsing & inserts.
		let query = format!(
            "INSERT INTO \"{}\" SELECT * FROM json_populate_recordset(null::\"{}\", $1::json)",
            table, table
        );

		let stmt = Statement::from_sql_and_values(
			DatabaseBackend::Postgres,
			query,
			vec![json_data.into()],
		);

		db.execute(stmt).await?;
	}

	Ok(())
}
