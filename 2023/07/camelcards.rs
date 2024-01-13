use core::panic;
use std::io::{self, BufRead};

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Card {
    Ace = 13,
    King = 12,
    Queen = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq)]
struct Hand {
    cards: [Card; 5],
    typ: HandType,
    bid: u64,
}

impl Hand {
    fn new(cards: [Card; 5], bid: u64) -> Self {
        let typ = Hand::calculate_hand_type(&cards);
        Hand { cards, typ, bid }
    }

    fn calculate_hand_type(cards: &[Card]) -> HandType {
        let cards: &mut [Card] = &mut cards.to_vec();
        cards.sort();

        let mut counts = [0; 15];
        let mut joker_count = 0;
        for card in cards {
            if *card == Card::Joker {
                joker_count += 1;
                continue;
            }
            counts[*card as usize] += 1;
        }
        let mut counts = counts.to_vec();
        counts.sort();
        counts.reverse();
        let mut counts = counts.iter();
        let highest_count = counts.next().unwrap() + joker_count;
        let second_highest_count = counts.next().unwrap();
        if highest_count == 5 {
            HandType::FiveOfAKind
        } else if highest_count == 4 {
            HandType::FourOfAKind
        } else if highest_count == 3 && *second_highest_count == 2 {
            HandType::FullHouse
        } else if highest_count == 3 {
            HandType::ThreeOfAKind
        } else if highest_count == 2 && *second_highest_count == 2 {
            HandType::TwoPairs
        } else if highest_count == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let l1 = self.typ.cmp(&other.typ);
        if l1 != std::cmp::Ordering::Equal {
            return l1;
        }
        // find higher card in case of tie
        for i in 0..5 {
            let l2 = self.cards[i].cmp(&other.cards[i]);
            if l2 != std::cmp::Ordering::Equal {
                return l2;
            }
        }
        return std::cmp::Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.typ == other.typ
    }
}

fn total_winnings_part_1(hands: &[Hand]) -> u64 {
    let mut total = 0;
    let mut multiplier = 1;
    for hand in hands {
        total += hand.bid * multiplier;
        multiplier += 1;
    }
    total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut input = read_input(lines)?;
    input.sort();
    let wins = total_winnings_part_1(&input);
    println!("total winnings: {}", wins);

    Ok(())
}

fn read_input(
    lines: impl Iterator<Item = Result<String, io::Error>>,
) -> Result<Vec<Hand>, Box<dyn std::error::Error>> {
    let mut hands = Vec::new();
    for line in lines {
        let line = line?;
        let parts = line.split_once(" ").ok_or("invalid input")?;
        let mut cards: [Card; 5] = [Card::Ace; 5];
        for (i, card) in parts.0.chars().enumerate() {
            let card = match card {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'J' => Card::Joker,
                'T' => Card::Ten,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                '2' => Card::Two,
                _ => panic!("invalid card"),
            };
            cards[i] = card;
        }

        let bid = parts.1.parse::<u64>()?;
        let hand = Hand::new(cards.try_into().unwrap(), bid);
        hands.push(hand);
    }
    Ok(hands)
}
