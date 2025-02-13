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
pub struct MessageContentTextAnnotationsFileCitationObjectFileCitation {
    /// The ID of the specific File the citation is from.
    #[serde(rename = "file_id")]
    pub file_id: String,
}

impl MessageContentTextAnnotationsFileCitationObjectFileCitation {
    pub fn new(file_id: String) -> MessageContentTextAnnotationsFileCitationObjectFileCitation {
        MessageContentTextAnnotationsFileCitationObjectFileCitation {
            file_id,
        }
    }
}

