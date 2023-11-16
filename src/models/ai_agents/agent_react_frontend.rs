use crate::ai_macros::ai_react_frontend::{
    print_react_code_bugs_resolution,
    print_react_recommended_site_pages,
    print_react_recommended_site_pages_with_apis,
    print_react_recommended_site_main_colours,
};

use crate::helper_funcs::general_funcs::{
    save_react_frontend_code,
    ai_task_request_decoded,
    ai_task_request,
    read_react_frontend_code_contents,
    WEB_SERVER_PROJECT_PATH,
    REACT_FRONTEND_PATH,
};

//use crate::models::agents::agent_frontend_comp::BuildComponent;
use crate::models::ai_agents::agent_content_traits::{SpecialFunctions, FactSheet};
use crate::models::ai_agent_skeleton::basic_agent::{BasicAgent, AgentState};
use crate::helper_funcs::cli_funcs::PrintCommand;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::fs;
use std::process::{Command, Stdio};
use std::collections::HashMap;
use strum::IntoEnumIterator;
