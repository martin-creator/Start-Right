use crate::apis::call_request::call_gpt;
use crate::helpers::command_line::PrintCommand;
use crate::models::general::llm::Message;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fs;