use std::collections::HashMap;
use crate::schema::*;

pub fn schemas() -> HashMap<String, ResourceSchema> {
    let mut m = HashMap::new();

    m.insert("resourceCatalog".into(), ResourceSchema {
        name: "resourceCatalog".into(),
        plural_label: "Resource Catalogs".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col_sort("lifecycleStatus", "Status", ColumnType::Badge),
            col_sort("lastUpdate", "Last Update", ColumnType::Date),
        ],
        filters: vec![
            filter_text("name", "Catalog Name"),
            filter_select("lifecycleStatus", "Status", &["Active", "Launched", "Retired", "Obsolete"]),
        ],
        default_sort: Some("-lastUpdate".into()),
    });

    m.insert("resourceCategory".into(), ResourceSchema {
        name: "resourceCategory".into(),
        plural_label: "Resource Categories".into(),
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

    m.insert("resourceCandidate".into(), ResourceSchema {
        name: "resourceCandidate".into(),
        plural_label: "Resource Candidates".into(),
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

    m.insert("resourceSpecification".into(), ResourceSchema {
        name: "resourceSpecification".into(),
        plural_label: "Resource Specifications".into(),
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
