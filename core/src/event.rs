use async_graphql::{SimpleObject, Union};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::job::{CoreJobOutput, JobUpdate};

#[derive(Clone, Serialize, Deserialize, Debug, SimpleObject)]
pub struct JobStarted {
	pub id: Uuid,
}

#[derive(Clone, Serialize, Deserialize, Debug, SimpleObject)]
pub struct JobOutput {
	pub id: Uuid,
	pub output: CoreJobOutput,
}

#[derive(Clone, Serialize, Deserialize, Debug, SimpleObject)]
pub struct DiscoveredMissingLibrary {
	pub id: Uuid,
}

#[derive(Clone, Serialize, Deserialize, Debug, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct CreatedMedia {
	pub id: Uuid,
	pub series_id: Uuid,
	pub library_id: Uuid,
}

#[derive(Clone, Serialize, Deserialize, Debug, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct CreatedManySeries {
	pub count: u64,
	pub library_id: Uuid,
}

#[derive(Clone, Serialize, Deserialize, Debug, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct CreatedOrUpdatedManyMedia {
	pub count: u64,
	pub series_id: Uuid,
	pub library_id: Uuid,
}

/// An event that is emitted by the core and consumed by a client
#[derive(Clone, Serialize, Deserialize, Debug, Union)]
#[serde(tag = "__typename")]
pub enum CoreEvent {
	JobStarted(JobStarted),
	JobUpdate(JobUpdate),
	JobOutput(JobOutput),
	DiscoveredMissingLibrary(DiscoveredMissingLibrary),
	CreatedMedia(CreatedMedia),
	CreatedManySeries(CreatedManySeries),
	CreatedOrUpdatedManyMedia(CreatedOrUpdatedManyMedia),
}
