//
// https://leetcode.com/problems/two-sum/
//

#[allow(unused)]
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map = HashMap::new();

        for (idx, val) in nums.iter().enumerate() {
            let difference = target - val;

            if let Some(&first_idx) = map.get(&difference) {
                return vec![first_idx, idx as i32];
            } else {
                map.insert(*val, idx as i32);
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::two_sum(vec![1, 3, 2, 4], 7),
            vec![1, 3]
        );
    }
}

fn main() { }