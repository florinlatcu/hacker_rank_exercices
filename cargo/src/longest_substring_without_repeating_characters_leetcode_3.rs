/*
    Given a string s, find the length of the longest substring without duplicate characters.
 */

use std::collections::HashSet;
pub fn length_of_longest_substring(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect(); // Convert string to a vector of chars
    let mut set = HashSet::new(); // To store unique characters
    let mut max_len = 0; // Result: the maximum length found
    let mut left = 0; // Left pointer of the sliding window

    for right in 0..chars.len() {
        // If we see a duplicate, move the left pointer forward and remove chars from the set
        while set.contains(&chars[right]) {
            set.remove(&chars[left]);
            left += 1;
        }
        set.insert(chars[right]);
         max_len = max_len.max(right - left + 1);
    }
    max_len as i32
}