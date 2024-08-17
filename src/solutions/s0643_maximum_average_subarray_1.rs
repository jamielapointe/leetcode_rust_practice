// 643. Maximum Average Subarray I
//
// Difficulty: Easy
//
// You are given an integer array nums consisting of n elements, and an integer
// k.
//
// Find a contiguous subarray whose length is equal to k that has the maximum
// average value and return this value. Any answer with a calculation error
// less than 10^(-5) will be accepted.
//
// Constraints:
//
// * n == nums.length
// * 1 <= k <= n <= 10^5
// * -10^4 <= nums[i] <= 10^4
//
// Solution Complexity Analysis:
//
// * Time complexity : O(n). We iterate over the given nums array of length n once only.
// * Space complexity : O(1). Constant extra space is used.
//
// https://leetcode.com/problems/maximum-average-subarray-i/

pub struct Solution {}

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum: i64 = 0;
        for i in 0..k as usize {
            sum += nums[i] as i64;
        }
        let mut max_sum: i64 = sum;
        for i in k as usize..nums.len() {
            sum += (nums[i] - nums[i - k as usize]) as i64;
            max_sum = max_sum.max(sum);
        }
        max_sum as f64 / k as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s0643_maximum_average_subarray_1() {
        assert_eq!(
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
            12.75
        );
        assert_eq!(Solution::find_max_average(vec![5], 1), 5.0);
    }
}
