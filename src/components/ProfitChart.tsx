import { ChartData, ChartDataset, ChartOptions } from "chart.js";
import { CurvePoint } from "../types";
import { Line } from "react-chartjs-2";
import "chart.js/auto";

export function ProfitChart(props: {points: CurvePoint[], unit: "eur"|"bb", showdown: boolean, nonshowdown: boolean}) {
  const pick = ((eur: number, bb: number) => (props.unit === "eur" ? eur / 100 : bb));
  const datasets: ChartDataset<"line">[] = [
    {
      label: "PnL",
      data: props.points.map((p) => pick(p.cumulative, p.cumulative_bb)),
      fill: true,
      backgroundColor: "rgba(0, 128, 0, 0.1)",
      pointRadius: 0,
      borderColor: "green"
    }
  ];
  if (props.showdown) {
    datasets.push({
      label: "Showdown",
      data: props.points.map((p) => pick(p.cumulative_showdown, p.cumulative_showdown_bb)),
      pointRadius: 0,
      borderColor: "blue",
    });
  }
  if (props.nonshowdown) {
    datasets.push({
      label: "Non Showdown",
      data: props.points.map((p) => pick(p.cumulative_nonshowdown, p.cumulative_nonshowdown_bb)),
      pointRadius: 0,
      borderColor: "red",
    });
  }

  const data: ChartData<"line"> = {
    labels: props.points.map((_, index) => index + 1),
    datasets: datasets,
  };
  const option: ChartOptions<"line"> = {
    animation: false,
    maintainAspectRatio: false,
  };
  return (
    <div className="chart-wrapper"><Line data={ data } options={ option } /></div>
  );
}
