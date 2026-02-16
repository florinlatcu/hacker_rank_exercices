use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //  Create a hash map to store value -> index
    println!("Two Sum - Input array: {:?}, Target: {}", nums, target);
    let mut map = HashMap::new();
    // Iterate over the array with index and value
    for (i, num) in nums.iter().enumerate() {
        // Calculate the complement needed to reach target
        let complement = target - num;
        // If the complement is already in the map, return the pair of indices
        if let Some(&j) = map.get(&complement) {
            return vec![j, i as i32];
        }
        // Otherwise, insert the current number and its index into the map
         map.insert(*num, i as i32);
    }
        // If no solution is found
    vec![]
}