use rand::seq::SliceRandom;

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
    // シャッフル
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
    println!("{:?}", deck);

    // 手札用のVecの用意
    let mut hand:Vec<Card> = Vec::new();
    // 手札にカードを5枚追加
    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }

    // 手札をソート
    hand.sort_by(|a,b| a.rank.cmp(&b.rank));

    // 手札を表示
    println!("--- Hand ---");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i+1, card.suit, card.rank);
    }
}
