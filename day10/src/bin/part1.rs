use day10::{parse_input, find_cycle};

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input))
}

fn solve(input: &str) -> usize {
    let (map, (animal_x, animal_y)) = parse_input(input);

    let cycle_len = find_cycle(&map, animal_x, animal_y).len();
    (cycle_len + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            4,
            solve(
                ".....
                 .S-7.
                 .|.|.
                 .L-J.
                 ....."
            )
        );
        assert_eq!(
            8,
            solve(
                "..F7.
                 .FJ|.
                 SJ.L7
                 |F--J
                 LJ..."
            )
        );
    }
}
