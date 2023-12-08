use std::cmp::Ordering;

#[derive(PartialEq, Eq, Ord)]
pub struct Hand {
    pub cards: [char; 5],
    pub arrangement: HandArrangement,
    pub enable_jokers: bool
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum HandArrangement {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.arrangement.cmp(&other.arrangement) {
            Ordering::Equal => {
                let enable_jokers = self.enable_jokers;

                for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    let self_rank = get_card_rank(*self_card, enable_jokers);
                    let other_rank = get_card_rank(*other_card, enable_jokers);

                    match self_rank.cmp(&other_rank) {
                        Ordering::Equal => continue,
                        ord => return Some(ord),
                    }
                }

                Some(Ordering::Equal)
            }
            ord => Some(ord),
        }
    }
}

pub fn get_card_rank(card: char, enable_jokers: bool) -> usize {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' if enable_jokers => 0,
        'J' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        _ => unreachable!(),
    }
}

pub fn parse_input(input: &str) -> Vec<(Hand, u32)> {
    input
        .lines()
        .into_iter()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            let (raw_hand, value) = line.split_once(' ').expect("Should split");

            let mut cards = ['\0'; 5];
            for (idx, card) in raw_hand.chars().take(5).enumerate() {
                cards[idx] = card;
            }
            let hand = Hand {
                cards,
                arrangement: HandArrangement::HighCard,
                enable_jokers: false
            };

            Some((hand, value.parse().unwrap()))
        })
        .collect()
}

pub fn compute_total(data: &mut Vec<(Hand, u32)>) -> u32 {
    data.sort();

    let mut total = 0;
    for (idx, (_, value)) in data.iter().enumerate() {
        total += (1 + idx as u32) * value;
    }

    total
}
