use std::collections::HashMap;
use crate::schema::*;

pub fn schemas() -> HashMap<String, ResourceSchema> {
    let mut m = HashMap::new();

    m.insert("billingAccount".into(), ResourceSchema {
        name: "billingAccount".into(),
        plural_label: "Billing Accounts".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col("description", "Description", ColumnType::String),
            col_sort("state", "State", ColumnType::Badge),
            col("accountType", "Type", ColumnType::String),
            col_sort("lastModified", "Last Modified", ColumnType::Date),
        ],
        filters: vec![
            filter_text("name", "Account Name"),
            filter_select("state", "State", &["defined", "active", "suspended", "closed"]),
            filter_text("accountType", "Account Type"),
            filter_text("relatedParty.name", "Related Party"),
        ],
        default_sort: Some("name".into()),
    });

    m.insert("financialAccount".into(), ResourceSchema {
        name: "financialAccount".into(),
        plural_label: "Financial Accounts".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col_sort("state", "State", ColumnType::Badge),
            col("accountType", "Type", ColumnType::String),
            col("creditLimit", "Credit Limit", ColumnType::Number),
            col_sort("lastModified", "Last Modified", ColumnType::Date),
        ],
        filters: vec![
            filter_text("name", "Account Name"),
            filter_select("state", "State", &["defined", "active", "suspended", "closed"]),
            filter_text("accountType", "Account Type"),
            filter_text("relatedParty.name", "Related Party"),
        ],
        default_sort: Some("name".into()),
    });

    m.insert("partyAccount".into(), ResourceSchema {
        name: "partyAccount".into(),
        plural_label: "Party Accounts".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col_sort("state", "State", ColumnType::Badge),
            col("accountType", "Type", ColumnType::String),
            col_sort("lastModified", "Last Modified", ColumnType::Date),
        ],
        filters: vec![
            filter_text("name", "Account Name"),
            filter_select("state", "State", &["defined", "active", "suspended", "closed"]),
            filter_text("relatedParty.name", "Related Party"),
        ],
        default_sort: Some("name".into()),
    });

    m.insert("settlementAccount".into(), ResourceSchema {
        name: "settlementAccount".into(),
        plural_label: "Settlement Accounts".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col_sort("state", "State", ColumnType::Badge),
            col("accountType", "Type", ColumnType::String),
            col_sort("lastModified", "Last Modified", ColumnType::Date),
        ],
        filters: vec![
            filter_text("name", "Account Name"),
            filter_select("state", "State", &["defined", "active", "suspended", "closed"]),
            filter_text("relatedParty.name", "Related Party"),
        ],
        default_sort: Some("name".into()),
    });

    m
}
