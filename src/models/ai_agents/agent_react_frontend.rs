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

use crate::models::ai_agents::agent_react_structs::{
    FrontendBuildMode,
    APIAssignment,
    PageAPIAssign,
    SitePages,
    PageRoutes,
    DesignBuildSheet,
};

//use crate::models::agents::agent_frontend_comp::BuildComponent;
use crate::models::ai_agents::agent_content_traits::{ SpecialFunctions, FactSheet };
use crate::models::ai_agent_skeleton::basic_agent::{ BasicAgent, AgentState };
use crate::helper_funcs::cli_funcs::PrintCommand;
use async_trait::async_trait;
use serde::{ Serialize, Deserialize };
use std::fs;
use std::process::{ Command, Stdio };
use std::collections::HashMap;
use strum::IntoEnumIterator;

// use super::agent_react_structs::FrontendBuildMode;

// React Frontend Developer
#[derive(Debug)]
pub struct AgentReactFrontendDeveloper {
    pub attributes: BasicAgent,
    pub buildsheet: DesignBuildSheet,
    pub bug_count: u8,
    //pub operation_focus: BuildComponent
}

impl AgentReactFrontendDeveloper {
    pub fn new() -> Self {
        // Define attributes
        let attributes: BasicAgent = BasicAgent {
            objective: "Develops react frontned code for website".to_string(),
            position: "React Frontend Developer".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        // Define Buildsheet
        let buildsheet: DesignBuildSheet = DesignBuildSheet {
            pages: None,
            pages_descriptons: None,
            api_assignments: None,
            brand_colours: None,
            build_mode: FrontendBuildMode::Infrastructure,
        };

        // Return Self
        Self {
            attributes,
            buildsheet,
            bug_count: 0,
            //operation_focus: BuildComponent::Logo
        }
    }

    /// Confirms what stage the Frontend Agent is in
    fn confirm_stage(&self) {
        match self.buildsheet.build_mode {
            FrontendBuildMode::Infrastructure => println!("[Working on Frontend Infrastructure]"),
            FrontendBuildMode::PageComponents => println!("[Working on Frontend Page Components]"),
            FrontendBuildMode::Completion => println!("[Working on Frontend Completion Items]"),
        }
    }
}
