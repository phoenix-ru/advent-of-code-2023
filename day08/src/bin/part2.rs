use day08::{get_idx, parse_input, Connections, Instruction};

fn main() {
    let input = include_str!("../../input.txt");
    // NOTE: THIS TAKES FOREVER
    println!("{}", solve(input))
}

fn solve(input: &str) -> usize {
    let (map, instructions) = parse_input(input);
    follow_instructions(&*map, &instructions)
}

fn follow_instructions(map: &[Connections], instructions: &[Instruction]) -> usize {
    let target_remainder = get_idx("Z");
    let starting_remainder = get_idx("A");

    assert!(get_idx("XYZ") % 36 == target_remainder);

    let mut steps_taken = 0;
    let mut current: Vec<usize> = map
        .iter()
        .enumerate()
        .filter_map(|(idx, item)| {
            if item.left != 0 && idx % 36 == starting_remainder {
                Some(idx)
            } else {
                None
            }
        })
        .collect();

    for instruction in instructions.iter().cycle() {
        if current.iter().all(|it| it % 36 == target_remainder) {
            break;
        }

        for path in current.iter_mut() {
            let current_connection = &map[*path];
            match instruction {
                Instruction::Left => *path = current_connection.left,
                Instruction::Right => *path = current_connection.right,
            }
        }

        steps_taken += 1;
    }

    steps_taken
}

#[allow(unused)]
fn show_current(current: &[usize]) {
    println!(
        "{:?}",
        current
            .iter()
            .map(|it| to_radix_36_string(*it))
            .collect::<Vec<String>>()
    )
}

#[allow(unused)]
fn to_radix_36_string(mut n: usize) -> String {
    const RADIX: usize = 36;
    let mut result = String::new();

    loop {
        let digit = n % RADIX;
        result.insert(
            0,
            match digit {
                0..=9 => (b'0' + digit as u8) as char,
                10..=35 => (b'A' + (digit - 10) as u8) as char,
                _ => panic!("Invalid digit in radix 36 conversion"),
            },
        );

        n /= RADIX;

        if n == 0 {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            6,
            solve(
                "LR

                11A = (11B, XXX)
                11B = (XXX, 11Z)
                11Z = (11B, XXX)
                22A = (22B, XXX)
                22B = (22C, 22C)
                22C = (22Z, 22Z)
                22Z = (22B, 22B)
                XXX = (XXX, XXX)"
            )
        )
    }

    #[test]
    fn it_works_part1() {
        assert_eq!(
            2,
            solve(
                "RL

                AAA = (BBB, CCC)
                BBB = (DDD, EEE)
                CCC = (ZZZ, GGG)
                DDD = (DDD, DDD)
                EEE = (EEE, EEE)
                GGG = (GGG, GGG)
                ZZZ = (ZZZ, ZZZ)"
            )
        )
    }

    #[test]
    fn it_cycles() {
        assert_eq!(
            6,
            solve(
                "LLR

                AAA = (BBB, BBB)
                BBB = (AAA, ZZZ)
                ZZZ = (ZZZ, ZZZ)"
            )
        )
    }
}
