use crate::helper_funcs::cli_funcs::PrintCommand;
use crate::models::general_http_content::llm_content_structure::Message;
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


/// Extended AI function to encourage specific output from LLMS
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str: &str = ai_func(func_input);

    // Extend the string to encourage only printing the output
    let msg: String = format!(
        "FUNCTION: {}
  INSTRUCTION: You are a function printer. You ONLY print the results of functions.
  Nothing else. No commentary. Here is the input to the function: {}.
  Print out what the function will return.",
        ai_function_str, func_input
    );

    // Return message
    Message {
        role: "system".to_string(),
        content: msg,
    }
}

