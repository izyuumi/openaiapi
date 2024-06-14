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
pub struct BatchErrorsDataInner {
    /// An error code identifying the error type.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// A human-readable message providing more details about the error.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The name of the parameter that caused the error, if applicable.
    #[serde(rename = "param", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub param: Option<Option<String>>,
    /// The line number of the input file where the error occurred, if applicable.
    #[serde(rename = "line", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub line: Option<Option<i32>>,
}

impl BatchErrorsDataInner {
    pub fn new() -> BatchErrorsDataInner {
        BatchErrorsDataInner {
            code: None,
            message: None,
            param: None,
            line: None,
        }
    }
}

