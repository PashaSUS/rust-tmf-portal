use serde::{Deserialize, Serialize};
use super::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BillingAccount {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub state: Option<String>,
    pub account_type: Option<String>,
    pub payment_status: Option<String>,
    pub rating_type: Option<String>,
    pub credit_limit: Option<Money>,
    pub last_modified: Option<String>,
    pub related_party: Option<Vec<RelatedParty>>,
    pub contact: Option<Vec<ContactMedium>>,
    pub account_balance: Option<Vec<AccountBalance>>,
    pub account_relationship: Option<Vec<AccountRelationship>>,
    pub financial_account: Option<FinancialAccountRef>,
    pub default_payment_method: Option<PaymentMethodRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FinancialAccount {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub state: Option<String>,
    pub account_type: Option<String>,
    pub last_modified: Option<String>,
    pub related_party: Option<Vec<RelatedParty>>,
    pub contact: Option<Vec<ContactMedium>>,
    pub account_balance: Option<Vec<AccountBalance>>,
    pub account_relationship: Option<Vec<AccountRelationship>>,
    pub tax_exemption: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    pub balance_type: Option<String>,
    pub amount: Option<Money>,
    pub valid_for: Option<TimePeriod>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountRelationship {
    pub relationship_type: Option<String>,
    pub account: Option<AccountRef>,
    pub valid_for: Option<TimePeriod>,
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
pub struct FinancialAccountRef {
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
