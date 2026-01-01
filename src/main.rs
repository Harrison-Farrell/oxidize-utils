mod easy_problems;

use std::env;

fn print_problem(name: &str) {
    println!("---------------------------------------------------------------");
    println!("\t{}", name);
    println!("---------------------------------------------------------------");
}

fn main() {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }
    
    print_problem("easy: plus one");
    easy_problems::plus_one::run_plus_one();
    
    print_problem("easy: two sum");
    easy_problems::two_sum::run_two_sum();
}