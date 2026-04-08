use serde::{Deserialize, Serialize};
use super::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub status: Option<String>,
    pub status_reason: Option<String>,
    pub valid_for: Option<TimePeriod>,
    pub engaged_party: Option<RelatedParty>,
    pub account: Option<Vec<AccountRef>>,
    pub contact_medium: Option<Vec<ContactMedium>>,
    pub characteristic: Option<Vec<Characteristic>>,
    pub related_party: Option<Vec<RelatedParty>>,
    pub credit_profile: Option<Vec<CreditProfile>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreditProfile {
    pub credit_profile_date: Option<String>,
    pub credit_risk_rating: Option<i32>,
    pub credit_score: Option<i32>,
    pub valid_for: Option<TimePeriod>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Individual {
    pub id: Option<String>,
    pub href: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub middle_name: Option<String>,
    pub full_name: Option<String>,
    pub formatted_name: Option<String>,
    pub gender: Option<String>,
    pub place_of_birth: Option<String>,
    pub country_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub birth_date: Option<String>,
    pub death_date: Option<String>,
    pub marital_status: Option<String>,
    pub title: Option<String>,
    pub status: Option<String>,
    pub contact_medium: Option<Vec<ContactMedium>>,
    pub external_reference: Option<Vec<ExternalReference>>,
    pub individual_identification: Option<Vec<IndividualIdentification>>,
    pub party_characteristic: Option<Vec<Characteristic>>,
    pub related_party: Option<Vec<RelatedParty>>,
    pub tax_exemption_certificate: Option<Vec<TaxExemptionCertificate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct IndividualIdentification {
    pub identification_id: Option<String>,
    pub identification_type: Option<String>,
    pub issuing_authority: Option<String>,
    pub issuing_date: Option<String>,
    pub valid_for: Option<TimePeriod>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub trading_name: Option<String>,
    pub name_type: Option<String>,
    pub organization_type: Option<String>,
    pub exists_during: Option<TimePeriod>,
    pub status: Option<String>,
    pub is_head_office: Option<bool>,
    pub is_legal_entity: Option<bool>,
    pub contact_medium: Option<Vec<ContactMedium>>,
    pub external_reference: Option<Vec<ExternalReference>>,
    pub organization_identification: Option<Vec<OrganizationIdentification>>,
    pub organization_child_relationship: Option<Vec<OrgRelationship>>,
    pub organization_parent_relationship: Option<OrgRelationship>,
    pub party_characteristic: Option<Vec<Characteristic>>,
    pub related_party: Option<Vec<RelatedParty>>,
    pub tax_exemption_certificate: Option<Vec<TaxExemptionCertificate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationIdentification {
    pub identification_id: Option<String>,
    pub identification_type: Option<String>,
    pub issuing_authority: Option<String>,
    pub issuing_date: Option<String>,
    pub valid_for: Option<TimePeriod>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrgRelationship {
    pub organization_id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub relationship_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TaxExemptionCertificate {
    pub id: Option<String>,
    pub certificate_number: Option<String>,
    pub issuing_jurisdiction: Option<String>,
    pub reason: Option<String>,
    pub valid_for: Option<TimePeriod>,
}
