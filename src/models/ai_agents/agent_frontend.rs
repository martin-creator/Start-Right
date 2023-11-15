use crate::ai_macros::ai_frontend::{print_fixed_frontend_code, print_frontend_code, print_index_html_file, print_improved_frontend_code};
use crate::helper_funcs::general_funcs::{
    check_status_code, read_backend_code_template_contents, read_exec_main_contents, save_api_endpoints,
    save_backend_code, MAIN_INDEX_PATH
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
}
