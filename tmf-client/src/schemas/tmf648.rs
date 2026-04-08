use std::collections::HashMap;
use crate::schema::*;

pub fn schemas() -> HashMap<String, ResourceSchema> {
    let mut m = HashMap::new();

    m.insert("quote".into(), ResourceSchema {
        name: "quote".into(),
        plural_label: "Quotes".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col("externalId", "External ID", ColumnType::String),
            col("description", "Description", ColumnType::String),
            col_sort("state", "State", ColumnType::Badge),
            col("category", "Category", ColumnType::String),
            col_sort("quoteDate", "Quote Date", ColumnType::Date),
            col("requestedQuoteCompletionDate", "Requested Completion", ColumnType::Date),
            col("effectiveQuoteCompletionDate", "Effective Completion", ColumnType::Date),
        ],
        filters: vec![
            filter_text("externalId", "External ID"),
            filter_select("state", "State", &[
                "inProgress", "pending", "cancelled",
                "approved", "accepted", "rejected",
            ]),
            filter_text("category", "Category"),
            filter_date("quoteDate", "Quote Date"),
            filter_date("requestedQuoteCompletionDate", "Requested Completion"),
            filter_text("relatedParty.name", "Related Party"),
        ],
        default_sort: Some("-quoteDate".into()),
    });

    m
}
