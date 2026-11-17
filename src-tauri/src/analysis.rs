use crate::types::{ActionKind, CurvePoint, GameFormat, Hand, Street};

fn net_result(hand: &Hand, player: &str) -> i64 {
    let mut collected: i64 = 0;
    let mut invested: i64 = 0;
    let mut street_invested: i64 = 0;
    let mut street: Street = Street::Preflop;
    for action in hand.actions.iter() {
        if action.player != player {
            continue;
        }
        if action.street != street {
            street = action.street;
            invested += street_invested;
            street_invested = 0;
        }
        match action.action {
            ActionKind::PostSmallBlind(x) => street_invested += x,
            ActionKind::PostBigBlind(x) => street_invested += x,
            ActionKind::Bet(x) => street_invested += x,
            ActionKind::Call(x) => street_invested += x,
            ActionKind::RaiseTo(x) => street_invested = x,
            ActionKind::Collect(x) => collected += x,
            _ => (),
        }
    }
    invested += street_invested;
    collected - invested
}

fn went_to_showdown(hand: &Hand, player: &str) -> bool {
    if !hand.is_showdown {
        return false;
    }
    if hand
        .actions
        .iter()
        .any(|action| matches!(action.action, ActionKind::Fold) && action.player == player)
    {
        return false;
    }
    true
}

pub fn profit_curve(hands: &[Hand], player: &str) -> Vec<CurvePoint> {
    let mut curve_points: Vec<CurvePoint> = Vec::new();
    let mut acc: i64 = 0;
    let mut acc_bb: f64 = 0.0;
    let mut hand_bb_pnl: f64;
    let mut cumulative_showdown: i64 = 0;
    let mut cumulative_nonshowdown: i64 = 0;
    let mut cumulative_showdown_bb: f64 = 0.0;
    let mut cumulative_nonshowdown_bb: f64 = 0.0;

    for hand in hands.iter() {
        if !matches!(hand.game_format, GameFormat::CashGame) {
            continue;
        }
        if !hand.seats.iter().any(|seat| seat.player == player) {
            continue;
        }
        let net: i64 = net_result(hand, player);
        acc += net;
        hand_bb_pnl = net as f64 / hand.big_blind as f64;
        acc_bb += hand_bb_pnl;
        if went_to_showdown(hand, player) {
            cumulative_showdown += net;
            cumulative_showdown_bb += hand_bb_pnl;
        } else {
            cumulative_nonshowdown += net;
            cumulative_nonshowdown_bb += hand_bb_pnl;
        }
        let curve_point = CurvePoint {
            hand_id: hand.id.clone().into(),
            date: hand.date.clone().into(),
            net: net,
            net_bb: hand_bb_pnl,
            cumulative: acc,
            cumulative_bb: acc_bb,
            cumulative_showdown: cumulative_showdown,
            cumulative_showdown_bb: cumulative_showdown_bb,
            cumulative_nonshowdown: cumulative_nonshowdown,
            cumulative_nonshowdown_bb: cumulative_nonshowdown_bb,
        };
        curve_points.push(curve_point);
    }
    curve_points
}
