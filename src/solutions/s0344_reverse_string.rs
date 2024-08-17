// 344. Reverse String
// Write a function that reverses a string. The input string is given as an
// array of characters s.
//
// You must do this by modifying the input array in-place with O(1) extra
// memory.
//
// Constraints:
//
// * 1 <= s.length <= 105
// * s[i] is a printable ascii character.
//
// Solution Complexity Analysis:
//
// * Time complexity : O(N) to swap N/2 element.
// * Space complexity : O(1), it's a constant space solution.
//
// https://leetcode.com/problems/reverse-string/description/

pub struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) -> Vec<char> {
        s.reverse();
        return s.to_vec();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s0344_reverse_string_case00() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        let e = vec!['o', 'l', 'l', 'e', 'h'];
        assert_eq!(Solution::reverse_string(&mut s), e);
    }

    #[test]
    fn s0344_reverse_string_case01() {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        let e = vec!['h', 'a', 'n', 'n', 'a', 'H'];
        assert_eq!(Solution::reverse_string(&mut s), e);
    }

    #[test]
    fn s0344_reverse_string_case02() {
        let mut s = vec!['I'];
        let e = vec!['I'];
        assert_eq!(Solution::reverse_string(&mut s), e);
    }
}
