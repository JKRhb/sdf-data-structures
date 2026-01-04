use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    json_pointer::JsonPointerResolvable,
    model::{
        InfoBlock, SchemaDefinition, SdfAction, SdfData, SdfEvent, SdfModel, SdfObject,
        SdfProperty, SdfThing,
    },
};

#[derive(PartialEq, Debug, Clone)]
pub enum ModelResult {
    InfoBlock(InfoBlock),
    SdfObject(SdfObject),
    SdfThing(SdfThing),
    SdfProperty(SdfProperty),
    SdfAction(SdfAction),
    SdfEvent(SdfEvent),
    SdfData(SdfData),
    SchemaDefinition(SchemaDefinition),
    SdfModel(SdfModel),
    Map(HashMap<String, ModelResult>),
    Value(serde_json::Value),
}

// TODO: Refactor the following implementations
impl JsonPointerResolvable<ModelResult> for HashMap<String, SdfThing> {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<ModelResult> {
        match json_pointer.split("/").next() {
            None => Some(ModelResult::Map(
                self.iter()
                    .map(|(key, value)| (key.clone(), ModelResult::SdfThing(value.clone())))
                    .collect(),
            )),
            Some(first_path_segment) => self
                .get(first_path_segment)?
                .clone()
                .resolve_json_pointer(json_pointer),
        }
    }
}

impl JsonPointerResolvable<ModelResult> for HashMap<String, SdfObject> {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<ModelResult> {
        match json_pointer.split("/").next() {
            None => Some(ModelResult::Map(
                self.iter()
                    .map(|(key, value)| (key.clone(), ModelResult::SdfObject(value.clone())))
                    .collect(),
            )),
            Some(first_path_segment) => self
                .get(first_path_segment)?
                .clone()
                .resolve_json_pointer(json_pointer),
        }
    }
}

impl JsonPointerResolvable<ModelResult> for HashMap<String, SdfProperty> {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<ModelResult> {
        match json_pointer.split("/").next() {
            None => Some(ModelResult::Map(
                self.iter()
                    .map(|(key, value)| (key.clone(), ModelResult::SdfProperty(value.clone())))
                    .collect(),
            )),
            Some(first_path_segment) => self
                .get(first_path_segment)?
                .clone()
                .resolve_json_pointer(json_pointer),
        }
    }
}

impl JsonPointerResolvable<ModelResult> for HashMap<String, SdfAction> {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<ModelResult> {
        match json_pointer.split("/").next() {
            None => Some(ModelResult::Map(
                self.iter()
                    .map(|(key, value)| (key.clone(), ModelResult::SdfAction(value.clone())))
                    .collect(),
            )),
            Some(first_path_segment) => self
                .get(first_path_segment)?
                .clone()
                .resolve_json_pointer(json_pointer),
        }
    }
}

impl JsonPointerResolvable<ModelResult> for HashMap<String, SdfEvent> {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<ModelResult> {
        match json_pointer.split("/").next() {
            None => Some(ModelResult::Map(
                self.iter()
                    .map(|(key, value)| (key.clone(), ModelResult::SdfEvent(value.clone())))
                    .collect(),
            )),
            Some(first_path_segment) => self
                .get(first_path_segment)?
                .clone()
                .resolve_json_pointer(json_pointer),
        }
    }
}

impl JsonPointerResolvable<ModelResult> for HashMap<String, SdfData> {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<ModelResult> {
        match json_pointer.split("/").next() {
            None => Some(ModelResult::Map(
                self.iter()
                    .map(|(key, value)| (key.clone(), ModelResult::SdfData(value.clone())))
                    .collect(),
            )),
            Some(first_path_segment) => self
                .get(first_path_segment)?
                .clone()
                .resolve_json_pointer(json_pointer),
        }
    }
}

impl JsonPointerResolvable<ModelResult> for InfoBlock {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<ModelResult> {
        let mut segment_iterator = json_pointer.split("/");

        match segment_iterator.next() {
            Some(first_path_segment) => match first_path_segment {
                "" => Some(ModelResult::InfoBlock(self)),
                "$comment" => ModelResult::Value(serde_json::json!(self.comment)).into(),
                "copyright" => ModelResult::Value(serde_json::json!(self.copyright)).into(),
                "description" => ModelResult::Value(serde_json::json!(self.description)).into(),
                "license" => ModelResult::Value(serde_json::json!(self.license)).into(),
                "title" => ModelResult::Value(serde_json::json!(self.title)).into(),
                "version" => ModelResult::Value(serde_json::json!(self.version)).into(),

                _ => None,
            },

            None => None,
        }
    }
}

impl JsonPointerResolvable<ModelResult> for SdfObject {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<ModelResult> {
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

            None => Some(ModelResult::SdfObject(self.clone())),
        }
    }
}

impl JsonPointerResolvable<ModelResult> for SdfData {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<ModelResult> {
        let mut segment_iterator = json_pointer.split("/");

        match segment_iterator.next() {
            None => Some(ModelResult::SdfData(self.clone())),
            Some(first_path_segment) => {
                let json_pointer = segment_iterator.join("/");

                if let Some(yeah) = self.r#type {
                    let yo = yeah.resolve_json_pointer(json_pointer);

                    if yo.is_some() {
                        return yo;
                    }
                }

                match first_path_segment {
                    "type" => Some(ModelResult::Value(serde_json::json!(first_path_segment))),
                    _ => None,
                }
            }
        }
    }
}

impl JsonPointerResolvable<ModelResult> for SchemaDefinition {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<ModelResult> {
        let first_segment = json_pointer.split("/").next();

        match first_segment {
            None => None,
            // TODO: Differentiate by different schema type
            Some(first_segment) => Some(ModelResult::Value(
                serde_json::to_value(self)
                    .ok()?
                    .as_object()?
                    .get(first_segment)?
                    .clone(),
            )),
        }
    }
}

impl JsonPointerResolvable<ModelResult> for SdfProperty {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<ModelResult> {
        let mut segment_iterator = json_pointer.split("/");

        match segment_iterator.next() {
            Some(first_path_segment) => match first_path_segment {
                _ => self.internal_data.resolve_json_pointer(json_pointer),
            },
            None => Some(ModelResult::SdfProperty(self)),
        }
    }
}

impl JsonPointerResolvable<ModelResult> for SdfEvent {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<ModelResult> {
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

            None => Some(ModelResult::SdfEvent(self.clone())),
        }
    }
}

impl JsonPointerResolvable<ModelResult> for SdfModel {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<ModelResult> {
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
            None => Some(ModelResult::SdfModel(self.clone())),
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

impl JsonPointerResolvable<ModelResult> for SdfAction {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<ModelResult> {
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

            None => Some(ModelResult::SdfAction(self.clone())),
        }
    }
}

impl JsonPointerResolvable<ModelResult> for SdfThing {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<ModelResult> {
        let mut segment_iterator = json_pointer.split("/");

        match segment_iterator.next() {
            None => Some(ModelResult::SdfThing(self.clone())),
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

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::{
        json_pointer::{JsonPointerResolvable, model::ModelResult},
        model::{InfoBlock, SdfModel},
    };

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
            ModelResult::InfoBlock(InfoBlock {
                title: Some("Example document for SDF (Semantic Definition Format)".into()),
                description: None,
                version: Some("2019-04-24".into()),
                copyright: Some("Copyright 2019 Example Corp. All rights reserved.".into()),
                license: Some("https://example.com/license".into()),
                comment: None,
                additional_qualities: None,
            }),
            info
        );
    }
}
