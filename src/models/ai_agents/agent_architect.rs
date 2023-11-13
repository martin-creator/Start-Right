use crate::ai_macros::ai_architect::{print_project_scope, print_site_urls};
use crate::helper_funcs::cli_funcs::PrintCommand;
use crate::helper_funcs::general_funcs::{ai_task_request_decoded, check_status_code};
use crate::models::ai_agent_skeleton::basic_agent::{AgentState, BasicAgent};
use crate::models::ai_agent_skeleton::basic_traits::BasicTraits;
use crate::models::ai_agents::agent_content_traits::{FactSheet, ProjectScope, SpecialFunctions};