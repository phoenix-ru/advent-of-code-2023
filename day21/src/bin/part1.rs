use std::collections::VecDeque;

use common::main;
use day21::{parse_input, Input, Node, DIRECTIONS};

main!();

fn solve(input: &str) -> usize {
    let input = parse_input(input);
    solve_for(input, 64)
}

// Solves part1 for any generic input without map expansion
pub fn solve_for(input: Input, steps: usize) -> usize {
    let Input {
        nodes,
        starting_x,
        starting_y,
    } = input;
    let dim_x = nodes.first().map_or(0, |row| row.len());
    let dim_y = nodes.len();
    if dim_x == 0 || dim_y == 0 {
        panic!("Input was not read correctly")
    }

    let mut current_steps = VecDeque::from([(starting_x, starting_y)]);

    // Deduplication using 
    let dedup_len = dim_x * dim_y;
    let mut dedup = Vec::<bool>::with_capacity(dedup_len);
    dedup.extend((0..dedup_len).map(|_| false));

    for _ in 0..steps {
        // Process exactly this amount of steps
        // Since I am using a ring buffer, it will get filled as steps are processed
        let process_n = current_steps.len();
        let mut processed = 0;
        // dbg!(step, process_n);

        while let Some((curr_x, curr_y)) = current_steps.pop_front() {
            for (delta_x, delta_y) in DIRECTIONS {
                let (Some(x), Some(y)) = (
                    curr_x.checked_add_signed(delta_x),
                    curr_y.checked_add_signed(delta_y),
                ) else {
                    continue;
                };

                if x >= dim_x || y >= dim_y {
                    continue;
                }

                // Deduplicate
                let is_visited_already = &mut dedup[y * dim_y + x];
                if *is_visited_already {
                    continue;
                }

                // Add if not an obstacle
                if let Some(Node::Available) = nodes.get(y).and_then(|row| row.get(x)) {
                    *is_visited_already = true;
                    current_steps.push_back((x, y));
                    // println!("Adding {x} {y}");
                }
            }

            // Stop iteration
            processed += 1;
            if processed == process_n {
                break;
            }
        }

        // Reset deduplication state
        dedup.fill(false);
    }

    current_steps.len()
}

#[cfg(test)]
mod tests {
    use day21::parse_input;

    use crate::solve_for;

    #[test]
    fn it_works() {
        let input = parse_input(
            "...........
             .....###.#.
             .###.##..#.
             ..#.#...#..
             ....#.#....
             .##..S####.
             .##..#...#.
             .......##..
             .##.#.####.
             .##..##.##.
             ...........",
        );
        assert_eq!(16, solve_for(input, 6));
    }
}
