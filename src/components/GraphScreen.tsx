import { Filters } from "../App";
import { CurvePoint } from "../types";
import { ChartControls } from "./ChartControls";
import { ProfitChart } from "./ProfitChart";
import { StatsPanel } from "./StatsPanel";


interface GraphScreenProps {
  points: CurvePoint[],
  unit: "eur" | "bb",
  filters: Filters,
  showdown: boolean,
  nonshowdown: boolean,
  onUnitChange: (value: "eur" | "bb") => void,
  onFilterChange: (value: Filters) => void,
  onShowdownChange:(value: boolean) => void,
  onNonshowdownChange:(value: boolean)=> void,
}

export default function GraphScreen({points, unit, filters, showdown, nonshowdown, onUnitChange, onFilterChange, onShowdownChange, onNonshowdownChange} : GraphScreenProps){
  return (
    <>
      <StatsPanel points={points} />
      <div className="chart-card">
        <ChartControls unit={unit} filters={filters} showdown={showdown} nonshowdown={nonshowdown} onUnitChange={onUnitChange} onFilterChange={onFilterChange} onShowdownChange={onShowdownChange} onNonshowdownChange={onNonshowdownChange} />
        <ProfitChart points={points} unit={unit} showdown={showdown} nonshowdown={nonshowdown} />
      </div>
    </>);
}
