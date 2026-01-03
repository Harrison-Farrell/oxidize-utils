mod easy_problems;

use std::env;



fn main() {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }
    
    easy_problems::run_easy_challenges();
}