use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone)]
pub struct Card {
    id: u32,
    winning: HashSet<u32>,
    hand: HashSet<u32>,
}

impl Card {
    pub fn new(id: u32, winning: HashSet<u32>, hand: HashSet<u32>) -> Self {
        Self { id, winning, hand }
    }

    pub fn winning_numbers(&self) -> impl Iterator<Item = u32> + '_ {
        self.winning.intersection(&self.hand).copied()
    }

    fn won_cards(&self) -> Vec<u32> {
        (self.id + 1..=self.id + self.winning_numbers().count() as u32).collect()
    }

    pub fn total_scratchcards(cards: &[Card]) -> u64 {
        let mut card_counts = vec![1; cards.len()];

        let mut total_cards = 0_u64;
        for i in 0..cards.len() {
            let card = &cards[i];
            let count = card_counts[i];
            total_cards += count;
            let won_cards = (i + 1..i + 1 + card.won_cards().len());
            for j in won_cards {
                card_counts[j] += count;
            }
        }

        total_cards
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}
