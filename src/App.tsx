import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { CurvePoint } from "./types";
import { StatsPanel } from "./components/StatsPanel";
import { ProfitChart } from "./components/ProfitChart";
import { ChartControls } from "./components/ChartControls";
import { Header } from "./components/Header";

function App() {
  const [points, setPoints] = useState<CurvePoint[]>([]);
  const [unit, setUnit] = useState<"eur" | "bb">("eur");
  const [showdown, setShowdown] = useState<true | false>(false);
  const [nonshowdown, setNonshowdown] = useState<true | false>(false);
  useEffect(() => {
    invoke<CurvePoint[]>("get_profit_curve", { player: "MrAlambic" })
      .then((data) => setPoints(data))
      .catch((err) => console.error(err));
  }, []);
  return (
    <>
      <Header />
      <StatsPanel points={points} />
      <div className="chart-card">
        <ChartControls unit={unit} showdown={ showdown} nonshowdown={ nonshowdown} onUnitChange={ setUnit} onShowdownChange={setShowdown} onNonshowdownChange={setNonshowdown} />
        <ProfitChart points={points} unit={unit} showdown={showdown} nonshowdown={nonshowdown} />
      </div>
      </>
  );
}

export default App;
