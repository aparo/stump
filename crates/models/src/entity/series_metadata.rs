use crate::shared::{
	ordering::{OrderBy, OrderDirection},
	series_metadata::CollectedItems,
};
use async_graphql::SimpleObject;
use filter_gen::Ordering;
use sea_orm::{entity::prelude::*, QueryOrder};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject, Ordering)]
#[graphql(name = "SeriesMetadataModel")]
#[sea_orm(table_name = "series_metadata")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
	pub series_id: Uuid,
	/// Age rating of the series
	pub age_rating: Option<i32>,
	#[sea_orm(column_type = "Text", nullable)]
	#[graphql(skip)]
	pub characters: Option<String>,
	/// Booktype of the series (Print, OneShot, TPB or GN)
	pub booktype: Option<String>,
	/// TPB or GN may collect various single issues/series. This will list them here
	#[sea_orm(column_type = "Json", nullable)]
	#[graphql(skip)]
	pub collects: Option<CollectedItems>,
	/// ComicVine comicid
	pub comicid: Option<i32>,
	/// Image URL pointing to CV image of series cover (usually issue #1)
	pub comic_image: Option<String>,
	/// Description (summary) with line breaks, carriage returns, etc.
	pub description_formatted: Option<String>,
	#[graphql(skip)]
	pub genres: Option<String>,
	/// Name of imprint while under publisher
	pub imprint: Option<String>,
	#[graphql(skip)]
	pub links: Option<String>,
	/// Type of series (e.g. "comicSeries")
	pub meta_type: Option<String>,
	/// Start and end of the series in "Month Year - Month Year" format. If series status is Continuing, the end value is "Present"
	pub publication_run: Option<String>,
	/// Publisher name
	pub publisher: Option<String>,
	/// Either "Continuing" or "Ended"
	pub status: Option<String>,
	/// Description taken from source (un-edited) with no line breaks, carriage returns, etc.
	/// Stump calls this 'summary' to align with other models, but is derived from 'description_text' in series.json
	pub summary: Option<String>,
	/// Title of series
	pub title: Option<String>,
	/// Total issues in the series up until this point in time
	pub total_issues: Option<i32>,
	/// Volume of the series in relation to other titles (this can be either numerical or the series year)
	pub volume: Option<i32>,
	#[graphql(skip)]
	pub writers: Option<String>,
	/// Year the series started (publication start)
	pub year: Option<i32>,
	/// The external metadata provider that supplied this metadata (e.g., "HARDCOVER")
	#[sea_orm(column_type = "Text", nullable)]
	pub metadata_source: Option<String>,
	/// The external ID on the metadata provider's system
	#[sea_orm(column_type = "Text", nullable)]
	pub metadata_external_id: Option<String>,
	/// JSON array of MetadataField enum values that are locked from being overwritten by fetch features
	#[sea_orm(column_type = "Json", nullable)]
	#[graphql(skip)]
	pub locked_fields: Option<serde_json::Value>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::series::Entity",
		from = "Column::SeriesId",
		to = "super::series::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	Series,
}

impl Related<super::series::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Series.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
