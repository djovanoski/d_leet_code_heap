use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn k_weakest_rows(nums: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let ln = nums.len() - k as usize;
        let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::from_iter(
            nums.into_iter()
                .enumerate()
                .map(|(idx, value)| {
                    let inside = value.into_iter().fold(0, |mut a, x| {
                        if x == 1 {
                            a += 1
                        };
                        a
                    });
                    Reverse((inside, idx))
                })
                .collect::<Vec<Reverse<(i32, usize)>>>(),
        );

        let mut res = vec![];
        while heap.len() > ln {
            res.push(heap.pop().unwrap().0 .1 as i32)
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_weakest_rows() {
        let mat = vec![
            vec![1, 0, 0, 0],
            vec![1, 1, 1, 1],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
        ];
        let mat2 = vec![
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
        ];

        assert_eq!(Solution::k_weakest_rows(mat, 2), vec![0, 2]);
        assert_eq!(Solution::k_weakest_rows(mat2, 3), vec![2, 0, 3]);
    }
}

