use crate::ai_macros::ai_backend::{
    print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
    print_rest_api_endpoints,
};
use crate::helper_funcs::general_funcs::{
    check_status_code, read_backend_code_template_contents, read_exec_main_contents, save_api_endpoints,
    save_backend_code, WEB_SERVER_PROJECT_PATH,
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
pub struct AgentBackendDeveloper {
    attributes: BasicAgent,
    bug_errors: Option<String>,
    bug_count: u8,
}