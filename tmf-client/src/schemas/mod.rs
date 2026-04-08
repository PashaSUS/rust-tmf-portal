mod tmf620;
mod tmf622;
mod tmf629;
mod tmf632;
mod tmf633;
mod tmf634;
mod tmf637;
mod tmf638;
mod tmf639;
mod tmf641;
mod tmf645;
mod tmf648;
mod tmf651;
mod tmf666;
mod tmf674;

use std::collections::HashMap;
use crate::schema::ResourceSchema;

pub fn get_schema(api_id: &str, resource: &str) -> Option<ResourceSchema> {
    let registry = build_registry();
    registry.get(api_id)
        .and_then(|m| m.get(resource))
        .cloned()
}

pub fn get_api_schemas(api_id: &str) -> HashMap<String, ResourceSchema> {
    let registry = build_registry();
    registry.get(api_id).cloned().unwrap_or_default()
}

fn build_registry() -> HashMap<String, HashMap<String, ResourceSchema>> {
    let mut m = HashMap::new();
    m.insert("tmf620".into(), tmf620::schemas());
    m.insert("tmf622".into(), tmf622::schemas());
    m.insert("tmf629".into(), tmf629::schemas());
    m.insert("tmf632".into(), tmf632::schemas());
    m.insert("tmf633".into(), tmf633::schemas());
    m.insert("tmf634".into(), tmf634::schemas());
    m.insert("tmf637".into(), tmf637::schemas());
    m.insert("tmf638".into(), tmf638::schemas());
    m.insert("tmf639".into(), tmf639::schemas());
    m.insert("tmf641".into(), tmf641::schemas());
    m.insert("tmf645".into(), tmf645::schemas());
    m.insert("tmf648".into(), tmf648::schemas());
    m.insert("tmf651".into(), tmf651::schemas());
    m.insert("tmf666".into(), tmf666::schemas());
    m.insert("tmf674".into(), tmf674::schemas());
    m
}
