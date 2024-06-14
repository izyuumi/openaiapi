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
pub struct CreateThreadRequest {
    /// A list of [messages](/docs/api-reference/messages) to start the thread with.
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<models::CreateMessageRequest>>,
    #[serde(rename = "tool_resources", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tool_resources: Option<Option<Box<models::CreateThreadRequestToolResources>>>,
    /// Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format. Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long. 
    #[serde(rename = "metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Option<serde_json::Value>>,
}

impl CreateThreadRequest {
    pub fn new() -> CreateThreadRequest {
        CreateThreadRequest {
            messages: None,
            tool_resources: None,
            metadata: None,
        }
    }
}

