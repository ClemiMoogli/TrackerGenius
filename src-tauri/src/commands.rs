use crate::analysis::profit_curve;
use crate::types::{CurvePoint, Hand};
use tauri::State;

#[tauri::command]
pub fn get_profit_curve(player: &str, hands: State<Vec<Hand>>) -> Vec<CurvePoint> {
    profit_curve(&hands, player)
}
