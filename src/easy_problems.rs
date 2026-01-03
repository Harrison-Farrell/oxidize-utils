pub mod plus_one;
pub mod two_sum;
pub mod valid_parentheses;

fn print_problem(name: &str) {
    println!("---------------------------------------------------------------");
    println!("\t{}", name);
    println!("---------------------------------------------------------------");
}

pub fn run_easy_challenges() {
    print_problem("easy: plus one");
    plus_one::run_plus_one();
    
    print_problem("easy: two sum");
    two_sum::run_two_sum();

    print_problem("easy: valid parentheses");
    valid_parentheses::run_valid_parentheses();
}