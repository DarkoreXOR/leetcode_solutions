//
// https://leetcode.com/problems/string-to-integer-atoi/
//

#[allow(unused)]
struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut result = 0;

        let s = s.trim_start();

        let (sign, new_s) = if s.starts_with('+') {
            (1, &s[1..])
        } else if s.starts_with('-') {
            (-1, &s[1..])
        } else {
            (1, &s[..])
        };

        for (i, c) in new_s.chars().enumerate() {
            if !c.is_digit(10) {
                break;
            }

            if i > 0 {
                if i32::checked_mul(result, 10).is_none() {
                    if sign > 0 {
                        result = i32::MAX;
                    } else {
                        result = i32::MIN;
                    }
    
                    break;
                }
                result *= 10;
            }

            let digit = c as i32 - '0' as i32;

            if i32::checked_add(result, digit).is_none() {
                if sign > 0 {
                    result = i32::MAX;
                } else {
                    result = i32::MIN;
                }

                break;
            }

            result += digit;
        }

        sign * result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::my_atoi("0".to_owned()),
            0
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::my_atoi("1".to_owned()),
            1
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::my_atoi("123".to_owned()),
            123
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::my_atoi("-0".to_owned()),
            0
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::my_atoi("-1".to_owned()),
            -1
        );
    }

    #[test]
    fn test6() {
        assert_eq!(
            Solution::my_atoi("-123".to_owned()),
            -123
        );
    }

    #[test]
    fn test7() {
        assert_eq!(
            Solution::my_atoi("".to_owned()),
            0
        );
    }
}

fn main() { }