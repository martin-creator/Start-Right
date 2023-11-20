#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {{
        stringify!($func)
    }};
}

#[macro_use]
mod ai_macros;
mod llm_external_api_calls;
mod helper_funcs;
mod models;

use helper_funcs::cli_funcs::{get_user_response, confirm_type_app, client_on_boarding, run_after_initialization};
use models::agents_manager::agent_manager::ManagingAgent;






#[tokio::main]
async fn main() {
    // print_logo();
    // print_welcome_message();

    client_on_boarding();
    // Get user input for the project name
    let usr_req: String = get_user_response("What project are we website are we building?");

    // Do you want to build a full stack app or just a backend?
    let usr_app_type:&str = confirm_type_app();


     // Create and initialize the ManagingAgent
    let mut manage_agent: ManagingAgent = ManagingAgent::new(usr_req)
        .await
        .expect("Error creating agent");

    if usr_app_type == "fullstack" {
       
        println!("StarRight is building for you a  {} application\n", usr_app_type);
        // Execute the project initiation
        manage_agent.execute_project().await;


    } else if usr_app_type == "backend"  {
        // Execute the project initiation
        println!("StarRight is building for you a  {} application\n", usr_app_type);
        manage_agent.execute_project_backend().await;

    } else {
        println!("You selected: {}, we are redirecting to the beginning\n", usr_app_type);
        // Recall user input function
        let _usr_req: String = get_user_response("What project are we website are we building?");

    }





    run_after_initialization();
}


