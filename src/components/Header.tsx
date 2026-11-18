import "../App.css"

interface HeaderProps {
  onScreenChange: (value:"graph"|"stats") => void,
}

export function Header({ onScreenChange }: HeaderProps){
  return (
    <div className="header">
      <h1>Tracker Genius</h1>
      <nav>
        <button onClick={() => onScreenChange("graph")} value="graph">Graph</button>
        <button onClick={() => onScreenChange("stats")} value="stats">Stats</button>
      </nav>
    </div>
      );
}
