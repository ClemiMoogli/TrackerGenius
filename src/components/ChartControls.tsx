interface ChartControlsProps {
  unit: "eur" | "bb",
  showdown: boolean,
  nonshowdown: boolean,
  onUnitChange: (value: "eur"|"bb")=> any,
  onShowdownChange:(value: boolean) => any,
  onNonshowdownChange:(value: boolean)=> any,
}

export function ChartControls({ unit, showdown, nonshowdown, onUnitChange, onShowdownChange, onNonshowdownChange} : ChartControlsProps) {
  return (
    <div className="chart-controls">
      <h2>Parametres du graphique:</h2>
      <div className="segmented">
        <input type="radio" id="unit-eur" name="unit" checked={unit == "eur"} onChange={ onUnitChange("eur")} />
        <label htmlFor="unit-eur">Euros</label>
        <input type="radio" id="unit-bb" name="unit" checked={unit == "bb"} onChange={ onUnitChange("bb")} />
        <label htmlFor="unit-bb">Bb</label>
      </div>


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
