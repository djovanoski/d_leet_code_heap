struct Solution;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
impl Solution {
    pub fn top_f_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let j = nums.into_iter().fold(HashMap::new(), |mut a, x| {
            let counter = a.entry(x).or_insert(0);
            *counter += 1;
            a
        });

        let mut d = j.into_iter().collect::<Vec<(i32, i32)>>();
        d.sort_by(|a, b| b.1.cmp(&a.1));

        let mut a = d.into_iter().map(|(a, _)| a).collect::<Vec<i32>>();

        a.truncate(k as usize);
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        println!(
            "{:?}",
            Solution::top_f_frequent(vec![5, 3, 1, 1, 1, 3, 73, 1], 3)
        );
        // println!("{:?}", Solution::top_f_frequent(vec![4, 1, -1, 2, -1, 2, 3], 2));
    }
}
