use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn connect_sticks(sticks: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut heap = BinaryHeap::from_iter(
            sticks
                .into_iter()
                .map(Reverse)
                .collect::<Vec<Reverse<i32>>>(),
        );
        loop {
            let one = heap.pop();
            let two = heap.pop();
            if two.is_none() {
                break;
            }
            let total = one.unwrap().0 + two.unwrap().0;
            res += total;
            heap.push(Reverse(total));
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect_sticks() {
        let stick1 = vec![2, 4, 3];
        let stick2 = vec![1, 8, 3, 5];
        let stick3 = vec![5];
        let res1 = Solution::connect_sticks(stick1);
        let res2 = Solution::connect_sticks(stick2);
        let res3 = Solution::connect_sticks(stick3);
        assert_eq!(res1, 14);
        assert_eq!(res2, 30);
        assert_eq!(res3, 0);
    }
}
