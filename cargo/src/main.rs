// Add mods and use statements here
mod solve_me_first;
mod simple_array_sum;
mod compare_triplets;
mod two_sum_leetcode1;

// Use crate to bring functions into scope
use crate::solve_me_first::solve_me_first;
use crate::simple_array_sum::simple_array_sum;
use crate::compare_triplets::compare_triplets;
use crate::two_sum_leetcode1::two_sum;

fn main() {
    println!("{}", solve_me_first(2, 3));
    println!("{}", simple_array_sum(&[1, 2, 3, 4, 5]));
    println!("{:?}", compare_triplets(&[5, 6, 7], &[3, 6, 10]));
    // Leetcode 1: Two Sum
    println!("{:?}", two_sum(Vec::from([2, 7, 11, 15]), 9));
}
