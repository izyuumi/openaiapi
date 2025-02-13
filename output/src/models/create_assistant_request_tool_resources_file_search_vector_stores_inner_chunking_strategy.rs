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

/// CreateAssistantRequestToolResourcesFileSearchVectorStoresInnerChunkingStrategy : The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy.
/// The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAssistantRequestToolResourcesFileSearchVectorStoresInnerChunkingStrategy {
    AutoChunkingStrategy(Box<models::AutoChunkingStrategy>),
    StaticChunkingStrategy(Box<models::StaticChunkingStrategy>),
}

impl Default for CreateAssistantRequestToolResourcesFileSearchVectorStoresInnerChunkingStrategy {
    fn default() -> Self {
        Self::AutoChunkingStrategy(Default::default())
    }
}
/// Always `auto`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "static")]
    Static,
}

impl Default for Type {
    fn default() -> Type {
        Self::Auto
    }
}

