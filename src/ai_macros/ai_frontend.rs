use ai_functions::ai_function;

#[ai_function]
pub fn print_frontend_code(_project_description_and_index_template: &str) {
    /// Takes in a PROJECT_DESCRIPTION, INDEX_TEMPLATE, and API_JSON Schema file from the backend developer to build the website frontend in HTML, CSS, and JavaScript.
    /// 
    /// # Important
    /// - The INDEX_TEMPLATE is provided as an example. Customize it as needed for the project, ensuring professional and elegant styling.
    /// - Generate new CSS styling for the code if required.
    /// - The frontend code does not need to be an exact match; customize it to align with the user's request and the API_JSON Schema in the PROJECT_DESCRIPTION.
    /// 
    /// # Function
    /// Takes an existing set of code marked as INDEX_TEMPLATE and updates or re-writes it to align with the API_JSON and PROJECT_DESCRIPTION.
    /// 
    /// # Libraries
    /// The code should include CSS, HTML, and JavaScript. The function assumes that the following libraries are already installed; no other external libraries should be used.
    /// 
    /// # Important
    /// - NO CODE SHOULD BE replaced with placeholder comments; ALL CODE SHOULD BE PRINTED.
    /// - NO CODE SHOULD BE COMMENTED OUT OR REPRESENTED BY COMMENTS.
    /// 
    /// # Output
    /// Print ONLY the code, nothing else. This function ONLY prints code; all code should be included in the output, with no commentary or comments.
    println!(OUTPUT)
}

#[ai_function]
pub fn print_improved_frontend_code(_project_description_and_index_template: &str) {
    /// Takes in a PROJECT_DESCRIPTION, INDEX_TEMPLATE, and API_JSON Schema file from the backend developer to enhance and build the website frontend in HTML, CSS, and JavaScript.
    /// 
    /// # Function
    /// Performs the following tasks:
    ///   1. Removes any bugs in the code and adds minor additional functionality.
    ///   2. Implements every endpoint provided in the API_JSON schema file from a frontend standpoint. Ensure all features are added now; no code should be implemented later.
    ///   3. ONLY writes the code; no commentary.
    /// 
    /// # Important
    /// - The code should include CSS, HTML, and JavaScript.
    /// - Only prints out the new and improved code; no commentary or anything else.
    /// - Ensure that all code is printed; no code should be represented by comments. All code should be represented by strings.
    /// - NO CODE SHOULD BE replaced with placeholder comments; ALL CODE SHOULD BE PRINTED.
    /// - NO CODE SHOULD BE COMMENTED OUT OR REPRESENTED BY COMMENTS.
    /// 
    /// # Output
    /// Print ONLY the code, nothing else. All code should be included in the output, with no commentary or comments.
    println!(OUTPUT)
}


#[ai_function]
pub fn print_fixed_frontend_code(_frontend_broken_code_with_bugs: &str) {
    /// Takes in broken HTML, CSS, and JavaScript BROKEN_CODE along with identified ERROR_BUGS and removes the bugs from the code.
    /// 
    /// # Important
    /// - Only prints out the new and improved code; no commentary or anything else.
    /// - Ensure that all code is printed; no code should be represented by comments. All code should be represented by strings.
    /// - NO CODE SHOULD BE replaced with placeholder comments; ALL CODE SHOULD BE PRINTED.
    /// - NO CODE SHOULD BE COMMENTED OUT OR REPRESENTED BY COMMENTS.
    /// 
    /// # Output
    /// Print ONLY the code, nothing else. This function ONLY prints code, including HTML, CSS, and JavaScript code. All code should be included in the output, with no commentary or comments.
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
    /// IMPORTANT: NO CODE SHOULD BE PRINTED AS COMMENTS. ALL CODE SHOULD BE PRINTED AS STRINGS
    /// IMPORTANT: Ensure that all code is printed. No code should be represented by comments. All code should be represented by strings.
    /// IMPORTANT: ALL CODE SHOULD BE PRINTED. NO CODE SHOULD BE COMMENTED OUT OR REPRESENTED BY COMMENTS
    /// IMPORTANT: No incomplete code should be printed. All code should be complete and ready to be used.
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
