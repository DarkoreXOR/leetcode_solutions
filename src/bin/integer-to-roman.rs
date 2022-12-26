//
// https://leetcode.com/problems/integer-to-roman/
//

#[allow(unused)]
struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut result = String::with_capacity(64);

        let mut n = num;
        let mut k = 1000;

        while n > 0 {
            let v = n / k;

            match (k, v) {
                (1000, q) => {
                    (0..q).for_each(|_| result.push('M'));
                }

                (100, 9) => {
                    result.push_str("CM");
                }

                (100, 8) => {
                    result.push_str("DCCC");
                }

                (100, 7) => {
                    result.push_str("DCC");
                }

                (100, 6) => {
                    result.push_str("DC");
                }

                (100, 5) => {
                    result.push_str("D");
                }

                (100, 4) => {
                    result.push_str("CD");
                }

                (100, q) => {
                    (0..q).for_each(|_| result.push('C'));
                }

                (10, 9) => {
                    result.push_str("XC");
                }

                (10, 8) => {
                    result.push_str("LXXX");
                }

                (10, 7) => {
                    result.push_str("LXX");
                }

                (10, 6) => {
                    result.push_str("LX");
                }

                (10, 5) => {
                    result.push_str("L");
                }

                (10, 4) => {
                    result.push_str("XL");
                }

                (10, q) => {
                    (0..q).for_each(|_| result.push('X'));
                }

                (1, 9) => {
                    result.push_str("IX");
                }

                (1, 8) => {
                    result.push_str("VIII");
                }

                (1, 7) => {
                    result.push_str("VII");
                }

                (1, 6) => {
                    result.push_str("VI");
                }

                (1, 5) => {
                    result.push_str("V");
                }

                (1, 4) => {
                    result.push_str("IV");
                }

                (1, q) => {
                    (0..q).for_each(|_| result.push('I'));
                }

                _ => {}
            }

            n -= k * v;
            k /= 10;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::int_to_roman(1994),
            "MCMXCIV".to_owned()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::int_to_roman(3),
            "III".to_owned()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::int_to_roman(4),
            "IV".to_owned()
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::int_to_roman(58),
            "LVIII".to_owned()
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::int_to_roman(60),
            "LX".to_owned()
        );
    }
}

fn main() { }