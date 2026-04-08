use std::collections::HashMap;
use crate::schema::*;

pub fn schemas() -> HashMap<String, ResourceSchema> {
    let mut m = HashMap::new();

    m.insert("serviceOrder".into(), ResourceSchema {
        name: "serviceOrder".into(),
        plural_label: "Service Orders".into(),
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
            filter_date("completionDate", "Completed"),
            filter_text("relatedParty.name", "Related Party"),
        ],
        default_sort: Some("-orderDate".into()),
    });

    m.insert("cancelServiceOrder".into(), ResourceSchema {
        name: "cancelServiceOrder".into(),
        plural_label: "Cancel Service Orders".into(),
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
        ],
        default_sort: None,
    });

    m
}
