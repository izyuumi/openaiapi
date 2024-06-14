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

/// RunStepDetailsToolCallsFunctionObjectFunction : The definition of the function that was called.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDetailsToolCallsFunctionObjectFunction {
    /// The name of the function.
    #[serde(rename = "name")]
    pub name: String,
    /// The arguments passed to the function.
    #[serde(rename = "arguments")]
    pub arguments: String,
    /// The output of the function. This will be `null` if the outputs have not been [submitted](/docs/api-reference/runs/submitToolOutputs) yet.
    #[serde(rename = "output", deserialize_with = "Option::deserialize")]
    pub output: Option<String>,
}

impl RunStepDetailsToolCallsFunctionObjectFunction {
    /// The definition of the function that was called.
    pub fn new(name: String, arguments: String, output: Option<String>) -> RunStepDetailsToolCallsFunctionObjectFunction {
        RunStepDetailsToolCallsFunctionObjectFunction {
            name,
            arguments,
            output,
        }
    }
}
