//
// https://leetcode.com/problems/regular-expression-matching/
//

#[allow(unused)]
struct Solution;

#[allow(unused)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    AnyOne,
    AnyMany,
    CharOne(char),
    CharMany(char),
}

#[allow(unused)]
impl Solution {
    fn parse(p: &[char]) -> Vec<Token> {
        let mut vec = Vec::new();

        let mut pi = 0_usize;

        while pi < p.len() {
            let pc = p[pi];

            if pc == '.' {
                if pi + 1 < p.len() && p[pi + 1] == '*' {
                    vec.push(Token::AnyMany);
                    pi += 2;
                } else {
                    vec.push(Token::AnyOne);
                    pi += 1;
                }
            } else {
                if pi + 1  < p.len() && p[pi + 1] == '*' {
                    vec.push(Token::CharMany(pc));
                    pi += 2;
                } else {
                    vec.push(Token::CharOne(pc));
                    pi += 1;
                }
            }
        }

        vec
    }

    fn is_match_slice(s: &[char], tokens: &[Token]) -> bool {

        if s.len() <= 0 && tokens.len() <= 0 {
            return true;
        }

        if s.len() <= 0 && tokens.len() > 0 {
            return !tokens.iter().any(|xs| match xs {
                Token::AnyOne => true,
                Token::CharOne(_) => true,
                _ => false,
            });
        }

        if s.len() <= 0 || tokens.len() <= 0 {
            return false;
        }

        match (tokens[0], s[0]) {
            (Token::AnyMany, _) => {
                Self::is_match_slice(&s[1..], &tokens) ||
                Self::is_match_slice(&s, &tokens[1..])
            }

            (Token::AnyOne, _) => {
                Self::is_match_slice(&s[1..], &tokens[1..])
            }

            (Token::CharMany(tc), mut sc) => {

                let mut si = 0;
                let mut r = false;

                while tc == sc && si < s.len() {
                    r = Self::is_match_slice(&s[1..], &tokens);

                    if r {
                        break;
                    }

                    sc = s[si];
                    si += 1;
                }

                r || Self::is_match_slice(&s, &tokens[1..])
            }

            (Token::CharOne(tc), sc) => {
                tc == sc && Self::is_match_slice(&s[1..], &tokens[1..])
            }
        }
    }

    pub fn is_match(s: String, p: String) -> bool {
        let ss = s.chars().collect::<Vec<char>>();
        let ps = p.chars().collect::<Vec<char>>();
        let tokens = Self::parse(&ps);
        Self::is_match_slice(&ss, &tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::Token;
    use super::Solution;

    #[test]
    fn test_parser_1() {
        let p = "c*a*b".chars().collect::<Vec<char>>();
        assert_eq!(
            Solution::parse(&p[..]),
            vec![
                Token::CharMany('c'),
                Token::CharMany('a'),
                Token::CharOne('b')
            ],
        );
    }

    #[test]
    fn test_parser_2() {
        let p = ".".chars().collect::<Vec<char>>();
        assert_eq!(
            Solution::parse(&p[..]),
            vec![
                Token::AnyOne,
            ],
        );
    }

    #[test]
    fn test_parser_3() {
        let p = "..*".chars().collect::<Vec<char>>();
        assert_eq!(
            Solution::parse(&p[..]),
            vec![
                Token::AnyOne,
                Token::AnyMany,
            ],
        );
    }

    #[test]
    fn test_regex_1() {
        let s = "b".to_string();
        let p = "c*a*b".to_string();

        assert_eq!(
            Solution::is_match(s, p),
            true
        );
    }

    #[test]
    fn test_regex_2() {
        let s = "aa".to_string();
        let p = "a".to_string();

        assert_eq!(
            Solution::is_match(s, p),
            false
        );
    }

    #[test]
    fn test_regex_3() {
        let s = "aa".to_string();
        let p = "a*".to_string();

        assert_eq!(
            Solution::is_match(s, p),
            true
        );
    }

    #[test]
    fn test_regex_4() {
        let s = "ab".to_string();
        let p = ".*".to_string();

        assert_eq!(
            Solution::is_match(s, p),
            true
        );
    }

    #[test]
    fn test_regex_5() {
        let s = "abc".to_string();
        let p = ".*c".to_string();

        assert_eq!(
            Solution::is_match(s, p),
            true
        );
    }

    
    #[test]
    fn test_regex_6() {
        let s = "abd".to_string();
        let p = ".*c".to_string();

        assert_eq!(
            Solution::is_match(s, p),
            false
        );
    }

    #[test]
    fn test_regex_7() {
        let s = "abdc".to_string();
        let p = "a.*c".to_string();

        assert_eq!(
            Solution::is_match(s, p),
            true
        );
    }

    #[test]
    fn test_regex_8() {
        let s = "ac".to_string();
        let p = "a".to_string();

        assert_eq!(
            Solution::is_match(s, p),
            false
        );
    }

    #[test]
    fn test_regex_9() {
        let s = "ac".to_string();
        let p = "ac".to_string();

        assert_eq!(
            Solution::is_match(s, p),
            true
        );
    }
}

fn main() { }
