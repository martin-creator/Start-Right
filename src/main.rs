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


fn main(){
    println!("Hello, world!");
}