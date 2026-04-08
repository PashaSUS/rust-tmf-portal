use std::collections::HashMap;
use crate::schema::*;

pub fn schemas() -> HashMap<String, ResourceSchema> {
    let mut m = HashMap::new();

    m.insert("individual".into(), ResourceSchema {
        name: "individual".into(),
        plural_label: "Individuals".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("givenName", "Given Name", ColumnType::String),
            col_sort("familyName", "Family Name", ColumnType::String),
            col("fullName", "Full Name", ColumnType::String),
            col("gender", "Gender", ColumnType::String),
            col_sort("birthDate", "Birth Date", ColumnType::Date),
            col("status", "Status", ColumnType::Badge),
        ],
        filters: vec![
            filter_text("givenName", "Given Name"),
            filter_text("familyName", "Family Name"),
            filter_text("fullName", "Full Name"),
            filter_select("gender", "Gender", &["Male", "Female", "Other"]),
            filter_select("status", "Status", &["initialized", "validated", "closed"]),
            filter_text("contactMedium.emailAddress", "Email"),
            filter_text("contactMedium.phoneNumber", "Phone"),
        ],
        default_sort: Some("familyName".into()),
    });

    m.insert("organization".into(), ResourceSchema {
        name: "organization".into(),
        plural_label: "Organizations".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("tradingName", "Trading Name", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col("nameType", "Name Type", ColumnType::String),
            col("organizationType", "Type", ColumnType::Badge),
            col("status", "Status", ColumnType::Badge),
            col("isLegalEntity", "Legal Entity", ColumnType::Boolean),
        ],
        filters: vec![
            filter_text("tradingName", "Trading Name"),
            filter_text("name", "Name"),
            filter_text("organizationType", "Organization Type"),
            filter_select("status", "Status", &["initialized", "validated", "closed"]),
            filter_bool("isLegalEntity", "Legal Entity"),
            filter_text("existsDuring.startDateTime", "Start Date"),
        ],
        default_sort: Some("tradingName".into()),
    });

    m
}
