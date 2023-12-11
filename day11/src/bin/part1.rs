use day11::solve_with_factor;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input))
}

fn solve(input: &str) -> usize {
    solve_with_factor(input, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            374,
            solve(
                "...#......
                 .......#..
                 #.........
                 ..........
                 ......#...
                 .#........
                 .........#
                 ..........
                 .......#..
                 #...#....."
            )
        )
    }
}
