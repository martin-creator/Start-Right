use ai_functions::ai_function;

#[ai_function]
pub fn print_frontend_code(_project_description_and_index_template: &str) {
    /// Takes in a PROJECT_DESCRIPTION, INDEX_TEMPLATE, and API_JSON Schema file from the backend developer to build the website frontend in HTML, tailwind css, and JavaScript.
    ///
    /// # Important
    /// - The INDEX_TEMPLATE is provided as an example. Customize it as needed for the project, ensuring professional and elegant styling.
    /// - Generate new tailwind css styling for the code if required.
    /// - The frontend code does not need to be an exact match; customize it to align with the user's request and the API_JSON Schema in the PROJECT_DESCRIPTION.
    /// - API BASE ROUTE: endpoints are called from http://localhost:8080
    ///
    /// # Function
    /// Takes an existing set of code marked as INDEX_TEMPLATE and updates or re-writes it to align with the API_JSON and PROJECT_DESCRIPTION.
    ///
    /// # Libraries
    /// The code should include tailwind css, HTML, and JavaScript. The function assumes that the following libraries are already installed; no other external libraries should be used.
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
    /// Takes in a PROJECT_DESCRIPTION, INDEX_TEMPLATE, and API_JSON Schema file from the backend developer to enhance and build the website frontend in HTML, tailwind css, and JavaScript.
    ///
    /// # Function
    /// Performs the following tasks:
    ///   1. Removes any bugs in the code and adds minor additional functionality.
    ///   2. Implements every endpoint provided in the API_JSON schema file from a frontend standpoint. Ensure all features are added now; no code should be implemented later.
    ///   3. ONLY writes the code; no commentary.
    ///
    /// # Important
    /// - The code should include tailwind css, HTML, and JavaScript.
    /// - Only prints out the new and improved code; no commentary or anything else.
    /// - Ensure that all code is printed; no code should be represented by comments. All code should be represented by strings.
    /// - NO CODE SHOULD BE replaced with placeholder comments; ALL CODE SHOULD BE PRINTED.
    /// - NO CODE SHOULD BE COMMENTED OUT OR REPRESENTED BY COMMENTS.
    /// - No incomplete code should be printed. All code should be complete and ready to be used.
    /// - All code should be printed now and ready to be used. No code should be implemented later.
    ///
    /// # Output
    /// Print ONLY the code, nothing else. All code should be included in the output, with no commentary or comments.
    /// IMPORTANT: All code should be printed now and ready to be used. No code should be implemented later.
    println!(OUTPUT)
}

#[ai_function]
pub fn print_fixed_frontend_code(_frontend_broken_code_with_bugs: &str) {
    /// Takes in broken HTML, tailwind css, and JavaScript BROKEN_CODE along with identified ERROR_BUGS and removes the bugs from the code.
    ///
    /// # Important
    /// - Only prints out the new and improved code; no commentary or anything else.
    /// - Ensure that all code is printed; no code should be represented by comments. All code should be represented by strings.
    /// - NO CODE SHOULD BE replaced with placeholder comments; ALL CODE SHOULD BE PRINTED.
    /// - NO CODE SHOULD BE COMMENTED OUT OR REPRESENTED BY COMMENTS.
    /// - No incomplete code should be printed. All code should be complete and ready to be used.
    ///
    /// # Output
    /// Print ONLY the code, nothing else. This function ONLY prints code, including HTML, tailwind css, and JavaScript code. All code should be included in the output, with no commentary or comments.
    /// IMPORTANT: All code should be printed now and ready to be used. No code should be implemented later.
    println!(OUTPUT)
}

#[ai_function]
pub fn print_index_html_file(_frontend_code_input: &str) {
    /// INPUT: Takes in HTML, tailwind css and JAVASCRIPT frontend CODE_INPUT based on HTML, tailwind css and JAVASCRIPT
    /// FUNCTION: Prints out the HTML, tailwind css and JAVASCRIPT code for the index.html file
    /// LOGIC: Script analyses all code and can categorize into the following object keys:
    ///  "html": This represents the html code
    /// "tailwind css": This represents the tailwind css code
    /// "javascript": This represents the javascript code
    /// IMPORTANT: Only prints out the HTML, tailwind css and JAVASCRIPT code. No commentary or anything else.
    /// IMPORTANT: NO CODE SHOULD BE PRINTED AS COMMENTS. ALL CODE SHOULD BE PRINTED AS STRINGS
    /// IMPORTANT: Ensure that all code is printed. No code should be represented by comments. All code should be represented by strings.
    /// IMPORTANT: ALL CODE SHOULD BE PRINTED. NO CODE SHOULD BE COMMENTED OUT OR REPRESENTED BY COMMENTS
    /// IMPORTANT: No incomplete code should be printed. All code should be complete and ready to be used.
    /// IMPORTANT: All code should be printed now and ready to be used. No code should be implemented later.
    /// INPUT_CODE:
    /// ...
    /// <html>
    ///   <head>
    ///     <title>My Website</title>
    ///     <link href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" rel="stylesheet">
    ///   </head>
    ///   <body class="font-sans bg-gray-100 text-gray-700 m-20">
    ///     <h1 id="main-heading" class="text-blue-500 text-center">My Website</h1>
    ///     <p id="website-description" class="text-lg leading-6">
    ///       My website is about ...
    ///     </p>
    ///
    ///     <script>
    ///       // JavaScript code to change the heading text color
    ///       document.addEventListener("DOMContentLoaded", function () {
    ///         const mainHeading = document.getElementById("main-heading");
    ///         mainHeading.classList.add("text-blue-800");
    ///       });
    ///     </script>
    ///   </body>
    /// </html>

    /// ...
    /// OUTPUT:
    /// ...
    /// <html>
    ///   <head>
    ///     <title>My Website</title>
    ///     <link href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" rel="stylesheet">
    ///   </head>
    ///   <body class="font-sans bg-gray-100 text-gray-700 m-20">
    ///     <h1 id="main-heading" class="text-blue-500 text-center">My Website</h1>
    ///     <p id="website-description" class="text-lg leading-6">
    ///       My website is about ...
    ///     </p>
    ///
    ///     <script>
    ///       // JavaScript code to change the heading text color
    ///       document.addEventListener("DOMContentLoaded", function () {
    ///         const mainHeading = document.getElementById("main-heading");
    ///         mainHeading.classList.add("text-blue-800");
    ///       });
    ///     </script>
    ///   </body>
    /// </html>

    /// ...

    println!(OUTPUT)
}
