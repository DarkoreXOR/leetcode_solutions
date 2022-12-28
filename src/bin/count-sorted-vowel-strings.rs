//
// https://leetcode.com/problems/count-sorted-vowel-strings/
//

#[allow(unused)]
struct Solution;

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let nn = n as usize;
        let mut dp = vec![vec![0; 5 + 1]; nn + 1];

        for j in 1..=5 {
            dp[1][j] = 1;
        }

        for i in 2..=nn {
            for j in 1..=5 {
                let mut sum = 0;

                for k in j..=5 {
                    sum += dp[i - 1][k];
                }

                dp[i][j] = sum;
            }
        }
        
        dp[nn].iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_vowel_strings(1),
            5
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::count_vowel_strings(2),
            15
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::count_vowel_strings(3),
            35
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::count_vowel_strings(33),
            66045
        );
    }
}

fn main() { }
