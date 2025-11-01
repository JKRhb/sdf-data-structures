use std::collections::HashMap;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Serialize, Deserialize, Debug, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InfoBlock {
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

    #[builder(setter(into))]
    pub message_id: String,
    // TODO: Add modified and features
}

#[skip_serializing_none]
#[derive(Default, Serialize, Deserialize, Debug, Builder, Clone)]
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
#[derive(Default, Serialize, Deserialize, Debug, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SdfInstanceMessage {
    #[builder(setter(strip_option))]
    pub info: InfoBlock,
    #[builder(setter(into, strip_option), default)]
    pub namespace: Option<HashMap<String, String>>,
    #[builder(setter(into, strip_option), default)]
    pub default_namespace: Option<String>,
    #[builder(setter(into, strip_option))]
    pub sdf_instance_of: SdfInstanceOf,
    pub sdf_instance: SdfInstance,
    #[builder(setter(into, strip_option), default)]
    pub sdf_binding: Option<HashMap<String, SdfBinding>>,
}

#[skip_serializing_none]
#[derive(Default, Serialize, Deserialize, Debug, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SdfInstanceOf {
    #[builder(setter(into))]
    model: String,
}

#[skip_serializing_none]
#[derive(Default, Serialize, Deserialize, Debug, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SdfInstance {
    #[builder(setter(into, strip_option), default)]
    pub sdf_thing: Option<HashMap<String, SdfInstance>>,
    #[builder(setter(into, strip_option), default)]
    pub sdf_object: Option<HashMap<String, SdfInstance>>,
    #[builder(setter(into, strip_option), default)]
    pub sdf_property: Option<HashMap<String, serde_json::Value>>,
    #[builder(setter(into, strip_option), default)]
    pub sdf_context: Option<HashMap<String, serde_json::Value>>,
    // TODO: What should actions and events look like?
    // #[builder(setter(strip_option), default)]
    // pub sdf_action: Option<HashMap<String, InteractionAffordance>>,
    // #[builder(setter(strip_option), default)]
    // pub sdf_event: Option<HashMap<String, InteractionAffordance>>,
    #[serde(flatten)]
    #[builder(default)]
    pub common_qualities: CommonQualities,
}

#[skip_serializing_none]
#[derive(Default, Serialize, Deserialize, Debug, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SdfBinding {
    #[builder(setter(into, strip_option), default)]
    pub sdf_thing: Option<HashMap<String, SdfBinding>>,
    #[builder(setter(into, strip_option), default)]
    pub sdf_object: Option<HashMap<String, SdfBinding>>,
    #[builder(setter(into, strip_option), default)]
    pub sdf_property: Option<HashMap<String, InteractionAffordance>>,
    #[builder(setter(into, strip_option), default)]
    pub sdf_action: Option<HashMap<String, InteractionAffordance>>,
    #[builder(setter(into, strip_option), default)]
    pub sdf_event: Option<HashMap<String, InteractionAffordance>>,

    #[serde(flatten)]
    #[builder(default)]
    pub common_qualities: CommonQualities,
}

#[skip_serializing_none]
#[derive(Default, Serialize, Deserialize, Debug, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SdfObject {
    #[builder(setter(into, strip_option), default)]
    pub sdf_property: Option<HashMap<String, InteractionAffordance>>,
    #[builder(setter(into, strip_option), default)]
    pub sdf_action: Option<HashMap<String, InteractionAffordance>>,
    #[builder(setter(into, strip_option), default)]
    pub sdf_event: Option<HashMap<String, InteractionAffordance>>,

    #[serde(flatten)]
    #[builder(default)]
    pub common_qualities: CommonQualities,
}

#[skip_serializing_none]
#[derive(Default, Serialize, Deserialize, Debug, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InteractionAffordance {
    #[builder(setter(into, strip_option), default)]
    pub description: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub label: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub comment: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub sdf_ref: Option<String>, // TODO: Add regex
    #[builder(setter(into, strip_option), default)]
    pub sdf_required: Option<Vec<String>>,

    #[serde(flatten)]
    #[builder(default)]
    pub protocol_instance_map: ProtocolInstanceMap,
}

#[skip_serializing_none]
#[derive(Default, Serialize, Deserialize, Debug, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoapInstanceMap {
    #[builder(setter(into))]
    pub host: String,
    #[builder(setter(strip_option), default)]
    pub port: Option<u16>,
}

#[derive(Default, Serialize, Deserialize, Debug, Builder, Clone)]
pub struct ProtocolInstanceMap {
    #[builder(setter(strip_option), default)]
    pub coap: Option<CoapInstanceMap>,
}
