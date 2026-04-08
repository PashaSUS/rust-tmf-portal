use std::collections::HashMap;
use crate::schema::*;

pub fn schemas() -> HashMap<String, ResourceSchema> {
    let mut m = HashMap::new();

    m.insert("resource".into(), ResourceSchema {
        name: "resource".into(),
        plural_label: "Resources".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col("description", "Description", ColumnType::String),
            col("category", "Category", ColumnType::String),
            col_sort("resourceStatus", "Status", ColumnType::Badge),
            col("resourceVersion", "Version", ColumnType::String),
            col_sort("startOperatingDate", "Start Date", ColumnType::Date),
        ],
        filters: vec![
            filter_text("name", "Resource Name"),
            filter_select("resourceStatus", "Status", &[
                "standby", "alarm", "available", "reserved",
                "unknown", "suspended",
            ]),
            filter_text("category", "Category"),
            filter_text("resourceSpecification.name", "Resource Specification"),
            filter_text("relatedParty.name", "Related Party"),
            filter_text("place.name", "Place"),
            filter_date("startOperatingDate", "Start Operating Date"),
        ],
        default_sort: Some("-startOperatingDate".into()),
    });

    m
}
