// https://leetcode.com/problems/plus-one/description/

// You are given a large integer represented as an integer array digits, 
// where each digits[i] is the ith digit of the integer. 
// The digits are ordered from most significant to least significant 
// in left-to-right order. The large integer does not contain any leading 0's.

// Increment the large integer by one and return the resulting array of digits.

use rand::{Rng, rng, rngs::ThreadRng};

pub fn plus_one(digits: Vec<u32>) -> Vec<u32>{
    let mut tmp_array = digits;

    // reverse the array order
    tmp_array.reverse();

    for index in 0..tmp_array.len() {
        if tmp_array[index] != 9 {
            tmp_array[index] = tmp_array[index] + 1;
            break;
        }
        tmp_array[index] = 0;

        if index == tmp_array.len() -1 {
            tmp_array.push(1);
        } 
    }

    tmp_array.reverse();
    return tmp_array;
}

pub fn setup(size: u32) -> Vec<u32> {
    let mut tmp_arry: Vec<u32> = Vec::new();
    let mut rng: ThreadRng = rng();

    for pos in 0..size {
        // the first value can't be zero
        if pos == 0 {
             tmp_arry.push( rng.random_range(1..10) );
        } else {
            tmp_arry.push( rng.random_range(0..10) );
        }
        
    }
    
    println!("Inital Array: {:?}", tmp_arry);
    return tmp_arry;
}

pub fn run_plus_one(){
    let input = setup(10);
    let ouput = plus_one(input);

    println!("Final Array: {:?}", ouput);
}