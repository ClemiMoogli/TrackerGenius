import { Filters, Position } from "../App";


interface ChartControlsProps {
  unit: "eur" | "bb",
  filters: Filters,
  showdown: boolean,
  nonshowdown: boolean,
  onUnitChange: (value: "eur" | "bb") => void,
  onFilterChange: (value: Filters) => void,
  onShowdownChange:(value: boolean) => void,
  onNonshowdownChange:(value: boolean)=> void,
}

export function ChartControls({ unit, filters, showdown, nonshowdown, onUnitChange, onFilterChange, onShowdownChange, onNonshowdownChange} : ChartControlsProps) {
  return (
    <div className="chart-controls">
      <h2>Parametres du graphique:</h2>
      <div className="segmented">
        <input type="radio" id="unit-eur" name="unit" checked={unit == "eur"} onChange={() => onUnitChange("eur")} />
        <label htmlFor="unit-eur">Euros</label>
        <input type="radio" id="unit-bb" name="unit" checked={unit == "bb"} onChange={() => onUnitChange("bb")} />
        <label htmlFor="unit-bb">Bb</label>
      </div>

      <select value={filters.position ?? ""} onChange={(e) => onFilterChange({ ...filters, position:e.target.value === "" ? null : (e.target.value as Position),})}>
        <option value="">Toutes les positions</option>
        <option value="UTG">UTG</option>
        <option value="MP">MP</option>
        <option value="CO">CO</option>
        <option value="BTN">BTN</option>
        <option value="SB">SB</option>
        <option value="BB">BB</option>
      </select>

      <label>Date de debut:
        <input type="date" value={filters.start_date ?? ""} onChange={(e) => onFilterChange({ ...filters, start_date:e.target.value === "" ? null : e.target.value})} />
      </label>
      <label>Date de fin:
        <input type="date" value={filters.end_date ?? ""} onChange={(e) => onFilterChange({ ...filters, end_date:e.target.value === "" ? null : e.target.value})} />
      </label>
      <p>player action</p>

      <div className="curve-controls">
        <p>Afficher les courbes:</p>
        <label className="toggle">
          <input type="checkbox" checked={showdown} className="switch" onChange={(e) => onShowdownChange(e.target.checked)} />
          Showdown
        </label>
        <label className="toggle">
          <input type="checkbox" checked={nonshowdown} className="switch" onChange={(e) => onNonshowdownChange(e.target.checked)} />
          Non-Showdown
        </label>
      </div>
    </div>
  );
}
