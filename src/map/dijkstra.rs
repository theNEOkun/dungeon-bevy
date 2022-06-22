use crate::prelude::*;
use std::collections::BinaryHeap;

fn find_shortest(map: Map, start: &Position, end: &Position) -> Vec<usize> {
    let mut start_position = map[start].clone();
    let mut pq = BinaryHeap::new();
}
