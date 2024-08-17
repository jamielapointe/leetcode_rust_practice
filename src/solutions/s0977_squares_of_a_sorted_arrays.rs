// Given an integer array nums sorted in non-decreasing order, return an array
// of the squares of each number sorted in non-decreasing order.
//
//
//
// Example 1:
//
// Input: nums = [-4,-1,0,3,10]
// Output: [0,1,9,16,100]
// Explanation: After squaring, the array becomes [16,1,0,9,100].
// After sorting, it becomes [0,1,9,16,100].
//
// Example 2:
//
// Input: nums = [-7,-3,2,3,11]
// Output: [4,9,9,49,121]
//
//
// Constraints:
//
// * 1 <= nums.length <= 10^4
// * -10^4 <= nums[i] <= 10^4
// * nums[i] <= nums[i + 1] for all 0 <= i < nums.length
//
//
// Note: Squaring each element and sorting the new array is very trivial,
// could you find an O(n) solution using a different approach?
//
//
// Solution Complexity Analysis:
//
// * Time Complexity: O(N), where N is the length of A.
// * Space Complexity: O(N) if you take output into account and O(1) otherwise.
//
// https://leetcode.com/problems/squares-of-a-sorted-array/description/

pub struct Solution {}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut i = 0;
        let mut j = nums.len() - 1;
        let mut k = nums.len() - 1;
        while i <= j {
            if nums[i].abs() > nums[j].abs() {
                result[k] = nums[i] * nums[i];
                i += 1;
            } else {
                result[k] = nums[j] * nums[j];
                if j == 0 {
                    break;
                }
                j -= 1;
            }
            if k == 0 {
                break;
            }
            k -= 1;
        }
        return result;
    }
}

// submission codes start here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s0977_squares_of_a_sorted_arrays_case00() {
        let nums = vec![-4, -1, 0, 3, 10];
        let e = vec![0, 1, 9, 16, 100];
        assert_eq!(Solution::sorted_squares(nums), e);
    }

    #[test]
    fn s0977_squares_of_a_sorted_arrays_case01() {
        let nums = vec![-7, -3, 2, 3, 11];
        let e = vec![4, 9, 9, 49, 121];
        assert_eq!(Solution::sorted_squares(nums), e);
    }

    #[test]
    fn s0977_squares_of_a_sorted_arrays_case02() {
        let nums = vec![-555];
        let e = vec![308025];
        assert_eq!(Solution::sorted_squares(nums), e);
    }
}
