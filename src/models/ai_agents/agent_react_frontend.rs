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

use crate::models::ai_agents::agent_react_component::BuildComponent;
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
    pub operation_focus: BuildComponent
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
        let backend_code: String = fs
            ::read_to_string(path)
            .expect("Something went wrong reading the file");

        // Structure Message
        let msg_context: String = format!(
            "PROJECT_DESCRIPTION: {:?}, CODE_LOGIC: {:?}",
            project_description,
            backend_code
        );

        // Call AI
        let ai_response: Vec<SitePages> = ai_task_request_decoded::<Vec<SitePages>>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_react_recommended_site_pages),
            print_react_recommended_site_pages
        ).await;

        // Extract pages
        let pages: Vec<String> = ai_response
            .iter()
            .filter_map(|item| Some(item.page_name.clone()))
            .collect();

        // Assign pages to buildsheet
        self.buildsheet.pages = Some(pages.clone());
        self.buildsheet.pages_descriptons = Some(ai_response);
    }

    /// Assign API Routes to pages
    async fn assign_api_routes(
        &mut self,
        project_description: &String,
        external_api_urls: &Option<Vec<String>>
    ) {
        // Extract internal API schema
        let path: String = String::from(API_SCHEMA_PATH);
        let internal_api_endpoints: String = fs
            ::read_to_string(path)
            .expect("Something went wrong reading the file");

        // Extract external API endpoints
        let external_api_endpoints: String = match external_api_urls {
            Some(endpoints) => format!("{:?}", endpoints),
            None => String::from(""),
        };

        // Structure message for api route assignment
        let msg_context: String = format!(
            "WEBSITE SPECIFICATION: {{
      PROJECT_DESCRIPTION: {},
      PAGES: {:?},
      INTERNAL_API_ROUTES: {},
      EXTERNAL_API_ROUTES: {} 
    }}",
            project_description,
            self.buildsheet.pages,
            internal_api_endpoints,
            external_api_endpoints
        );

        // Make AI request
        let ai_response: PageRoutes = ai_task_request_decoded::<PageRoutes>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_recommended_site_pages_with_apis),
            print_react_recommended_site_pages_with_apis
        ).await;

        // Add API assignments to buildsheet
        self.buildsheet.api_assignments = Some(ai_response);
    }

    /// Define Brand Colours
    async fn define_brand_colours(&mut self, project_description: &String) {
        // Structure message
        let msg_context: String = format!(
            "PROJECT_DESCRIPTION: {}, WEBSITE_CONTENT: {:?}",
            project_description,
            self.buildsheet.pages_descriptons
        );

        // Call AI
        let ai_response: Vec<String> = ai_task_request_decoded::<Vec<String>>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_react_recommended_site_main_colours),
            print_react_recommended_site_main_colours
        ).await;

        // Add decoded brand colours
        self.buildsheet.brand_colours = Some(ai_response);
    }

     /// Fix buggy component code
  async fn run_code_correction(&self, file_path: String, error_code: String) {

    // Initialize
    PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(), 
      "Fixing component bugs");
    let buggy_code: String = read_react_frontend_code_contents(&file_path);

    // Structure message
    let msg_context: String = format!("ORIGINAL_CODE: {}, ERROR_MESSAGE: {:?}", buggy_code, error_code);

    // Retrieve AI Reponse
    let ai_response: String = ai_task_request(
      msg_context, 
      &self.attributes.position, 
      get_function_string!(print_react_code_bugs_resolution), 
      print_react_code_bugs_resolution).await;

    // Save corrected code
    save_react_frontend_code(&file_path, &ai_response);
  }

   /// Frontend component test
   async fn perform_component_test(&mut self) -> Result<(), String> {

    let test_statement = format!("Testing Component: {}", self.operation_focus.name());

    PrintCommand::UnitTest.
    print_agent_message(
        self.attributes.position.as_str(), 
        test_statement.as_str());

    // Build frontend server
    let build_frontend_server: std::process::Output = Command::new("yarn")
      .arg("build")
      .current_dir(REACT_FRONTEND_PATH)
      .stdout(Stdio::piped())
      .stderr(Stdio::piped())
      .output()
      .expect("Failed to run component test");

    // Determine if build errors
    if build_frontend_server.status.success() {
      PrintCommand::UnitTest
      .print_agent_message(
        self.attributes.position.as_str(),
         "Component build test successful");
      self.bug_count = 0;
      return Ok(());

    // Handle Build error
    } else {
      let error_arr: Vec<u8> = build_frontend_server.stderr;
      let error_str: String = String::from_utf8(error_arr).unwrap();

      // Check and return error
      self.bug_count += 1;
      if self.bug_count >= 2 {
        PrintCommand::Issue
        .print_agent_message(
            self.attributes.position.as_str(), 
            "Too many code failures");
        PrintCommand::Issue
        .print_agent_message(
            self.attributes.position.as_str(), 
            "Remember: check frontend builds before retrying");
        panic!("Too many code failed attempts for {}", self.operation_focus.name());
      } else {
        return Err(error_str)
      }
    }
  }

}



#[async_trait]
impl SpecialFunctions for AgentReactFrontendDeveloper {

  fn get_attributes_from_agent(&self) -> &BasicAgent {
    &self.attributes
  }

  async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>> {

    // Extract required project factsheet items
    let project_description: &String = &factsheet.project_description;
    let external_api_urls: &Option<Vec<String>> = &factsheet.external_urls;

    // Continue until finished
    // !!! WARNING !!!
    while self.attributes.state != AgentState::Finished {

      // Execute logic based on Agent State
      match &self.attributes.state {

        // Get pages, api assignments and branding
        AgentState::Discovery => {

          // Confirm Stage
          self.confirm_stage();

          // Get pages and page context
          self.get_page_context(&project_description).await;

          // Assign API routes to pages
          self.assign_api_routes(&project_description, &external_api_urls).await;

          // Define Brand Colours
          self.define_brand_colours(&project_description).await;

          // Proceed to Working status
          self.attributes.state = AgentState::Working;
          continue;
        },

        // Get pages, api assignments and branding
        AgentState::Working => {
          
          // Loop through components
          for component in BuildComponent::iter() {
            

            // !!!! REMOVE ONLY FOR TESTING !!!
            if component != BuildComponent::PageContent1 {
              continue;
            }
            if component == BuildComponent::PageContent2 {
              break;
            }


            // Update current operation focus to component
            self.operation_focus = component.clone();
            component.create_component(&self, &project_description).await;

            // Unit test component
            let test_res: Result<(), String> = self.perform_component_test().await;
            match test_res {

              // Continue to next component
              Ok(()) => continue,

              // Fix bugs for current component
              Err(err_str) => {
                let file_path: String = self.operation_focus.filepath();
                self.run_code_correction(file_path, err_str).await;

                // Perform one more test
                let _ = self.perform_component_test().await;
                continue;
              }
            }
          }

          // Complete
          self.attributes.state = AgentState::Finished;
        },

        // Ensure all cases are covered
        _ => {}
      }
    }
    Ok(())
  }
}

