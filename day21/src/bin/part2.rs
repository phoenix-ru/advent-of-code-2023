use std::collections::VecDeque;

use common::main;
use day21::{parse_input, Input, Node, DIRECTIONS};
use rustc_hash::FxHashMap;

main!();

fn solve(input: &str) -> usize {
    let input = parse_input(input);
    solve_for(input)
}

// Solves part2 mathematically
// Inspired by https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
pub fn solve_for(input: Input) -> usize {
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

    let mut queue = VecDeque::from([(starting_x, starting_y, 0)]);

    // Deduplicate using a HashMap<(x: usize, y: usize), (steps_taken: usize)>
    let mut visited = FxHashMap::<(usize, usize), usize>::default();

    // Populate the full map
    while let Some((curr_x, curr_y, curr_steps)) = queue.pop_front() {
        for (delta_x, delta_y) in DIRECTIONS {
            let (Some(x), Some(y)) = (
                curr_x.checked_add_signed(delta_x),
                curr_y.checked_add_signed(delta_y),
            ) else {
                continue;
            };

            // Bounds and deduplication
            if x >= dim_x || y >= dim_y || visited.contains_key(&(x, y)) {
                continue;
            }

            // Add if not an obstacle
            if let Some(Node::Available) = nodes.get(y).and_then(|row| row.get(x)) {
                visited.insert((x, y), curr_steps + 1);
                queue.push_back((x, y, curr_steps + 1));
            }
        }
    }

    // Half-dimensions because input has "paths" at verticals and horizontals where S lies:
    // #.#
    // .S.
    // #.#
    let half_dim = dim_x / 2;
    assert_eq!(dim_x, dim_y);
    assert_eq!(dim_x, 131);
    assert_eq!(half_dim, 65);

    // Check number of odd and even visited
    // We are also interested in nodes lying beyond bounds of input (> half_dim)
    let even_total = visited
        .values()
        .filter(|&&distance| distance % 2 == 0)
        .count();
    let odd_total = visited.len() - even_total;
    let even_corners = visited
        .values()
        .filter(|&&distance| distance > half_dim && distance % 2 == 0)
        .count();
    let odd_corners = visited
        .values()
        .filter(|&&distance| distance > half_dim && distance % 2 != 0)
        .count();

    // From https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
    let n = ((26501365 - half_dim) / dim_x) as usize;
    assert_eq!(n, 202300);

    (n + 1) * (n + 1) * odd_total + n * n * even_total - (n + 1) * odd_corners + n * even_corners
}
