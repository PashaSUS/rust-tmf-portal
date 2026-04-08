use std::collections::HashMap;
use crate::schema::*;

pub fn schemas() -> HashMap<String, ResourceSchema> {
    let mut m = HashMap::new();

    m.insert("serviceCatalog".into(), ResourceSchema {
        name: "serviceCatalog".into(),
        plural_label: "Service Catalogs".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col_sort("lifecycleStatus", "Status", ColumnType::Badge),
            col_sort("lastUpdate", "Last Update", ColumnType::Date),
            col("version", "Version", ColumnType::String),
        ],
        filters: vec![
            filter_text("name", "Catalog Name"),
            filter_select("lifecycleStatus", "Status", &["Active", "Launched", "Retired", "Obsolete"]),
            filter_date("lastUpdate", "Last Update"),
        ],
        default_sort: Some("-lastUpdate".into()),
    });

    m.insert("serviceCategory".into(), ResourceSchema {
        name: "serviceCategory".into(),
        plural_label: "Service Categories".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col_sort("lifecycleStatus", "Status", ColumnType::Badge),
            col("isRoot", "Root", ColumnType::Boolean),
            col_sort("lastUpdate", "Last Update", ColumnType::Date),
        ],
        filters: vec![
            filter_text("name", "Category Name"),
            filter_select("lifecycleStatus", "Status", &["Active", "Launched", "Retired", "Obsolete"]),
            filter_bool("isRoot", "Root Category"),
        ],
        default_sort: Some("name".into()),
    });

    m.insert("serviceCandidate".into(), ResourceSchema {
        name: "serviceCandidate".into(),
        plural_label: "Service Candidates".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col_sort("lifecycleStatus", "Status", ColumnType::Badge),
            col("version", "Version", ColumnType::String),
            col_sort("lastUpdate", "Last Update", ColumnType::Date),
        ],
        filters: vec![
            filter_text("name", "Candidate Name"),
            filter_select("lifecycleStatus", "Status", &["Active", "Launched", "Retired", "Obsolete"]),
        ],
        default_sort: Some("-lastUpdate".into()),
    });

    m.insert("serviceSpecification".into(), ResourceSchema {
        name: "serviceSpecification".into(),
        plural_label: "Service Specifications".into(),
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
            filter_select("lifecycleStatus", "Status", &["In study", "In design", "In test", "Active", "Launched", "Retired", "Obsolete"]),
            filter_bool("isBundle", "Bundle"),
            filter_date("lastUpdate", "Last Update"),
        ],
        default_sort: Some("-lastUpdate".into()),
    });

    m
}
