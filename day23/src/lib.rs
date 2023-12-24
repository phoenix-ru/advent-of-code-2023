use std::collections::{HashSet, VecDeque};

// use rustc_hash::FxHashMap;
use smallvec::SmallVec;

#[derive(Debug)]
pub enum Node {
    Obstacle,
    Empty,
    Ltr,
    Rtl,
    Utd,
    Dtu,
}

// Start and end are the same across inputs
pub const START: (usize, usize) = (1, 0);

// Where moving is allowed
pub const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub struct RecursiveSolver {
    // map: Vec<Vec<Node>>,
    edges: Vec<GraphEdge>,
    end_x: usize,
    end_y: usize,
}

impl RecursiveSolver {
    pub fn new(map: Vec<Vec<Node>>, allow_ice: bool) -> RecursiveSolver {
        let dim_x = map.first().map_or(0, |row| row.len());
        let dim_y = map.len();
        if dim_x == 0 || dim_y == 0 {
            panic!()
        }
        let end_x = dim_x - 2;
        let end_y = dim_y - 1;

        RecursiveSolver {
            edges: build_graph(map, allow_ice),
            end_x,
            end_y,
        }
    }

    pub fn solve_recursive_smart(
        &self,
        start_from: (usize, usize),
        nodes_visited: Vec<(usize, usize, usize)>,
    ) -> usize {
        if start_from.0 == self.end_x && start_from.1 == self.end_y {
            let sum = nodes_visited
                .iter()
                .map(|(_, _, e)| self.edges[*e].cost)
                .sum();
            // println!("Sum {sum} acquired for {:?}", nodes_visited);
            return sum;
        }

        let mut max: usize = 0;
        for (idx, edge) in self.edges.iter().enumerate() {
            let (edge_end_x, edge_end_y) = if edge.x1 == start_from.0 && edge.y1 == start_from.1 {
                // Edge start fits
                (edge.x2, edge.y2)
            } else {
                continue;
            };

            // Not visited
            if nodes_visited.iter().any(|&(node_x, node_y, edge_idx)| {
                edge_idx == idx || node_x == edge_end_x && node_y == edge_end_y
            }) {
                continue;
            }

            let mut new_visited = nodes_visited.clone();
            // new_visited.push((edge.x1, edge.y1, idx));
            new_visited.push((edge_end_x, edge_end_y, idx));
            max = max.max(self.solve_recursive_smart((edge_end_x, edge_end_y), new_visited))
        }

        max
    }
}

pub fn parse_input(input: &str) -> Vec<Vec<Node>> {
    let mut nodes = Vec::new();

    for line in input.lines().map(str::trim) {
        let mut row = Vec::new();

        for c in line.chars() {
            row.push(match c {
                '#' => Node::Obstacle,
                '.' => Node::Empty,
                '>' => Node::Ltr,
                '<' => Node::Rtl,
                '^' => Node::Dtu,
                'v' => Node::Utd,
                _ => unreachable!(),
            });
        }

        nodes.push(row);
    }

    nodes
}

#[derive(Debug)]
pub struct GraphEdge {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    cost: usize,
}

pub fn build_graph(map: Vec<Vec<Node>>, allow_ice: bool) -> Vec<GraphEdge> {
    let dim_x = map.first().map_or(0, |row| row.len());
    let dim_y = map.len();
    if dim_x == 0 || dim_y == 0 {
        panic!()
    }

    // End result
    let mut graph = Vec::<GraphEdge>::new();

    // Which nodes were visited
    let mut visited = HashSet::<(usize, usize)>::new();

    // Which branches should be expanded
    let mut queue = VecDeque::<(usize, usize, usize, usize)>::new();
    queue.push_front((START.0, START.1, START.0, START.1));

    // Cache coordinates where we can travel to
    let mut can_travel_to = SmallVec::<[(usize, usize); 4]>::new();

    while let Some((start_x, start_y, edge_start_x, edge_start_y)) = queue.pop_front() {
        let mut curr_x = start_x;
        let mut curr_y = start_y;
        let mut prev_x = edge_start_x;
        let mut prev_y = edge_start_y;
        let mut steps = 0;

        // Follow the path till conjunction
        loop {
            let curr_node = &map[curr_y][curr_x];
            // let mut found_visited 

            for (dir_x, dir_y) in DIRECTIONS {
                // Lower bound
                let (Some(new_x), Some(new_y)) = (
                    curr_x.checked_add_signed(dir_x),
                    curr_y.checked_add_signed(dir_y),
                ) else {
                    continue;
                };

                // Upper bound
                if new_x >= dim_x || new_y >= dim_y {
                    continue;
                }

                // // Branch visited already
                // if visited.contains(&(new_x, new_y)) {
                //     continue;
                // }

                // Check that we have a choice if standing on an icy slope
                match (&curr_node, dir_x, dir_y) {
                    // Allowed
                    (Node::Empty, _, _)
                    | (Node::Ltr, 1, _)
                    | (Node::Rtl, -1, _)
                    | (Node::Utd, _, 1)
                    | (Node::Dtu, _, -1) => {}

                    // Anything else is disallowed
                    _ => continue,
                }

                // Going to the icy slope in wrong direction is forbidden
                let next_node = &map[new_y][new_x];
                match (&next_node, dir_x, dir_y) {
                    // Allowed
                    (Node::Empty, _, _)
                    | (Node::Ltr, 1, _)
                    | (Node::Rtl, -1, _)
                    | (Node::Utd, _, 1)
                    | (Node::Dtu, _, -1) => {}

                    // Anything else is disallowed
                    _ => continue,
                }

                // Do not go back
                // dbg!(new_x, prev_x, new_y, prev_y);
                if new_x == prev_x && new_y == prev_y {
                    continue;
                }

                can_travel_to.push((new_x, new_y))
            }

            match can_travel_to.len() {
                0 => {
                    // Nowhere to go
                    break;
                }
                1 => {
                    // Only one direction to go
                    let next_step = can_travel_to.pop().unwrap();
                    prev_x = curr_x;
                    prev_y = curr_y;
                    curr_x = next_step.0;
                    curr_y = next_step.1;
                    steps += 1;
                }
                _ => {
                    // Stop, conjunction found
                    // Mark previous as visited (it is not a conjunction)
                    visited.insert((prev_x, prev_y));

                    // Add nodes to queue if they were not visited already
                    for (next_x, next_y) in can_travel_to.drain(..) {
                        if !visited.contains(&(next_x, next_y)) {
                            queue.push_back((next_x, next_y, curr_x, curr_y));
                        }
                    }

                    break;
                }
            }
        }

        // Adjust cost if we are on a branch
        let cost = if start_x == edge_start_x && start_y == edge_start_y {
            steps
        } else {
            steps + 1
        };

        // Check duplicates
        if graph.iter().any(|e| e.x1 == edge_start_x && e.y1 == edge_start_y && e.x2 == curr_x && e.y2 == curr_y && e.cost == cost) {
            continue;
        }
        
        // Add graph edge
        graph.push(GraphEdge {
            x1: edge_start_x,
            y1: edge_start_y,
            x2: curr_x,
            y2: curr_y,
            cost,
        });

        // Add backwards edge as well if needed
        if allow_ice {
            graph.push(GraphEdge {
                x1: curr_x,
                y1: curr_y,
                x2: edge_start_x,
                y2: edge_start_y,
                cost,
            })
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use crate::{build_graph, parse_input, Node};

    #[test]
    fn it_works() {
        println!(
            "{:#?}",
            build_graph(parse_input(
                "#.#####################
                 #.......#########...###
                 #######.#########.#.###
                 ###.....#.>.>.###.#.###
                 ###v#####.#v#.###.#.###
                 ###.>...#.#.#.....#...#
                 ###v###.#.#.#########.#
                 ###...#.#.#.......#...#
                 #####.#.#.#######.#.###
                 #.....#.#.#.......#...#
                 #.#####.#.#.#########v#
                 #.#...#...#...###...>.#
                 #.#.#v#######v###.###v#
                 #...#.>.#...>.>.#.###.#
                 #####v#.#.###v#.#.###.#
                 #.....#...#...#.#.#...#
                 #.#########.###.#.#.###
                 #...###...#...#...#.###
                 ###.###.#.###v#####v###
                 #...#...#.#.>.>.#.>.###
                 #.###.###.#.###.#.#v###
                 #.....###...###...#...#
                 #####################.#"
            ), false),
        );

        let mut map = parse_input(
            "#.#####################
             #.......#########...###
             #######.#########.#.###
             ###.....#.>.>.###.#.###
             ###v#####.#v#.###.#.###
             ###.>...#.#.#.....#...#
             ###v###.#.#.#########.#
             ###...#.#.#.......#...#
             #####.#.#.#######.#.###
             #.....#.#.#.......#...#
             #.#####.#.#.#########v#
             #.#...#...#...###...>.#
             #.#.#v#######v###.###v#
             #...#.>.#...>.>.#.###.#
             #####v#.#.###v#.#.###.#
             #.....#...#...#.#.#...#
             #.#########.###.#.#.###
             #...###...#...#...#.###
             ###.###.#.###v#####v###
             #...#...#.#.>.>.#.>.###
             #.###.###.#.###.#.#v###
             #.....###...###...#...#
             #####################.#",
        );

        // Part2 ignores slopes
        for row in map.iter_mut() {
            for node in row.iter_mut() {
                match node {
                    Node::Obstacle | Node::Empty => {}
                    _ => *node = Node::Empty,
                }
            }
        }

        println!("{:#?}", build_graph(map, true));
    }
}
