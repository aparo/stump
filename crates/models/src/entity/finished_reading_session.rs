use crate::prefixer::{parse_query_to_model, parse_query_to_model_optional, Prefixer};

use super::{media, registered_reading_device, user::AuthUser};
use async_graphql::SimpleObject;
use sea_orm::{
	entity::prelude::*, ConnectionTrait, FromQueryResult, QueryOrder, QuerySelect,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[graphql(name = "FinishedReadingSessionModel")]
#[sea_orm(table_name = "finished_reading_sessions")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = true)]
	pub id: i32,
	pub started_at: DateTimeWithTimeZone,
	pub completed_at: DateTimeWithTimeZone,
	#[sea_orm(column_type = "Uuid")]
	pub media_id: Uuid,
	#[sea_orm(column_type = "Uuid")]
	pub user_id: Uuid,
	#[sea_orm(column_type = "Uuid", nullable)]
	pub device_id: Option<Uuid>,
	pub elapsed_seconds: Option<i64>,
}

pub struct ModelWithDevice {
	pub model: Model,
	pub device: Option<registered_reading_device::Model>,
}

impl ModelWithDevice {
	pub fn find() -> Select<Entity> {
		Prefixer::new(Entity::find().select_only())
			.add_columns(Entity)
			.add_columns(registered_reading_device::Entity)
			.selector
			.left_join(registered_reading_device::Entity)
	}
}

impl FromQueryResult for ModelWithDevice {
	fn from_query_result(
		res: &sea_orm::QueryResult,
		_pre: &str,
	) -> Result<Self, sea_orm::DbErr> {
		let model = parse_query_to_model::<Model, Entity>(res)?;
		let device = parse_query_to_model_optional::<
			registered_reading_device::Model,
			registered_reading_device::Entity,
		>(res)?;
		Ok(Self { model, device })
	}
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::media::Entity",
		from = "Column::MediaId",
		to = "super::media::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	Media,
	#[sea_orm(
		belongs_to = "super::registered_reading_device::Entity",
		from = "Column::DeviceId",
		to = "super::registered_reading_device::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	RegisteredReadingDevice,
	#[sea_orm(
		belongs_to = "super::user::Entity",
		from = "Column::UserId",
		to = "super::user::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	User,
}

impl Related<super::media::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Media.def()
	}
}

impl Related<super::registered_reading_device::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::RegisteredReadingDevice.def()
	}
}

impl Related<super::user::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::User.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
	pub fn find_finished_in_series(user: &AuthUser, series_id: Uuid) -> Select<Self> {
		Self::find()
			.inner_join(media::Entity)
			.filter(media::Column::SeriesId.eq(series_id))
			.filter(Column::UserId.eq(user.id))
			.distinct_on([Column::MediaId])
	}

	pub async fn recent_completed_record(
		conn: &impl ConnectionTrait,
		user_id: Uuid,
		media_id: Uuid,
		timeout_secs: i64,
	) -> Result<Option<Model>, DbErr> {
		let cutoff = chrono::Utc::now() - chrono::Duration::seconds(timeout_secs);
		Self::find()
			.filter(Column::UserId.eq(user_id))
			.filter(Column::MediaId.eq(media_id))
			.filter(Column::CompletedAt.gt(cutoff))
			.order_by_desc(Column::CompletedAt)
			.one(conn)
			.await
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests::common::*;
	use pretty_assertions::assert_eq;

	#[test]
	fn test_find_finished_in_series() {
		let user = get_default_user();
		let select = Entity::find_finished_in_series(
			&user,
			Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap(),
		);
		let stmt_str = select_no_cols_to_string(select);
		assert_eq!(
			stmt_str,
			r#"SELECT   FROM "finished_reading_sessions" INNER JOIN "media" ON "finished_reading_sessions"."media_id" = "media"."id" WHERE "media"."series_id" = '123e4567-e89b-12d3-a456-426614174000' AND "finished_reading_sessions"."user_id" = '0ad39398-ce6a-4bcc-b044-719163a07c53'"#.to_string()
		);
	}
}
