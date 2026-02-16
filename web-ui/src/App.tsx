import { useEffect, useRef, useState } from "react";
import { useSimulation, COUNT_IDS } from "./hooks/useSimulation";
import { usePainting } from "./hooks/usePainting";
import "./App.css";

const MATERIALS = [
  { id: 0, label: "Erase", color: "#111111", key: "1" },
  { id: 2, label: "Sand", color: "#d2b96e", key: "2" },
  { id: 3, label: "Water", color: "#286ed2", key: "3" },
  { id: 4, label: "Stone", color: "#6e6e73", key: "4" },
  { id: 1, label: "Wall", color: "#64605a", key: "5" },
  { id: 5, label: "Wood", color: "#784b1e", key: "6" },
  { id: 6, label: "Fire", color: "#dc3c0a", key: "7" },
  { id: 9, label: "Lava", color: "#cf460a", key: "8" },
  { id: 11, label: "Obsidian", color: "#19102a", key: "9" },
];

const KEY_MAP: Record<string, number> = {
  "1": 0,
  "2": 2,
  "3": 3,
  "4": 4,
  "5": 1,
  "6": 5,
  "7": 6,
  "8": 9,
  "9": 11,
};

export default function App() {
  const [W, setW] = useState(480);
  const [H, setH] = useState(270);
  const [scale, setScale] = useState(2);
  const [currentMaterial, setCurrentMaterial] = useState(2);
  const [brushRadius, setBrushRadius] = useState(10);
  const [paused, setPaused] = useState(false);
  const [ticksPerStep, setTicksPerStep] = useState(4);

  const canvasRef = useRef<HTMLCanvasElement>(null);
  const sim = useSimulation(canvasRef, W, H, paused, ticksPerStep);

  const { onPointerDown, onPointerMove, onPointerUp } = usePainting(canvasRef, W, H, (x, y) => sim.paint(x, y, currentMaterial, brushRadius));

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (KEY_MAP[e.key] !== undefined) setCurrentMaterial(KEY_MAP[e.key]);
      if (e.key === " ") {
        e.preventDefault();
        setPaused((p) => !p);
      }
      if (e.key === ".") sim.step();
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [sim.step]);

  return (
    <div className="app">
      <aside className="sidebar">
        <h1 className="title">
          SandSim <span className="fps">{sim.fps} fps</span>
        </h1>

        <section>
          <label>Material</label>
          <div className="mat-grid">
            {MATERIALS.map((m) => (
              <button key={m.id} className={"mat-btn" + (currentMaterial === m.id ? " active" : "")} onClick={() => setCurrentMaterial(m.id)} title={`${m.label} [${m.key}]`}>
                <span className="mat-swatch" style={{ background: m.color }} />
                <span>{m.label}</span>
                <kbd>{m.key}</kbd>
              </button>
            ))}
          </div>
        </section>

        <section>
          <label>Brush — {brushRadius}</label>
          <input type="range" min={1} max={50} value={brushRadius} onChange={(e) => setBrushRadius(+e.target.value)} />
        </section>

        <section>
          <label>Speed — {ticksPerStep}x</label>
          <input type="range" min={1} max={16} value={ticksPerStep} onChange={(e) => setTicksPerStep(+e.target.value)} />
        </section>

        <section>
          <label>Width — {W}px</label>
          <input type="range" min={80} max={640} step={40} value={W} onChange={(e) => setW(+e.target.value)} />
          <label>Height — {H}px</label>
          <input type="range" min={60} max={480} step={30} value={H} onChange={(e) => setH(+e.target.value)} />
          <label>Scale — {scale}x</label>
          <input type="range" min={1} max={4} step={1} value={scale} onChange={(e) => setScale(+e.target.value)} />
        </section>

        <section>
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
          </div>
        </section>

        <section>
          <label>Counts</label>
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
        </section>
      </aside>

      <main className="canvas-wrap">
        {!sim.ready && <div className="loading">Loading…</div>}
        <canvas ref={canvasRef} width={W * scale} height={H * scale} onPointerDown={onPointerDown} onPointerMove={onPointerMove} onPointerUp={onPointerUp} onContextMenu={(e) => e.preventDefault()} />
      </main>
    </div>
  );
}
