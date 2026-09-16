pub enum Suit {
    Blank   = 0,
    Spade   = 1,
    Heart   = 2,
    Diamond = 3,
    Club    = 4
}

pub enum Rank {
    Blank = 0,
    Ace   = 1,
    Two   = 2,
    Three = 3,
    Four  = 4,
    Five  = 5,
    Six   = 6,
    Seven = 7,
    Eight = 8,
    Nine  = 9,
    Ten   = 10,
    Jack  = 11,
    Queen = 12,
    King  = 13,
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank
}

pub struct Hand {
}

pub struct ShelemGame {

}
