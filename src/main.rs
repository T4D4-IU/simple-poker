#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suit: Suit,
    rank: i32,
}

fn main() {
    // Vecの用意
    let mut deck:Vec<Card> = Vec::new();
    let suit = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    // Deckの生成
    for suit in suit {
        for rank in 1..=13 {
            // Vecにカードを入れる
            deck.push(Card { suit, rank });
        }
    }
    println!("{:?}", deck);
}
