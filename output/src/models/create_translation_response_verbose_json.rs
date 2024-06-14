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
pub struct CreateTranslationResponseVerboseJson {
    /// The language of the output translation (always `english`).
    #[serde(rename = "language")]
    pub language: String,
    /// The duration of the input audio.
    #[serde(rename = "duration")]
    pub duration: String,
    /// The translated text.
    #[serde(rename = "text")]
    pub text: String,
    /// Segments of the translated text and their corresponding details.
    #[serde(rename = "segments", skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<models::TranscriptionSegment>>,
}

impl CreateTranslationResponseVerboseJson {
    pub fn new(language: String, duration: String, text: String) -> CreateTranslationResponseVerboseJson {
        CreateTranslationResponseVerboseJson {
            language,
            duration,
            text,
            segments: None,
        }
    }
}

