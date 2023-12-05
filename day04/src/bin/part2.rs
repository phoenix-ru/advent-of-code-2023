fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> u32 {
    let mut card_counts: Vec<u32> = Vec::new();
    let mut total_cards = 0;

    for (card_idx, line) in input.lines().enumerate() {
        let (_card_header, card_data) = line.split_once(": ").unwrap();

        let (card_nums, player_nums) = card_data.trim().split_once(" | ").unwrap();

        // Parse nums
        let card_nums = parse_nums(card_nums);
        let mut player_nums = parse_nums(player_nums);

        // Compute how many numbers intersect. Sort is for binary search
        player_nums.sort();
        let mut intersects = 0;
        for card_num in card_nums {
            if player_nums.binary_search(&card_num).is_ok() {
                intersects += 1;
            }
        }

        // Add copies
        let current_copies = card_counts.get(card_idx).map_or(1, |v| *v);
        for idx in (card_idx + 1)..(card_idx + 1 + intersects) {
            // Fill missing slots
            for _ in card_counts.len()..=idx {
                card_counts.push(1);
            }

            if let Some(num) = card_counts.get_mut(idx) {
                *num += current_copies;
            } else {
                unreachable!("This is a bug")
            }
        }

        // Keep track of total cards
        total_cards += 1;
    }

    // Push missing counts if any
    for _ in card_counts.len()..total_cards {
        card_counts.push(1);
    }

    card_counts.into_iter().reduce(|acc, e| acc + e).unwrap()
}

fn parse_nums(input: &str) -> Vec<u8> {
    input
        .split(' ')
        .filter_map(|num| {
            let trimmed = num.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.parse().unwrap())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            30,
            solve(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            )
        );
    }
}
