use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
#[derive(PartialEq, Debug)]
pub struct MinNonNan {
    distance: f64,
    point: Vec<i32>,
}

impl MinNonNan {
    pub fn new(distance: f64, point: Vec<i32>) -> Self {
        Self { distance, point }
    }
}

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
pub struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut heap = BinaryHeap::from_iter(
            points
                .iter()
                .enumerate()
                .map(|(idx, value)| {
                    let d = f64::sqrt(value[0].pow(2) as f64 + value[1].pow(2) as f64);
                    Reverse(((d.to_degrees()).round() as i32, idx))
                })
                .collect::<Vec<Reverse<(i32, usize)>>>(),
        );
        println!("{:?}", heap);
        for _ in 0..k {
            let idx = heap.pop().unwrap().0;
            res.push(points[idx.1].clone());
        }
        println!("Final Result {:?}", res);
        res
    }
    pub fn k_closest2(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut heap = BinaryHeap::from_iter(
            points
                .into_iter()
                .enumerate()
                .map(|(_, value)| {
                    let d = f64::sqrt(value[0].pow(2) as f64 + value[1].pow(2) as f64);
                    MinNonNan::new(d, value)
                })
                .collect::<Vec<MinNonNan>>(),
        );
        for _ in 0..k {
            let idx = heap.pop().unwrap();
            res.push(idx.point);
        }
        println!("{:?}", res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_closest() {
        println!("dasdadads");
        let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        Solution::k_closest2(vec![vec![-5, 4], vec![-6, 5], vec![4, 6]], 2);
        // Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1);
        // assert_eq!(
        //     Solution::k_closest(points.clone(), 2),
        //     vec![vec![3, 3], vec![-2, 4]]
        // );
    }
}
