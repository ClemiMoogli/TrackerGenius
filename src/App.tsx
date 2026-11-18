import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { CurvePoint } from "./types";
import { Header } from "./components/Header";
import GraphScreen from "./components/GraphScreen";

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
  const [screen, setScreen] = useState<"graph" | "stats">("graph");
  const [filters, setFilters] = useState<Filters>({start_date:null, end_date:null, position:null, street: null, action:null});
  useEffect(() => {
    invoke<CurvePoint[]>("get_profit_curve", { player: "MrAlambic", filters:filters})
      .then((data) => setPoints(data))
      .catch((err) => console.error(err));
  }, [filters]);
  return (
    <>
      <Header onScreenChange={setScreen} />
      {screen === "graph"
        ? <GraphScreen points={points} unit={unit} filters={filters} showdown={showdown} nonshowdown={nonshowdown} onUnitChange={setUnit} onFilterChange={setFilters} onShowdownChange={setShowdown} onNonshowdownChange={setNonshowdown} />
        : <p>Page stats en cours de creation</p>
      }
    </>
  );
}

export default App;
