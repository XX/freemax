use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ParticipantId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CircleId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circle {
    pub id: CircleId,
    pub name: String,
    pub description: String,
    pub owner: ParticipantId,
    /// Условия вхождения (формат правил — см. `docs/domain.md`).
    pub rules: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Owner,
    Admin,
    Member,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MembershipStatus {
    Pending,
    Active,
    Rejected,
    Left,
    Banned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Membership {
    pub circle: CircleId,
    pub participant: ParticipantId,
    pub role: Role,
    pub status: MembershipStatus,
    pub joined_at: DateTime<Utc>,
}
