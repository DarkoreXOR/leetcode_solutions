//
// https://leetcode.com/problems/longest-substring-without-repeating-characters/
//

#[allow(unused)]
struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::{HashSet, VecDeque};

        let mut char_set = HashSet::new();
        let mut char_deq = VecDeque::new();
        let mut max_length = 0_i32;

        for c in s.chars() {
            while char_set.contains(&c) {
                max_length = max_length.max(char_set.len() as i32);
                let head_c = char_deq.pop_front().unwrap();
                char_set.remove(&head_c);
            }

            char_set.insert(c);
            char_deq.push_back(c);
        }

        max_length.max(char_set.len() as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_owned()),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_owned()),
            1
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_owned()),
            3
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::length_of_longest_substring("asjrgapa".to_owned()),
            6
        );
    }

   #[test]
    fn test5() {
        assert_eq!(
            Solution::length_of_longest_substring("qrsvbspk".to_owned()),
            5
        );
    } 
}

fn main() { }