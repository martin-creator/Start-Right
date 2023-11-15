use crate::ai_macros::ai_frontend::{print_fixed_frontend_code, print_frontend_code, print_index_html_file, print_improved_frontend_code};
use crate::helper_funcs::general_funcs::{
    check_status_code, read_frontend_code_template_contents,   read_main_index_contents,  save_frontend_code, read_api_json_contents, MAIN_INDEX_PATH
};

use crate::helper_funcs::cli_funcs::{confirm_safe_code, PrintCommand};
use crate::helper_funcs::general_funcs::ai_task_request;
use crate::models::ai_agent_skeleton::basic_agent::{AgentState, BasicAgent};
use crate::models::ai_agents::agent_content_traits::{FactSheet, RouteObject, SpecialFunctions};

use async_trait::async_trait;
use reqwest::Client;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time;

#[derive(Debug)]
pub struct AgentFrontendDeveloper {
    attributes: BasicAgent,
    bug_errors: Option<String>,
    bug_count: u8,
}

impl AgentFrontendDeveloper {
    pub fn new() -> Self {
        let attributes: BasicAgent = BasicAgent {
            objective: "Develops fronted code for website using API_JSON Schema and prints out index.html file".to_string(),
            position: "Frontend Developer".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        Self {
            attributes,
            bug_errors: None,
            bug_count: 0,
        }
    }

    async fn call_initial_frontend_code(&mut self, factsheet: &mut FactSheet) {
        let index_template_str: String = read_frontend_code_template_contents();
        let api_json_str: String = read_api_json_contents();

        //PROJECT_DESCRIPTION, INDEX_TEMPLATE and API_JSON Schema
        // Concatenate Instruction
        let msg_context: String = format!(
            "INDEX_TEMPLATE: {} \n  API_JSON Schema {} \n PROJECT_DESCRIPTION: {} \n",
            index_template_str, api_json_str, factsheet.project_description
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_frontend_code),
            print_frontend_code,
        )
        .await;

        save_frontend_code(&ai_response);
        factsheet.frontend_code = Some(ai_response);
    }
}
