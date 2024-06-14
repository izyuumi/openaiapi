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

/// FinetuneCompletionRequestInput : The per-line training example of a fine-tuning input file for completions models
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FinetuneCompletionRequestInput {
    /// The input prompt for this training example.
    #[serde(rename = "prompt", skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// The desired completion for this training example.
    #[serde(rename = "completion", skip_serializing_if = "Option::is_none")]
    pub completion: Option<String>,
}

impl FinetuneCompletionRequestInput {
    /// The per-line training example of a fine-tuning input file for completions models
    pub fn new() -> FinetuneCompletionRequestInput {
        FinetuneCompletionRequestInput {
            prompt: None,
            completion: None,
        }
    }
}

