use rand::seq::SliceRandom;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Card {
    suit: Suit,
    rank: i32,
}

fn main() {
    // 카드 준비
    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    for suit in suits {
        for rank in 1..=13 {
            deck.push(Card { suit, rank });
        }
    }

    // 섞기
    let mut rng = rand::rng();
    deck.shuffle(&mut rng);

    // 카드 5장 뽑기
    let mut hand: Vec<Card> = Vec::new();
    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    println!("---Hand---");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i + 1, card.suit, card.rank);
    }

    println!("교환하고 싶은 카드 번호를 입력하세요 (예: 1 2 3)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();
    for number in numbers {
        hand[number - 1] = deck.pop().unwrap();
    }

    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    println!("---Hand---");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i + 1, card.suit, card.rank);
    }

    // 판정
    let suit = hand.first().unwrap().suit;
    let flash = hand.iter().all(|c| c.suit == suit);

    let mut count = 0;
    for i in 0..hand.len() - 1 {
        for j in i + 1..hand.len() {
            if hand[i].rank == hand[j].rank {
                count += 1;
            }
        }
    }

    if flash {
        println!("플러시!")
    } else if count >= 3 {
        println!("쓰리 카드!")
    } else if count == 2 {
        println!("투 페어!")
    } else if count == 1 {
        println!("원 페어!")
    } else {
        println!("노 페어...")
    }
}
