use std::collections::HashMap;

use derive_builder::Builder;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(PartialEq, Debug, Clone)]
pub enum JsonPointerResolutionResult {
    InfoBlock(InfoBlock),
    SdfObject(SdfObject),
    SdfThing(SdfThing),
    SdfProperty(SdfProperty),
    SdfAction(SdfAction),
    SdfEvent(SdfEvent),
    SdfData(SdfData),
    SchemaDefinition(SchemaDefinition),
    SdfModel(SdfModel),
    Map(HashMap<String, JsonPointerResolutionResult>),
    Value(serde_json::Value),
}

pub trait JsonPointerResolvable {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<JsonPointerResolutionResult>;
}

// TODO: Refactor the following implementations
impl JsonPointerResolvable for HashMap<String, SdfThing> {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<JsonPointerResolutionResult> {
        match json_pointer.split("/").next() {
            None => Some(JsonPointerResolutionResult::Map(
                self.iter()
                    .map(|(key, value)| {
                        (
                            key.clone(),
                            JsonPointerResolutionResult::SdfThing(value.clone()),
                        )
                    })
                    .collect(),
            )),
            Some(first_path_segment) => self
                .get(first_path_segment)?
                .clone()
                .resolve_json_pointer(json_pointer),
        }
    }
}

impl JsonPointerResolvable for HashMap<String, SdfObject> {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<JsonPointerResolutionResult> {
        match json_pointer.split("/").next() {
            None => Some(JsonPointerResolutionResult::Map(
                self.iter()
                    .map(|(key, value)| {
                        (
                            key.clone(),
                            JsonPointerResolutionResult::SdfObject(value.clone()),
                        )
                    })
                    .collect(),
            )),
            Some(first_path_segment) => self
                .get(first_path_segment)?
                .clone()
                .resolve_json_pointer(json_pointer),
        }
    }
}

impl JsonPointerResolvable for HashMap<String, SdfProperty> {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<JsonPointerResolutionResult> {
        match json_pointer.split("/").next() {
            None => Some(JsonPointerResolutionResult::Map(
                self.iter()
                    .map(|(key, value)| {
                        (
                            key.clone(),
                            JsonPointerResolutionResult::SdfProperty(value.clone()),
                        )
                    })
                    .collect(),
            )),
            Some(first_path_segment) => self
                .get(first_path_segment)?
                .clone()
                .resolve_json_pointer(json_pointer),
        }
    }
}

impl JsonPointerResolvable for HashMap<String, SdfAction> {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<JsonPointerResolutionResult> {
        match json_pointer.split("/").next() {
            None => Some(JsonPointerResolutionResult::Map(
                self.iter()
                    .map(|(key, value)| {
                        (
                            key.clone(),
                            JsonPointerResolutionResult::SdfAction(value.clone()),
                        )
                    })
                    .collect(),
            )),
            Some(first_path_segment) => self
                .get(first_path_segment)?
                .clone()
                .resolve_json_pointer(json_pointer),
        }
    }
}

impl JsonPointerResolvable for HashMap<String, SdfEvent> {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<JsonPointerResolutionResult> {
        match json_pointer.split("/").next() {
            None => Some(JsonPointerResolutionResult::Map(
                self.iter()
                    .map(|(key, value)| {
                        (
                            key.clone(),
                            JsonPointerResolutionResult::SdfEvent(value.clone()),
                        )
                    })
                    .collect(),
            )),
            Some(first_path_segment) => self
                .get(first_path_segment)?
                .clone()
                .resolve_json_pointer(json_pointer),
        }
    }
}

impl JsonPointerResolvable for HashMap<String, SdfData> {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<JsonPointerResolutionResult> {
        match json_pointer.split("/").next() {
            None => Some(JsonPointerResolutionResult::Map(
                self.iter()
                    .map(|(key, value)| {
                        (
                            key.clone(),
                            JsonPointerResolutionResult::SdfData(value.clone()),
                        )
                    })
                    .collect(),
            )),
            Some(first_path_segment) => self
                .get(first_path_segment)?
                .clone()
                .resolve_json_pointer(json_pointer),
        }
    }
}

#[skip_serializing_none]
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Builder, Clone)]
pub struct InfoBlock {
    // TODO: Add modified and features
    #[builder(setter(into, strip_option), default)]
    pub title: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub description: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub version: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub copyright: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub license: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(rename = "$comment")]
    pub comment: Option<String>,
}

impl JsonPointerResolvable for InfoBlock {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<JsonPointerResolutionResult> {
        let mut segment_iterator = json_pointer.split("/");

        match segment_iterator.next() {
            Some(first_path_segment) => match first_path_segment {
                "" => Some(JsonPointerResolutionResult::InfoBlock(self)),
                "$comment" => {
                    JsonPointerResolutionResult::Value(serde_json::json!(self.comment)).into()
                }
                "copyright" => {
                    JsonPointerResolutionResult::Value(serde_json::json!(self.copyright)).into()
                }
                "description" => {
                    JsonPointerResolutionResult::Value(serde_json::json!(self.description)).into()
                }
                "license" => {
                    JsonPointerResolutionResult::Value(serde_json::json!(self.license)).into()
                }
                "title" => JsonPointerResolutionResult::Value(serde_json::json!(self.title)).into(),
                "version" => {
                    JsonPointerResolutionResult::Value(serde_json::json!(self.version)).into()
                }

                _ => None,
            },

            None => None,
        }
    }
}

#[skip_serializing_none]
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommonQualities {
    #[builder(setter(into, strip_option), default)]
    pub description: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub label: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(rename = "$comment")]
    pub comment: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub sdf_ref: Option<String>, // TODO: Add regex
    #[builder(setter(into, strip_option), default)]
    pub sdf_required: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SdfModel {
    #[builder(setter(strip_option), default)]
    pub info: Option<InfoBlock>,
    #[builder(setter(into, strip_option), default)]
    pub namespace: Option<HashMap<String, String>>,
    #[builder(setter(into, strip_option), default)]
    pub default_namespace: Option<String>,
    #[builder(setter(strip_option), default)]
    pub sdf_thing: Option<HashMap<String, SdfThing>>,
    #[builder(setter(strip_option), default)]
    pub sdf_object: Option<HashMap<String, SdfObject>>,
    #[builder(setter(strip_option), default)]
    pub sdf_property: Option<HashMap<String, SdfProperty>>,
    #[builder(setter(strip_option), default)]
    pub sdf_action: Option<HashMap<String, SdfAction>>,
    #[builder(setter(strip_option), default)]
    pub sdf_event: Option<HashMap<String, SdfEvent>>,
    #[builder(setter(strip_option), default)]
    pub sdf_data: Option<HashMap<String, SdfData>>,
}

#[skip_serializing_none]
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SdfThing {
    #[builder(setter(strip_option), default)]
    pub sdf_thing: Option<HashMap<String, SdfThing>>,
    #[builder(setter(strip_option), default)]
    pub sdf_object: Option<HashMap<String, SdfObject>>,
    #[builder(setter(strip_option), default)]
    pub sdf_property: Option<HashMap<String, SdfProperty>>,
    #[builder(setter(strip_option), default)]
    pub sdf_action: Option<HashMap<String, SdfAction>>,
    #[builder(setter(strip_option), default)]
    pub sdf_event: Option<HashMap<String, SdfEvent>>,
    #[builder(setter(strip_option), default)]
    pub sdf_data: Option<HashMap<String, SdfData>>,

    #[serde(flatten)]
    #[builder(default)]
    pub common_qualities: CommonQualities,
}

impl JsonPointerResolvable for SdfThing {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<JsonPointerResolutionResult> {
        let mut segment_iterator = json_pointer.split("/");

        match segment_iterator.next() {
            None => Some(JsonPointerResolutionResult::SdfThing(self.clone())),
            Some(first_path_segment) => {
                let json_pointer = segment_iterator.join("/");

                match first_path_segment {
                    "sdfAction" => self.sdf_action?.resolve_json_pointer(json_pointer),
                    _ => {
                        panic!();
                    }
                }
            }
        }
    }
}

#[skip_serializing_none]
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SdfObject {
    #[builder(setter(strip_option), default)]
    pub sdf_property: Option<HashMap<String, SdfProperty>>,
    #[builder(setter(strip_option), default)]
    pub sdf_action: Option<HashMap<String, SdfAction>>,
    #[builder(setter(strip_option), default)]
    pub sdf_event: Option<HashMap<String, SdfEvent>>,
    #[builder(setter(strip_option), default)]
    pub sdf_data: Option<HashMap<String, SdfData>>,

    #[serde(flatten)]
    #[builder(default)]
    pub common_qualities: CommonQualities,

    #[builder(setter(strip_option), default)]
    pub min_items: Option<u64>,
    #[builder(setter(strip_option), default)]
    pub max_items: Option<u64>,
}

impl JsonPointerResolvable for SdfObject {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<JsonPointerResolutionResult> {
        let mut segment_iterator = json_pointer.split("/");

        match segment_iterator.next() {
            Some(first_path_segment) => {
                let json_pointer = segment_iterator.join("/");

                match first_path_segment {
                    "sdfProperty" => self.sdf_property?.resolve_json_pointer(json_pointer),
                    "sdfAction" => self.sdf_action?.resolve_json_pointer(json_pointer),
                    "sdfEvent" => self.sdf_event?.resolve_json_pointer(json_pointer),
                    "sdfData" => self.sdf_data?.resolve_json_pointer(json_pointer),
                    _ => {
                        panic!();
                    }
                }
            }

            None => Some(JsonPointerResolutionResult::SdfObject(self.clone())),
        }
    }
}

#[skip_serializing_none]
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SdfData {
    #[serde(flatten)]
    #[builder(default)]
    pub common_qualities: CommonQualities,

    #[builder(setter(strip_option), default)]
    #[serde(flatten)]
    pub r#type: Option<SchemaDefinition>,

    #[builder(setter(into, strip_option), default)]
    pub sdf_choice: Option<HashMap<String, SdfData>>,
    #[builder(setter(strip_option), default)]
    pub r#enum: Option<Vec<String>>,

    #[builder(setter(strip_option), default)]
    pub r#const: Option<serde_json::Value>,
    #[builder(setter(strip_option), default)]
    #[serde(rename = "default")]
    pub default_value: Option<serde_json::Value>,
}

impl JsonPointerResolvable for SdfData {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<JsonPointerResolutionResult> {
        let mut segment_iterator = json_pointer.split("/");

        match segment_iterator.next() {
            None => Some(JsonPointerResolutionResult::SdfData(self.clone())),
            Some(first_path_segment) => {
                let json_pointer = segment_iterator.join("/");

                if let Some(yeah) = self.r#type {
                    let yo = yeah.resolve_json_pointer(json_pointer);

                    if yo.is_some() {
                        return yo;
                    }
                }

                match first_path_segment {
                    "type" => Some(JsonPointerResolutionResult::Value(serde_json::json!(
                        first_path_segment
                    ))),
                    _ => None,
                }
            }
        }
    }
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SchemaDefinition {
    Boolean,
    String(StringSchema),
    Integer(NumericSchema<i64>),
    Number(NumericSchema<f64>),
    Array(ArraySchema),
    Object(ObjectSchema),
}

impl JsonPointerResolvable for SchemaDefinition {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<JsonPointerResolutionResult> {
        let first_segment = json_pointer.split("/").next();

        match first_segment {
            None => None,
            // TODO: Differentiate by different schema type
            Some(first_segment) => Some(JsonPointerResolutionResult::Value(
                serde_json::to_value(self)
                    .ok()?
                    .as_object()?
                    .get(first_segment)?
                    .clone(),
            )),
        }
    }
}

#[skip_serializing_none]
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct StringSchema {
    #[builder(setter(strip_option), default)]
    pub min_length: Option<u64>,
    #[builder(setter(strip_option), default)]
    pub max_length: Option<u64>,
    #[builder(setter(into, strip_option), default)]
    pub pattern: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub format: Option<String>,
}

#[skip_serializing_none]
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct NumericSchema<T> {
    #[builder(setter(strip_option), default)]
    pub minimum: Option<T>,
    #[builder(setter(strip_option), default)]
    pub maximum: Option<T>,
    #[builder(setter(strip_option), default)]
    pub exclusive_minimum: Option<T>,
    #[builder(setter(strip_option), default)]
    pub exclusive_maximum: Option<T>,
    #[builder(setter(strip_option), default)]
    pub multiple_of: Option<T>,
}

#[skip_serializing_none]
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ArraySchema {
    #[builder(setter(strip_option), default)]
    pub min_items: Option<u64>,
    #[builder(setter(strip_option), default)]
    pub max_items: Option<u64>,
    #[builder(setter(strip_option), default)]
    pub unique_items: Option<bool>,
}

#[skip_serializing_none]
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ObjectSchema {
    #[builder(setter(into, strip_option), default)]
    pub required: Option<Vec<String>>,
    #[builder(setter(into, strip_option), default)]
    pub properties: Option<HashMap<String, SdfData>>,
}

#[inline]
fn bool_true() -> bool {
    true
}

#[inline]
fn skip_bool_true(value: &bool) -> bool {
    *value
}

#[skip_serializing_none]
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Builder, Clone)]
pub struct SdfProperty {
    #[serde(flatten)]
    #[builder(default)]
    pub internal_data: SdfData,

    #[builder(setter(strip_option), default = "true")]
    // TODO: Refactor this
    #[serde(default = "bool_true", skip_serializing_if = "skip_bool_true")]
    pub readable: bool,
    #[builder(setter(strip_option), default = "true")]
    #[serde(default = "bool_true", skip_serializing_if = "skip_bool_true")]
    pub writable: bool,
    #[builder(setter(strip_option), default = "true")]
    #[serde(default = "bool_true", skip_serializing_if = "skip_bool_true")]
    pub observable: bool,
}

impl JsonPointerResolvable for SdfProperty {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<JsonPointerResolutionResult> {
        let mut segment_iterator = json_pointer.split("/");

        match segment_iterator.next() {
            Some(first_path_segment) => match first_path_segment {
                _ => self.internal_data.resolve_json_pointer(json_pointer),
            },
            None => Some(JsonPointerResolutionResult::SdfProperty(self)),
        }
    }
}

#[skip_serializing_none]
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SdfAction {
    #[serde(flatten)]
    #[builder(default)]
    pub common_qualities: CommonQualities,

    #[builder(setter(strip_option), default)]
    pub sdf_data: Option<HashMap<String, SdfData>>,
    #[builder(setter(strip_option), default)]
    pub sdf_input_data: Option<SdfData>,
    #[builder(setter(strip_option), default)]
    pub sdf_output_data: Option<SdfData>,
}

impl JsonPointerResolvable for SdfAction {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<JsonPointerResolutionResult> {
        let mut segment_iterator = json_pointer.split("/");

        match segment_iterator.next() {
            Some(first_path_segment) => {
                let json_pointer = segment_iterator.join("/");

                match first_path_segment {
                    "sdfInputData" => self.sdf_output_data?.resolve_json_pointer(json_pointer),
                    "sdfOutputData" => self.sdf_output_data?.resolve_json_pointer(json_pointer),
                    "sdfData" => self.sdf_data?.resolve_json_pointer(json_pointer),
                    _ => {
                        panic!();
                    }
                }
            }

            None => Some(JsonPointerResolutionResult::SdfAction(self.clone())),
        }
    }
}

#[skip_serializing_none]
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SdfEvent {
    #[serde(flatten)]
    #[builder(default)]
    pub common_qualities: CommonQualities,

    #[builder(setter(strip_option), default)]
    pub sdf_data: Option<HashMap<String, SdfData>>,
    #[builder(setter(strip_option), default)]
    pub sdf_output_data: Option<SdfData>,
}

impl JsonPointerResolvable for SdfEvent {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<JsonPointerResolutionResult> {
        let mut segment_iterator = json_pointer.split("/");

        match segment_iterator.next() {
            Some(first_path_segment) => {
                let json_pointer = segment_iterator.join("/");

                match first_path_segment {
                    "sdfOutputData" => self.sdf_output_data?.resolve_json_pointer(json_pointer),
                    "sdfData" => self.sdf_data?.resolve_json_pointer(json_pointer),
                    _ => {
                        panic!()
                    }
                }
            }

            None => Some(JsonPointerResolutionResult::SdfEvent(self.clone())),
        }
    }
}

impl JsonPointerResolvable for SdfModel {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<JsonPointerResolutionResult> {
        let start_index: usize;
        if json_pointer.starts_with("#:/") {
            start_index = 3
        } else if json_pointer.starts_with("#/") {
            start_index = 2
        } else if json_pointer.starts_with("/") {
            start_index = 1;
        } else {
            start_index = 0;
        }

        println!("{}", start_index);

        let mut segment_iterator = json_pointer[start_index..].split("/");

        match segment_iterator.next() {
            None => Some(JsonPointerResolutionResult::SdfModel(self.clone())),
            Some(first_path_segment) => {
                let json_pointer = segment_iterator.join("/");

                println!("{}", first_path_segment);
                println!("{}", json_pointer);

                match first_path_segment {
                    "info" => self.info?.resolve_json_pointer(json_pointer),
                    "sdfObject" => self.sdf_object?.resolve_json_pointer(json_pointer),
                    "sdfThing" => self.sdf_thing?.resolve_json_pointer(json_pointer),
                    "sdfAction" => self.sdf_action?.resolve_json_pointer(json_pointer),
                    "sdfProperty" => self.sdf_property?.resolve_json_pointer(json_pointer),
                    "sdfEvent" => self.sdf_event?.resolve_json_pointer(json_pointer),
                    "sdfData" => self.sdf_data?.resolve_json_pointer(json_pointer),
                    _ => {
                        panic!();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_common_qualities() {
        let common_qualities = CommonQualitiesBuilder::default()
            .comment("This is a comment")
            .build()
            .unwrap();

        let serialized_common_qualities = "{\"$comment\":\"This is a comment\"}".to_string();

        assert_eq!(
            serde_json::to_string(&common_qualities).unwrap(),
            serialized_common_qualities
        );
    }

    #[test]
    fn test_sdf_property() {
        let sdf_property = SdfPropertyBuilder::default()
            .writable(false)
            .build()
            .unwrap();

        let serialized_sdf_property = "{\"writable\":false}".to_string();

        assert_eq!(
            serde_json::to_string(&sdf_property).unwrap(),
            serialized_sdf_property
        );
    }

    #[test]
    fn test_const_and_default() {
        let sdf_data = SdfDataBuilder::default()
            .r#const(serde_json::Value::Null)
            .default_value(json!(5))
            .build()
            .unwrap();

        let serialized_sdf_property = "{\"const\":null,\"default\":5}".to_string();

        assert_eq!(
            serde_json::to_string(&sdf_data).unwrap(),
            serialized_sdf_property
        );
    }

    #[test]
    fn test_json_pointers() {
        let sdf_model = SdfModel::deserialize(serde_json::json!({
          "info": {
            "title": "Example document for SDF (Semantic Definition Format)",
            "version": "2019-04-24",
            "copyright": "Copyright 2019 Example Corp. All rights reserved.",
            "license": "https://example.com/license"
          },
          "namespace": {
            "cap": "https://example.com/capability/cap"
          },
          "defaultNamespace": "cap",
          "sdfObject": {
            "Switch": {
              "sdfProperty": {
                "value": {
                  "description":
        "The state of the switch; false for off and true for on.",
                  "type": "boolean"
                }
              },
              "sdfAction": {
                "on": {
                  "description":
        "Turn the switch on; equivalent to setting value to true."
                },
                "off": {
                  "description":
        "Turn the switch off; equivalent to setting value to false."
                },
                "toggle": {
                  "description":
        "Toggle the switch; equivalent to setting value to its complement."
                }
              }
            }
          }
        }))
        .unwrap();

        let info = sdf_model
            .resolve_json_pointer("#:/info".to_string())
            .unwrap();

        assert_eq!(
            JsonPointerResolutionResult::InfoBlock(InfoBlock {
                title: Some("Example document for SDF (Semantic Definition Format)".into()),
                description: None,
                version: Some("2019-04-24".into()),
                copyright: Some("Copyright 2019 Example Corp. All rights reserved.".into()),
                license: Some("https://example.com/license".into()),
                comment: None
            }),
            info
        );
    }
}
