use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Strongly-typed TMF Open API query parameters.
///
/// Covers standard TMF pagination, field selection, sorting,
/// and arbitrary filter expressions (`{field}={value}` or `{field}.{op}={value}`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TmfQueryParams {
    pub fields: Option<String>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    pub sort: Option<String>,
    #[serde(flatten)]
    pub filters: HashMap<String, String>,
}

impl TmfQueryParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_fields(mut self, fields: &str) -> Self {
        self.fields = Some(fields.to_string());
        self
    }

    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_sort(mut self, sort: &str) -> Self {
        self.sort = Some(sort.to_string());
        self
    }

    pub fn with_filter(mut self, key: &str, value: &str) -> Self {
        self.filters.insert(key.to_string(), value.to_string());
        self
    }

    /// Convert to key-value pairs suitable for `reqwest::RequestBuilder::query()`.
    pub fn to_pairs(&self) -> Vec<(String, String)> {
        let mut pairs = Vec::new();
        if let Some(ref f) = self.fields {
            pairs.push(("fields".into(), f.clone()));
        }
        if let Some(o) = self.offset {
            pairs.push(("offset".into(), o.to_string()));
        }
        if let Some(l) = self.limit {
            pairs.push(("limit".into(), l.to_string()));
        }
        if let Some(ref s) = self.sort {
            pairs.push(("sort".into(), s.clone()));
        }
        for (k, v) in &self.filters {
            pairs.push((k.clone(), v.clone()));
        }
        pairs
    }

    /// Parse from a raw URL query string (e.g. from `HttpRequest::query_string()`).
    pub fn from_query_string(qs: &str) -> Self {
        let mut params = Self::default();
        if qs.is_empty() {
            return params;
        }
        for pair in qs.split('&') {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next().unwrap_or("").to_string();
            let value = parts.next().unwrap_or("").to_string();
            if key.is_empty() {
                continue;
            }
            match key.as_str() {
                "fields" => params.fields = Some(value),
                "offset" => params.offset = value.parse().ok(),
                "limit" => params.limit = value.parse().ok(),
                "sort" => params.sort = Some(value),
                _ => {
                    params.filters.insert(key, value);
                }
            }
        }
        params
    }
}

impl std::fmt::Display for TmfQueryParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pairs = self.to_pairs();
        if pairs.is_empty() {
            return write!(f, "(none)");
        }
        let s: Vec<String> = pairs.iter().map(|(k, v)| format!("{}={}", k, v)).collect();
        write!(f, "{}", s.join("&"))
    }
}
