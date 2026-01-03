// https://leetcode.com/problems/valid-parentheses/

// Given a string s containing just the characters 
// '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

// An input string is valid if:
// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Every close bracket has a corresponding open bracket of the same type.

use std::collections::HashMap;

use rand::{Rng, rng, rngs::ThreadRng};

fn is_valid(input: String) -> bool {
    let mut stack = Vec::new();
    let mut hash_table: HashMap<char, char> = HashMap::new();

    hash_table.insert(')', '(');
    hash_table.insert('}', '{');
    hash_table.insert(']', '[');

    for parentheses in input.chars() {
        if parentheses == '(' || parentheses == '{' || parentheses == '[' {
            stack.push(parentheses);
        }else {
            if stack.is_empty() || Some(stack.last()) != Some(hash_table.get(&parentheses)) {
                return false;
            }
            stack.pop();
        }
    }
    return stack.is_empty();
}

fn setup(input_size: i32) -> String {
    let mut result_string = String::new();
    let mut rng: ThreadRng = rng();
    let options = ['{', '}', '[', ']', '(', ')'];
    
    for _parentheses in 0..input_size {
        result_string.push(options[rng.random_range(0..6)]);

    }
    return result_string;
}

pub fn run_valid_parentheses() {
    let input_string = setup(6);
    // let msg = "[({()}[{}])]";
    // let input_string = msg.to_owned();
    let result_bool = is_valid(input_string.clone());

    println!("Input:\t{}\nResult:\t{}", input_string, result_bool);
}