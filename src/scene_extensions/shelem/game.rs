use super::data_structures::Arr;

#[derive(Copy, Clone, Debug)]
pub enum Suit {
    Blank   = 0,
    Spade   = 1,
    Heart   = 2,
    Diamond = 3,
    Club    = 4
}


#[derive(Copy, Clone, Debug)]
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


#[derive(Copy, Clone, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub visible: bool
}

pub struct Hand {
    pub cards: Arr<Card, 13>,
}

pub struct ShelemGame {
    pub playerA: Hand,
    pub playerB: Hand,
}
