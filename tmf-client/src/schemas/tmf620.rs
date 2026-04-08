use std::collections::HashMap;
use crate::schema::*;

pub fn schemas() -> HashMap<String, ResourceSchema> {
    let mut m = HashMap::new();

    m.insert("catalog".into(), ResourceSchema {
        name: "catalog".into(),
        plural_label: "Catalogs".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col_sort("lifecycleStatus", "Status", ColumnType::Badge),
            col("catalogType", "Type", ColumnType::String),
            col_sort("lastUpdate", "Last Update", ColumnType::Date),
            col("version", "Version", ColumnType::String),
        ],
        filters: vec![
            filter_text("name", "Catalog Name"),
            filter_select("lifecycleStatus", "Status", &["Active", "Launched", "Retired", "Obsolete"]),
            filter_text("catalogType", "Catalog Type"),
            filter_date("lastUpdate", "Last Update"),
        ],
        default_sort: Some("-lastUpdate".into()),
    });

    m.insert("category".into(), ResourceSchema {
        name: "category".into(),
        plural_label: "Categories".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col_sort("lifecycleStatus", "Status", ColumnType::Badge),
            col("isRoot", "Root", ColumnType::Boolean),
            col("parentId", "Parent ID", ColumnType::String),
            col_sort("lastUpdate", "Last Update", ColumnType::Date),
        ],
        filters: vec![
            filter_text("name", "Category Name"),
            filter_select("lifecycleStatus", "Status", &["Active", "Launched", "Retired", "Obsolete"]),
            filter_bool("isRoot", "Root Category"),
            filter_text("parentId", "Parent ID"),
        ],
        default_sort: Some("name".into()),
    });

    m.insert("productOffering".into(), ResourceSchema {
        name: "productOffering".into(),
        plural_label: "Product Offerings".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col("description", "Description", ColumnType::String),
            col_sort("lifecycleStatus", "Status", ColumnType::Badge),
            col("version", "Version", ColumnType::String),
            col_sort("lastUpdate", "Last Update", ColumnType::Date),
        ],
        filters: vec![
            filter_text("name", "Offering Name"),
            filter_select("lifecycleStatus", "Status", &["In study", "In design", "In test", "Active", "Launched", "Retired", "Obsolete"]),
            filter_text("category.name", "Category"),
            filter_text("channel.name", "Channel"),
            filter_text("place.name", "Place"),
            filter_date("lastUpdate", "Last Update"),
            filter_bool("isSellable", "Sellable"),
            filter_bool("isBundle", "Bundle"),
        ],
        default_sort: Some("-lastUpdate".into()),
    });

    m.insert("productSpecification".into(), ResourceSchema {
        name: "productSpecification".into(),
        plural_label: "Product Specifications".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col("brand", "Brand", ColumnType::String),
            col_sort("lifecycleStatus", "Status", ColumnType::Badge),
            col("productNumber", "Product #", ColumnType::String),
            col_sort("lastUpdate", "Last Update", ColumnType::Date),
        ],
        filters: vec![
            filter_text("name", "Spec Name"),
            filter_select("lifecycleStatus", "Status", &["In study", "In design", "In test", "Active", "Launched", "Retired", "Obsolete"]),
            filter_text("brand", "Brand"),
            filter_text("productNumber", "Product Number"),
            filter_bool("isBundle", "Bundle"),
            filter_date("lastUpdate", "Last Update"),
        ],
        default_sort: Some("-lastUpdate".into()),
    });

    m.insert("importJob".into(), ResourceSchema {
        name: "importJob".into(),
        plural_label: "Import Jobs".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col("url", "URL", ColumnType::String),
            col("status", "Status", ColumnType::Badge),
            col("contentType", "Content Type", ColumnType::String),
            col_sort("creationDate", "Created", ColumnType::Date),
        ],
        filters: vec![
            filter_select("status", "Status", &["Not Started", "Running", "Succeeded", "Failed"]),
            filter_date("creationDate", "Created"),
        ],
        default_sort: Some("-creationDate".into()),
    });

    m.insert("exportJob".into(), ResourceSchema {
        name: "exportJob".into(),
        plural_label: "Export Jobs".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col("url", "URL", ColumnType::String),
            col("status", "Status", ColumnType::Badge),
            col("contentType", "Content Type", ColumnType::String),
            col_sort("creationDate", "Created", ColumnType::Date),
        ],
        filters: vec![
            filter_select("status", "Status", &["Not Started", "Running", "Succeeded", "Failed"]),
            filter_date("creationDate", "Created"),
        ],
        default_sort: Some("-creationDate".into()),
    });

    m
}
