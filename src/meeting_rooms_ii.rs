use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let mut r = 1;
        let mut end_times: Vec<(i32, i32)> = vec![];
        if intervals.len() == 1 {
            return 1;
        }
        let mut heap: BinaryHeap<Reverse<(i32, i32)>> =
            BinaryHeap::from_iter(intervals.into_iter().map(|x| {
                // let a = x.into_iter().reduce(|a, item| a - item);
                Reverse((x[0], x[1]))
            }));
        println!("{:?}", heap);
        end_times.push(heap.pop().unwrap().0);
        println!("{:?}", heap);
        while !heap.is_empty() {
            let one = heap.pop();
            if end_times.len() >= 1 {
                if end_times[end_times.len() -1].1 > one.unwrap().0.0 {
                    end_times.push(one.unwrap().0);
                };
            } 
            println!("{:?}", end_times);
    
        }
        println!("Darko {:?}", end_times);
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
        assert_eq!(Solution::min_meeting_rooms(intervals), 2);
        assert_eq!(Solution::min_meeting_rooms(intervals2), 1);
        assert_eq!(Solution::min_meeting_rooms(intervals3), 2);
    }
}
