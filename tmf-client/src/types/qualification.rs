use serde::{Deserialize, Serialize};
use super::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceQualification {
    pub id: Option<String>,
    pub href: Option<String>,
    pub description: Option<String>,
    pub state: Option<String>,
    pub external_id: Option<String>,
    pub effective_qualification_date: Option<String>,
    pub estimated_response_date: Option<String>,
    pub expected_qualification_date: Option<String>,
    pub expiration_date: Option<String>,
    pub qualification_result: Option<String>,
    pub service_qualification_item: Option<Vec<ServiceQualificationItem>>,
    pub related_party: Option<Vec<RelatedParty>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceQualificationItem {
    pub id: Option<String>,
    pub state: Option<String>,
    pub expected_activation_date: Option<String>,
    pub qualification_result: Option<String>,
    pub termination_error: Option<Vec<serde_json::Value>>,
    pub service: Option<QualificationServiceRef>,
    pub eligible_service_specification: Option<Vec<ServiceSpecRef>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QualificationServiceRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub service_type: Option<String>,
    pub service_characteristic: Option<Vec<Characteristic>>,
    pub service_specification: Option<ServiceSpecRef>,
    pub place: Option<Vec<RelatedPlaceRefOrValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceSpecRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
}
