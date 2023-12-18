use common::main;
use day18::{Direction, Instruction, solve_for};

main!();

fn solve(input: &str) -> usize {
    let instructions = parse_input(input);
    solve_for(&instructions)
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    let mut result = Vec::new();
    for line in input.lines().map(str::trim) {
        let (_, rest) = line.split_once(' ').unwrap();
        let (_, encoded) = rest.split_once(' ').unwrap();
        let encoded = &encoded[2..encoded.len() - 1];
        let hex_steps = &encoded[..encoded.len() - 1];
        let encoded_dir = &encoded[encoded.len() - 1..];

        let steps = isize::from_str_radix(hex_steps, 16).unwrap();
        let (dir, steps) = match encoded_dir {
            "0" => (Direction::X, -steps),
            "1" => (Direction::Y, steps),
            "2" => (Direction::X, steps),
            "3" => (Direction::Y, -steps),
            _ => unreachable!(),
        };

        result.push(Instruction { dir, steps })
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            952408144115,
            solve(
                "R 6 (#70c710)
                 D 5 (#0dc571)
                 L 2 (#5713f0)
                 D 2 (#d2c081)
                 R 2 (#59c680)
                 D 2 (#411b91)
                 L 5 (#8ceee2)
                 U 2 (#caa173)
                 L 1 (#1b58a2)
                 U 2 (#caa171)
                 R 2 (#7807d2)
                 U 3 (#a77fa3)
                 L 2 (#015232)
                 U 2 (#7a21e3)"
            )
        );
    }
}
