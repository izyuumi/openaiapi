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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RunStreamEvent {
    RunStreamEventOneOf(Box<models::RunStreamEventOneOf>),
    RunStreamEventOneOf1(Box<models::RunStreamEventOneOf1>),
    RunStreamEventOneOf2(Box<models::RunStreamEventOneOf2>),
    RunStreamEventOneOf3(Box<models::RunStreamEventOneOf3>),
    RunStreamEventOneOf4(Box<models::RunStreamEventOneOf4>),
    RunStreamEventOneOf5(Box<models::RunStreamEventOneOf5>),
    RunStreamEventOneOf6(Box<models::RunStreamEventOneOf6>),
    RunStreamEventOneOf7(Box<models::RunStreamEventOneOf7>),
    RunStreamEventOneOf8(Box<models::RunStreamEventOneOf8>),
    RunStreamEventOneOf9(Box<models::RunStreamEventOneOf9>),
}

impl Default for RunStreamEvent {
    fn default() -> Self {
        Self::RunStreamEventOneOf(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Event {
    #[serde(rename = "thread.run.expired")]
    ThreadPeriodRunPeriodExpired,
}

impl Default for Event {
    fn default() -> Event {
        Self::ThreadPeriodRunPeriodExpired
    }
}

