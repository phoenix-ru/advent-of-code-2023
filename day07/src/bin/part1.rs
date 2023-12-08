use day07::{parse_input, Hand, get_card_rank, HandArrangement, compute_total};

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input))
}

fn solve(input: &str) -> u32 {
    let mut data = parse_input(input);

    // Set arrangements as per part 1
    for (hand, _) in data.iter_mut() {
        set_arrangement(hand);
    }

    compute_total(&mut data)
}

fn set_arrangement(hand: &mut Hand) {
    let mut bins = [0; 14];
    for card in hand.cards.iter() {
        let rank = get_card_rank(*card, false);
        bins[rank] += 1;
    }

    let mut max1 = 0;
    let mut max2 = 0;

    for bin_value in bins {
        if bin_value >= max1 {
            max2 = max1;
            max1 = bin_value;
        } else if bin_value >= max2 {
            max2 = bin_value;
        }
    }

    hand.arrangement = match (max1, max2) {
        (5, _) => HandArrangement::FiveOfAKind,
        (4, _) => HandArrangement::FourOfAKind,
        (3, 2) => HandArrangement::FullHouse,
        (3, _) => HandArrangement::ThreeOfAKind,
        (2, 2) => HandArrangement::TwoPair,
        (2, _) => HandArrangement::OnePair,
        _ => HandArrangement::HighCard
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            6440,
            solve(
                "32T3K 765
                T55J5 684
                KK677 28
                KTJJT 220
                QQQJA 483"
            )
        )
    }
}
