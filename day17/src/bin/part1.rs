use rustc_hash::FxHashSet;
use std::collections::BTreeSet;

use common::main;
use day17::parse_input;

main!();

fn solve(input: &str) -> u32 {
    let map = parse_input(input);
    let dim_x = map.first().map_or(0, |v| v.len());
    let dim_y = map.len();
    let end_x = dim_x - 1;
    let end_y = dim_y - 1;

    // Priority map using BTreeSet
    // heat loss, curr_x, curr_y, next_dir_x, next_dir_y
    let mut queue = BTreeSet::<(u32, usize, usize, isize, isize)>::default();
    // x, y, dir_x, dir_y
    let mut visited = FxHashSet::<(usize, usize, isize, isize)>::default();
    queue.insert((0, 0, 0, 1, 0));
    queue.insert((0, 0, 0, 0, 1));

    while let Some((curr_heat_loss, curr_x, curr_y, curr_dir_x, curr_dir_y)) = queue.pop_first() {
        // Target reached
        if curr_x == end_x && curr_y == end_y {
            return curr_heat_loss;
        }

        // Was seen already
        if !visited.insert((curr_x, curr_y, curr_dir_x, curr_dir_y)) {
            continue;
        }

        // Switch directions
        let possible_directions = if curr_dir_x != 0 {
            [(0, 1), (0, -1)]
        } else if curr_dir_y != 0 {
            [(1, 0), (-1, 0)]
        } else {
            unreachable!()
        };

        for (dir_x, dir_y) in possible_directions {
            // Accummulate heat loss
            let mut heat_loss = curr_heat_loss;

            // Go 3 tiles forward in the selected direction
            for mult in 1..=3 {
                if let (Some(new_x), Some(new_y)) = (
                    check_bounds(curr_x, dir_x * mult, end_x),
                    check_bounds(curr_y, dir_y * mult, end_y),
                ) {
                    heat_loss += &map[new_y][new_x];
                    queue.insert((heat_loss, new_x, new_y, dir_x, dir_y));
                }
            }
        }
    }

    0
}

#[inline]
fn check_bounds(v: usize, diff: isize, upper_bound: usize) -> Option<usize> {
    if v == 0 && diff < 0 {
        return None;
    }
    v.checked_add_signed(diff)
        .and_then(|v| if v > upper_bound { None } else { Some(v) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            102,
            solve(
                "2413432311323
                 3215453535623
                 3255245654254
                 3446585845452
                 4546657867536
                 1438598798454
                 4457876987766
                 3637877979653
                 4654967986887
                 4564679986453
                 1224686865563
                 2546548887735
                 4322674655533"
            )
        );
    }
}
