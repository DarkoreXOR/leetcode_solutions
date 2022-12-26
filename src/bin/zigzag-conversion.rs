//
// https://leetcode.com/problems/zigzag-conversion/
//

#[allow(unused)]
struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || num_rows as usize >= s.len() {
            return s;
        }

        let chars: Vec<_> = s.chars().collect();
        let mut sb = String::with_capacity(s.len());

        for row in 0..num_rows {
            if !(row == 0 || row + 1 == num_rows) {
                let mut column = 0;

                loop {
                    let neg_offset = -row + 2 * (num_rows - 1) * column;

                    if neg_offset >= 0 && (neg_offset as isize) < (chars.len() as isize) {
                        let neg_offset = neg_offset as usize;

                        sb.push(chars[neg_offset]);
                    }

                    let pos_offset = (row + 2 * (num_rows - 1) * column) as usize;

                    if pos_offset < chars.len() {
                        sb.push(chars[pos_offset]);
                    } else {
                        break;
                    }

                    column += 1;
                }
            } else {
                let mut column = 0;

                loop {
                    let pos_offset = (row + 2 * (num_rows - 1) * column) as usize;

                    if pos_offset < chars.len() {
                        sb.push(chars[pos_offset]);
                    } else {
                        break;
                    }

                    column += 1;
                }
            }
        }

        sb
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".into(), 3),
            "PAHNAPLSIIGYIR".to_owned()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".into(), 4),
            "PINALSIGYAHRPI".to_owned()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".into(), 1),
            "PAYPALISHIRING".to_owned()
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::convert("ABCD".into(), 3),
            "ABDC".to_owned()
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::convert("ABCD".into(), 4),
            "ABCD".to_owned()
        );
    }

    #[test]
    fn test6() {
        assert_eq!(
            Solution::convert("ABCDE".into(), 4),
            "ABCED".to_owned()
        );
    }
}

fn main() { }