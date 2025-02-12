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
pub struct StaticChunkingStrategyStatic {
    /// The maximum number of tokens in each chunk. The default value is `800`. The minimum value is `100` and the maximum value is `4096`.
    #[serde(rename = "max_chunk_size_tokens")]
    pub max_chunk_size_tokens: i32,
    /// The number of tokens that overlap between chunks. The default value is `400`.  Note that the overlap must not exceed half of `max_chunk_size_tokens`. 
    #[serde(rename = "chunk_overlap_tokens")]
    pub chunk_overlap_tokens: i32,
}

impl StaticChunkingStrategyStatic {
    pub fn new(max_chunk_size_tokens: i32, chunk_overlap_tokens: i32) -> StaticChunkingStrategyStatic {
        StaticChunkingStrategyStatic {
            max_chunk_size_tokens,
            chunk_overlap_tokens,
        }
    }
}

