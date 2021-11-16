#![feature(step_trait)]
mod rank;
mod suit;
pub use rank::Rank;
use std::cmp::Ordering;
pub use suit::Suit;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Hand {
    StraightFlush = 0,
    ThreeOfAKind,
    Straight,
    Flush,
    Pair,
    HighCard,
}
impl From<usize> for Hand {
    fn from(i: usize) -> Self {
        match i {
            0 => Hand::StraightFlush,
            1 => Hand::ThreeOfAKind,
            2 => Hand::Straight,
            3 => Hand::Flush,
            4 => Hand::Pair,
            5 => Hand::HighCard,
            _ => panic!("invalid hand: {}", i),
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((*other as usize).cmp(&(*self as usize)))
    }
}
fn optimal_play(
    cards: [Card; 3],
    paytable: [usize; Hand::HighCard as usize + 1],
) -> [Option<Card>; 3] {
    let deck = generate_deck();
    let mut switch_tables = vec![];
    for i in 0..cards.len() {
        switch_tables.push(HandTable::default());
        for card in deck.iter() {
            if !cards.contains(card) {
                let mut new_hand = cards.clone();
                new_hand[i] = *card;
                switch_tables[i].push_sequence(new_hand);
            }
        }
    }
    let mut max_return = 0.0f32;
    let mut max_idx = 0;
    for i in 0..switch_tables.len() {
        let pay = switch_tables[i].calcualte_return_full(&paytable);
        if pay > max_return {
            max_return = pay;
            max_idx = i;
        }
    }
    let current_hand = get_hand(&cards);
    let current_return = paytable[current_hand as usize] as f32;
    if max_return > current_return {
        println!("max return: {}, i = {}", max_return, max_idx);
        let mut out = [Some(cards[0]), Some(cards[1]), Some(cards[2])];
        out[max_idx] = None;
        out
    } else {
        println!("current return: {}", current_return);
        [Some(cards[0]), Some(cards[1]), Some(cards[2])]
    }
}
pub fn is_sequence((c1, c2, c3): (Card, Card, Card)) -> bool {
    let mut suites = vec![c1.rank, c2.rank, c3.rank];
    suites.sort();
    let c0 = suites[0] as u32;
    let c1 = suites[1] as u32;
    let c2 = suites[2] as u32;

    (c0 + 2 == c1 + 1 && c1 + 1 == c2) || suites == [Rank::A, Rank::Queen, Rank::King]
}
fn is_same_suit(cards: (Card, Card, Card)) -> bool {
    cards.0.suit == cards.1.suit && cards.1.suit == cards.2.suit
}
fn is_same_rank(cards: (Card, Card, Card)) -> bool {
    cards.0.rank == cards.1.rank && cards.1.rank == cards.2.rank
}
fn is_two_same_rank((c1, c2, c3): (Card, Card, Card)) -> bool {
    let mut suites = vec![c1.rank, c2.rank, c3.rank];
    suites.sort();
    suites[0] == suites[1] || suites[1] == suites[2]
}
fn get_hand(cards: &[Card; 3]) -> Hand {
    if is_sequence((cards[0], cards[1], cards[2])) {
        if is_same_suit((cards[0], cards[1], cards[2])) {
            return Hand::StraightFlush;
        } else {
            return Hand::Straight;
        }
    }
    if is_same_suit((cards[0], cards[1], cards[2])) {
        return Hand::Flush;
    }
    if is_same_rank((cards[0], cards[1], cards[2])) {
        return Hand::ThreeOfAKind;
    }
    if is_two_same_rank((cards[0], cards[1], cards[2])) {
        return Hand::Pair;
    }
    Hand::HighCard
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}
fn generate_deck() -> Vec<Card> {
    (Suit::Clubs..=Suit::Diamonds)
        .flat_map(|suit| (Rank::A..=Rank::King).map(move |rank| Card { rank, suit }))
        .collect::<Vec<_>>()
}
#[derive(Clone, Debug)]
struct HandPayGuess {
    pub pay: [Option<usize>; Hand::HighCard as usize + 1],
}
struct HandTable {
    hands: [usize; Hand::HighCard as usize + 1],
}
impl std::ops::Add for HandTable {
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        for i in 0..self.hands.len() {
            self.hands[i] += other.hands[i]
        }
        self
    }
}
impl HandTable {
    pub fn push_sequence(&mut self, cards: [Card; 3]) {
        let hand = get_hand(&cards);
        self.hands[hand as usize] += 1;
    }
    pub fn calculate_return(&self, guess: &HandPayGuess) -> f32 {
        let total: usize = self.hands.iter().sum();
        let probs = self.hands.iter().map(|n| *n as f32 / (total as f32));
        probs
            .zip(guess.pay.iter())
            .filter(|(_prob, pay)| pay.is_some())
            .map(|(prob, pay)| (prob, pay.unwrap()))
            .map(|(prob, pay)| prob * (pay as f32))
            .fold(0.0, |acc, x| acc + x)
    }
    fn calcualte_return_full(&self, guess: &[usize; Hand::HighCard as usize + 1]) -> f32 {
        let total: usize = self.hands.iter().sum();
        let probs = self.hands.iter().map(|n| *n as f32 / (total as f32));
        probs
            .zip(guess.iter())
            .map(|(prob, pay)| prob * (*pay as f32))
            .fold(0.0, |acc, x| acc + x)
    }
    /// Uses brute force to find the pay table that fits inside
    /// of the given range with the pays in decending order,
    /// assumes first pay is filled out and lays out unfilled pays in decending order
    pub fn build_paytable(
        &self,
        guess: HandPayGuess,
        low_return: f32,
        high_return: f32,
    ) -> Option<HandPayGuess> {
        let mut check_idx = guess
            .pay
            .iter()
            .enumerate()
            .filter(|(_i, pay)| pay.is_none())
            .map(|(i, _pay)| i);
        let expected_pay = self.calculate_return(&guess);
        if expected_pay > high_return {
            None
        } else if expected_pay > low_return {
            Some(guess)
        } else if let Some(idx) = check_idx.next() {
            let mut guesses = (0..guess.pay[idx - 1].unwrap())
                .rev()
                .map(|guess_pay| {
                    let mut guess = guess.clone();
                    guess.pay[idx] = Some(guess_pay);
                    self.build_paytable(guess, low_return, high_return)
                })
                .flatten();
            guesses.next()
        } else {
            None
        }
    }
}
pub struct PrintTable {}
impl std::fmt::Display for HandTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:^20}|{:>6}", "Hand", "n")?;
        writeln!(f, "----------------------------")?;
        for (i, num) in self.hands.iter().enumerate() {
            let hand: Hand = i.into();
            writeln!(f, "{:^20}|{:>6}", format!("{:?}", hand), num)?;
        }
        Ok(())
    }
}
impl Default for HandTable {
    fn default() -> Self {
        Self {
            hands: [0; Hand::HighCard as usize + 1],
        }
    }
}
fn generate_all_games() {
    let deck = generate_deck();
    let mut table = HandTable::default();
    for i in 0..deck.len() {
        for j in 0..deck.len() {
            for k in 0..deck.len() {
                if i != j && j != k && i != k {
                    let c1 = deck[i];
                    let c2 = deck[j];
                    let c3 = deck[k];
                    table.push_sequence([c1, c2, c3]);
                }
            }
        }
    }
    println!("{}", table);
    let pay_table = table
        .build_paytable(
            HandPayGuess {
                pay: [Some(250), None, None, None, None, Some(0)],
            },
            0.99,
            1.00,
        )
        .unwrap();
    println!("{:?}", pay_table);
    println!("return: {}", table.calculate_return(&pay_table));
}
fn main() -> Result<(), ()> {
    generate_all_games();
    Ok(())
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t_optimal_play() {
        let hand = [
            Card {
                rank: Rank::Eight,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Clubs,
            },
        ];
        let best_hand = optimal_play(hand, [1, 0, 0, 0, 0, 0]);
        assert_eq!(
            best_hand,
            [
                Some(Card {
                    rank: Rank::Eight,
                    suit: Suit::Clubs,
                }),
                Some(Card {
                    rank: Rank::Nine,
                    suit: Suit::Clubs,
                }),
                Some(Card {
                    rank: Rank::Ten,
                    suit: Suit::Clubs,
                }),
            ]
        );
    }
    #[test]
    fn sequence() {
        let hand = (
            Card {
                rank: Rank::A,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
        );
        assert_eq!(is_sequence(hand), true);
        let hand = (
            Card {
                rank: Rank::A,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
        );
        assert_eq!(is_sequence(hand), false);
    }
}
