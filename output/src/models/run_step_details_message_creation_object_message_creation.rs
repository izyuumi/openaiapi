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
pub struct RunStepDetailsMessageCreationObjectMessageCreation {
    /// The ID of the message that was created by this run step.
    #[serde(rename = "message_id")]
    pub message_id: String,
}

impl RunStepDetailsMessageCreationObjectMessageCreation {
    pub fn new(message_id: String) -> RunStepDetailsMessageCreationObjectMessageCreation {
        RunStepDetailsMessageCreationObjectMessageCreation {
            message_id,
        }
    }
}
