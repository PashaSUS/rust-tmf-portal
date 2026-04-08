use std::collections::HashMap;
use crate::schema::*;

pub fn schemas() -> HashMap<String, ResourceSchema> {
    let mut m = HashMap::new();

    m.insert("checkServiceQualification".into(), ResourceSchema {
        name: "checkServiceQualification".into(),
        plural_label: "Service Qualifications (Check)".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col("description", "Description", ColumnType::String),
            col_sort("state", "State", ColumnType::Badge),
            col("qualificationResult", "Result", ColumnType::Badge),
            col_sort("effectiveQualificationDate", "Qualification Date", ColumnType::Date),
        ],
        filters: vec![
            filter_select("state", "State", &[
                "acknowledged", "inProgress", "terminatedWithError", "done",
            ]),
            filter_select("qualificationResult", "Result", &["qualified", "unqualified", "alternate"]),
            filter_date("effectiveQualificationDate", "Qualification Date"),
        ],
        default_sort: Some("-effectiveQualificationDate".into()),
    });

    m.insert("queryServiceQualification".into(), ResourceSchema {
        name: "queryServiceQualification".into(),
        plural_label: "Service Qualifications (Query)".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col("description", "Description", ColumnType::String),
            col_sort("state", "State", ColumnType::Badge),
            col_sort("effectiveQualificationDate", "Qualification Date", ColumnType::Date),
        ],
        filters: vec![
            filter_select("state", "State", &[
                "acknowledged", "inProgress", "terminatedWithError", "done",
            ]),
        ],
        default_sort: None,
    });

    m
}
