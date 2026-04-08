use std::collections::HashMap;
use crate::schema::*;

pub fn schemas() -> HashMap<String, ResourceSchema> {
    let mut m = HashMap::new();

    m.insert("productOrder".into(), ResourceSchema {
        name: "productOrder".into(),
        plural_label: "Product Orders".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col("externalId", "External ID", ColumnType::String),
            col("description", "Description", ColumnType::String),
            col_sort("state", "State", ColumnType::Badge),
            col("priority", "Priority", ColumnType::Badge),
            col_sort("orderDate", "Order Date", ColumnType::Date),
            col("requestedStartDate", "Requested Start", ColumnType::Date),
            col("requestedCompletionDate", "Requested Completion", ColumnType::Date),
            col("completionDate", "Completed", ColumnType::Date),
        ],
        filters: vec![
            filter_text("externalId", "External ID"),
            filter_select("state", "State", &[
                "acknowledged", "rejected", "pending",
                "held", "inProgress", "cancelled",
                "completed", "failed", "partial",
                "assessingCancellation", "pendingCancellation",
            ]),
            filter_select("priority", "Priority", &["0", "1", "2", "3", "4"]),
            filter_date("orderDate", "Order Date"),
            filter_date("requestedStartDate", "Requested Start"),
            filter_date("requestedCompletionDate", "Requested Completion"),
            filter_date("completionDate", "Completed"),
            filter_text("relatedParty.name", "Related Party"),
            filter_text("channel.name", "Channel"),
        ],
        default_sort: Some("-orderDate".into()),
    });

    m.insert("cancelProductOrder".into(), ResourceSchema {
        name: "cancelProductOrder".into(),
        plural_label: "Cancel Product Orders".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col("state", "State", ColumnType::Badge),
            col("cancellationReason", "Reason", ColumnType::String),
            col("requestedCancellationDate", "Requested Date", ColumnType::Date),
            col("effectiveCancellationDate", "Effective Date", ColumnType::Date),
        ],
        filters: vec![
            filter_select("state", "State", &[
                "acknowledged", "rejected", "inProgress",
                "terminatedWithError", "done",
            ]),
            filter_date("requestedCancellationDate", "Requested Date"),
        ],
        default_sort: Some("-requestedCancellationDate".into()),
    });

    m
}
