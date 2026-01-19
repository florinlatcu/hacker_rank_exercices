// Add mods and use statements here
mod solve_me_first;
mod simple_array_sum;
mod compare_triplets;

// Use crate to bring functions into scope
use crate::solve_me_first::solve_me_first;
use crate::simple_array_sum::simple_array_sum;
use crate::compare_triplets::compare_triplets;

fn main() {
    println!("{}", solve_me_first(2, 3));
    println!("{}", simple_array_sum(&[1, 2, 3, 4, 5]));
    println!("{:?}", compare_triplets(&[5, 6, 7], &[3, 6, 10]));
}
