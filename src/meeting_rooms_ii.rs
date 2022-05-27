use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let mut end_times = BinaryHeap::new();
        if intervals.len() == 1 {
            return 1;
        }
        let mut heap: BinaryHeap<Reverse<(i32, i32)>> =
            BinaryHeap::from_iter(intervals.into_iter().map(|x| Reverse((x[0], x[1]))));
        end_times.push(Reverse(heap.pop().unwrap().0 .1));
        while !heap.is_empty() {
            let new_room = heap.pop().unwrap().0;
            if end_times.peek().unwrap().0 <= new_room.0 {
                end_times.pop();
            }
            end_times.push(Reverse(new_room.1));
        }
        end_times.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_meeting_rooms() {
        let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
        let intervals2 = vec![vec![7, 10], vec![2, 4]];
        let intervals3 = vec![vec![5, 8], vec![6, 8]];
        let intervals4 = vec![vec![9, 10], vec![4, 9], vec![4, 17]];
        assert_eq!(Solution::min_meeting_rooms(intervals), 2);
        assert_eq!(Solution::min_meeting_rooms(intervals2), 1);
        assert_eq!(Solution::min_meeting_rooms(intervals3), 2);
        assert_eq!(Solution::min_meeting_rooms(intervals4), 2);
    }
}
