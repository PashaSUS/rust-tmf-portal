use serde::{Deserialize, Serialize};
use super::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub status_date: Option<String>,
    pub payment_date: Option<String>,
    pub correlation_id: Option<String>,
    pub amount: Option<Money>,
    pub tax_amount: Option<Money>,
    pub total_amount: Option<Money>,
    pub channel: Option<ChannelRef>,
    pub account: Option<AccountRef>,
    pub payment_item: Option<Vec<PaymentItem>>,
    pub payment_method: Option<PaymentMethodRef>,
    pub payer: Option<RelatedParty>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaymentItem {
    pub id: Option<String>,
    pub amount: Option<Money>,
    pub tax_amount: Option<Money>,
    pub total_amount: Option<Money>,
    pub payment_item_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChannelRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodRef {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
}
