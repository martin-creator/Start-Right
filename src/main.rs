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

use helper_funcs::cli_funcs::get_user_response;
use models::agents_manager::agent_manager::ManagingAgent;


fn print_welcome_message() {
    println!(
        r#"
🤖 ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ 🚀
                                Welcome to StartRight - AI Kickstart! 🛠️

     Developers, focus on creativity, not syntax! 🎨
     Kickstart your projects with ready-to-use templates and accelerate your development.
     Our AI-driven app automates boilerplate code, making coding efficient and enjoyable.

     Currently Supporting: Rust Backend with Actix. 🦀

     Join us in enhancing StartRight! Let's innovate together!
    ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
"#
    );
}

fn print_success_message() {
    println!(
        r#"
🚀 ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ 🎉
                                        Project Successfully Initialized! ✨

     Your project is ready to roll with StartRight! 🚀 Happy coding!
   ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
"#
    );
}

fn print_support_message() {
    println!(
        r#"
🦀 ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ⚙️
                                                Rust + Actix Support

     *Currently supporting Rust backend with Actix!
    ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
"#
    );
}

// A simple AI-inspired logo
fn print_logo() {
    println!(
        r#"
     🤖🚀 
                                                        ╭──────╮
                                                        │ S.AI │
                                                        ╰──────╯
"#
    );
}

#[tokio::main]
async fn main() {
    print_logo();
    print_welcome_message();


    

    // Get user input for the project name
    let usr_req: String = get_user_response("What project are we starting today?");

    // Do you want to build a full stack app or just a backend?
    let usr_app_type: String = get_user_response("What type of app are we building today?");

    // add logic to determine if we are building a full stack app or just a backend
    
    // Create and initialize the ManagingAgent
    let mut manage_agent: ManagingAgent = ManagingAgent::new(usr_req)
        .await
        .expect("Error creating agent");

    // Execute the project initiation
    manage_agent.execute_project().await;

    print_success_message();
    print_support_message();
    print_logo();
}


