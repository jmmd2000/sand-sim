import { useRef, useState } from "react";
import { useSimulation, COUNT_IDS } from "./hooks/useSimulation";
import { usePainting } from "./hooks/usePainting";
import "./App.css";

type Material = { id: number; label: string; color: string; key: string; group: string | null; desc: string };

const MATERIALS: Material[] = [
  { id: 0,  label: "Erase",     color: "#111111", key: "", group: null,      desc: "Remove material" },
  { id: 2,  label: "Sand",      color: "#d2b96e", key: "", group: "Powders", desc: "Falls and piles; sinks through water" },
  { id: 16, label: "Gunpowder", color: "#3c3732", key: "", group: "Powders", desc: "Falls like sand; explodes on contact with fire, lava, or ember; chain-detonates adjacent gunpowder" },
  { id: 3,  label: "Water",     color: "#286ed2", key: "", group: "Liquids", desc: "Flows and spreads; boils to steam near heat; extinguishes fire on contact" },
  { id: 9,  label: "Lava",      color: "#cf460a", key: "", group: "Liquids", desc: "Viscous and extremely hot; ignites wood; solidifies to obsidian when it contacts water" },
  { id: 12, label: "Acid",      color: "#03a02d", key: "", group: "Liquids", desc: "Dissolves most materials over time; even obsidian and walls erode slowly" },
  { id: 14, label: "Oil",       color: "#141210", key: "", group: "Liquids", desc: "Floats on water; slowly catches fire from adjacent flames; spreads across the surface as it burns" },
  { id: 4,  label: "Stone",     color: "#6e6e73", key: "", group: "Solids",  desc: "Stable solid; extremely rarely melts to lava when near a heat source as intense as lava" },
  { id: 1,  label: "Wall",      color: "#64605a", key: "", group: "Solids",  desc: "Nearly indestructible; immune to fire and lava; dissolves very slowly in acid" },
  { id: 5,  label: "Wood",      color: "#784b1e", key: "", group: "Solids",  desc: "Burns when touched by fire, lava, or ember; produces smoke and ash" },
  { id: 11, label: "Obsidian",  color: "#19102a", key: "", group: "Solids",  desc: "Hard solid formed when lava contacts water; immune to fire and lava; dissolves slowly in acid" },
  { id: 15, label: "Ice",       color: "#a0d8f0", key: "", group: "Solids",  desc: "Melts to water near heat; slowly spreads to freeze adjacent water" },
  { id: 6,  label: "Fire",      color: "#dc3c0a", key: "", group: "Fire",    desc: "Spreads directly to wood; ignites nearby oil; extinguished by water; drips slowly downward" },
  { id: 13, label: "Ember",     color: "#ffa014", key: "", group: "Fire",    desc: "Rises through smoke and steam; short-lived; small chance to reignite as fire" },
];

const GROUPS = ["Powders", "Liquids", "Solids", "Fire"] as const;

// const KEY_MAP: Record<string, number> = {
//   "1": 0,
//   "2": 2,
//   "3": 3,
//   "4": 4,
//   "5": 1,
//   "6": 5,
//   "7": 6,
//   "8": 9,
//   "9": 11,
//   "0": 12,
// };

export default function App() {
  const [W, setW] = useState(480);
  const [H, setH] = useState(270);
  const [scale, setScale] = useState(2);
  const [currentMaterial, setCurrentMaterial] = useState(2);
  const [brushRadius, setBrushRadius] = useState(10);
  const [paused, setPaused] = useState(false);
  const [ticksPerStep, setTicksPerStep] = useState(4);
  const [showHeat, setShowHeat] = useState(false);

  const canvasRef = useRef<HTMLCanvasElement>(null);
  const sim = useSimulation(canvasRef, W, H, paused, ticksPerStep, showHeat);

  const { onPointerDown, onPointerMove, onPointerUp } = usePainting(canvasRef, W, H, (x, y) => sim.paint(x, y, currentMaterial, brushRadius));

  // useEffect(() => {
  //   const onKey = (e: KeyboardEvent) => {
  //     if (KEY_MAP[e.key] !== undefined) setCurrentMaterial(KEY_MAP[e.key]);
  //     if (e.key === " ") {
  //       e.preventDefault();
  //       setPaused((p) => !p);
  //     }
  //     if (e.key === ".") sim.step();
  //   };
  //   window.addEventListener("keydown", onKey);
  //   return () => window.removeEventListener("keydown", onKey);
  // }, [sim.step]);

  return (
    <div className="app">
      <aside className="sidebar">
        <h1 className="title">
          SandSim <span className="fps">{sim.fps} fps</span>
        </h1>
        <div className="mat-grid">
          {MATERIALS.filter((m) => m.group === null).map((m) => (
            <button key={m.id} className={"mat-btn" + (currentMaterial === m.id ? " active" : "")} onClick={() => setCurrentMaterial(m.id)} title={m.desc}>
              <span className="mat-swatch" style={{ background: m.color }} />
              <span>{m.label}</span>
              {m.key && <kbd>{m.key}</kbd>}
            </button>
          ))}
          {GROUPS.map((group) => (
            <div key={group}>
              <div className="mat-group-label">{group}</div>
              {MATERIALS.filter((m) => m.group === group).map((m) => (
                <button key={m.id} className={"mat-btn" + (currentMaterial === m.id ? " active" : "")} onClick={() => setCurrentMaterial(m.id)} title={m.desc}>
                  <span className="mat-swatch" style={{ background: m.color }} />
                  <span>{m.label}</span>
                  {m.key && <kbd>{m.key}</kbd>}
                </button>
              ))}
            </div>
          ))}
        </div>
      </aside>

      <div className="main-area">
        <main className="canvas-wrap">
          {!sim.ready && <div className="loading">Loading…</div>}
          <canvas ref={canvasRef} width={W * scale} height={H * scale} onPointerDown={onPointerDown} onPointerMove={onPointerMove} onPointerUp={onPointerUp} onContextMenu={(e) => e.preventDefault()} />
        </main>

        <div className="bottombar">
          <div className="ctrl-group">
            <label>Brush — {brushRadius}</label>
            <input type="range" min={1} max={50} value={brushRadius} onChange={(e) => setBrushRadius(+e.target.value)} />
          </div>
          <div className="ctrl-group">
            <label>Speed — {ticksPerStep}x</label>
            <input type="range" min={1} max={16} value={ticksPerStep} onChange={(e) => setTicksPerStep(+e.target.value)} />
          </div>
          <div className="ctrl-group">
            <label>Width — {W}px</label>
            <input type="range" min={80} max={640} step={40} value={W} onChange={(e) => setW(+e.target.value)} />
          </div>
          <div className="ctrl-group">
            <label>Height — {H}px</label>
            <input type="range" min={60} max={480} step={30} value={H} onChange={(e) => setH(+e.target.value)} />
          </div>
          <div className="ctrl-group">
            <label>Scale — {scale}x</label>
            <input type="range" min={1} max={4} step={1} value={scale} onChange={(e) => setScale(+e.target.value)} />
          </div>
          <div className="ctrl-group">
            <label>Controls</label>
            <div className="btn-row">
              <button className="ctrl-btn primary" onClick={() => setPaused((p) => !p)}>
                {paused ? "▶ Play" : "⏸ Pause"}
              </button>
              <button className="ctrl-btn" onClick={sim.step} disabled={!paused}>
                ⏭ Step
              </button>
              <button className="ctrl-btn danger" onClick={sim.clear}>
                Clear
              </button>
              <button className={`ctrl-btn${showHeat ? " active" : ""}`} onClick={() => setShowHeat((v) => !v)}>
                Heat
              </button>
            </div>
          </div>
          <div className="ctrl-group counts">
            <label>Counts</label>
            <div className="count-grid">
              {COUNT_IDS.map((id) => {
                const mat = MATERIALS.find((m) => m.id === id);
                const count = sim.counts[id] ?? 0;
                if (!mat || count === 0) return null;
                return (
                  <div key={id} className="count-row">
                    <span className="count-swatch" style={{ background: mat.color }} />
                    <span>{mat.label}</span>
                    <span className="count-val">{count.toLocaleString()}</span>
                  </div>
                );
              })}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
