// https://leetcode.com/problems/two-sum/description/

// Given an array of integers nums and an integer target, 
// return indices of the two numbers such that they add up to target.

// You may assume that each input would have exactly one solution, 
// and you may not use the same element twice.

// You can return the answer in any order.

use std::{collections::HashMap};
use rand::{Rng, rng, rngs::ThreadRng};

fn two_sum_hash_map(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let size = numbers.len();
    let mut hash_map:HashMap<i32,i32> = HashMap::new();

    for idx in 0..size {
        let complement = target - numbers[idx];
        if hash_map.contains_key(&complement) {
            return vec!{handle_option(hash_map.get(&complement)), idx as i32};
        }
        hash_map.insert(numbers[idx], idx as i32);
    }

    return vec!{};
}

fn two_sum_brute_force(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = Vec::new();
    for first_index in 0..numbers.len() {
        for second_index in (first_index+1)..numbers.len(){
            if numbers[first_index] + numbers[second_index] == target {
                result.push(first_index as i32);
                result.push(second_index as i32);
                return result;
            }
        }
    }
    return result;
}

fn handle_option(data: Option<&i32>) -> i32 {
    match data {
        Some(&number) => {
            return number
        }
        None => {
            return 0;
        }
    }
}

fn setup(array_size: i32) -> (Vec<i32>, i32) {
    let mut tmp_arry: Vec<i32> = Vec::new();
    let mut rng: ThreadRng = rng();

    for _pos in 0..array_size {
        tmp_arry.push( rng.random_range(0..1501) );
    }
    
    let part_a = rng.random_range(0..array_size/2);
    let part_b = rng.random_range(array_size/2..array_size);
    return (tmp_arry.clone(), (tmp_arry[part_a as usize] + tmp_arry[part_b as usize]));
}

pub fn run_two_sum() {
    let (input, result_target) = setup(10);

    println!("Input:\t{:?}\nTarget:\t{}\n", input, result_target);

    let output_brute_force = two_sum_brute_force(input.clone(), result_target);
    println!("Brute force:\t{:?}", output_brute_force);

    let output_hash_map = two_sum_hash_map(input.clone(), result_target);
    println!("Hash map:\t{:?}", output_hash_map);
} 