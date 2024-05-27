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
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl Card {
    fn from(card: char) -> Card {
        match card {
            'J' => Card::J,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
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
        let mut hand_points = counts[0] * 2; // little trick

        if (hand_points == 4 || hand_points == 6) && counts[1..].contains(&2) {
            hand_points += 1;
        }

        hand_points += HandType::_count_j_cards(cards) * 2; //the same little trick

        match hand_points {
            2 => return HandType::HighCard,
            3 => return HandType::HighCard,
            4 => return HandType::OnePair,
            5 => return HandType::TwoPair,
            6 => return HandType::ThreeOfAKind,
            7 => return HandType::FullHouse,
            8 => return HandType::FourOfAKind,
            9 => return HandType::FourOfAKind,
            10 => return HandType::FiveOfAKind,
            _ => panic!("Invalid hand"),
        }
    }

    fn _count(cards: &[Card; 5]) -> [u8; 5] {
        let mut counts = [0u8; 13]; // 13 possible ranks

        for card in cards.iter().filter(|&card| card.clone() != Card::J) {
            counts[card.clone() as usize] += 1;
        }

        counts.sort_unstable_by(|a, b| b.cmp(a));
        counts[..5].try_into().unwrap()
    }

    fn _count_j_cards(cards: &[Card; 5]) -> u8 {
        cards.iter().fold(0, |acc, elem| match elem {
            Card::J => acc + 1,
            _ => acc,
        })
    }
}

fn main() {
    let mut hands = read_input_file();
    hands.sort_by(|hand, other_hand| hand.clone().compare_to(&other_hand));
    let total_winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, hand)| acc + (idx as u32 + 1) * hand.bid);
    println!("{:?}", total_winnings);
}
