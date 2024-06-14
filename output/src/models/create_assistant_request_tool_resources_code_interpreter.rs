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
pub struct CreateAssistantRequestToolResourcesCodeInterpreter {
    /// A list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool. 
    #[serde(rename = "file_ids", skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
}

impl CreateAssistantRequestToolResourcesCodeInterpreter {
    pub fn new() -> CreateAssistantRequestToolResourcesCodeInterpreter {
        CreateAssistantRequestToolResourcesCodeInterpreter {
            file_ids: None,
        }
    }
}

