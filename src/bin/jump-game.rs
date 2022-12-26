//
// https://leetcode.com/problems/jump-game/
//

use std::collections::VecDeque;

#[allow(unused)]
struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {

        let mut walked = vec![false; nums.len()];

        let mut q = VecDeque::new();

        q.push_back(0);

        while let Some(v) = q.pop_back() {

            if v == nums.len() as i32 - 1 {
                return true;
            }

            if walked[v as usize] {
                continue;
            }

            walked[v as usize] = true;

            for j in 1..=nums[v as usize] {
                let idx = (v + j).min(nums.len() as i32 - 1);

                q.push_back(idx as i32);
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::can_jump(vec![2, 3, 1, 1, 4]),
            true
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::can_jump(vec![3, 2, 1, 0, 4]),
            false
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::can_jump(vec![1, 1, 1, 0, 1, 1]),
            false
        );
    }
}

fn main() {}
