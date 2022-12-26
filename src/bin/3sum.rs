//
// https://leetcode.com/problems/3sum/
//

#[allow(unused)]
struct Solution;

impl Solution {
    fn binary_search(sorted: &[i32], elem: i32) -> Option<usize> {
        let mut left = 0;
        let mut right = sorted.len() - 1;

        loop {
            let mid = (left + right) / 2;

            if sorted[mid] < elem {
                left = mid + 1;
            } else if sorted[mid] > elem {
                right = mid;
            } else {
                return Some(mid);
            }

            if left == mid || left > right {
                break;
            }
        }

        None
    }

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sorted = nums.clone();
        sorted.sort();

        let mut result = Vec::new();

        for ai in 0..sorted.len() {
            if ai > 0 && sorted[ai - 1] == sorted[ai] {
                continue;
            }

            let mut bi = ai + 1;
            let mut ci = sorted.len() - 1;

            while bi < ci {
                let s = sorted[ai] + sorted[bi] + sorted[ci];

                if s > 0 {
                    ci -= 1;
                } else if s < 0 {
                    bi += 1;
                } else {
                    result.push(vec![sorted[ai], sorted[bi], sorted[ci]]);
    
                    ci -= 1;
    
                    while ci != 0 && sorted[ci + 1] == sorted[ci] {
                        ci -= 1;
                    }
                }
            }            
        }

        result
    }
}


#[cfg(test)]
mod bs_tests {
    use super::Solution;

    #[test]
    fn test_bs1() {
        assert_eq!(
            Solution::binary_search(&vec![-1], -1),
            Some(0)
        );
    }

    #[test]
    fn test_bs2() {
        assert_eq!(
            Solution::binary_search(&vec![-1], 0),
            None
        );
    }

    #[test]
    fn test_bs3() {
        assert_eq!(
            Solution::binary_search(&vec![-1, 1], -1),
            Some(0)
        );
    }

    #[test]
    fn test_bs4() {
        assert_eq!(
            Solution::binary_search(&vec![-1, 1], 1),
            Some(1)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::three_sum(vec![0, 1, 1]),
            Vec::<Vec<_>>::new()
        );
    }
    
    #[test]
    fn test3() {
        assert_eq!(
            Solution::three_sum(vec![0, 0, 0]),
            vec![vec![0, 0, 0]]
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::three_sum(vec![-4, 1, 3]),
            vec![
                vec![-4, 1, 3],
            ]
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::three_sum(vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6]),
            vec![
                vec![-4, -2, 6],
                vec![-4, 0, 4],
                vec![-4, 1, 3],
                vec![-4, 2, 2],
                vec![-2, -2, 4],
                vec![-2, 0, 2],
            ]
        );
    }
}

fn main() { }
