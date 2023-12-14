use common::main;
use day13::{mirror_vertical, mirror_horizontal};

main!();

fn solve(input: &str) -> usize {
    let mut inputs: Vec<Vec<Vec<bool>>> = Vec::new();
    let mut current_input: Vec<Vec<bool>> = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            if !current_input.is_empty() {
                inputs.push(std::mem::take(&mut current_input));
            }
            continue;
        }

        let line = line.chars().map(|c| c == '#').collect();
        current_input.push(line);
    }

    if !current_input.is_empty() {
        inputs.push(std::mem::take(&mut current_input));
    }

    let mut result = 0;
    for input in inputs.iter() {
        if let Some(value) = mirror_point(input) {
            result += value;
        }
    }

    result
}

fn mirror_point(input: &Vec<Vec<bool>>) -> Option<usize> {
    mirror_vertical(input, 0).or_else(|| mirror_horizontal(input, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            405,
            solve(
                "#.##..##.
                 ..#.##.#.
                 ##......#
                 ##......#
                 ..#.##.#.
                 ..##..##.
                 #.#.##.#.
 
                 #...##..#
                 #....#..#
                 ..##..###
                 #####.##.
                 #####.##.
                 ..##..###
                 #....#..#"
            )
        );

        assert_eq!(
            5,
            solve(
                "..##..##.
                 ..#.##.#.
                 ##......#
                 ##......#
                 ..#.##.#.
                 ..##..##.
                 #.#.##.#."
            )
        );
    }
}
