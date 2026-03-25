use async_graphql::{Context, Object, Result, ID};
use models::entity::{bookmark, media, media_annotation};
use uuid::Uuid;

use crate::{
	data::{AuthContext, CoreContext},
	object::{bookmark::Bookmark, epub::Epub, media_annotation::MediaAnnotation},
};

#[derive(Default)]
pub struct EpubQuery;

#[Object]
impl EpubQuery {
	/// Get a single epub by its media ID
	async fn epub_by_id(&self, ctx: &Context<'_>, id: ID) -> Result<Epub> {
		let AuthContext { user, .. } = ctx.data::<AuthContext>()?;
		let conn = ctx.data::<CoreContext>()?.conn.as_ref();

		let model = media::Entity::find_media_ids_for_user(id.to_string(), user)
			.into_model::<media::MediaIdentSelect>()
			.one(conn)
			.await?
			.ok_or("Media not found")?;

		Epub::try_from(model)
	}

	/// Get all bookmarks for a single epub by its media ID
	async fn bookmarks_by_media_id(
		&self,
		ctx: &Context<'_>,
		id: ID,
	) -> Result<Vec<Bookmark>> {
		let AuthContext { user, .. } = ctx.data::<AuthContext>()?;
		let conn = ctx.data::<CoreContext>()?.conn.as_ref();
		let id = Uuid::parse_str(id.as_str())?;
		Ok(bookmark::Entity::find_for_user_and_media_id(user, id)
			.into_model::<bookmark::Model>()
			.all(conn)
			.await?
			.into_iter()
			.map(Bookmark::from)
			.collect())
	}

	/// Get all annotations (highlights/notes) for a single book
	async fn annotations_by_media_id(
		&self,
		ctx: &Context<'_>,
		id: ID,
	) -> Result<Vec<MediaAnnotation>> {
		let AuthContext { user, .. } = ctx.data::<AuthContext>()?;
		let conn = ctx.data::<CoreContext>()?.conn.as_ref();
		let id = Uuid::parse_str(id.as_str())?;

		Ok(
			media_annotation::Model::find_for_user_and_media_id(user.id, id, conn)
				.await?
				.into_iter()
				.map(MediaAnnotation::from)
				.collect(),
		)
	}
}
