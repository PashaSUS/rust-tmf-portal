use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum ColumnType {
    String,
    Number,
    Date,
    Boolean,
    Badge,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum FilterType {
    Text,
    Select,
    DateRange,
    Boolean,
    Number,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ColumnDef {
    pub key: String,
    pub label: String,
    pub col_type: ColumnType,
    #[serde(default)]
    pub sortable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FilterDef {
    pub key: String,
    pub label: String,
    pub filter_type: FilterType,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceSchema {
    pub name: String,
    pub plural_label: String,
    pub columns: Vec<ColumnDef>,
    pub filters: Vec<FilterDef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_sort: Option<String>,
}

// Builder helpers

pub fn col(key: &str, label: &str, ct: ColumnType) -> ColumnDef {
    ColumnDef { key: key.into(), label: label.into(), col_type: ct, sortable: false }
}

pub fn col_sort(key: &str, label: &str, ct: ColumnType) -> ColumnDef {
    ColumnDef { key: key.into(), label: label.into(), col_type: ct, sortable: true }
}

pub fn filter_text(key: &str, label: &str) -> FilterDef {
    FilterDef {
        key: key.into(), label: label.into(),
        filter_type: FilterType::Text, options: vec![], placeholder: None,
    }
}

pub fn filter_select(key: &str, label: &str, opts: &[&str]) -> FilterDef {
    FilterDef {
        key: key.into(), label: label.into(),
        filter_type: FilterType::Select,
        options: opts.iter().map(|s| s.to_string()).collect(),
        placeholder: None,
    }
}

pub fn filter_date(key: &str, label: &str) -> FilterDef {
    FilterDef {
        key: key.into(), label: label.into(),
        filter_type: FilterType::DateRange, options: vec![], placeholder: None,
    }
}

pub fn filter_bool(key: &str, label: &str) -> FilterDef {
    FilterDef {
        key: key.into(), label: label.into(),
        filter_type: FilterType::Boolean, options: vec![], placeholder: None,
    }
}
