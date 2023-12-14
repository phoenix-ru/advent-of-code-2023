use common::main;
use day13::{mirror_horizontal, mirror_vertical};

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
        result += mirror_vertical(input, 1)
            .or_else(|| mirror_horizontal(input, 1))
            .unwrap();
        // let cols_len = input[0].len();
        // let rows_len = input.len();

        // // Brute-force a "smudge" by flipping bits
        // for y in 0..rows_len {
        //     for x in 0..cols_len {
        //         let old_value = input[y][x];

        //         // flip
        //         input[y][x] = !old_value;

        //         match mirror_horizontal(input, 1) {
        //             Some(new_value) if new_value != initial_value => {
        //                 result += new_value;
        //                 continue 'input;
        //             }
        //             _ => {
        //                 match mirror_vertical(input,1) {
        //                     Some(new_value) if new_value != initial_value => {
        //                         result += new_value;
        //                         continue 'input;
        //                     }
        //                     _ => {
        //                         // restore bit
        //                         input[y][x] = old_value;
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            400,
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
        )
    }
}
