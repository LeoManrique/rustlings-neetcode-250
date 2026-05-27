use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        // Build adjacency: for each airport, a min-heap of destinations so we
        // always explore the lexicographically smallest next airport first.
        let mut graph: HashMap<String, BinaryHeap<Reverse<String>>> = HashMap::new();
        for t in tickets {
            let mut it = t.into_iter();
            let from = it.next().unwrap();
            let to = it.next().unwrap();
            graph.entry(from).or_default().push(Reverse(to));
        }
        // Iterative Hierholzer's algorithm.
        let mut stack: Vec<String> = vec!["JFK".to_string()];
        let mut route: Vec<String> = Vec::new();
        while let Some(top) = stack.last().cloned() {
            if let Some(heap) = graph.get_mut(&top) {
                if let Some(Reverse(next)) = heap.pop() {
                    stack.push(next);
                    continue;
                }
            }
            route.push(stack.pop().unwrap());
        }
        route.reverse();
        route
    }
}
