fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> u32 {
    let mut input_vec: Vec<Vec<char>> = Vec::new();

    // Read into vecs of chars. Not performant, but it's not the focus here
    for line in input.lines() {
        input_vec.push(line.trim().chars().collect());
    }

    let mut sum = 0;

    let get_character_at =
        |x: usize, y: usize| -> Option<&char> { input_vec.get(y).and_then(|line| line.get(x)) };

    // Iterate over chars now
    for (line_idx, line) in input_vec.iter().enumerate() {
        // A number we build
        let mut number = 0;
        let mut is_number_suitable = false;

        // Close previous number
        macro_rules! close_number {
            () => {
                if number != 0 {
                    if is_number_suitable {
                        sum += number;
                    }

                    number = 0;
                    is_number_suitable = false;
                }
            };
        }

        for (character_idx, character) in line.iter().enumerate() {
            // Try converting to digit or ignore the character
            let Some(digit) = character.to_digit(10) else {
                close_number!();
                continue;
            };

            // Add to the number
            number = number * 10 + digit;

            // 3*3 area
            'x: for x in -1..=1 {
                'y: for y in -1..=1 {
                    // Ignore the digit itself or negative bounds
                    if (x == 0 && y == 0) || (x == -1 && character_idx == 0) || (y == -1 && line_idx == 0) {
                        continue 'y;
                    }

                    let new_x = (x + character_idx as i32) as usize;
                    let new_y = (y + line_idx as i32) as usize;

                    let is_symbol = get_character_at(new_x, new_y)
                        .map_or(false, |c| !c.is_numeric() && c != &'.');
                    is_number_suitable |= is_symbol;

                    if is_symbol {
                        break 'x;
                    }
                }
            }
        }

        if number != 0 && is_number_suitable {
            sum += number;
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
            35,
            solve(
                "..*..
                 ..35.
                 ....."
            )
        );

        assert_eq!(
            4361,
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
        )
    }
}
