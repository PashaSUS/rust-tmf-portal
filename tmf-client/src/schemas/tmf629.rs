use std::collections::HashMap;
use crate::schema::*;

pub fn schemas() -> HashMap<String, ResourceSchema> {
    let mut m = HashMap::new();

    m.insert("customer".into(), ResourceSchema {
        name: "customer".into(),
        plural_label: "Customers".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col_sort("status", "Status", ColumnType::Badge),
            col("statusReason", "Status Reason", ColumnType::String),
            col_sort("customerRank", "Rank", ColumnType::String),
        ],
        filters: vec![
            filter_text("name", "Customer Name"),
            filter_select("status", "Status", &["approved", "initialized", "validated", "closed"]),
            filter_text("contactMedium.emailAddress", "Email"),
            filter_text("relatedParty.name", "Related Party"),
        ],
        default_sort: Some("name".into()),
    });

    m
}
