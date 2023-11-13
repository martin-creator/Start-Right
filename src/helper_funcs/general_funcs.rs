use crate::helper_funcs::cli_funcs::PrintCommand;
use crate::models::general_http_content::llm_content_structure::Message;
use crate::llm_external_api_calls::api_calls::call_gpt;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fs;

// Webserve template
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


/// Performs call to LLM GPT, prints AI agent status and returns response
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    // Extend AI function
    let extended_msg: Message = extend_ai_function(function_pass, &msg_context);

    // Print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // Get LLM response
    let llm_response_res: Result<String, Box<dyn std::error::Error + Send>> =
        call_gpt(vec![extended_msg.clone()]).await;

    // Return Success or try again
    match llm_response_res {
        Ok(llm_resp) => llm_resp,
        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed twice to call OpenAI"),
    }
}

/// Performs call to LLM GPT and returns decoded response(GPT response into a structured Rust object)
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response: String =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;
    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode ai response from serde_json");
    return decoded_response;
}

/// Check whether request url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}



