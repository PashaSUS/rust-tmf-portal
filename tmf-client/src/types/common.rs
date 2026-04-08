use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimePeriod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Characteristic {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub value: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "@baseType", skip_serializing_if = "Option::is_none")]
    pub at_base_type: Option<String>,
    #[serde(rename = "@schemaLocation", skip_serializing_if = "Option::is_none")]
    pub at_schema_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characteristic_relationship: Option<Vec<CharacteristicRelationship>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CharacteristicRelationship {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RelatedParty {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "@referredType", skip_serializing_if = "Option::is_none")]
    pub at_referred_type: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Money {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Quantity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentRefOrValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContactMedium {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characteristic: Option<MediumCharacteristic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MediumCharacteristic {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street2: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExternalReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reference_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RelatedPlaceRefOrValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
}

/// Normalize a characteristic value based on its declared valueType.
/// Handles: string, number, integer, float, double, long, boolean, object, array.
pub fn normalize_characteristic_value(value: &Value, value_type: &str) -> Value {
    match value_type {
        "string" => match value {
            Value::String(_) => value.clone(),
            other => Value::String(other.to_string().trim_matches('"').to_string()),
        },
        "number" | "integer" | "float" | "double" | "long" => match value {
            Value::Number(_) => value.clone(),
            Value::String(s) => s
                .parse::<f64>()
                .ok()
                .and_then(|n| serde_json::Number::from_f64(n))
                .map(Value::Number)
                .unwrap_or_else(|| value.clone()),
            _ => value.clone(),
        },
        "boolean" => match value {
            Value::Bool(_) => value.clone(),
            Value::String(s) => match s.to_lowercase().as_str() {
                "true" | "1" | "yes" => Value::Bool(true),
                "false" | "0" | "no" => Value::Bool(false),
                _ => value.clone(),
            },
            Value::Number(n) => Value::Bool(n.as_f64().map_or(false, |f| f != 0.0)),
            _ => value.clone(),
        },
        "object" => match value {
            Value::Object(_) => value.clone(),
            Value::String(s) => serde_json::from_str(s).unwrap_or_else(|_| value.clone()),
            _ => value.clone(),
        },
        "array" => match value {
            Value::Array(_) => value.clone(),
            Value::String(s) => serde_json::from_str(s).unwrap_or_else(|_| value.clone()),
            _ => value.clone(),
        },
        _ => value.clone(),
    }
}

/// Recursively walk a JSON value and normalize all characteristic values found.
pub fn normalize_characteristics_in_json(value: &mut Value) {
    match value {
        Value::Object(map) => {
            if map.contains_key("name") && map.contains_key("value") {
                if let Some(Value::String(vt)) = map.get("valueType") {
                    let vt = vt.clone();
                    if let Some(v) = map.get("value") {
                        let normalized = normalize_characteristic_value(v, &vt);
                        map.insert("value".to_string(), normalized);
                    }
                }
            }
            for v in map.values_mut() {
                normalize_characteristics_in_json(v);
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                normalize_characteristics_in_json(item);
            }
        }
        _ => {}
    }
}
