import init, { Simulation } from "./wasm/sim.js";

const canvas = document.getElementById("c") as HTMLCanvasElement;
const ctx = canvas.getContext("2d")!;
const W = canvas.width;
const H = canvas.height;

await init();

const sim = new Simulation(W, H);

// counters
const frameCounter = document.getElementById("frame") as HTMLSpanElement;
const fpsCounter = document.getElementById("fps") as HTMLSpanElement;
const minFrameCounter = document.getElementById("min-frame") as HTMLSpanElement;
const maxFrameCounter = document.getElementById("max-frame") as HTMLSpanElement;
const emptyCountSpan = document.getElementById("empty-count") as HTMLSpanElement;
const sandCountSpan = document.getElementById("sand-count") as HTMLSpanElement;
const waterCountSpan = document.getElementById("water-count") as HTMLSpanElement;

// Pixels
let px = sim.pixels(); // Uint8Array view into WASM
let clamped = new Uint8ClampedArray(px.buffer as ArrayBuffer, px.byteOffset, px.byteLength);

// Physics timing
const TPS = 60;
const dt = 1000 / TPS; // ms per fixed step
let acc = 0;
let lastTick = performance.now();

// Render timing, FPS measured over a rolling window
let lastRaf = performance.now();
let frameNum = 0;

// FPS window - average over ~0.5s
let fpsWindowTime = 0;
let fpsWindowFrames = 0;
let fpsMin = Infinity;
let fpsMax = 0;

// Brush
let drawing = false;
let currentMaterial = 2; // 1 Wall, 2 Sand, 3 Water, 4 Stone
let brushRadius = 10;

// Input
canvas.addEventListener("pointerdown", (e) => {
  drawing = true;
  paint(e);
});

canvas.addEventListener("pointermove", (e) => {
  if (drawing) paint(e);
});

window.addEventListener("pointerup", () => (drawing = false));

function paint(e: PointerEvent) {
  const rect = canvas.getBoundingClientRect();
  const x = Math.floor(((e.clientX - rect.left) / rect.width) * W);
  const y = Math.floor(((e.clientY - rect.top) / rect.height) * H);
  sim.paint_circle(x, y, brushRadius, currentMaterial);

  // If wasm memory grows, refresh the view
  const fresh = sim.pixels();
  if (fresh.buffer !== px.buffer || fresh.byteOffset !== px.byteOffset || fresh.byteLength !== px.byteLength) {
    px = fresh;
    clamped = new Uint8ClampedArray(px.buffer as ArrayBuffer, px.byteOffset, px.byteLength);
  }
}

function draw() {
  const img = new ImageData(new Uint8ClampedArray(px.buffer as ArrayBuffer, px.byteOffset, px.byteLength), W, H);
  ctx.putImageData(img, 0, 0);
}

function loop(now: number) {
  // Fixed timestep physics
  const tickElapsed = now - lastTick;
  lastTick = now;
  acc += tickElapsed;

  // Adjust this to taste - how many CA ticks per fixed step
  const TICKS_PER_STEP = 3;

  while (acc >= dt) {
    sim.step(TICKS_PER_STEP);

    // Refresh pixel view if memory changed
    const fresh = sim.pixels();
    if (fresh.buffer !== px.buffer || fresh.byteOffset !== px.byteOffset || fresh.byteLength !== px.byteLength) {
      px = fresh;
      clamped = new Uint8ClampedArray(px.buffer as ArrayBuffer, px.byteOffset, px.byteLength);
    }

    acc -= dt;
  }

  draw();

  // FPS and counters
  const frameElapsed = now - lastRaf;
  lastRaf = now;
  fpsWindowTime += frameElapsed;
  fpsWindowFrames += 1;

  if (fpsWindowTime >= 500) {
    const fps = (fpsWindowFrames * 1000) / fpsWindowTime;
    fpsMin = Math.min(fpsMin, fps);
    fpsMax = Math.max(fpsMax, fps);

    fpsCounter.textContent = `FPS: ${fps.toFixed(1)}`;
    minFrameCounter.textContent = `Min FPS: ${fpsMin.toFixed(1)}`;
    maxFrameCounter.textContent = `Max FPS: ${fpsMax.toFixed(1)}`;

    fpsWindowTime = 0;
    fpsWindowFrames = 0;
  }

  frameNum += 1;
  frameCounter.textContent = `Frame: ${frameNum}`;

  // Material counts
  if (emptyCountSpan) emptyCountSpan.textContent = sim.count_mat(0).toString();
  if (sandCountSpan) sandCountSpan.textContent = sim.count_mat(2).toString();
  if (waterCountSpan) waterCountSpan.textContent = sim.count_mat(3).toString();

  requestAnimationFrame(loop);
}

requestAnimationFrame(loop);

window.addEventListener("keydown", (e) => {
  if (e.key === "1") currentMaterial = 0; // Empty
  if (e.key === "2") currentMaterial = 2; // Sand
  if (e.key === "3") currentMaterial = 3; // Water
  if (e.key === "4") currentMaterial = 4; // Stone
  if (e.key === "5") currentMaterial = 1; // Wall
});
