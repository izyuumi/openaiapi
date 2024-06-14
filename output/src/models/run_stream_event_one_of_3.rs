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

/// RunStreamEventOneOf3 : Occurs when a [run](/docs/api-reference/runs/object) moves to a `requires_action` status.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStreamEventOneOf3 {
    #[serde(rename = "event")]
    pub event: Event,
    #[serde(rename = "data")]
    pub data: Box<models::RunObject>,
}

impl RunStreamEventOneOf3 {
    /// Occurs when a [run](/docs/api-reference/runs/object) moves to a `requires_action` status.
    pub fn new(event: Event, data: models::RunObject) -> RunStreamEventOneOf3 {
        RunStreamEventOneOf3 {
            event,
            data: Box::new(data),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Event {
    #[serde(rename = "thread.run.requires_action")]
    ThreadPeriodRunPeriodRequiresAction,
}

impl Default for Event {
    fn default() -> Event {
        Self::ThreadPeriodRunPeriodRequiresAction
    }
}

