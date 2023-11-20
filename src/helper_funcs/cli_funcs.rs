use crossterm::{
    style::{
        Color,
        ResetColor,
        Print,
        SetForegroundColor,
        SetAttribute
    },
    ExecutableCommand,
};
use std::io::{ stdin, stdout, Write };

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    /// Prints  AI agent message in a specific color
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        // Decide on the print color
        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        // Print the agent statement in a specific color
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_pos);

        // Make selected color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        // Reset color
        stdout.execute(ResetColor).unwrap();
    }
}

/// Get human user request from termainal
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // Print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    // Reset Color
    stdout.execute(ResetColor).unwrap();

    // Read user input
    let mut user_response: String = String::new();
    stdin().read_line(&mut user_response).expect("Failed to read response");

    // Trim whitespace and return
    return user_response.trim().to_string();
}

/// Get human user response that code is safe to execute from terminal
pub fn confirm_safe_code() -> bool {
    let mut stdout: std::io::Stdout = stdout();
    loop {
        // Print the question in specified color
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!("");
        print!("WARNING: You are about to run code written entirely by AI. ");
        println!("Review your code and confirm you wish to continue.");

        // Reset Color
        stdout.execute(ResetColor).unwrap();

        //Present Options with different colors
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] All good");
        stdout.execute(SetForegroundColor(Color::DarkRed)).unwrap();
        println!("[2] Lets stop this project");

        // Reset Color
        stdout.execute(ResetColor).unwrap();

        // Read user input
        let mut human_response: String = String::new();
        stdin().read_line(&mut human_response).expect("Failed to read response");

        // Trim whitespace and convert to lowercase
        let human_response: String = human_response.trim().to_lowercase();

        // Match response
        match human_response.as_str() {
            "1" | "ok" | "y" => {
                return true;
            }
            "2" | "no" | "n" => {
                return false;
            }
            _ => {
                println!("Invalid input. Please select '1' or '2'");
            }
        }
    }
}

/// Get human user response that code is safe to execute from terminal
pub fn confirm_type_app() -> &'static str {
    let mut stdout: std::io::Stdout = stdout();
    loop {
        // Print the question in specified color
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!("");
        print!("What kind of app are we building today? (Respond with numbers) \n");
        // Reset Color
        stdout.execute(ResetColor).unwrap();

        //Present Options with different colors
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!(
            "[1] Fullstack App [Fronted(HTML, JAVASCRIPT, TAILWINDCSS)+ (REACT) + Backend(RUST, ACTIX)]"
        );
        stdout.execute(SetForegroundColor(Color::DarkRed)).unwrap();
        println!("[2] BACKEND APP [RUST, ACTIX]");
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!("[3] Rewrite prompt for generating web app");

        // Reset Color
        stdout.execute(ResetColor).unwrap();

        // Read user input
        let mut human_response: String = String::new();
        stdin().read_line(&mut human_response).expect("Failed to read response");

        // Trim whitespace and convert to lowercase
        let human_response: String = human_response.trim().to_lowercase();

        // Match response
        match human_response.as_str() {
            "1" => return "fullstack",
            "2" =>  return "backend",
            "3" =>  return "restart",
            _ => {
                println!("Invalid input. Please select '1' or '2' or '3'");
                continue;
            }
        };
    }
}

/// Print welcome message
pub fn client_on_boarding() -> &'static str {
    let mut stdout = stdout();

    print_colored_text(
        &mut stdout,
        Color::Cyan,
        "
    ---------------------------------------------
    |  ____/\\  |__   __|  ____|                  |
    | |__ /  \\    | |  | |__     __  ___  ______|
    |    / /\\ \\   | |  |  __|   / _` \\ \\/ / __| |
    |   / ____ \\  | |  | |____ | (_| |>  <\\__ \\_|
    |  /_/    \\_\\ |_|  |______| \\__,_/_/\\_\\___(_)
    |                                           |
    ---------------------------------------------
    "
    );

    print_colored_logo(
        &mut stdout,
        Color::DarkYellow,
        " *****************************|  STARTRIGHT 2023 |***************************** "
    );

    print_colored_text(
        &mut stdout,
        Color::DarkMagenta,
        "
    Welcome to StartRight - AI Kickstart! 🛠️
    
         Developers, focus on creativity, not syntax! 🎨
         Kickstart your projects with ready-to-use templates and accelerate your development.
         Our AI-driven app automates boilerplate code, making coding efficient and enjoyable.
    
         Currently Supporting: Rust Backend with Actix. 🦀 
         We have plans of adding support for other languages and frameworks like 
                - Python 
                - Ruby
                - JavaScript
                - Go
         and frameworks such as 
                - Flask 
                - Node.js/Express
                - Django
                -Ruby on Rails
        among others.

        We shall be adding support for databases and ORMs like PostgreSQL, MySQL, and MongoDB.
    
         *Join us in enhancing StartRight! Let's innovate together!
    "
    );

    print_colored_text(
        &mut stdout,
        Color::Green,
        "
    --> Instructions: You will need your own OpenAI API keys.
       - You will need both your organization ID and secret key to generate apps for this application.
       - We currently support GPT-4 and the latest GPT-4 turbo, 
       - We plan to add support for other models like Llama2, Palm, among others.
       - For the best results, we advise using GPT-4 or more!
    "
    );

    print_colored_text(
        &mut stdout,
        Color::DarkBlue,
        "
    --> Setup: Make sure that you have configured the following
       - You have added your the OPEN_AI_KEY & OPEN_AI_ORG in the '/.env'.
       - Update your index.html template here 'code_templates/index_template.html'(optional), 
       - Update your main_webserver_template here 'code_templates/main_websever_template.rs'(optional)
       - Update your react app template here 'frontend_react_templates/'(optional)
    "
    );

    print_colored_text(
        &mut stdout,
        Color::DarkYellow,
        "
    --> Useful Links:
       - Learn about GPT-4 models: https://platform.openai.com/docs/models
       - Learn about GPT-4 pricing: https://openai.com/pricing
       - How to find your OpenAI keys and organization keys: https://www.google.com/search?sca_esv=583956873
    "
    );


    print_colored_text(
        &mut stdout,
        Color::Grey,
        "
    --> Authors Contact Details:
        Feel free to reach out to us for any questions, suggestions, or feedback.
       - Martin Lubowa (martinlubowa@outlook.com)
       - Moreen Irungu (irungumaureen1@gmail.com)
       - Git repo: https://github.com/martin-creator/Start-Right


       &&&&&&&&****** Happy Coding! ******&&&&&&&&
    "
    );

    loop {
        // Your existing code for user response here...

        // For the purpose of this example, I'm returning a placeholder value.
        return "done";
    }
}

fn print_colored_text(stdout: &mut impl Write, color: Color, text: &str) {
    stdout.execute(SetForegroundColor(color)).unwrap();
    writeln!(stdout, "{}", text).unwrap();
    stdout.execute(ResetColor).unwrap();
}

fn print_colored_logo(stdout: &mut impl Write, color: Color, text: &str) {
    stdout
        .execute(SetForegroundColor(color))
        .unwrap()
        .execute(SetAttribute(crossterm::style::Attribute::Bold))
        .unwrap()
        .execute(Print(text))
        .unwrap()
        .execute(SetAttribute(crossterm::style::Attribute::Reset))
        .unwrap();
    writeln!(stdout).unwrap(); // Move to the next line
}

///  Run after successful initialization
pub fn run_after_initialization() {
    let mut stdout = stdout();

    print_ascii_art(&mut stdout);
    print_colored_text(&mut stdout, Color::Green, "####################################### Congratulations! #######################################\n");
    print_colored_footer(
        &mut stdout,
        Color::Cyan,
        "******** Your StartRight project has been fully initialized.********\n"
    );

    print_colored_footer(
        &mut stdout,
        Color::Yellow,
        "******** You are now ready to kickstart your fullstack development journey!********\n"
    );

    print_colored_footer(
        &mut stdout,
        Color::Blue,
        "To run your webserver :  cd src &&  run 'cargo run --bin main_webserver_template'********\n"
    );

    print_colored_footer(
        &mut stdout,
        Color::Blue,
        "To react app :  cd frontend_react_templates && run 'npm run dev' ********\n"
    );

    print_colored_footer(
        &mut stdout,
        Color::Yellow,
        "******** You are now ready to kickstart your development journey!********\n"
    );

    // Add any additional actions or messages you want to perform after initialization here.
}

fn print_colored_footer(stdout: &mut std::io::Stdout, color: Color, text: &str) {
    stdout
        .execute(SetForegroundColor(color))
        .unwrap()
        .execute(Print(text))
        .unwrap()
        .execute(SetForegroundColor(Color::Reset))
        .unwrap();
    writeln!(stdout).unwrap(); // Move to the next line
}

fn print_ascii_art(stdout: &mut std::io::Stdout) {
    let art =
        r#"
         _    __
  *    / \_ *    / \
*    /     \    /     \
   /  _    _\  /  _   _ \
   / / \  / \ / / \ / \ \
   ||   ||   ||   ||   ||
   ||   ||   ||   ||   ||
   ||   ||   ||   ||   ||
   ||   ||   ||   ||   ||
   
   
   "#;

    stdout.execute(Print(art)).unwrap().execute(SetForegroundColor(Color::Reset)).unwrap();
    writeln!(stdout).unwrap(); // Move to the next line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_prints_agent_msg() {
        PrintCommand::AICall.print_agent_message(
            "Managing Agent",
            "Testing testing, processing something"
        );
    }
}
