use serde::{Deserialize, Serialize};
use super::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProductOrder {
    pub id: Option<String>,
    pub href: Option<String>,
    pub external_id: Option<String>,
    pub priority: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub state: Option<String>,
    pub order_date: Option<String>,
    pub completion_date: Option<String>,
    pub requested_start_date: Option<String>,
    pub requested_completion_date: Option<String>,
    pub expected_completion_date: Option<String>,
    pub notification_contact: Option<String>,
    pub note: Option<Vec<Note>>,
    pub related_party: Option<Vec<RelatedParty>>,
    pub channel: Option<Vec<ChannelRef>>,
    pub product_order_item: Option<Vec<ProductOrderItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProductOrderItem {
    pub id: Option<String>,
    pub action: Option<OrderItemAction>,
    pub state: Option<String>,
    pub quantity: Option<i32>,
    pub product_offering: Option<ProductOfferingRef>,
    pub product: Option<ProductRefOrValue>,
    pub product_order_item: Option<Vec<ProductOrderItemRef>>,
    pub product_order_item_relationship: Option<Vec<OrderItemRelationship>>,
    pub item_price: Option<Vec<OrderPrice>>,
    pub item_term: Option<Vec<OrderTerm>>,
    pub appointment: Option<serde_json::Value>,
    pub billing_account: Option<BillingAccountRef>,
    pub payment: Option<Vec<PaymentRef>>,
    pub qualification: Option<Vec<QualificationRef>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderItemAction {
    Add,
    Modify,
    Delete,
    NoChange,
}

impl Default for OrderItemAction {
    fn default() -> Self {
        Self::Add
    }
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
pub struct ProductRefOrValue {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_bundle: Option<bool>,
    pub product_characteristic: Option<Vec<Characteristic>>,
    pub product_specification: Option<ProductSpecRef>,
    pub related_party: Option<Vec<RelatedParty>>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProductSpecRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProductOrderItemRef {
    pub id: Option<String>,
    pub href: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrderItemRelationship {
    pub id: Option<String>,
    pub relationship_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrderPrice {
    pub description: Option<String>,
    pub name: Option<String>,
    pub price_type: Option<String>,
    pub recurring_charge_period: Option<String>,
    pub unit_of_measure: Option<String>,
    pub price: Option<Money>,
    pub price_alteration: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrderTerm {
    pub description: Option<String>,
    pub name: Option<String>,
    pub duration: Option<Quantity>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChannelRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BillingAccountRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QualificationRef {
    pub id: Option<String>,
    pub href: Option<String>,
}

// TMF641 Service Order types

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceOrder {
    pub id: Option<String>,
    pub href: Option<String>,
    pub external_id: Option<String>,
    pub priority: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub state: Option<String>,
    pub order_date: Option<String>,
    pub completion_date: Option<String>,
    pub requested_start_date: Option<String>,
    pub requested_completion_date: Option<String>,
    pub expected_completion_date: Option<String>,
    pub start_date: Option<String>,
    pub notification_contact: Option<String>,
    pub note: Option<Vec<Note>>,
    pub related_party: Option<Vec<RelatedParty>>,
    pub service_order_item: Option<Vec<ServiceOrderItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceOrderItem {
    pub id: Option<String>,
    pub action: Option<OrderItemAction>,
    pub state: Option<String>,
    pub service: Option<ServiceRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub state: Option<String>,
    pub service_type: Option<String>,
    pub service_characteristic: Option<Vec<Characteristic>>,
    pub service_specification: Option<ServiceSpecRef>,
    pub supporting_service: Option<Vec<ServiceRefSimple>>,
    pub supporting_resource: Option<Vec<ResourceRefSimple>>,
    pub related_party: Option<Vec<RelatedParty>>,
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
