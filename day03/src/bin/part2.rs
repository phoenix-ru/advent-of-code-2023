fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug, Default)]
struct Range {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Number {
    value: u32,
    line: usize,
    chars_range: Range,
}

struct StarPosition {
    x: usize,
    y: usize,
}

fn solve(input: &str) -> u32 {
    let mut input_vec: Vec<Vec<char>> = Vec::new();

    // Read into vecs of chars. Not performant, but it's not the focus here
    for line in input.lines() {
        input_vec.push(line.trim().chars().collect());
    }

    // Build a map of numbers and also of `*` characters
    let mut nums_map: Vec<Number> = Vec::new();
    let mut star_positions: Vec<StarPosition> = Vec::new();

    for (line_idx, line) in input_vec.iter().enumerate() {
        // A number we build
        let mut range = Range::default();
        let mut number = 0;

        // Close previous number
        macro_rules! close_number {
            () => {
                if number != 0 {
                    nums_map.push(Number {
                        value: number,
                        line: line_idx,
                        chars_range: range,
                    })
                }

                number = 0;
                range = Range::default();
            };
        }

        for (character_idx, character) in line.iter().enumerate() {
            // Try converting to digit or ignore the character
            let Some(digit) = character.to_digit(10) else {
                close_number!();

                if character == &'*' {
                    star_positions.push(StarPosition {
                        x: character_idx,
                        y: line_idx,
                    });
                }

                continue;
            };

            // Start the range when starting the number
            if number == 0 {
                range.start = character_idx;
            }
            range.end = character_idx;

            // Add to the number
            number = number * 10 + digit;
        }

        if number != 0 {
            nums_map.push(Number {
                value: number,
                line: line_idx,
                chars_range: range,
            });
        }
    }

    // End value
    let mut sum = 0;

    let is_neighbor = |star_x: usize,
                       star_y: usize,
                       number_x_start: usize,
                       number_x_end: usize,
                       number_y: usize| {
        for number_x in number_x_start..=number_x_end {
            if star_x.abs_diff(number_x) <= 1 && star_y.abs_diff(number_y) <= 1 {
                return true;
            }
        }

        false
    };

    // Walk across a map of 🌟 stars 🌟
    'star: for star in star_positions {
        let star_x = star.x;
        let star_y = star.y;

        let mut neighbor1 = 0;
        let mut neighbor2 = 0;

        // Check neighboring numbers
        for number in nums_map.iter() {
            if !is_neighbor(
                star_x,
                star_y,
                number.chars_range.start,
                number.chars_range.end,
                number.line,
            ) {
                continue;
            }

            if neighbor1 == 0 {
                neighbor1 = number.value;
            } else if neighbor2 == 0 {
                neighbor2 = number.value;
            } else {
                // 3 neighbors is not okay
                continue 'star;
            }
        }

        if neighbor1 != 0 && neighbor2 != 0 {
            sum += neighbor1 * neighbor2;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            467835,
            solve(
                "467..114..
                 ...*......
                 ..35..633.
                 ......#...
                 617*......
                 .....+.58.
                 ..592.....
                 ......755.
                 ...$.*....
                 .664.598.."
            )
        );

        assert_eq!(
            885 * 970,
            solve(
                "...........
                 .885.970...
                 ....*......
                 ..$....281."
            )
        );

        assert_eq!(
            347 * 524 + 538 * 964,
            solve(
                "................
                 ..347*524..538..
                 ..........*.....
                 ...#131....964..
                 ................"
            )
        );

        assert_eq!(
            253 * 876,
            solve(
                ".....
                 ..253
                 .*...
                 ..876"
            )
        );
    }
}
