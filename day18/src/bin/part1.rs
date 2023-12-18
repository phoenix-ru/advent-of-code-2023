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
        let (dir, rest) = line.split_once(' ').unwrap();
        let (steps, _color) = rest.split_once(' ').unwrap();

        let steps: isize = steps.parse().unwrap();
        let (dir, steps) = match dir {
            "R" => (Direction::X, -steps),
            "L" => (Direction::X, steps),
            "U" => (Direction::Y, -steps),
            "D" => (Direction::Y, steps),
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
            62,
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
