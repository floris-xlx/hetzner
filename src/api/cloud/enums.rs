use serde::{Deserialize, Serialize};

// Generated from hetzner-cloud-openapi.json

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServerStatus {
    Running,
    Initializing,
    Starting,
    Stopping,
    Off,
    Deleting,
    Migrating,
    Rebuilding,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerSort {
    #[serde(rename = "id")]
    IdAsc,
    #[serde(rename = "id:asc")]
    IdAscExplicit,
    #[serde(rename = "id:desc")]
    IdDesc,
    #[serde(rename = "name")]
    NameAsc,
    #[serde(rename = "name:asc")]
    NameAscExplicit,
    #[serde(rename = "name:desc")]
    NameDesc,
    #[serde(rename = "created")]
    CreatedAsc,
    #[serde(rename = "created:asc")]
    CreatedAscExplicit,
    #[serde(rename = "created:desc")]
    CreatedDesc,
}

impl ServerSort {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdAsc => "id",
            Self::IdAscExplicit => "id:asc",
            Self::IdDesc => "id:desc",
            Self::NameAsc => "name",
            Self::NameAscExplicit => "name:asc",
            Self::NameDesc => "name:desc",
            Self::CreatedAsc => "created",
            Self::CreatedAscExplicit => "created:asc",
            Self::CreatedDesc => "created:desc",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionStatus {
    Running,
    Success,
    Error,
}

impl ServerStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Initializing => "initializing",
            Self::Starting => "starting",
            Self::Stopping => "stopping",
            Self::Off => "off",
            Self::Deleting => "deleting",
            Self::Migrating => "migrating",
            Self::Rebuilding => "rebuilding",
            Self::Unknown => "unknown",
        }
    }
}
