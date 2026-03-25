use async_graphql::InputObject;
use models::{entity::reading_list, shared::enums::EntityVisibility};
use sea_orm::ActiveValue::Set;
use uuid::Uuid;

#[derive(InputObject, Clone)]
pub struct ReadingListInput {
	pub id: Uuid,
	pub name: String,
	pub visibility: Option<EntityVisibility>,
	pub media_ids: Vec<Uuid>,
}

impl ReadingListInput {
	pub fn into_active_model(self, user_id: Uuid) -> reading_list::ActiveModel {
		reading_list::ActiveModel {
			id: Set(self.id),
			name: Set(self.name),
			updated_at: Set(chrono::Utc::now().into()),
			visibility: Set(self
				.visibility
				.unwrap_or_default()
				.to_string()
				.to_uppercase()),
			creating_user_id: Set(user_id),
			..Default::default()
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests::common::*;

	#[test]
	fn test_readinglistinput_into_activemodel_default() {
		let input = ReadingListInput {
			id: Uuid::new_v4(),
			name: "Some Name".to_string(),
			visibility: None,
			media_ids: vec![],
		};

		let user_id = Uuid::new_v4();
		let model = input.clone().into_active_model(user_id);

		assert_eq!(model.id.unwrap(), input.id);
		assert_eq!(model.name.unwrap(), "Some Name");
		assert!(is_close_to_now(model.updated_at.unwrap().into()));
		assert_eq!(model.visibility.unwrap(), "PRIVATE");
		assert_eq!(model.creating_user_id.unwrap(), user_id);
	}

	#[test]
	fn test_readinglistinput_into_activemodel_with_visibility() {
		let input_id = Uuid::new_v4();
		let input = ReadingListInput {
			id: input_id,
			name: "Some Name".to_string(),
			visibility: Some(EntityVisibility::Public), // Adjust as per your enum
			media_ids: vec![],
		};

		let user_id = Uuid::new_v4();
		let model = input.into_active_model(user_id);

		assert_eq!(model.id.unwrap(), input_id);
		assert_eq!(model.name.unwrap(), "Some Name");
		assert!(is_close_to_now(model.updated_at.unwrap().into()));
		assert_eq!(model.visibility.unwrap(), "PUBLIC");
		assert_eq!(model.creating_user_id.unwrap(), user_id);
	}
}
