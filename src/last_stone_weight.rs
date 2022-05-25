use std::{cmp::Ordering, collections::BinaryHeap};

struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        if stones.len() == 1 {
            return stones[0];
        }
        let mut heap = BinaryHeap::from_iter(stones);
        while heap.len() >= 1 {
            let second = heap.pop();
            let first = heap.pop();
            if first.is_none() || second.is_none() {
                match second {
                    Some(s) => return s,
                    None => return 0,
                }
            }

            match second.unwrap().cmp(&first.unwrap()) {
                Ordering::Less | Ordering::Equal => {
                    continue;
                }
                Ordering::Greater => heap.push(second.unwrap() - first.unwrap()),
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_stone_weight() {
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }
}
