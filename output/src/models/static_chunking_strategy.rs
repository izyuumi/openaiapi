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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticChunkingStrategy {
    /// Always `static`.
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "static")]
    pub r#static: Box<models::StaticChunkingStrategyStatic>,
}

impl StaticChunkingStrategy {
    pub fn new(r#type: Type, r#static: models::StaticChunkingStrategyStatic) -> StaticChunkingStrategy {
        StaticChunkingStrategy {
            r#type,
            r#static: Box::new(r#static),
        }
    }
}
/// Always `static`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "static")]
    Static,
}

impl Default for Type {
    fn default() -> Type {
        Self::Static
    }
}

