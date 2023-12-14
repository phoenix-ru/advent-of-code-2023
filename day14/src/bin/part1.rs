use common::main;
use day14::{State, calculate_weighted_sum, apply_vertical_gravity, Gravity};

main!();

fn solve(input: &str) -> usize {
    let (mut map, row_len) = parse_input(input);
    let total_rows = map.len() / row_len;

    // To north
    apply_vertical_gravity(&mut map, total_rows, row_len, Gravity::Positive);

    // day14::show_map(&map, row_len);

    calculate_weighted_sum(&map, row_len, total_rows)
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
            136,
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
                 #OO..#...."
            )
        );
    }
}
