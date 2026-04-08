use serde::{Deserialize, Serialize};
use super::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Catalog {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub catalog_type: Option<String>,
    pub version: Option<String>,
    pub lifecycle_status: Option<String>,
    pub valid_for: Option<TimePeriod>,
    pub last_update: Option<String>,
    pub related_party: Option<Vec<RelatedParty>>,
    pub category: Option<Vec<CategoryRef>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CategoryRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub lifecycle_status: Option<String>,
    pub is_root: Option<bool>,
    pub valid_for: Option<TimePeriod>,
    pub last_update: Option<String>,
    pub parent_id: Option<String>,
    pub sub_category: Option<Vec<CategoryRef>>,
    pub product_offering: Option<Vec<ProductOfferingRef>>,
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
pub struct ProductOffering {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub lifecycle_status: Option<String>,
    pub is_bundle: Option<bool>,
    pub is_sellable: Option<bool>,
    pub status_reason: Option<String>,
    pub valid_for: Option<TimePeriod>,
    pub last_update: Option<String>,
    pub category: Option<Vec<CategoryRef>>,
    pub product_offering_price: Option<Vec<ProductOfferingPriceRef>>,
    pub product_specification: Option<ProductSpecificationRef>,
    pub related_party: Option<Vec<RelatedParty>>,
    pub attachment: Option<Vec<AttachmentRefOrValue>>,
    pub bundled_product_offering: Option<Vec<BundledProductOffering>>,
    pub prod_spec_char_value_use: Option<Vec<ProdSpecCharValueUse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProductOfferingPriceRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProductSpecificationRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BundledProductOffering {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub lifecycle_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProdSpecCharValueUse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub value_type: Option<String>,
    pub min_cardinality: Option<i32>,
    pub max_cardinality: Option<i32>,
    pub product_spec_char_value: Option<Vec<ProductSpecCharacteristicValue>>,
    pub product_specification: Option<ProductSpecificationRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProductSpecification {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub brand: Option<String>,
    pub version: Option<String>,
    pub lifecycle_status: Option<String>,
    pub is_bundle: Option<bool>,
    pub product_number: Option<String>,
    pub valid_for: Option<TimePeriod>,
    pub last_update: Option<String>,
    pub product_spec_characteristic: Option<Vec<ProductSpecCharacteristic>>,
    pub related_party: Option<Vec<RelatedParty>>,
    pub attachment: Option<Vec<AttachmentRefOrValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProductSpecCharacteristic {
    pub name: Option<String>,
    pub description: Option<String>,
    pub value_type: Option<String>,
    pub configurable: Option<bool>,
    pub min_cardinality: Option<i32>,
    pub max_cardinality: Option<i32>,
    pub is_unique: Option<bool>,
    pub regex: Option<String>,
    pub extensible: Option<bool>,
    pub valid_for: Option<TimePeriod>,
    pub product_spec_characteristic_value: Option<Vec<ProductSpecCharacteristicValue>>,
    pub product_spec_char_relationship: Option<Vec<ProductSpecCharRelationship>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProductSpecCharacteristicValue {
    pub is_default: Option<bool>,
    pub range_interval: Option<String>,
    pub regex: Option<String>,
    pub unit_of_measure: Option<String>,
    pub value_from: Option<String>,
    pub value_to: Option<String>,
    pub value_type: Option<String>,
    pub valid_for: Option<TimePeriod>,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProductSpecCharRelationship {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub relationship_type: Option<String>,
    pub valid_for: Option<TimePeriod>,
}

// TMF633 Service Catalog types

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceSpecification {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub lifecycle_status: Option<String>,
    pub is_bundle: Option<bool>,
    pub valid_for: Option<TimePeriod>,
    pub last_update: Option<String>,
    pub spec_char: Option<Vec<ServiceSpecCharacteristic>>,
    pub related_party: Option<Vec<RelatedParty>>,
    pub attachment: Option<Vec<AttachmentRefOrValue>>,
    pub service_spec_relationship: Option<Vec<ServiceSpecRelationship>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceSpecCharacteristic {
    pub name: Option<String>,
    pub description: Option<String>,
    pub value_type: Option<String>,
    pub configurable: Option<bool>,
    pub min_cardinality: Option<i32>,
    pub max_cardinality: Option<i32>,
    pub is_unique: Option<bool>,
    pub extensible: Option<bool>,
    pub valid_for: Option<TimePeriod>,
    pub service_spec_char_value: Option<Vec<ProductSpecCharacteristicValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceSpecRelationship {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub relationship_type: Option<String>,
    pub role: Option<String>,
    pub valid_for: Option<TimePeriod>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCategory {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub lifecycle_status: Option<String>,
    pub is_root: Option<bool>,
    pub valid_for: Option<TimePeriod>,
    pub last_update: Option<String>,
    pub category: Option<Vec<CategoryRef>>,
    pub service_candidate: Option<Vec<ServiceCandidateRef>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCandidateRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
}
