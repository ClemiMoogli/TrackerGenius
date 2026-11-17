import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { CurvePoint } from "./types";
import { StatsPanel } from "./components/StatsPanel";
import { ProfitChart } from "./components/ProfitChart";
import { ChartControls } from "./components/ChartControls";
import { Header } from "./components/Header";

// faire un deuxieme ecran pour montrer les statistiques.


export type Position = "SB" | "BB" | "UTG" | "MP" | "CO" | "BTN";
export type Street = "Preflop" | "Flop" | "Turn" | "River";
export type ActionType = "PostSmallBlind" | "PostBigBlind" | "Fold" | "Check" | "Call" | "Bet" | "RaiseTo" | "Collect";

export interface Filters {
  start_date: string | null,
  end_date: string | null,
  position: Position | null,
  street: Street | null,
  action: ActionType| null,
}

function App() {
  const [points, setPoints] = useState<CurvePoint[]>([]);
  const [unit, setUnit] = useState<"eur" | "bb">("eur");
  const [showdown, setShowdown] = useState<true | false>(false);
  const [nonshowdown, setNonshowdown] = useState<true | false>(false);
  const [filters, setFilters] = useState<Filters>({start_date:null, end_date:null, position:null, street: null, action:null});
  useEffect(() => {
    invoke<CurvePoint[]>("get_profit_curve", { player: "MrAlambic", filters:filters})
      .then((data) => setPoints(data))
      .catch((err) => console.error(err));
  }, [filters]);
  return (
    <>
      <Header />
      <StatsPanel points={points} />
      <div className="chart-card">
        <ChartControls unit={unit} filters={filters} showdown={showdown} nonshowdown={nonshowdown} onUnitChange={setUnit} onFilterChange={setFilters} onShowdownChange={setShowdown} onNonshowdownChange={setNonshowdown} />
        <ProfitChart points={points} unit={unit} showdown={showdown} nonshowdown={nonshowdown} />
      </div>
      </>
  );
}

export default App;
