enum Suit {
    Spade,
    Club,
    Heart,
    Diamond,
}

enum Card {
    Ace,
    Jack,
    Queen,
    King,
}

enum NumberCard {
    Ace = 1,
    Jack = 11,
    Queen = 12,
    King = 13,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Suit::Spade => write!(f, "♠︎"),
            Suit::Club => write!(f, "♣︎"),
            Suit::Heart => write!(f, "❤︎"),
            Suit::Diamond => write!(f, "♦︎"),
        }
    }
}

fn main() {
    let s1: Suit = Suit::Spade;
    let s2: Suit = Suit::Club;
    let s3: Suit = Suit::Heart;
    let s4: Suit = Suit::Diamond;

    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
    println!("s3 = {}", s3);
    println!("s4 = {}", s4);

    println!("番号なしカードを表示");
    println!("Ace = {}", Card::Ace as u8);
    println!("Jack = {}", Card::Jack as u8);
    println!("Queen = {}", Card::Queen as u8);
    println!("King = {}", Card::King as u8);

    println!("番号ありカードを表示");
    println!("Ace = {}", NumberCard::Ace as u8);
    println!("Jack = {}", NumberCard::Jack as u8);
    println!("Queen = {}", NumberCard::Queen as u8);
    println!("King = {}", NumberCard::King as u8);
}
