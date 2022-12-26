//
// https://leetcode.com/problems/domino-and-tromino-tiling/
//

#[allow(unused)]
struct Solution;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        const M: i32 = 1000000007;
        let mut tiles = vec![0, 1, 2, 5];

        for i in 4..=n {
            //let mut sum = 0;

            // for k in 1..=(n / 2) {
            //     let idx2 = n - k;
            //     sum = sum % M + (tiles[k as usize] * tiles[idx2 as usize]) % M;
            //     sum %= M;
            // }

            tiles.push(((2 * tiles[i as usize - 1]) % M + tiles[i as usize - 3] % M) % M);

            //tiles.push(sum);
        }

        tiles[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::num_tilings(1), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::num_tilings(2), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::num_tilings(3), 5);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::num_tilings(4), 11);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::num_tilings(5), 24);
    }
}

fn main() {}
