#![feature(step_trait)]
mod rank;
mod suit;
use rank::Rank;
use suit::Suit;

fn generate_all_games() {
    for i in Suit::CLUBS..=Suit::DIAMONDS {
        println!("{:?}", i);
    }
    for i in Rank::A..=Rank::KING {
        println!("{:?}", i);
    }
}
fn main() {
    generate_all_games();
    println!("Hello, world!");
}
