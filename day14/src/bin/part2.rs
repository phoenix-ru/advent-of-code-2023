use std::hash::{Hash, Hasher};
use rustc_hash::{FxHasher, FxHashMap};

use common::main;
use day14::{State, calculate_weighted_sum, apply_vertical_gravity, Gravity, apply_horizontal_gravity};

main!(1_000_000_000);

fn solve(input: &str, cycles: usize) -> usize {
    let (mut map, row_len) = parse_input(input);
    let total_rows = map.len() / row_len;

    // Detect cycles
    let mut hashes = FxHashMap::default();
    let mut loop_start = 0;
    let mut previous_loop_start = 0;

    for i in 0..cycles {
        do_cycle(&mut map, total_rows, row_len);

        // Check for cycle
        let hash = calculate_hash(&map);
        if let Some(previous_idx) = hashes.get(&hash) {
            loop_start = i;
            previous_loop_start = *previous_idx;
            break;
        } else {
            hashes.insert(hash, i);
        }
    }

    // If stopped at a loop, fast-forward to the future
    if loop_start != 0 {
        let loop_len = loop_start - previous_loop_start;
        let remaining_iters = (cycles - 1 - loop_start) % loop_len;

        for _ in 0..remaining_iters {
            do_cycle(&mut map, total_rows, row_len);
        }
    }

    day14::show_map(&map, row_len);

    calculate_weighted_sum(&map, row_len, total_rows)
}

fn do_cycle(map: &mut Vec<State>, total_rows: usize, row_len: usize) {
    // North
    apply_vertical_gravity(map, total_rows, row_len, Gravity::Positive);
    // West
    apply_horizontal_gravity(map, total_rows, row_len, Gravity::Positive);
    // South
    apply_vertical_gravity(map, total_rows, row_len, Gravity::Negative);
    // East
    apply_horizontal_gravity(map, total_rows, row_len, Gravity::Negative);
}

fn calculate_hash(map: &[State]) -> u64 {
    let mut hasher = FxHasher::default();
    map.hash(&mut hasher);
    hasher.finish()
}

fn parse_input(input: &str) -> (Vec<State>, usize) {
    let mut row_len = 0;
    let mut result: Vec<State> = Vec::with_capacity(input.len());
    for (line_idx, line) in input.lines().enumerate() {
        let line = line.trim();
        if line_idx == 0 {
            row_len = line.len();
        }

        result.extend(line.chars().map(|c| State::from(c)))
    }

    (result, row_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            64,
            solve(
                "O....#....
                 O.OO#....#
                 .....##...
                 OO.#O....O
                 .O.....O#.
                 O.#..O.#.#
                 ..O..#O..O
                 .......O..
                 #....###..
                 #OO..#....",
                 1_000_000_000
            )
        );
    }

    #[test]
    fn it_solves_arbitrary() {
        // From reddit: cycle len 2520
        // https://www.reddit.com/r/adventofcode/comments/18i45eo/2023_day_14_part_2_worst_case_complexity/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
        assert_eq!(
            165,
            solve(include_str!("../../reddit.txt"), 1_000_000_000)
        );
    }
}
