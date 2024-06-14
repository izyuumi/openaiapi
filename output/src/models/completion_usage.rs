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

/// CompletionUsage : Usage statistics for the completion request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompletionUsage {
    /// Number of tokens in the generated completion.
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: i32,
    /// Number of tokens in the prompt.
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: i32,
    /// Total number of tokens used in the request (prompt + completion).
    #[serde(rename = "total_tokens")]
    pub total_tokens: i32,
}

impl CompletionUsage {
    /// Usage statistics for the completion request.
    pub fn new(completion_tokens: i32, prompt_tokens: i32, total_tokens: i32) -> CompletionUsage {
        CompletionUsage {
            completion_tokens,
            prompt_tokens,
            total_tokens,
        }
    }
}

