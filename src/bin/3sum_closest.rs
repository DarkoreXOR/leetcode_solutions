//
// https://leetcode.com/problems/3sum-closest/
//

#[allow(unused)]
struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut sorted = nums.clone();
        sorted.sort();

        let mut result = i32::MAX;
        let mut last_diff = i32::MAX;

        for ai in 0..sorted.len() {
            if ai > 0 && sorted[ai - 1] == sorted[ai] {
                continue;
            }

            let mut bi = ai + 1;
            let mut ci = sorted.len() - 1;

            while bi < ci {
                let s = sorted[ai] + sorted[bi] + sorted[ci];

                if s > target {
                    ci -= 1;
                } else if s < target {
                    bi += 1;

                } else {
                    return target;
                }

                let diff = i32::abs(target - s) as i32;

                if diff < last_diff {
                    result = s;
                    last_diff = diff;
                }
            }            
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
            Solution::three_sum_closest(vec![-1, 2, 1, -4], 1),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::three_sum_closest(vec![0, 0, 0], 1),
            0
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::three_sum_closest(vec![0, 1, 2], 3),
            3
        );
    }
}

fn main() { }
