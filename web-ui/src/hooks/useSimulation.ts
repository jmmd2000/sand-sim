import { useCallback, useEffect, useRef, useState, type RefObject } from "react";
import init, { Simulation } from "../wasm/sim_core.js";

const TPS = 60;
const dt = 1000 / TPS; // ms per simulation tick

export const COUNT_IDS = [2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

export function useSimulation(canvasRef: RefObject<HTMLCanvasElement | null>, W: number, H: number, paused: boolean, ticksPerStep: number, showHeat: boolean) {
  // Mutable values read/written inside the requestAnimationFrame loop — never trigger re-renders
  const simRef = useRef<Simulation | null>(null);
  const bufferRef = useRef<ArrayBuffer | null>(null); // WASM linear memory buffer
  const animRef = useRef<number>(0); // requestAnimationFrame handle for cleanup
  const accRef = useRef(0); // accumulated ms since last tick
  const lastTickRef = useRef(0); // timestamp of previous frame

  // Synced from props each render so the loop closure always sees current values
  const pausedRef = useRef(paused);
  const ticksRef = useRef(ticksPerStep);
  const showHeatRef = useRef(showHeat);
  pausedRef.current = paused;
  ticksRef.current = ticksPerStep;
  showHeatRef.current = showHeat;

  const [ready, setReady] = useState(false);
  const [counts, setCounts] = useState<Record<number, number>>({});
  const [fps, setFps] = useState(0);

  // Re-runs when W or H changes, reinitialising the simulation and render loop
  useEffect(() => {
    let cancelled = false;

    async function setup() {
      const canvas = canvasRef.current!;
      const ctx = canvas.getContext("2d")!;
      const exports = await init(); // load + instantiate the .wasm module
      if (cancelled) return;

      bufferRef.current = exports.memory.buffer;

      const sim = new Simulation(W, H);
      simRef.current = sim;

      // Off-screen targets at sim resolution; scaled up to canvas size via drawImage
      const simCanvas = new OffscreenCanvas(W, H);
      const simCtx = simCanvas.getContext("2d")!;
      const glow = new OffscreenCanvas(W, H);
      const glowCtx = glow.getContext("2d")!;
      const heatCanvas = new OffscreenCanvas(W, H);
      const heatCtx = heatCanvas.getContext("2d")!;
      const heatImageData = heatCtx.createImageData(W, H);

      // Precompute RGBA colour for every possible heat value (0–255) once at startup.
      // Ramp: dark blue -> cyan -> yellow -> red.
      const heatLut = new Uint8Array(256 * 4); // 256 entries × 4 bytes (RGBA)
      for (let i = 0; i < 256; i++) {
        const t = Math.sqrt(i / 255); // 0.0–1.0, gamma-corrected
        let r, g, b;
        // Each branch covers one quarter of t (0–1), blending between two colours.
        // s rescales that quarter to 0–1 so the lerps are always simple.
        if (t < 0.25) {
          const s = t / 0.25;
          r = 0;
          g = Math.round(s * 100);
          b = Math.round(80 + s * 175);
        } // dark blue -> blue
        else if (t < 0.5) {
          const s = (t - 0.25) / 0.25;
          r = 0;
          g = Math.round(100 + s * 155);
          b = Math.round(255 - s * 255);
        } // blue -> cyan/green
        else if (t < 0.75) {
          const s = (t - 0.5) / 0.25;
          r = Math.round(s * 255);
          g = 255;
          b = 0;
        } // green -> yellow
        else {
          const s = (t - 0.75) / 0.25;
          r = 255;
          g = Math.round(255 - s * 175);
          b = 0;
        } // yellow -> red-orange
        heatLut[i * 4] = r;
        heatLut[i * 4 + 1] = g;
        heatLut[i * 4 + 2] = b;
        heatLut[i * 4 + 3] = 200;
      }

      setReady(true);
      let frame = 0;
      let fpsTs = 0;

      function loop(now: number) {
        const elapsed = Math.min(now - (lastTickRef.current || now), 200);
        lastTickRef.current = now;

        // Fixed-timestep accumulator: step the sim at TPS regardless of frame rate
        if (!pausedRef.current) {
          accRef.current += elapsed;
          while (accRef.current >= dt) {
            sim.step(ticksRef.current);
            accRef.current -= dt;
          }
        }

        // WASM heap can grow and replace the buffer, keep ref current
        if (exports.memory.buffer !== bufferRef.current) bufferRef.current = exports.memory.buffer;
        const buf = bufferRef.current;
        const dw = canvas.width; // read each frame so scale changes take effect immediately
        const dh = canvas.height;

        // Base pixels: write WASM memory view -> offscreen -> scale up to display canvas
        simCtx.putImageData(new ImageData(new Uint8ClampedArray(buf, sim.pixels_ptr(), sim.pixels_len()), W, H), 0, 0);
        ctx.imageSmoothingEnabled = false;
        ctx.drawImage(simCanvas, 0, 0, dw, dh);

        // Glow layer: same pixels but blurred and blended additively for glow effect
        glowCtx.putImageData(new ImageData(new Uint8ClampedArray(buf, sim.glow_pixels_ptr(), sim.glow_pixels_len()), W, H), 0, 0);
        ctx.filter = "blur(8px)";
        ctx.globalCompositeOperation = "lighter";
        ctx.drawImage(glow, 0, 0, dw, dh);
        ctx.filter = "none";
        ctx.globalCompositeOperation = "source-over";

        // Heat overlay: full thermal view, dark blue (cold) -> cyan -> yellow -> red (hot)
        if (showHeatRef.current) {
          const heatBuf = new Uint8Array(buf, sim.heat_ptr(), sim.heat_len());
          const px = heatImageData.data;
          for (let i = 0; i < heatBuf.length; i++) {
            const lut = heatBuf[i] * 4;
            const p = i * 4;
            px[p] = heatLut[lut];
            px[p + 1] = heatLut[lut + 1];
            px[p + 2] = heatLut[lut + 2];
            px[p + 3] = heatLut[lut + 3];
          }
          heatCtx.putImageData(heatImageData, 0, 0);
          ctx.drawImage(heatCanvas, 0, 0, dw, dh);
        }

        // Update counts and FPS every 30 frames
        if (++frame % 30 === 0) {
          const c: Record<number, number> = {};
          for (const id of COUNT_IDS) c[id] = sim.count_mat(id);
          setCounts(c);
          if (fpsTs) setFps(Math.round(30000 / (now - fpsTs)));
          fpsTs = now;
        }

        animRef.current = requestAnimationFrame(loop);
      }

      animRef.current = requestAnimationFrame(loop);
    }

    setup();
    return () => {
      cancelled = true;
      cancelAnimationFrame(animRef.current);
    };
  }, [W, H]);

  const paint = useCallback((x: number, y: number, material: number, brush: number) => {
    simRef.current?.paint_circle(x, y, brush, material);
  }, []);

  const step = useCallback(() => simRef.current?.step(ticksRef.current), []);
  const clear = useCallback(() => simRef.current?.clear(), []);

  return { ready, counts, fps, paint, step, clear };
}
