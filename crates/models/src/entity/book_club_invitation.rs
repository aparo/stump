use async_graphql::SimpleObject;
use sea_orm::{entity::prelude::*, prelude::async_trait::async_trait, ActiveValue};

use crate::{entity::user::AuthUser, shared::book_club::BookClubMemberRole};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[graphql(name = "BookClubInvitationModel")]
#[sea_orm(table_name = "book_club_invitations")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub role: BookClubMemberRole,
	pub user_id: Uuid,
	pub book_club_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::book_club::Entity",
		from = "Column::BookClubId",
		to = "super::book_club::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	BookClub,
	#[sea_orm(
		belongs_to = "super::user::Entity",
		from = "Column::UserId",
		to = "super::user::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	User,
}

impl Related<super::book_club::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::BookClub.def()
	}
}

impl Related<super::user::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::User.def()
	}
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
	async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
	where
		C: ConnectionTrait,
	{
		if insert && self.id.is_not_set() {
			self.id = ActiveValue::Set(Uuid::new_v4());
		}

		Ok(self)
	}
}

impl Entity {
	pub fn find_for_book_club_id(book_club_id: Uuid) -> Select<Entity> {
		Self::find().filter(Column::BookClubId.eq(book_club_id))
	}

	pub fn find_for_user(user: &AuthUser) -> Select<Entity> {
		Self::find().filter(Column::UserId.eq(user.id))
	}

	pub fn find_for_user_and_id(user: &AuthUser, id: Uuid) -> Select<Entity> {
		Self::find_by_id(id).filter(Column::UserId.eq(user.id))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests::common::*;
	use pretty_assertions::assert_eq;
	use sea_orm::{DatabaseBackend, MockDatabase, Set};
	use tokio_test;

	#[test]
	fn test_find_for_user_and_id() {
		let user = get_default_user();
		let select = Entity::find_for_user_and_id(
			&user,
			Uuid::parse_str("f35f3fb0-bb14-43e2-91b9-234b4503cff2").unwrap(),
		);
		assert_eq!(
			select_no_cols_to_string(select),
			r#"SELECT  FROM "book_club_invitations" WHERE "book_club_invitations"."id" = 'f35f3fb0-bb14-43e2-91b9-234b4503cff2' AND "book_club_invitations"."user_id" = '42'"#.to_string()
		);
	}

	#[test]
	fn test_find_for_user() {
		let user = get_default_user();
		let select = Entity::find_for_user(&user);
		assert_eq!(
			select_no_cols_to_string(select),
			r#"SELECT  FROM "book_club_invitations" WHERE "book_club_invitations"."user_id" = '0ad39398-ce6a-4bcc-b044-719163a07c53'"#.to_string()
		);
	}

	#[test]
	fn test_find_for_book_club_id() {
		let id = Uuid::parse_str("ba2f52af-0939-459b-85cc-263bf0855f44").unwrap();
		let select = Entity::find_for_book_club_id(id);
		assert_eq!(
			select_no_cols_to_string(select),
			r#"SELECT  FROM "book_club_invitations" WHERE "book_club_invitations"."book_club_id" = 'ba2f52af-0939-459b-85cc-263bf0855f44'"#.to_string()
		);
	}

	#[test]
	fn test_active_model() {
		let db = MockDatabase::new(DatabaseBackend::Sqlite);
		let conn = db.into_connection();

		let model = ActiveModel {
			id: Set(Uuid::parse_str("6f6d2e63-1532-4f85-9d3c-c19f9f915b38").unwrap()),
			role: Set(BookClubMemberRole::Member),
			user_id: Set(Uuid::parse_str("6f6d2e63-1532-4f85-9d3c-c19f9f915b38").unwrap()),
			book_club_id: Set(
				Uuid::parse_str("6f6d2e63-1532-4f85-9d3c-c19f9f915b38").unwrap()
			),
		};

		let model = tokio_test::block_on(model.before_save(&conn, true)).unwrap();
		assert!(!model.id.unwrap().is_nil());
	}
}
