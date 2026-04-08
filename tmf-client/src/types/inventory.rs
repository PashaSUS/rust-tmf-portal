use serde::{Deserialize, Serialize};
use super::common::*;

// TMF638 Service Inventory

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub state: Option<String>,
    pub service_type: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub start_mode: Option<String>,
    pub is_stateful: Option<bool>,
    pub is_service_enabled: Option<bool>,
    pub has_started: Option<bool>,
    pub service_date: Option<String>,
    pub service_characteristic: Option<Vec<Characteristic>>,
    pub service_relationship: Option<Vec<ServiceRelationship>>,
    pub related_party: Option<Vec<RelatedParty>>,
    pub note: Option<Vec<Note>>,
    pub place: Option<Vec<RelatedPlaceRefOrValue>>,
    pub service_specification: Option<ServiceSpecRef>,
    pub supporting_service: Option<Vec<ServiceRefSimple>>,
    pub supporting_resource: Option<Vec<ResourceRefSimple>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRelationship {
    pub id: Option<String>,
    pub href: Option<String>,
    pub relationship_type: Option<String>,
    pub service: Option<ServiceRefSimple>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceSpecRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRefSimple {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRefSimple {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
}

// TMF639 Resource Inventory

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub state: Option<String>,
    pub category: Option<String>,
    pub resource_status: Option<String>,
    pub usage_state: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub resource_characteristic: Option<Vec<Characteristic>>,
    pub resource_relationship: Option<Vec<ResourceRelationship>>,
    pub related_party: Option<Vec<RelatedParty>>,
    pub note: Option<Vec<Note>>,
    pub place: Option<RelatedPlaceRefOrValue>,
    pub resource_specification: Option<ResourceSpecRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRelationship {
    pub id: Option<String>,
    pub href: Option<String>,
    pub relationship_type: Option<String>,
    pub resource: Option<ResourceRefSimple>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceSpecRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
}
