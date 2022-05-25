use std::iter::FromIterator;
use std::{cmp::Reverse, collections::BinaryHeap};
struct Solution;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut res: Option<Reverse<i32>> = None;
        let d = matrix.len() * matrix[0].len();
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::from_iter(
            matrix
                .into_iter()
                .map(|x| {
                    let k = x.into_iter().map(|a| Reverse(a)).collect();
                    k
                })
                .collect::<Vec<Vec<Reverse<i32>>>>()
                .into_iter()
                .flatten(),
        );
        while !(heap.len() <= d - k as usize) {
            res = heap.pop();
        }
        res.unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_smallest() {
        let matrix = vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]];
        assert_eq!(Solution::kth_smallest(matrix, 8), 13);
        assert_eq!(Solution::kth_smallest(vec![vec![-5]], 1), -5);
    }
}
