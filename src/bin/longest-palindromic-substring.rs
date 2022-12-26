//
// https://leetcode.com/problems/longest-palindromic-substring/
//

#[allow(unused)]
struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        

        let chars: Vec<char> = s.chars().collect();

        let mut current_palindrome_slice = &chars[0..1];

        for i in 1..chars.len() {
            let mut j = 1_usize;

            while (i as isize - j as isize) >= 0 && (i + j - 1) < chars.len() && chars[i - j] == chars[i + j - 1] {
                let tmp = &chars[(i - j)..=(i + j - 1)];

                if tmp.len() > current_palindrome_slice.len() {
                    current_palindrome_slice = tmp;
                }

                j += 1;
            }


            j = 1;

            while (i as isize - j as isize) >= 0 && (i + j) < chars.len() && chars[i - j] == chars[i + j] {
                let tmp = &chars[(i - j)..=(i + j)];

                if tmp.len() > current_palindrome_slice.len() {
                    current_palindrome_slice = tmp;
                }

                j += 1;
            }
        }

        use std::iter::FromIterator;
        String::from_iter(current_palindrome_slice)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::longest_palindrome("babad".into()),
            "bab"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::longest_palindrome("bababd".into()),
            "babab"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::longest_palindrome("abcdef".into()),
            "a"
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::longest_palindrome("a".into()),
            "a"
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::longest_palindrome("cbbd".into()),
            "bb"
        );
    }
}

fn main() { }