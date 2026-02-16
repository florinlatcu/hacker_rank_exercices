// Add mods and use statements here
mod solve_me_first;
mod simple_array_sum;
mod compare_triplets;
mod two_sum_leetcode1;
mod add_two_numbers_leetcode2;
mod utils;

// Use crate to bring functions into scope
use crate::solve_me_first::solve_me_first;
use crate::simple_array_sum::simple_array_sum;
use crate::compare_triplets::compare_triplets;
use crate::two_sum_leetcode1::two_sum;
use crate::add_two_numbers_leetcode2::{add_two_numbers, ListNode};
use crate::utils::listnode_to_vec;


fn main() {
    println!("{}", solve_me_first(2, 3));
    println!("{}", simple_array_sum(&[1, 2, 3, 4, 5]));
    println!("{:?}", compare_triplets(&[5, 6, 7], &[3, 6, 10]));
    // Leetcode 1: Two Sum
    println!("{:?}", two_sum(Vec::from([2, 7, 11, 15]), 9));
    // Leetcode 2: Add Two Numbers
    let l1 = Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 4, next: Some(Box::new(ListNode { val: 3, next: None })) })) }));
    let l2 = Some(Box::new(ListNode { val: 5, next: Some(Box::new(ListNode { val: 6, next: Some(Box::new(ListNode { val: 4, next: None })) })) }));
    println!("l1 as vec: {:?}", listnode_to_vec(l1.clone()));
    println!("l2 as vec: {:?}", listnode_to_vec(l2.clone()));
    let result = add_two_numbers(l1, l2);
    println!("Sum  = {:?}", listnode_to_vec(result));
}
