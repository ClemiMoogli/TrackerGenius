use crate::loader::load_data;
use crate::types::{
    Action, ActionKind, GameFormat, Hand, Header, PokerFormat, Seat, Street, TableInfo,
};

fn parse_amount(text: &str) -> Result<i64, String> {
    let value: f64 = text
        .trim_end_matches('€')
        .parse()
        .map_err(|_| format!("Montant invalid: {}", text))?;
    Ok((value * 100.0).round() as i64)
}

fn parse_seat(line: &str) -> Result<Seat, String> {
    let error = || format!("Ligne de siege invalid: {}", line);
    let rest = line.strip_prefix("Seat ").ok_or_else(error)?; // 1: ERIC GOAT932 (1.83$)
    let (number, rest) = rest.split_once(':').ok_or_else(error)?; // ("1", " ERIC GOAT932 (1.83$)")
    let (player, stack) = rest.split_once(" (").ok_or_else(error)?; // (" ERIC GOAT932 ", "1.83$)")
    let stack = stack.strip_suffix(')').ok_or_else(error)?; // stack = 1.83$
    Ok(Seat {
        number: number.parse::<u8>().map_err(|_| error())?,
        player: player.trim().into(),
        stack: parse_amount(stack)?,
    })
}
fn parse_header(text: &str) -> Result<Header, String> {
    // Winamax Poker - CashGame - HandId: #23172014-12-1794243590 - Holdem no limit (0.01€/0.02€) - 2026/11/09 16:59:50 UTC
    let error = || format!("Ligne de header invalid: {}", text);
    let (_, rem) = text.split_once(" - ").ok_or_else(error)?; // CashGame - HandId: #23172014-12-1794243590 - Holdem no limit (0.01€/0.02€) - 2026/11/09 16:59:50 UTC
    let (game_format, rem) = rem.split_once(" - ").ok_or_else(error)?; // (CashGame, HandId: #23172014-12-1794243590 - Holdem n...)
    let (id, rem) = rem.split_once(" - ").ok_or_else(error)?; // (HandId: #23172014-12-1794243590, Holdem n)
    let id = id.strip_prefix("HandId: #").ok_or_else(error)?;
    let (poker_format, rem) = rem.split_once(" - ").ok_or_else(error)?; // (Holdem no limit (0.01€/0.02€), 2026/11/09 16:59:50 UTC)
    let (poker_format, blinds) = poker_format.split_once('(').ok_or_else(error)?; // (Holdem no limit ,0.01/0.02$))
    let poker_format = poker_format.trim();
    let (small_blind, big_blind) = blinds.split_once('/').ok_or_else(error)?; // 0.01€/0.02€)
    let big_blind = big_blind.strip_suffix(')').ok_or_else(error)?;
    let small_blind = parse_amount(small_blind)?;
    let big_blind = parse_amount(big_blind)?;
    let date = rem.trim();
    let game_format = match game_format {
        "CashGame" => GameFormat::CashGame,
        _ => GameFormat::Other,
    };
    let poker_format = match poker_format {
        "Holdem no limit" => PokerFormat::NoLimitHoldem,
        _ => PokerFormat::Other,
    };
    let header = Header {
        game_format: game_format,
        id: id.into(),
        poker_format: poker_format,
        small_blind: small_blind,
        big_blind: big_blind,
        date: date.into(),
    };
    Ok(header)
}

fn parse_table_info(text: &str) -> Result<TableInfo, String> {
    let error = || format!("Erreur, table infos incorrect: {}", text);
    let (before, button_number) = text.split_once("Seat #").ok_or_else(error)?;
    let (_, nb_player) = before.rsplit_once('\'').ok_or_else(error)?;
    let (nb_player, _) = nb_player.split_once("-max").ok_or_else(error)?;
    let button_number = button_number
        .strip_suffix(" is the button")
        .ok_or_else(error)?;
    let table_info = TableInfo {
        nb_player: nb_player.trim().parse::<u8>().map_err(|_| error())?,
        button_number: button_number.parse::<u8>().map_err(|_| error())?,
    };
    Ok(table_info)
}

fn parse_action(street: Street, line: &str) -> Result<Option<Action>, String> {
    let error = || format!("Erreur, invalid action: {}", line);
    let action: Action;

    if let Some((player, rem)) = line.split_once(" posts small blind ") {
        let amount = rem.split_whitespace().next().ok_or_else(error)?;
        return Ok(Some(Action {
            street: street,
            player: player.into(),
            action: ActionKind::PostSmallBlind(parse_amount(amount)?),
        }));
    }
    if let Some((player, rem)) = line.split_once(" posts big blind ") {
        let amount = rem.split_whitespace().next().ok_or_else(error)?;
        return Ok(Some(Action {
            street: street,
            player: player.into(),
            action: ActionKind::PostBigBlind(parse_amount(amount)?),
        }));
    }
    // soit le joueur checks
    if let Some(player) = line.strip_suffix(" checks") {
        action = Action {
            street: street,
            player: player.into(),
            action: ActionKind::Check,
        };
        return Ok(Some(action));
    }
    // soit le joueur folds
    if let Some(player) = line.strip_suffix(" folds") {
        action = Action {
            street: street,
            player: player.into(),
            action: ActionKind::Fold,
        };
        return Ok(Some(action));
    }
    // soit le joueur raises
    if let Some((player, rem)) = line.split_once(" raises ") {
        let (_, mut str_amount) = rem.split_once(" to ").ok_or_else(error)?;
        str_amount = str_amount.split_whitespace().next().ok_or_else(error)?;
        let amount = parse_amount(str_amount)?;
        action = Action {
            street: street,
            player: player.into(),
            action: ActionKind::RaiseTo(amount),
        };
        return Ok(Some(action));
    }
    // soit le joueur calls
    if let Some((player, str_amount)) = line.split_once(" calls ") {
        let str_amount = str_amount.split_whitespace().next().ok_or_else(error)?;
        let amount = parse_amount(str_amount)?;
        action = Action {
            street: street,
            player: player.into(),
            action: ActionKind::Call(amount),
        };
        return Ok(Some(action));
    }
    // soit le joueur bets
    if let Some((player, str_amount)) = line.split_once(" bets ") {
        let str_amount = str_amount.split_whitespace().next().ok_or_else(error)?;
        let amount = parse_amount(str_amount)?;
        action = Action {
            street: street,
            player: player.into(),
            action: ActionKind::Bet(amount),
        };
        return Ok(Some(action));
    }
    // ou bien "collected"
    if let Some((player, rem)) = line.split_once(" collected ") {
        let (str_amount, _) = rem.split_once(" from ").ok_or_else(error)?;
        let amount = parse_amount(str_amount)?;
        action = Action {
            street: street,
            player: player.into(),
            action: ActionKind::Collect(amount),
        };
        return Ok(Some(action));
    }
    Ok(None)
}

fn parse_hand(text: &str) -> Result<Hand, String> {
    let mut lines = text.lines();
    // recuperer les infos sur le header
    let header = parse_header(lines.next().ok_or("Main vide")?)?;
    // recuperer les infos sur la table
    let table_info = parse_table_info(lines.next().ok_or("Main vide")?)?;
    // recuperer les informations sur les seats.
    let mut seats: Vec<Seat> = Vec::new();
    let mut is_showdown: bool = false;
    while let Some(line) = lines.next() {
        if line.starts_with("***") {
            break;
        }
        let tmp = parse_seat(line)?;
        seats.push(tmp);
    }
    // recuperer les infos sur les actions.
    let mut actions: Vec<Action> = Vec::new();
    let mut street: Street = Street::Preflop;
    while let Some(line) = lines.next() {
        if line.starts_with("*** SUMMARY ***") {
            break;
        }
        if line.contains("*** FLOP ***") {
            street = Street::Flop;
        }
        if line.contains("*** TURN ***") {
            street = Street::Turn;
        }
        if line.contains("*** RIVER ***") {
            street = Street::River;
        }
        if line.contains("*** SHOW DOWN ***") {
            street = Street::River;
            is_showdown = true;
        }
        if let Some(action) = parse_action(street, line)? {
            actions.push(action);
        }
    }
    // recuperer les infos sur le SUMMARY
    let hand: Hand = Hand {
        game_format: header.game_format,
        id: header.id,
        date: header.date,
        poker_format: header.poker_format,
        small_blind: header.small_blind,
        big_blind: header.big_blind,
        button_seat: table_info.button_number,
        seats: seats,
        actions: actions,
        is_showdown: is_showdown,
    };
    Ok(hand)
}

fn parse_hands(texts: Vec<String>) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();
    for text in texts {
        match parse_hand(&text) {
            Ok(hand) => hands.push(hand),
            Err(e) => println!("Main ignoree: {}", e),
        }
    }
    hands
}

pub fn parsing() -> Result<Vec<Hand>, std::io::Error> {
    let texts = load_data()?;
    let mut hands = parse_hands(texts);
    hands.sort_by(|a, b| a.date.cmp(&b.date));
    Ok(hands)
}
