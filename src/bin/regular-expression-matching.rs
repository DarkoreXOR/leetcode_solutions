//
// https://leetcode.com/*
//

#[allow(unused)]
struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_match("aab".to_owned(), "c*a*b".to_owned()),
            true
        );
    }
}

fn main() { }
