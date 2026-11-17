import { CurvePoint } from "../types";

export function StatsPanel(props: {points: CurvePoint[]}) {

  const profit: number = props.points.length > 0 ? (props.points[props.points.length - 1].cumulative) / 100 : 0;
  const bb_100: number = props.points.length > 0 ? ((props.points[props.points.length - 1].cumulative_bb / props.points.length) * 100) :  0;
  return (
    <div className="stats-panel">
      <p className="stats">Mains jouées: {props.points.length}</p>
      <p className="stats">Profit: <span className={ profit > 0 ? "win" : "loss" }>{profit.toFixed(2)}</span> Euros</p>
      <p className="stats">Win rate: <span className={ bb_100 > 0 ? "win" : "loss" }>{bb_100.toFixed(2)}</span> bb/100</p>
    </div>
      );
}
