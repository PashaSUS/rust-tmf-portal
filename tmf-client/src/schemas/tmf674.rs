use std::collections::HashMap;
use crate::schema::*;

pub fn schemas() -> HashMap<String, ResourceSchema> {
    let mut m = HashMap::new();

    m.insert("geographicSite".into(), ResourceSchema {
        name: "geographicSite".into(),
        plural_label: "Geographic Sites".into(),
        columns: vec![
            col("id", "ID", ColumnType::String),
            col_sort("name", "Name", ColumnType::String),
            col("description", "Description", ColumnType::String),
            col("code", "Code", ColumnType::String),
            col_sort("status", "Status", ColumnType::Badge),
            col("siteType", "Type", ColumnType::String),
        ],
        filters: vec![
            filter_text("name", "Site Name"),
            filter_text("code", "Site Code"),
            filter_text("siteType", "Site Type"),
            filter_select("status", "Status", &["planned", "installed", "operational", "retired"]),
            filter_text("place.streetNr", "Street Number"),
            filter_text("place.streetName", "Street Name"),
            filter_text("place.city", "City"),
            filter_text("place.postcode", "Postcode"),
            filter_text("place.country", "Country"),
            filter_text("relatedParty.name", "Related Party"),
        ],
        default_sort: Some("name".into()),
    });

    m
}
