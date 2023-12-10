use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cards {
    cards: [Card; 5],
    hand_type: HandType,
    bid: u32,
}

impl Cards {
    pub fn new(cards: [Card; 5], bid: u32) -> Self {
        let hand_type = Self::get_hand_type(&cards);
        Self {
            cards,
            hand_type,
            bid,
        }
    }

    fn get_hand_type(cards: &[Card; 5]) -> HandType {
        if let Some(five_of_a_kind) = Self::get_five_of_a_kind(cards) {
            return five_of_a_kind;
        }
        let card_counts = Self::count_cards(cards);
        if let Some(four_of_a_kind) = Self::get_four_of_a_kind(cards, &card_counts) {
            return four_of_a_kind;
        }
        if let Some(full_house) = Self::get_full_house(cards, &card_counts) {
            return full_house;
        }
        if let Some(three_of_a_kind) = Self::get_three_of_a_kind(cards, &card_counts) {
            return three_of_a_kind;
        }
        if let Some(two_pair) = Self::get_two_pair(cards, &card_counts) {
            return two_pair;
        }
        if let Some(one_pair) = Self::get_one_pair(cards, &card_counts) {
            return one_pair;
        }
        Self::get_high_card(*cards)
    }

    fn get_five_of_a_kind(cards: &[Card; 5]) -> Option<HandType> {
        let set: HashSet<_> = cards.iter().copied().collect();
        if set.len() == 1 {
            Some(HandType::FiveOfAKind(cards[0]))
        } else {
            None
        }
    }

    fn get_four_of_a_kind(cards: &[Card; 5], card_counts: &HashMap<Card, u32>) -> Option<HandType> {
        if let Some((card, _)) = card_counts.iter().find(|(_, &count)| count == 4) {
            let (other, _) = card_counts.iter().find(|(_, &v)| v == 1).unwrap();
            Some(HandType::FourOfAKind {
                card: *card,
                other: *other,
            })
        } else {
            None
        }
    }

    fn get_full_house(cards: &[Card; 5], card_counts: &HashMap<Card, u32>) -> Option<HandType> {
        if let Some((three, _)) = card_counts.iter().find(|(_, &v)| v == 3) {
            if let Some((two, _)) = card_counts.iter().find(|(_, &v)| v == 2) {
                return Some(HandType::FullHouse {
                    three: *three,
                    two: *two,
                });
            }
        }

        None
    }

    fn get_three_of_a_kind(
        cards: &[Card; 5],
        card_counts: &HashMap<Card, u32>,
    ) -> Option<HandType> {
        if let Some((three, _)) = card_counts.iter().find(|(_, &v)| v == 3) {
            let other: Vec<_> = card_counts
                .iter()
                .filter(|(k, &v)| v != 3)
                .map(|(k, _)| *k)
                .collect();
            debug_assert_eq!(other.len(), 2);
            return Some(HandType::ThreeOfAKind {
                card: *three,
                other: (other[0], other[1]),
            });
        }

        None
    }

    fn get_two_pair(cards: &[Card; 5], card_counts: &HashMap<Card, u32>) -> Option<HandType> {
        let mut twos: Vec<_> = card_counts
            .iter()
            .filter(|(_, &v)| v == 2)
            .map(|(c, _)| c)
            .collect();
        if twos.len() == 2 {
            twos.sort();
            let (other, _) = card_counts.iter().find(|(_, &v)| v == 1).unwrap();
            Some(HandType::TwoPair {
                high_pair: *twos[1],
                low_pair: *twos[0],
                other: *other,
            })
        } else {
            None
        }
    }

    fn get_one_pair(cards: &[Card; 5], card_counts: &HashMap<Card, u32>) -> Option<HandType> {
        if let Some((card, _)) = card_counts.iter().find(|(_, c)| **c == 2) {
            let other: Vec<_> = card_counts
                .iter()
                .filter(|(_, c)| **c == 1)
                .map(|(card, _)| card)
                .collect();
            debug_assert_eq!(other.len(), 3);
            Some(HandType::OnePair {
                pair: *card,
                other: (*other[0], *other[1], *other[2]),
            })
        } else {
            None
        }
    }

    fn get_high_card(mut cards: [Card; 5]) -> HandType {
        cards.sort();
        HandType::HighCard {
            high: cards[4],
            other: (cards[0], cards[1], cards[2], cards[3]),
        }
    }

    fn count_cards(cards: &[Card; 5]) -> HashMap<Card, u32> {
        let mut card_counts: HashMap<Card, u32> = HashMap::new();
        for &card in cards {
            *card_counts.entry(card).or_insert(0) += 1;
        }
        card_counts
    }

    pub fn bid(&self) -> u32 {
        self.bid
    }

    pub fn cards(&self) -> [Card; 5] {
        self.cards
    }

    pub fn hand_type(&self) -> HandType {
        self.hand_type
    }
}

impl Ord for Cards {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self
            .hand_type
            .numeric_value()
            .cmp(&other.hand_type.numeric_value());
        if ord.is_ne() {
            return ord;
        }

        self.cards
            .iter()
            .zip(other.cards.iter())
            .find_map(|(s, o)| match s.cmp(o) {
                Ordering::Equal => None,
                ord => Some(ord),
            })
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Card(u32);

impl Card {
    pub fn new(v: u32) -> Result<Self, ()> {
        if v < 1 || v > 14 {
            Err(())
        } else {
            Ok(Self(v))
        }
    }

    pub fn card_value(&self) -> u32 {
        self.0
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if let Some(v) = value.to_digit(10) {
            Ok(Self(v))
        } else {
            match value {
                'A' => Ok(Self(14)),
                'K' => Ok(Self(13)),
                'Q' => Ok(Self(12)),
                'J' => Ok(Self(11)),
                'T' => Ok(Self(10)),
                _ => Err(()),
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HandType {
    FiveOfAKind(Card),
    FourOfAKind {
        card: Card,
        other: Card,
    },
    FullHouse {
        three: Card,
        two: Card,
    },
    ThreeOfAKind {
        card: Card,
        other: (Card, Card),
    },
    TwoPair {
        high_pair: Card,
        low_pair: Card,
        other: Card,
    },
    OnePair {
        pair: Card,
        other: (Card, Card, Card),
    },
    HighCard {
        high: Card,
        other: (Card, Card, Card, Card),
    },
}

impl HandType {
    pub fn numeric_value(&self) -> u32 {
        match self {
            HandType::FiveOfAKind(_) => 6,
            HandType::FourOfAKind { card, other } => 5,
            HandType::FullHouse { three, two } => 4,
            HandType::ThreeOfAKind { card, other } => 3,
            HandType::TwoPair {
                high_pair,
                low_pair,
                other,
            } => 2,
            HandType::OnePair { pair, other } => 1,
            HandType::HighCard { high, other } => 0,
        }
    }
}
