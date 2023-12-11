use day11::solve_with_factor;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve_with_factor(input, 1_000_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            1030,
            solve_with_factor(
                "...#......
                 .......#..
                 #.........
                 ..........
                 ......#...
                 .#........
                 .........#
                 ..........
                 .......#..
                 #...#.....",
                 10
            )
        );
        assert_eq!(
            8410,
            solve_with_factor(
                "...#......
                 .......#..
                 #.........
                 ..........
                 ......#...
                 .#........
                 .........#
                 ..........
                 .......#..
                 #...#.....",
                 100
            )
        )
    }
}
