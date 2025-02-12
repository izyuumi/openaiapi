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

/// RunObject : Represents an execution run on a [thread](/docs/api-reference/threads).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunObject {
    /// The identifier, which can be referenced in API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    /// The object type, which is always `thread.run`.
    #[serde(rename = "object")]
    pub object: Object,
    /// The Unix timestamp (in seconds) for when the run was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The ID of the [thread](/docs/api-reference/threads) that was executed on as a part of this run.
    #[serde(rename = "thread_id")]
    pub thread_id: String,
    /// The ID of the [assistant](/docs/api-reference/assistants) used for execution of this run.
    #[serde(rename = "assistant_id")]
    pub assistant_id: String,
    /// The status of the run, which can be either `queued`, `in_progress`, `requires_action`, `cancelling`, `cancelled`, `failed`, `completed`, `incomplete`, or `expired`.
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "required_action", deserialize_with = "Option::deserialize")]
    pub required_action: Option<Box<models::RunObjectRequiredAction>>,
    #[serde(rename = "last_error", deserialize_with = "Option::deserialize")]
    pub last_error: Option<Box<models::RunObjectLastError>>,
    /// The Unix timestamp (in seconds) for when the run will expire.
    #[serde(rename = "expires_at", deserialize_with = "Option::deserialize")]
    pub expires_at: Option<i32>,
    /// The Unix timestamp (in seconds) for when the run was started.
    #[serde(rename = "started_at", deserialize_with = "Option::deserialize")]
    pub started_at: Option<i32>,
    /// The Unix timestamp (in seconds) for when the run was cancelled.
    #[serde(rename = "cancelled_at", deserialize_with = "Option::deserialize")]
    pub cancelled_at: Option<i32>,
    /// The Unix timestamp (in seconds) for when the run failed.
    #[serde(rename = "failed_at", deserialize_with = "Option::deserialize")]
    pub failed_at: Option<i32>,
    /// The Unix timestamp (in seconds) for when the run was completed.
    #[serde(rename = "completed_at", deserialize_with = "Option::deserialize")]
    pub completed_at: Option<i32>,
    #[serde(rename = "incomplete_details", deserialize_with = "Option::deserialize")]
    pub incomplete_details: Option<Box<models::RunObjectIncompleteDetails>>,
    /// The model that the [assistant](/docs/api-reference/assistants) used for this run.
    #[serde(rename = "model")]
    pub model: String,
    /// The instructions that the [assistant](/docs/api-reference/assistants) used for this run.
    #[serde(rename = "instructions")]
    pub instructions: String,
    /// The list of tools that the [assistant](/docs/api-reference/assistants) used for this run.
    #[serde(rename = "tools")]
    pub tools: Vec<models::AssistantObjectToolsInner>,
    /// Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format. Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long. 
    #[serde(rename = "metadata", deserialize_with = "Option::deserialize")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "usage", deserialize_with = "Option::deserialize")]
    pub usage: Option<Box<models::RunCompletionUsage>>,
    /// The sampling temperature used for this run. If not set, defaults to 1.
    #[serde(rename = "temperature", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Option<f64>>,
    /// The nucleus sampling value used for this run. If not set, defaults to 1.
    #[serde(rename = "top_p", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub top_p: Option<Option<f64>>,
    /// The maximum number of prompt tokens specified to have been used over the course of the run. 
    #[serde(rename = "max_prompt_tokens", deserialize_with = "Option::deserialize")]
    pub max_prompt_tokens: Option<i32>,
    /// The maximum number of completion tokens specified to have been used over the course of the run. 
    #[serde(rename = "max_completion_tokens", deserialize_with = "Option::deserialize")]
    pub max_completion_tokens: Option<i32>,
    #[serde(rename = "truncation_strategy")]
    pub truncation_strategy: Box<models::TruncationObject>,
    #[serde(rename = "tool_choice")]
    pub tool_choice: Box<models::AssistantsApiToolChoiceOption>,
    /// Whether to enable [parallel function calling](/docs/guides/function-calling/parallel-function-calling) during tool use.
    #[serde(rename = "parallel_tool_calls")]
    pub parallel_tool_calls: bool,
    #[serde(rename = "response_format")]
    pub response_format: Box<models::AssistantsApiResponseFormatOption>,
}

impl RunObject {
    /// Represents an execution run on a [thread](/docs/api-reference/threads).
    pub fn new(id: String, object: Object, created_at: i32, thread_id: String, assistant_id: String, status: Status, required_action: Option<models::RunObjectRequiredAction>, last_error: Option<models::RunObjectLastError>, expires_at: Option<i32>, started_at: Option<i32>, cancelled_at: Option<i32>, failed_at: Option<i32>, completed_at: Option<i32>, incomplete_details: Option<models::RunObjectIncompleteDetails>, model: String, instructions: String, tools: Vec<models::AssistantObjectToolsInner>, metadata: Option<serde_json::Value>, usage: Option<models::RunCompletionUsage>, max_prompt_tokens: Option<i32>, max_completion_tokens: Option<i32>, truncation_strategy: models::TruncationObject, tool_choice: models::AssistantsApiToolChoiceOption, parallel_tool_calls: bool, response_format: models::AssistantsApiResponseFormatOption) -> RunObject {
        RunObject {
            id,
            object,
            created_at,
            thread_id,
            assistant_id,
            status,
            required_action: if let Some(x) = required_action {Some(Box::new(x))} else {None},
            last_error: if let Some(x) = last_error {Some(Box::new(x))} else {None},
            expires_at,
            started_at,
            cancelled_at,
            failed_at,
            completed_at,
            incomplete_details: if let Some(x) = incomplete_details {Some(Box::new(x))} else {None},
            model,
            instructions,
            tools,
            metadata,
            usage: if let Some(x) = usage {Some(Box::new(x))} else {None},
            temperature: None,
            top_p: None,
            max_prompt_tokens,
            max_completion_tokens,
            truncation_strategy: Box::new(truncation_strategy),
            tool_choice: Box::new(tool_choice),
            parallel_tool_calls,
            response_format: Box::new(response_format),
        }
    }
}
/// The object type, which is always `thread.run`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "thread.run")]
    ThreadPeriodRun,
}

impl Default for Object {
    fn default() -> Object {
        Self::ThreadPeriodRun
    }
}
/// The status of the run, which can be either `queued`, `in_progress`, `requires_action`, `cancelling`, `cancelled`, `failed`, `completed`, `incomplete`, or `expired`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "requires_action")]
    RequiresAction,
    #[serde(rename = "cancelling")]
    Cancelling,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "expired")]
    Expired,
}

impl Default for Status {
    fn default() -> Status {
        Self::Queued
    }
}

