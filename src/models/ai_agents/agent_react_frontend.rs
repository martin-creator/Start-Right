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

/// To define what stage the frontend developer is at
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FrontendBuildMode {
  Infrastructure,
  PageComponents,
  Completion
}


/// For decoding the serde_json api routes for a given page
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct APIAssignment {
  pub api_route: String,
  pub method: String,
  pub route_type: String,
}


/// Used for creating a type to be used for decoding shorthand
type PageRoutes = HashMap<String, Vec<APIAssignment>>;


/// Used for decoding page names and suggested content
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageAPIAssign {
  pub page: Vec<APIAssignment>
}


/// Used for decoding page names and suggested content
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SitePages {
  pub page_name: String,
  pub suggested_content_sections: serde_json::Value
}


/// Used for DesignBuildSheet
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DesignBuildSheet {
  pub pages: Option<Vec<String>>,
  pub pages_descriptons: Option<Vec<SitePages>>,
  pub api_assignments: Option<PageRoutes>,
  pub brand_colours: Option<Vec<String>>,
  pub build_mode: FrontendBuildMode
}


// React Frontend Developer
#[derive(Debug)]
pub struct AgentFrontendDeveloper {
  pub attributes: BasicAgent,
  pub buildsheet: DesignBuildSheet,
  pub bug_count: u8,
  pub operation_focus: BuildComponent
}