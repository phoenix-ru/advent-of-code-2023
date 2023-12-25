use common::main;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

main!();

fn solve(input: &str) -> usize {
    let (mut graph, _names_to_ids) = parse_input(input);
    // dbg!(&names_to_ids);
    dbg!(&graph.connections.len());

    // Brute-force across all the nodes
    for curr_node in 1..graph.connections.len() {
        // Take the first 3 paths between node 0 and current node and remove them
        let paths: Vec<Vec<usize>> = (0..3)
            .map(|_| {
                let path = graph.find_path(0, curr_node).expect("Path should exist");
                for w in path.windows(2) {
                    graph.remove_connection(w[0], w[1]);
                }
                path
            })
            .collect();

        // Try finding path without these 3 paths
        match graph.find_path(0, curr_node) {
            Some(_) => {
                // Rollback
                for path in paths {
                    for w in path.windows(2) {
                        graph.insert_connection(w[0], w[1]);
                    }
                }
            }
            None => {
                // Voila, answer found
                // Find the most connected node
                let max_conn =
                    graph
                        .connections
                        .iter()
                        .enumerate()
                        .fold((0, 0), |acc, (c_idx, c)| {
                            if c.len() > acc.1 {
                                (c_idx, c.len())
                            } else {
                                acc
                            }
                        });

                let subgraph1_size = graph.nodes_reachable_from(max_conn.0);
                let subgraph2_size = graph.connections.len() - subgraph1_size;
                dbg!(subgraph1_size, subgraph2_size);
                return subgraph1_size * subgraph2_size;
            }
        }
    }

    0
}

#[derive(Debug, Default)]
pub struct Graph {
    /// id -> which ids it connects to
    pub connections: Vec<Vec<usize>>,
}

impl Graph {
    /// Inserts two-way connection
    pub fn insert_connection(&mut self, id: usize, to: usize) {
        // Pre-fill the empty spaces
        for _ in self.connections.len()..=(id.max(to)) {
            self.connections.push(vec![]);
        }

        self.connections[id].push(to);
        self.connections[to].push(id);
    }

    /// Removes two-way connection
    pub fn remove_connection(&mut self, id: usize, to: usize) {
        self.connections[id].retain(|&it| it != to);
        self.connections[to].retain(|&it| it != id);
    }

    pub fn find_path(&self, from: usize, to: usize) -> Option<Vec<usize>> {
        // Djikstra
        let mut visited = FxHashSet::default();
        let mut parents = FxHashMap::default();
        let mut queue = VecDeque::new();
        queue.push_front(from);

        while let Some(curr) = queue.pop_back() {
            if !visited.insert(curr) {
                continue;
            }

            if curr == to {
                break;
            }

            for connection in self.connections[curr].iter() {
                if !visited.contains(connection) {
                    queue.push_back(*connection);
                    parents.insert(connection, curr);
                }
            }
        }

        let mut path = Vec::new();
        let mut node = to;
        while node != from {
            path.push(node);
            if parents.contains_key(&node) {
                node = parents[&node];
            } else {
                return None;
            }
        }
        path.push(from);
        path.reverse();
        Some(path)
    }

    pub fn nodes_reachable_from(&self, from: usize) -> usize {
        let mut seen = FxHashSet::<usize>::default();
        let mut queue = VecDeque::new();
        queue.push_front(from);

        while let Some(curr) = queue.pop_back() {
            if !seen.insert(curr) {
                continue;
            }

            for connection in self.connections[curr].iter() {
                queue.push_back(*connection);
            }
        }

        seen.len()
    }
}

fn parse_input(input: &str) -> (Graph, FxHashMap<&str, usize>) {
    let mut names_to_ids = FxHashMap::<&str, usize>::default();
    let mut next_id = 0;

    let mut graph = Graph::default();

    for line in input.lines().map(str::trim) {
        let (from, tos) = line.split_once(": ").unwrap();

        let from_id = *names_to_ids.entry(from).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });

        for connects_to in tos.split_whitespace() {
            let to_id = *names_to_ids.entry(connects_to).or_insert_with(|| {
                let id = next_id;
                next_id += 1;
                id
            });
            graph.insert_connection(from_id, to_id);
        }
    }

    (graph, names_to_ids)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(
            54,
            solve(
                "jqt: rhn xhk nvd
                rsh: frs pzl lsr
                xhk: hfx
                cmg: qnr nvd lhk bvb
                rhn: xhk bvb hfx
                bvb: xhk hfx
                pzl: lsr hfx nvd
                qnr: nvd
                ntq: jqt hfx bvb xhk
                nvd: lhk
                lsr: lhk
                rzs: qnr cmg lsr rsh
                frs: qnr lhk lsr"
            )
        );
    }
}
