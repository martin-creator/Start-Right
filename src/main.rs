#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {{
        stringify!($func)
    }};
}

#[macro_use]
mod ai_macros;
// mod apis;
mod helper_funcs;
mod models;


fn main(){
    println!("Hello, world!");
}