use crate::types::Hand;
use crate::{analysis::profit_curve, types::Street};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    SB,
    BB,
    UTG,
    MP,
    CO,
    BTN,
}

#[derive(Serialize, Deserialize)]
pub enum ActionType {
    PostSmallBlind,
    PostBigBlind,
    Fold,
    Check,
    Call,
    Bet,
    RaiseTo,
    Collect,
}

#[derive(Serialize, Deserialize)]
pub struct Filters {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub position: Option<Position>,
    pub street: Option<Street>,
    pub action: Option<ActionType>,
}

#[derive(Serialize, Deserialize)]
pub struct CurvePoint {
    pub hand_id: String,
    pub date: String,
    pub net: i64,
    pub net_bb: f64,
    pub cumulative: i64,
    pub cumulative_bb: f64,
    pub cumulative_showdown: i64,
    pub cumulative_showdown_bb: f64,
    pub cumulative_nonshowdown: i64,
    pub cumulative_nonshowdown_bb: f64,
}

#[tauri::command]
pub fn get_profit_curve(
    player: &str,
    filters: Filters,
    hands: State<Vec<Hand>>,
) -> Vec<CurvePoint> {
    profit_curve(&hands, player, filters)
}
