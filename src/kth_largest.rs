struct Solution;
use crate::heap::Heap;
use std::{cmp::Reverse, collections::BinaryHeap, fmt::Debug};
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = Heap::from_iter(nums);
        println!("{:?}", heap);
        let mut res = 0;
        for _ in 0..k {
            res = heap.next().unwrap();
        }
        res
    }

    pub fn find_kth_largest_s(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from_iter(nums);
        let mut res: i32 = 0;
        for _ in 0..k {
            res = heap.pop().unwrap();
        }
        res
    }

    pub fn find_kth_largest_j(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for num in nums {
            if heap.len() != k as usize {
                heap.push(Reverse(num));
            } else {
                if num > heap.peek().unwrap().0 {
                    heap.pop();
                    heap.push(Reverse(num))
                };
            }
        }
        heap.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_largest() {
        assert_eq!(
            Solution::find_kth_largest_j(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
        assert_eq!(Solution::find_kth_largest_s(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }
}
