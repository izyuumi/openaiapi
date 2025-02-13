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
pub struct CreateAssistantRequestToolResourcesFileSearchVectorStoresInner {
    /// A list of [file](/docs/api-reference/files) IDs to add to the vector store. There can be a maximum of 10000 files in a vector store. 
    #[serde(rename = "file_ids", skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    #[serde(rename = "chunking_strategy", skip_serializing_if = "Option::is_none")]
    pub chunking_strategy: Option<Box<models::CreateAssistantRequestToolResourcesFileSearchVectorStoresInnerChunkingStrategy>>,
    /// Set of 16 key-value pairs that can be attached to a vector store. This can be useful for storing additional information about the vector store in a structured format. Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long. 
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl CreateAssistantRequestToolResourcesFileSearchVectorStoresInner {
    pub fn new() -> CreateAssistantRequestToolResourcesFileSearchVectorStoresInner {
        CreateAssistantRequestToolResourcesFileSearchVectorStoresInner {
            file_ids: None,
            chunking_strategy: None,
            metadata: None,
        }
    }
}

