fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> u32 {
    let mut sum = 0;

    const TARGET_RED: u32 = 12;
    const TARGET_GREEN: u32 = 13;
    const TARGET_BLUE: u32 = 14;

    'line: for line in input.lines() {
        let line = line.trim();

        let (game_header, game_data) = line.split_once(": ").unwrap();
        let game_id = game_header.split(' ').nth(1).unwrap();

        for turn in game_data.split("; ") {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for draw in turn.split(", ") {
                if let Some((amount, color)) = draw.split_once(' ') {
                    let amount: u32 = amount.parse().unwrap();

                    match color {
                        "red" => red += amount,
                        "green" => green += amount,
                        "blue" => blue += amount,
                        _ => unreachable!("Unknown color")
                    }
                }
            }

            if red > TARGET_RED || green > TARGET_GREEN || blue > TARGET_BLUE {
                continue 'line;
            }
        }

        let game_num: u32 = game_id.parse().unwrap();
        sum += game_num;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            8,
            solve("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
        )
    }
}
