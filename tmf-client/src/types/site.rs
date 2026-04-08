use serde::{Deserialize, Serialize};
use super::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeographicSite {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub code: Option<String>,
    pub status: Option<String>,
    pub related_party: Option<Vec<RelatedParty>>,
    pub place: Option<Vec<RelatedPlaceRefOrValue>>,
    pub site_relationship: Option<Vec<SiteRelationship>>,
    pub external_reference: Option<Vec<ExternalReference>>,
    pub calendar: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SiteRelationship {
    pub id: Option<String>,
    pub href: Option<String>,
    pub relationship_type: Option<String>,
    pub role: Option<String>,
    pub valid_for: Option<TimePeriod>,
}
