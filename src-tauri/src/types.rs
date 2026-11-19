use serde::{Deserialize, Serialize};

pub struct TableInfo {
    pub nb_player: u8,
    pub button_number: u8,
}

pub enum GameFormat {
    Tournament,
    CashGame,
    SitAndGo,
    Other,
}
pub enum PokerFormat {
    NoLimitHoldem,
    PotLimitOmaha,
    Other,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

pub struct Hand {
    pub game_format: GameFormat,
    pub id: String,
    pub date: String,
    pub poker_format: PokerFormat,
    pub small_blind: i64,
    pub big_blind: i64,
    pub button_seat: u8,
    pub seats: Vec<Seat>,
    pub actions: Vec<Action>,
    pub is_showdown: bool,
    pub board: Vec<Card>,
}

pub struct Seat {
    pub number: u8,
    pub player: String,
    pub stack: i64,
    pub cards: Option<[Card; 2]>,
}

pub struct Action {
    pub street: Street,
    pub player: String,
    pub action: ActionKind,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Street {
    Preflop,
    Flop,
    Turn,
    River,
}

#[derive(Serialize, Deserialize)]
pub enum ActionKind {
    PostSmallBlind(i64),
    PostBigBlind(i64),
    Fold,
    Check,
    Call(i64),
    Bet(i64),
    RaiseTo(i64),
    Collect(i64),
}

pub struct Header {
    pub game_format: GameFormat,
    pub id: String,
    pub poker_format: PokerFormat,
    pub small_blind: i64,
    pub big_blind: i64,
    pub date: String,
}
