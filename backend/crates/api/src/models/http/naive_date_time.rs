use std::ops::Deref;

use chrono::NaiveDateTime;
use schemars::gen::SchemaGenerator;
use schemars::schema::{InstanceType, Schema, SchemaObject};
use schemars::JsonSchema;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ApiNaiveDateTime(pub NaiveDateTime);

impl JsonSchema for ApiNaiveDateTime {
    fn is_referenceable() -> bool {
        false
    }
    fn schema_name() -> String {
        "ApiNaiveDateTime".to_string()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            format: None,
            ..Default::default()
        }
        .into()
    }
}

impl Deref for ApiNaiveDateTime {
    type Target = NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
