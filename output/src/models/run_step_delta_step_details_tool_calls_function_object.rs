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
pub struct RunStepDeltaStepDetailsToolCallsFunctionObject {
    /// The index of the tool call in the tool calls array.
    #[serde(rename = "index")]
    pub index: i32,
    /// The ID of the tool call object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of tool call. This is always going to be `function` for this type of tool call.
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "function", skip_serializing_if = "Option::is_none")]
    pub function: Option<Box<models::RunStepDeltaStepDetailsToolCallsFunctionObjectFunction>>,
}

impl RunStepDeltaStepDetailsToolCallsFunctionObject {
    pub fn new(index: i32, r#type: Type) -> RunStepDeltaStepDetailsToolCallsFunctionObject {
        RunStepDeltaStepDetailsToolCallsFunctionObject {
            index,
            id: None,
            r#type,
            function: None,
        }
    }
}
/// The type of tool call. This is always going to be `function` for this type of tool call.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "function")]
    Function,
}

impl Default for Type {
    fn default() -> Type {
        Self::Function
    }
}

