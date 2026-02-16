
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // Always binary search the smaller array for efficiency
    let (a, b) = if nums1.len() > nums2.len() {
        (nums2, nums1)
    } else {
        (nums1, nums2)
    };
    let m = a.len();
    let n = b.len();
    // The total number of elements on the left side of the partition
    let total_left = (m + n + 1) / 2;

    let mut left = 0;
    let mut right = m;

    // Binary search on the smaller array
    while left <= right {
        let i = (left + right) / 2;         // Partition index for a
        let j = total_left - i;             // Partition index for b

        // Get the values just left and right of the partition in both arrays
        let a_left = if i == 0 { std::i32::MIN } else { a[i - 1] };
        let a_right = if i == m { std::i32::MAX } else { a[i] };
        let b_left = if j == 0 { std::i32::MIN } else { b[j - 1] };
        let b_right = if j == n { std::i32::MAX } else { b[j] };

        // Check if we have found the correct partition
        if a_left <= b_right && b_left <= a_right {
        // If total length is even, median is the average of the two middle values
            if (m + n) % 2 == 0 {
                return (a_left.max(b_left) as f64 + a_right.min(b_right) as f64) / 2.0;
            } else {
                // If odd, median is the max of the left side
                return a_left.max(b_left) as f64;
            }
        } else if a_left > b_right {
                // Move partition in a to the left
                right = i - 1;
        } else {
            // Move partition in a to the right
            left = i + 1;
        }
    }
    0.0 // Should never reach here if input is valid
}