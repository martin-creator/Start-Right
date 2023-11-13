use ai_functions::ai_function;

#[ai_function]
pub fn print_frontend_code(_project_description_and_index_template: &str) {
    /// INPUT: Takes in a PROJECT_DESCRIPTION, INDEX_TEMPLATE and API_JSON Schema file from backend developer to build  website fronted in HTML, CSS and JAVASCRIPT
    /// IMPORTANT: The INDEX_TEMPLATE is ONLY an example. If the Project Description requires it, make as many changes as you like.
    /// IMPORTANT: You do not need to follow the frontend code exactly.  Customize the file provided on the frontend to  write  html, css and javacript code that make sense for the users request if required.
    /// FUNCTION: Takes an existing set of code marked as INDEX_TEMPLATE and updates or re-writes it to work for the purpose in the  API_JSON  and PROJECT_DESCRIPTION
    /// IMPORTANT: The following libraries are already installed
    ///   reqwest, serde, serde_json, tokio, actix-web, async-trait, actix_cors
    /// No other external libraries should be used. Write HTML, CSS and JAVASCRIPT CODE that  implement the functionality of th API_JSON SCHEMA FILE  and the code should also fit with the description from the PROJECT_DESCRIPTION
    /// OUTPUT: Print ONLY the code, nothing else. This function ONLY prints code.
    println!(OUTPUT)
}

#[ai_function]
pub fn print_improved_frontend_code(_project_description_and_index_template: &str) {
    /// INPUT: Takes in a PROJECT_DESCRIPTION, INDEX_TEMPLATE and API_JSON Schema file from backend developer to build  website fronted in HTML, CSS and JAVASCRIPT
    /// FUNCTION: Performs the following tasks:
    ///   1. Removes any bugs in the code and adds minor additional functionality
    ///   2. Makes sure everything every endpoint provided in the API_JSON schema file is implemnted from a frontend standpoint. If not, add the feature. No code should be implemented later. Everything should be written now.
    ///   3. ONLY writes the code. No commentary.
    /// IMPORTANT: The following libraries are already installed. Does not use ANY libraries other than what was provided in the template. You should use only HTML, CSS and JAVASCRIPT to implement the functionality of the API_JSON schema file
    ///   reqwest, serde, serde_json, tokio, actix-web, async-trait
    println!(OUTPUT)
}

#[ai_function]
pub fn print_fixed_frontend_code(_frontend_broken_code_with_bugs: &str) {
    /// INPUT: Takes in broken HTML, CSS and JAVASCRIPT BROKEN_CODE and the ERROR_BUGS found
    /// FUNCTION: Removes bugs from code
    /// IMPORTANT: Only prints out the new and improved code. No commentary or anything else
    println!(OUTPUT)
}

#[ai_function]
pub fn print_index_html_file(_frontend_code_input: &str) {
    /// INPUT: Takes in HTML, CSS and JAVASCRIPT frontend CODE_INPUT based on HTML, CSS and JAVASCRIPT
    /// FUNCTION: Prints out the HTML, CSS and JAVASCRIPT code for the index.html file
    /// LOGIC: Script analyses all code and can categorize into the following object keys:
    ///  "html": This represents the html code
    /// "css": This represents the css code
    /// "javascript": This represents the javascript code
    /// IMPORTANT: Only prints out the HTML, CSS and JAVASCRIPT code. No commentary or anything else.
    /// MUST READ: All keys are strings. Even bool should be wrapped in double quotes as "bool"
    /// EXAMPLE:
    /// INPUT_CODE:
    /// ...
    /// <html>
    ///   <head>
    ///     <title>My Website</title>
    ///     <style>
    ///       /* Styles for the body */
    ///       body {
    ///         font-family: Arial, sans-serif;
    ///         background-color: #f4f4f4;
    ///         color: #333;
    ///         margin: 20px;
    ///       }
    ///
    ///       /* Styles for the main heading */
    ///       h1#main-heading {
    ///         color: #007bff;
    ///         text-align: center;
    ///       }
    ///
    ///       /* Styles for the website description paragraph */
    ///       p#website-description {
    ///         font-size: 18px;
    ///         line-height: 1.6;
    ///       }
    ///     </style>
    ///   </head>
    ///   <body>
    ///     <h1 id="main-heading">My Website</h1>
    ///     <p id="website-description">My website is about ...</p>
    ///
    ///     <script>
    ///       // JavaScript code to change the heading text color
    ///       document.addEventListener("DOMContentLoaded", function () {
    ///         const mainHeading = document.getElementById("main-heading");
    ///         mainHeading.style.color = "#0056b3";
    ///       });
    ///     </script>
    ///   </body>
    /// </html>
    /// ...
    /// OUTPUT:
    /// ...
    /// /// <html>
    ///   <head>
    ///     <title>My Website</title>
    ///     <style>
    ///       /* Styles for the body */
    ///       body {
    ///         font-family: Arial, sans-serif;
    ///         background-color: #f4f4f4;
    ///         color: #333;
    ///         margin: 20px;
    ///       }
    ///
    ///       /* Styles for the main heading */
    ///       h1#main-heading {
    ///         color: #007bff;
    ///         text-align: center;
    ///       }
    ///
    ///       /* Styles for the website description paragraph */
    ///       p#website-description {
    ///         font-size: 18px;
    ///         line-height: 1.6;
    ///       }
    ///     </style>
    ///   </head>
    ///   <body>
    ///     <h1 id="main-heading">My Website</h1>
    ///     <p id="website-description">My website is about ...</p>
    ///
    ///     <script>
    ///       // JavaScript code to change the heading text color
    ///       document.addEventListener("DOMContentLoaded", function () {
    ///         const mainHeading = document.getElementById("main-heading");
    ///         mainHeading.style.color = "#0056b3";
    ///       });
    ///     </script>
    ///   </body>
    /// </html>
    /// ...
    
    println!(OUTPUT)
}
