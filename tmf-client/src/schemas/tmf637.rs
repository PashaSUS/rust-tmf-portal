use std::collections::HashMap;
use crate::schema::*;

pub fn schemas() -> HashMap<String, ResourceSchema> {
    let mut m = HashMap::new();

    m.insert("product".into(), ResourceSchema {
        name: "product".into(),
        plural_label: "Products".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col("description", "Description", ColumnType::String),
            col_sort("status", "Status", ColumnType::Badge),
            col("isBundle", "Bundle", ColumnType::Boolean),
            col("isCustomerVisible", "Customer Visible", ColumnType::Boolean),
            col_sort("startDate", "Start Date", ColumnType::Date),
            col("orderDate", "Order Date", ColumnType::Date),
        ],
        filters: vec![
            filter_text("name", "Product Name"),
            filter_select("status", "Status", &[
                "created", "pendingActive", "cancelled",
                "active", "aborted", "suspended", "terminated",
            ]),
            filter_bool("isBundle", "Bundle"),
            filter_bool("isCustomerVisible", "Customer Visible"),
            filter_text("productOffering.name", "Product Offering"),
            filter_text("productSpecification.name", "Product Specification"),
            filter_text("relatedParty.name", "Related Party"),
            filter_text("place.name", "Place"),
            filter_date("startDate", "Start Date"),
            filter_date("orderDate", "Order Date"),
        ],
        default_sort: Some("-startDate".into()),
    });

    m
}
