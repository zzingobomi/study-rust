#[derive(Clone, Copy, PartialEq, Debug)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

struct Card {
    suit: Suit,
    rank: i32,
}

fn main() {
    let card = Card {
        suit: Suit::Club,
        rank: 1,
    };
}
