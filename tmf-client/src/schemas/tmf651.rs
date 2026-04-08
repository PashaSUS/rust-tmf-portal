use std::collections::HashMap;
use crate::schema::*;

pub fn schemas() -> HashMap<String, ResourceSchema> {
    let mut m = HashMap::new();

    m.insert("agreement".into(), ResourceSchema {
        name: "agreement".into(),
        plural_label: "Agreements".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col("description", "Description", ColumnType::String),
            col("agreementType", "Type", ColumnType::String),
            col_sort("status", "Status", ColumnType::Badge),
            col("version", "Version", ColumnType::String),
            col_sort("completionDate.startDateTime", "Start", ColumnType::Date),
        ],
        filters: vec![
            filter_text("name", "Agreement Name"),
            filter_text("agreementType", "Agreement Type"),
            filter_select("status", "Status", &["initialized", "active", "closed"]),
            filter_text("engagedPartyRole.name", "Party"),
        ],
        default_sort: Some("name".into()),
    });

    m.insert("agreementSpecification".into(), ResourceSchema {
        name: "agreementSpecification".into(),
        plural_label: "Agreement Specifications".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col("description", "Description", ColumnType::String),
            col_sort("lifecycleStatus", "Status", ColumnType::Badge),
            col("version", "Version", ColumnType::String),
            col_sort("lastUpdate", "Last Update", ColumnType::Date),
        ],
        filters: vec![
            filter_text("name", "Spec Name"),
            filter_select("lifecycleStatus", "Status", &["Active", "Launched", "Retired", "Obsolete"]),
            filter_bool("isBundle", "Bundle"),
        ],
        default_sort: Some("-lastUpdate".into()),
    });

    m
}
