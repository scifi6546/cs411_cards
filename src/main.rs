#![feature(step_trait)]
mod rank;
mod suit;
pub use rank::Rank;
pub use suit::Suit;
pub enum Hand {
    StraightFlush = 0,
    ThreeOfAKind,
    Straight,
    Flush,
    Pair,
    HighCard,
}
pub fn is_sequence((c1, c2, c3): (Card, Card, Card)) -> bool {
    let mut suites = vec![c1.rank, c2.rank, c3.rank];
    suites.sort();
    let c0 = suites[0] as u32;
    let c1 = suites[1] as u32;
    let c2 = suites[2] as u32;

    return (c0 + 2 == c1 + 1 && c1 + 1 == c2) || suites == [Rank::A, Rank::Queen, Rank::King];
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
fn get_hand(cards: (Card, Card, Card)) -> Hand {
    if is_sequence(cards) {
        if is_same_suit(cards) {
            return Hand::StraightFlush;
        } else {
            return Hand::Straight;
        }
    }
    if is_same_suit(cards) {
        return Hand::Flush;
    }
    if is_same_rank(cards) {
        return Hand::ThreeOfAKind;
    }
    if is_two_same_rank(cards) {
        return Hand::Pair;
    }
    return Hand::HighCard;
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}
fn generate_deck() -> Vec<Card> {
    (Suit::Clubs..=Suit::Diamonds)
        .flat_map(|suit| (Rank::A..=Rank::King).map(move |rank| Card { rank, suit }))
        .collect::<Vec<_>>()
}
fn generate_all_games() {
    let deck = generate_deck();
    let mut num_hands = [0usize; Hand::HighCard as usize + 1];
    for i in 0..deck.len() {
        for j in 0..deck.len() {
            for k in 0..deck.len() {
                if i != j && j != k && i != k {
                    let c1 = deck[i];
                    let c2 = deck[j];
                    let c3 = deck[k];
                    num_hands[get_hand((c1, c2, c3)) as usize] += 1;
                }
            }
        }
    }
    for i in num_hands {
        println!("{}", i);
    }
}
fn main() {
    generate_all_games();
    println!("Hello, world!");
}
#[cfg(test)]
mod test {
    use super::*;
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
