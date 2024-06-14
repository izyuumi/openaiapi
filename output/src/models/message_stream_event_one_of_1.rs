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

/// MessageStreamEventOneOf1 : Occurs when a [message](/docs/api-reference/messages/object) moves to an `in_progress` state.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageStreamEventOneOf1 {
    #[serde(rename = "event")]
    pub event: Event,
    #[serde(rename = "data")]
    pub data: Box<models::MessageObject>,
}

impl MessageStreamEventOneOf1 {
    /// Occurs when a [message](/docs/api-reference/messages/object) moves to an `in_progress` state.
    pub fn new(event: Event, data: models::MessageObject) -> MessageStreamEventOneOf1 {
        MessageStreamEventOneOf1 {
            event,
            data: Box::new(data),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Event {
    #[serde(rename = "thread.message.in_progress")]
    ThreadPeriodMessagePeriodInProgress,
}

impl Default for Event {
    fn default() -> Event {
        Self::ThreadPeriodMessagePeriodInProgress
    }
}

