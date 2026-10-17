use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{CircleId, ParticipantId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ListingId(pub Uuid);

/// Потребность или предложение.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ListingKind {
    Need,
    Offer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ListingStatus {
    Active,
    Reserved,
    Closed,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    pub id: ListingId,
    pub kind: ListingKind,
    pub author: ParticipantId,
    pub title: String,
    pub description: String,
    pub category: Option<String>,
    /// Круги, в которых опубликовано объявление (минимум один).
    pub circles: Vec<CircleId>,
    pub status: ListingStatus,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    Proposed,
    Accepted,
    Declined,
    Completed,
    Cancelled,
}

/// Отклик участника на объявление.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub listing: ListingId,
    pub responder: ParticipantId,
    pub message: String,
    pub status: ResponseStatus,
    pub created_at: DateTime<Utc>,
}
