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
pub struct CreateVectorStoreFileBatchRequest {
    /// A list of [File](/docs/api-reference/files) IDs that the vector store should use. Useful for tools like `file_search` that can access files.
    #[serde(rename = "file_ids")]
    pub file_ids: Vec<String>,
    #[serde(rename = "chunking_strategy", skip_serializing_if = "Option::is_none")]
    pub chunking_strategy: Option<Box<models::ChunkingStrategyRequestParam>>,
}

impl CreateVectorStoreFileBatchRequest {
    pub fn new(file_ids: Vec<String>) -> CreateVectorStoreFileBatchRequest {
        CreateVectorStoreFileBatchRequest {
            file_ids,
            chunking_strategy: None,
        }
    }
}
