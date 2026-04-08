use serde::{Deserialize, Serialize};
use super::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Agreement {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub agreement_type: Option<String>,
    pub version: Option<String>,
    pub status: Option<String>,
    pub status_reason: Option<String>,
    pub initial_date: Option<String>,
    pub completion_date: Option<String>,
    pub statement_of_intent: Option<String>,
    pub valid_for: Option<TimePeriod>,
    pub agreement_period: Option<TimePeriod>,
    pub characteristic: Option<Vec<Characteristic>>,
    pub related_party: Option<Vec<RelatedParty>>,
    pub agreement_item: Option<Vec<AgreementItem>>,
    pub agreement_authorization: Option<Vec<AgreementAuthorization>>,
    pub engaged_party_role: Option<Vec<RelatedParty>>,
    pub associated_agreement: Option<Vec<AgreementRef>>,
    pub agreement_specification: Option<AgreementSpecRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgreementItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_offering: Option<Vec<ProductOfferingRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_or_condition: Option<Vec<AgreementTermOrCondition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProductOfferingRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgreementTermOrCondition {
    pub id: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub term_type: Option<String>,
    pub valid_for: Option<TimePeriod>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgreementAuthorization {
    pub date: Option<String>,
    pub signature_representation: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgreementRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgreementSpecRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
}
