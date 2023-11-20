use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DataProcessorConfiguration {
    #[serde(rename = "type")]
    pub data_processor_type: String,
    pub args: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RendererConfiguration {
    #[serde(rename = "type")]
    pub renderer_type: String,
    pub args: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerConfiguration {
    #[serde(rename = "type")]
    pub consumer_type: String,
    pub args: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub data_processor: Vec<DataProcessorConfiguration>,
    pub renderer: RendererConfiguration,
    pub consumer: ConsumerConfiguration,
}
