use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_table = HashMap::new();
    for (i, val) in nums.iter().enumerate() {
        if let Some(val) = hash_table.get(&(target - val)) {
            return vec![i as i32, *val as i32];
        } else {
            hash_table.insert(val, i);
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(vec![1, 0], two_sum(vec![2, 7, 11, 15], 9));
    }
    #[test]
    fn test_2() {
        assert_eq!(vec![2, 1], two_sum(vec![3, 2, 4], 6));
    }
    #[test]
    fn test_3() {
        assert_eq!(vec![1, 0], two_sum(vec![3, 3], 6));
    }
}
