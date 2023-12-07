use day06::solve_races;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> usize {
    let (times, distances) = parse_input(input);
    solve_races(times, distances)
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut lines = input.lines();
    let time_data = lines.next().unwrap();
    let distance_data = lines.next().unwrap();
    let times = read(time_data, "Time: ");
    let distances = read(distance_data, "Distance: ");

    (times, distances)
}

fn read(input: &str, header: &str) -> Vec<usize> {
    let input = &input.trim()[header.len()..];

    // Input is concatenated instead
    let mut concatenated = String::with_capacity(input.len());
    for s in input.split_whitespace() {
        concatenated.push_str(s.trim());
    }

    // Parse a single number
    vec![concatenated.parse().unwrap()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            71503,
            solve(
                "Time:      7  15   30
                 Distance:  9  40  200"
            )
        )
    }
}
