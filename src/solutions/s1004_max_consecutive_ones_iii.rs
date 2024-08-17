// 1004. Max Consecutive Ones III
// Given a binary array `nums` and an integer `k`, return the maximum number of
// consecutive 1's in the array if you can flip at most `k` 0's.
//
// Constraints:
// * 1 <= nums.length <= 10^5
// * nums[i] is either 0 or 1
// * 0 <= k <= nums.length
//
// Example 1:
// Input: nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2
// Output: 6
// Explanation: [1,1,1,0,0,`**1**,1,1,1,1,**1**``]
// Bolded numbers were flipped from 0 to 1. The longest subarray is ticked.
//
// Example 2:
// Input: nums = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], k = 3
// Output: 10
// Explanation: [0,0,`1,1,**1**,**1**,1,1,1,**1**,1,1`,0,0,0,1,1,1,1]
// Bolded numbers were flipped from 0 to 1. The longest subarray is ticked.
//
// Solution Complexity Analysis:
// * Time Complexity: O(N), where N is the length of nums.
// * Space Complexity: O(1).
//
// https://leetcode.com/problems/max-consecutive-ones-iii/description/

pub struct Solution {}

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut max_len = 0;
        let mut num_zeros = 0;
        let mut left_index = 0;
        for right_index in 0..nums.len() {
            if nums[right_index] == 0 {
                num_zeros += 1;
            }
            while num_zeros > k {
                if nums[left_index] == 0 {
                    num_zeros -= 1;
                }
                left_index += 1;
            }
            max_len = max_len.max(right_index - left_index + 1);
        }
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s1004_max_consecutive_ones_iii() {
        assert_eq!(
            Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2),
            6
        );
        assert_eq!(
            Solution::longest_ones(
                vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                3
            ),
            10
        );
    }
}
