use crate::apis::call_request::call_gpt;
use crate::helpers::command_line::PrintCommand;
use crate::models::general::llm::Message;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fs;

/// Webserve template
pub const CODE_TEMPLATE_PATH: &str = "code_templates/main_websever_template.rs";

pub const INDEX_TEMPLATE_PATH: &str = "code_templates/index_template.html";

pub const WEB_SERVER_PROJECT_PATH: &str = "src/";

pub const EXEC_MAIN_PATH: &str = "src/main_web_template.rs";

pub const MAIN_INDEX_PATH: &str = "frontend/index.html";

pub const API_SCHEMA_PATH: &str = "schemas/api_schema.json";
