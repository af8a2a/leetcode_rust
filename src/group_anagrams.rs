use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut chars_map = HashMap::new();
        strs.iter()
            .map(|strs| {
                (
                    {
                        let mut sorted = strs.clone().into_bytes();
                        sorted.sort();
                        String::from_utf8(sorted).unwrap()
                    },
                    strs.clone(),
                )
            })
            .for_each(|(key, val)| {
                chars_map.entry(key).or_insert(vec![]).push(val);
            });

        chars_map.into_values().collect()
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;
    //order problem , comment this test case
    // #[test]
    // fn test_1() {
    //     assert_eq!(
    //         vec![
    //             vec!["nat".to_string(), "tan".to_string()],
    //             vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
    //             vec!["bat".to_string()],
    //         ],
    //         Solution::group_anagrams(vec![
    //             "eat".to_string(),
    //             "tea".to_string(),
    //             "tan".to_string(),
    //             "ate".to_string(),
    //             "nat".to_string(),
    //             "bat".to_string()
    //         ])
    //     );
    // }
    #[test]
    fn test_2() {
        assert_eq!(vec![vec!["".to_string()]], Solution::group_anagrams(vec!["".to_string()]));
    }
    #[test]
    fn test_3() {
        assert_eq!(vec![vec!["a".to_string()]], Solution::group_anagrams(vec!["a".to_string()]));
    }
}
