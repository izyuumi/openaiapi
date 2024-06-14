/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see https://platform.openai.com/docs/api-reference for more details.
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// OtherChunkingStrategyResponseParam : This is returned when the chunking strategy is unknown. Typically, this is because the file was indexed before the `chunking_strategy` concept was introduced in the API.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OtherChunkingStrategyResponseParam {
    /// Always `other`.
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl OtherChunkingStrategyResponseParam {
    /// This is returned when the chunking strategy is unknown. Typically, this is because the file was indexed before the `chunking_strategy` concept was introduced in the API.
    pub fn new(r#type: Type) -> OtherChunkingStrategyResponseParam {
        OtherChunkingStrategyResponseParam {
            r#type,
        }
    }
}
/// Always `other`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "other")]
    Other,
}

impl Default for Type {
    fn default() -> Type {
        Self::Other
    }
}
