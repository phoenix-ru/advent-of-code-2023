fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> u32 {
    let mut sum = 0;

    for game in input.lines() {
        let line = game.trim();

        let (_game_header, game_data) = line.split_once(": ").unwrap();

        let mut least_red = 0;
        let mut least_green = 0;
        let mut least_blue = 0;

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

            least_red = least_red.max(red);
            least_green = least_green.max(green);
            least_blue = least_blue.max(blue);
        }

        let game_power = least_red * least_green * least_blue;
        sum += game_power;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            2286,
            solve("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
        )
    }
}
