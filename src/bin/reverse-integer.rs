//
// https://leetcode.com/problems/reverse-integer/
//

#[allow(unused)]
struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x_copy = x;
        let mut n = 0;
        let mut result: i32 = 0;

        let table = [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000];

        let digits_count = if x_copy == 0 {
            1
        } else {
            loop {
                if x_copy == 0 {
                    break n;
                }
    
                x_copy /= 10;
                n += 1;
            }
        };

        x_copy = x;

        for i in (0..digits_count).rev() {
            let mul_tmp = (x_copy % 10).checked_mul(table[i]);

            if mul_tmp.is_none() {
                result = 0;
                break;
            }

            let add_tmp = result.checked_add(mul_tmp.unwrap());

            if add_tmp.is_none() {
                result = 0;
                break;
            }

            result += (x_copy % 10) * table[i];
            x_copy /= 10;
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::reverse(123),
            321
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::reverse(2147483647),
            0
        );
    }

    // #[test]
    // fn test3() {
    //     assert_eq!(
    //         Solution::reverse(7463847412),
    //         0
    //     );
    // }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::reverse(-147483647),
            -746384741
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::reverse(-12),
            -21
        );
    }

    #[test]
    fn test6() {
        assert_eq!(
            Solution::reverse(-1),
            -1
        );
    }

    #[test]
    fn test7() {
        assert_eq!(
            Solution::reverse(-0),
            0
        );
    }
}

fn main() { }
