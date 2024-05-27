use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

const FILE_PATH: &str = "input.txt";
// const FILE_PATH: &str = "smaller_input.txt";

fn read_input_file() -> Vec<Hand> {
    let contents = fs::read_to_string(FILE_PATH)
        .unwrap_or_else(|err| panic!("Couldn't read input file: {}", err));

    let hands: Vec<Hand> = contents
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').expect("Invalid input format");
            let line_cards: Vec<Card> = cards.chars().map(Card::from).collect();
            let line_cards: [Card; 5] = line_cards.try_into().expect("Expected 5 cards");
            let hand_type = HandType::from(&line_cards);

            Hand {
                cards: line_cards,
                bid: bid.parse().expect("Invalid bid value"),
                hand_type,
            }
        })
        .collect();
    hands
}

#[derive(Debug, Clone)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
    hand_type: HandType,
}

impl Hand {
    fn compare_to(self, other: &Hand) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => {}
        }

        for (card, other_card) in self.cards.iter().zip(other.cards.clone()) {
            match card.cmp(&other_card) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => {}
            }
        }
        Ordering::Equal
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from(card: char) -> Card {
        match card {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("Invalid card: {}", card),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from(cards: &[Card; 5]) -> HandType {
        let counts = HandType::_count(cards);
        for (i, count) in counts.iter().enumerate() {
            match count {
                5 => return HandType::FiveOfAKind,
                4 => return HandType::FourOfAKind,
                3 => {
                    if counts.contains(&2) {
                        return HandType::FullHouse;
                    }
                    return HandType::ThreeOfAKind;
                }
                2 => {
                    if counts[i + 1..].contains(&2) {
                        return HandType::TwoPair;
                    }
                    return HandType::OnePair;
                }
                _ => {}
            }
        }
        HandType::HighCard
    }

    fn _count(cards: &[Card; 5]) -> [u8; 5] {
        let mut counts = [0; 13]; // 13 possible ranks

        for card in cards {
            counts[card.clone() as usize] += 1;
        }

        // Only take the non-zero counts and sort them
        let mut non_zero_counts: Vec<u8> = counts.iter().cloned().filter(|&count| count > 0).collect();
        non_zero_counts.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order

        let mut result = [0; 5];
        for (i, &count) in non_zero_counts.iter().enumerate() {
            result[i] = count;
        }

        result
    }
}

fn main() {
    let mut hands = read_input_file();
    hands.sort_by(|hand, other_hand| hand.clone().compare_to(&other_hand));
    let total_winings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, hand)| acc + (idx as u32 + 1) * hand.bid);
    println!("{:?}", total_winings);
}
