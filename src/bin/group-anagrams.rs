//
// https://leetcode.com/problems/group-anagrams/
//

#[allow(unused)]
struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut x = HashMap::<Vec<char>, Vec<Vec<String>>>::new();

        x.insert(vec!['x', 'y', 'z'], vec![vec!["123".to_owned()]]);

        use std::collections::HashMap;
        use std::iter::FromIterator;

        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut chars: Vec<_> = s.chars().collect();
            chars.sort();

            let new_s = String::from_iter(chars);

            let entry = map.entry(new_s).or_insert(Vec::new());
            entry.push(s);
        }

        let mut result = Vec::new();

        for (_, v) in map {
            result.push(v);
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
            Solution::group_anagrams(vec!["eat","tea","tan","ate","nat","bat"].iter().map(ToString::to_string).collect()),
            vec![
                vec!["tan".to_owned(), "nat".to_owned()],
                vec!["eat".to_owned(), "tea".to_owned(), "eat".to_owned()],
                vec!["bat".to_owned()],
            ]
        );
    }
}

fn main() { }
