//
// https://leetcode.com/problems/top-k-frequent-elements/
//

#[allow(unused)]
struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut hmap = HashMap::new();

        for n in nums {
            *hmap.entry(n).or_insert(0) += 1;
        }

        let mut vec = Vec::new();

        for (k, v) in hmap {
            vec.push((v, k));
        }

        vec.sort_by_cached_key(|k| k.0);

        vec.iter().rev().take(k as usize).map(|kv| kv.1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::top_k_frequent(vec![-1, -1], 1),
            vec![-1]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 2], 2),
            vec![1, 2]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::top_k_frequent(vec![4, 1, -1, 2, -1, 2, 3], 2),
            vec![-1, 2]
        );
    }    
}

fn main() { }