use std::collections::HashMap;
use crate::schema::*;

pub fn schemas() -> HashMap<String, ResourceSchema> {
    let mut m = HashMap::new();

    m.insert("service".into(), ResourceSchema {
        name: "service".into(),
        plural_label: "Services".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col("description", "Description", ColumnType::String),
            col_sort("state", "State", ColumnType::Badge),
            col("category", "Category", ColumnType::String),
            col("isBundle", "Bundle", ColumnType::Boolean),
            col("isServiceEnabled", "Enabled", ColumnType::Boolean),
            col("serviceType", "Type", ColumnType::String),
            col_sort("startDate", "Start Date", ColumnType::Date),
        ],
        filters: vec![
            filter_text("name", "Service Name"),
            filter_select("state", "State", &[
                "feasibilityChecked", "designed", "reserved",
                "inactive", "active", "terminated",
            ]),
            filter_text("category", "Category"),
            filter_text("serviceType", "Service Type"),
            filter_bool("isBundle", "Bundle"),
            filter_bool("isServiceEnabled", "Enabled"),
            filter_text("serviceSpecification.name", "Service Specification"),
            filter_text("relatedParty.name", "Related Party"),
            filter_date("startDate", "Start Date"),
        ],
        default_sort: Some("-startDate".into()),
    });

    m
}
