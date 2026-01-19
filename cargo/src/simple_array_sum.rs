/*
Given an array of integers, find the sum of its elements.
For example, if the array ar = [1, 2, 3], 1 + 2 + 3 = 6, so return 6.

Function Description:
Complete the simple_array_sum function with the following parameter(s):
 - int ar[n]: an array of integers
Returns
- int: the sum of the array elements
Constraints
0 <= n <= 1000
0 <= ar[i] <= 1000
*/
pub fn simple_array_sum(ar: &[i32]) -> i32 {
    println!("simple_array_sum module called");
    ar.iter().sum()
}