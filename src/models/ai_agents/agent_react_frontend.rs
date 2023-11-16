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
    EXEC_MAIN_PATH,
    API_SCHEMA_PATH,
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

    /// Get pages and page context from description and backend code
  async fn get_page_context(&mut self, project_description: &String) {

    // Extract main backend code
    let path: String = String::from(EXEC_MAIN_PATH);
    let backend_code: String = fs::read_to_string(path).expect("Something went wrong reading the file");

    // Structure Message
    let msg_context: String = format!("PROJECT_DESCRIPTION: {:?}, CODE_LOGIC: {:?}", project_description, backend_code);

    // Call AI
    let ai_response: Vec<SitePages> = ai_task_request_decoded::<Vec<SitePages>>(
      msg_context, 
      &self.attributes.position, 
      get_function_string!(print_react_recommended_site_pages), 
      print_react_recommended_site_pages).await;

    // Extract pages
    let pages: Vec<String> = ai_response
      .iter().filter_map(|item| Some(item.page_name.clone())).collect();

    // Assign pages to buildsheet
    self.buildsheet.pages = Some(pages.clone());
    self.buildsheet.pages_descriptons = Some(ai_response);
  }

  /// Assign API Routes to pages
  async fn assign_api_routes(&mut self, project_description: &String, external_api_urls: &Option<Vec<String>>) {

    // Extract internal API schema
    let path: String = String::from(API_SCHEMA_PATH);
    let internal_api_endpoints: String = fs::read_to_string(path).expect("Something went wrong reading the file");

    // Extract external API endpoints
    let external_api_endpoints: String = match external_api_urls {
      Some(endpoints) => format!("{:?}", endpoints),
      None => String::from("")
    };

    // Structure message for api route assignment
    let msg_context: String = format!("WEBSITE SPECIFICATION: {{
      PROJECT_DESCRIPTION: {},
      PAGES: {:?},
      INTERNAL_API_ROUTES: {},
      EXTERNAL_API_ROUTES: {} 
    }}", project_description, self.buildsheet.pages, internal_api_endpoints, external_api_endpoints);

    // Make AI request
    let ai_response: PageRoutes = ai_task_request_decoded::<PageRoutes>(
      msg_context, 
      &self.attributes.position, 
      get_function_string!(print_recommended_site_pages_with_apis), 
      print_react_recommended_site_pages_with_apis).await;

    // Add API assignments to buildsheet
    self.buildsheet.api_assignments = Some(ai_response);
  }
}
