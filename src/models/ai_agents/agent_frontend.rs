use crate::ai_macros::ai_frontend::{print_fixed_frontend_code, print_frontend_code, print_index_html_file, print_improved_frontend_code};
use crate::helper_funcs::general_funcs::{
    read_frontend_code_template_contents,   read_main_index_contents,  save_frontend_code, read_api_json_contents, WEB_SERVER_PROJECT_PATH
};

use crate::helper_funcs::cli_funcs::{confirm_safe_code, PrintCommand};
use crate::helper_funcs::general_funcs::ai_task_request;
use crate::models::ai_agent_skeleton::basic_agent::{AgentState, BasicAgent};
use crate::models::ai_agents::agent_content_traits::{FactSheet,  SpecialFunctions};

use async_trait::async_trait;
// use reqwest::Client;
use std::process::{Command, Stdio};
// use std::time::Duration;
// use tokio::time;

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

    async fn call_improved_frontend_code(&mut self, factsheet: &mut FactSheet) {

        let msg_context: String = format!(
            "INDEX_TEMPLATE: {:?} \n  API_JSON Schema {:?} \n PROJECT_DESCRIPTION: {:?} \n",
            factsheet.frontend_code, factsheet.api_endpoint_schema, factsheet
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_improved_frontend_code),
            print_improved_frontend_code,
        )
        .await;

        save_frontend_code(&ai_response);
        factsheet.frontend_code = Some(ai_response);
    }

    async fn call_fix_frontend_code_bugs(&mut self, factsheet: &mut FactSheet) {
        let msg_context: String = format!(
            "BROKEN_CODE: {:?} \n ERROR_BUGS: {:?} \n
      THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE.",
            factsheet.frontend_code, self.bug_errors
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_fixed_frontend_code),
            print_fixed_frontend_code,
        )
        .await;

        save_frontend_code(&ai_response);
        factsheet.frontend_code = Some(ai_response);
    }

    async fn call_extract_index_html_code(&self) -> String {
        let frontend_code: String = read_main_index_contents();

        // Structure message context
        let msg_context: String = format!("CODE_INPUT: {}", frontend_code);

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_index_html_file),
            print_index_html_file,
        )
        .await;

        ai_response
    }
}

#[async_trait]
impl SpecialFunctions for AgentFrontendDeveloper {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.state != AgentState::Finished {
            match &self.attributes.state {
                AgentState::Discovery => {
                    self.call_initial_frontend_code(factsheet).await;
                    self.attributes.state = AgentState::Working;
                    continue;
                }

                AgentState::Working => {
                    if self.bug_count == 0 {
                        self.call_improved_frontend_code(factsheet).await;
                    } else {
                        self.call_fix_frontend_code_bugs(factsheet).await;
                    }
                    self.attributes.state = AgentState::UnitTesting;
                    continue;
                }

                AgentState::UnitTesting => {
                    // Guard:: ENSURE AI SAFETY
                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "About to run both frontend and backend code : Requesting user input",
                    );

                    let is_safe_code: bool = confirm_safe_code();

                    if !is_safe_code {
                        panic!("Better go work on some AI alignment instead...")
                    }

                    // Build and Test Code
                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Frontend and Backend code compilation: building project...",
                    );

                    // Build Code
                    let build_backend_server: std::process::Output = Command::new("cargo")
                        .arg("build")
                        .current_dir(WEB_SERVER_PROJECT_PATH)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output()
                        .expect("Failed to build backend application");

                    // Determine if build errors
                    if build_backend_server.status.success() {
                        self.bug_count = 0;
                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.position.as_str(),
                            " Frontend & Backend Code Unit Testing: Test server build successful...",
                        );
                    } else {
                        let error_arr: Vec<u8> = build_backend_server.stderr;
                        let error_str: String = String::from_utf8(error_arr).unwrap();

                        // Update error stats
                        self.bug_count += 1;
                        self.bug_errors = Some(error_str);

                        // Exit if too many bugs
                        if self.bug_count > 2 {
                            PrintCommand::Issue.print_agent_message(
                                self.attributes.position.as_str(),
                                " Frontend Code Testing: Too many bugs found in code",
                            );
                            panic!("Error: Too many bugs")
                        }

                        // Pass back for rework
                        self.attributes.state = AgentState::Working;
                        continue;
                    }

                    /*
                      Extract Index HTML file
                    */

                    // Extract API Endpoints
                    let index_html_str: String = self.call_extract_index_html_code().await;

                    // // Convert API Endpoints into Values
                    // let api_endpoints: Vec<RouteObject> =
                    //     serde_json::from_str(api_endpoints_str.as_str())
                    //         .expect("Failed to decode API Endpoints");

                    // // Define endpoints to check
                    // let check_endpoints: Vec<RouteObject> = api_endpoints
                    //     .iter()
                    //     .filter(|&route_object| {
                    //         route_object.method == "get" && route_object.is_route_dynamic == "false"
                    //     })
                    //     .cloned()
                    //     .collect();

                    // Store frontend code
                    factsheet.frontend_code = Some(index_html_str.clone());

                    // Run backend application
                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Restarting Backend Server: Starting web server...",
                    );

                    // Execute running server
                    let mut run_backend_server: std::process::Child = Command::new("cargo")
                        .arg("run --bin main_web_server")
                        .current_dir(WEB_SERVER_PROJECT_PATH)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("Failed to run backend application");

                    // // Let user know testing on server will take place soon
                    // PrintCommand::UnitTest.print_agent_message(
                    //     self.attributes.position.as_str(),
                    //     "Backend Code Unit Testing: Launching tests on server in 5 seconds... check http://localhost:8080/ to see your AI generated home page",
                    // );

                    // let seconds_sleep: Duration = Duration::from_secs(5);
                    //  time::sleep(seconds_sleep).await;

                    // // Check status code
                    // for endpoint in check_endpoints {
                    //     // Confirm url testing
                    //     let testing_msg: String =
                    //         format!("Testing endpoint '{}'...", endpoint.route);
                    //     PrintCommand::UnitTest.print_agent_message(
                    //         self.attributes.position.as_str(),
                    //         testing_msg.as_str(),
                    //     );

                        // // Create client with timout
                        // let client: Client = Client::builder()
                        //     .timeout(Duration::from_secs(5))
                        //     .build()
                        //     .unwrap();

                    //     // Test url
                    //     let url: String = format!("http://localhost:8080{}", endpoint.route);
                    //     match check_status_code(&client, &url).await {
                    //         Ok(status_code) => {
                    //             if status_code != 200 {
                    //                 let err_msg: String = format!(
                    //                     "WARNING: Failed to call backend url endpoint {}",
                    //                     endpoint.route
                    //                 );
                    //                 PrintCommand::Issue.print_agent_message(
                    //                     self.attributes.position.as_str(),
                    //                     err_msg.as_str(),
                    //                 );
                    //             }
                    //         }
                    //         Err(e) => {
                    //             // kill $(lsof -t -i:8080)
                    //             run_backend_server
                    //                 .kill()
                    //                 .expect("Failed to kill backend web server");
                    //             let err_msg: String = format!("Error checking backend {}", e);
                    //             PrintCommand::Issue.print_agent_message(
                    //                 self.attributes.position.as_str(),
                    //                 err_msg.as_str(),
                    //             );
                    //         }
                    //     }
                    // }

                    save_frontend_code(&index_html_str);

                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Frontend  code saving complete... Run 'cargo run --bin main_web_template' in your terminal and then visit http://localhost:8080/ to see your AI generated home page",
                    );

                    // run_backend_server
                    //     .kill()
                    //     .expect("Failed to kill backend web server on completion");

                    self.attributes.state = AgentState::Finished;
                }

                _ => {}
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_frontend_developer() {
        let mut agent: AgentFrontendDeveloper = AgentFrontendDeveloper::new();

        let factsheet_str: &str = r#"
      {
        "project_description": "build a website that fetches and tracks fitness progress with timezone information",
        "project_scope": {
          "is_crud_required": true,
          "is_user_login_and_logout": true,
          "is_external_urls_required": true
        },
        "external_urls": [
          "http://worldtimeapi.org/api/timezone"
        ],
        "frontend_code": null,
        "api_endpoint_schema": null
      }"#;

        let mut factsheet: FactSheet = serde_json::from_str(factsheet_str).unwrap();

        agent.attributes.state = AgentState::Discovery;
        agent
            .execute(&mut factsheet)
            .await
            .expect("Failed to execute Frontend Developer agent");
    }
}