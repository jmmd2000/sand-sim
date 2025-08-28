import init, { Simulation } from "./wasm/sim.js";

const canvas = document.getElementById("c") as HTMLCanvasElement;
const ctx = canvas.getContext("2d")!;
const W = canvas.width;
const H = canvas.height;

// Initialize WASM
await init();
const sim = new Simulation(W, H);
const frameCounter = document.getElementById("frame") as HTMLSpanElement;
const fpsCounter = document.getElementById("fps") as HTMLSpanElement;
const minFrameCounter = document.getElementById("min-frame") as HTMLSpanElement;
const maxFrameCounter = document.getElementById("max-frame") as HTMLSpanElement;
const emptyCountSpan = document.getElementById("empty-count") as HTMLSpanElement;
const sandCountSpan = document.getElementById("sand-count") as HTMLSpanElement;
const waterCountSpan = document.getElementById("water-count") as HTMLSpanElement;

// Mouse handling
let currentMaterial = 1; // Sand

function paintSquare(x: number, y: number, radius: number, material: number) {
  for (let dx = -radius; dx <= radius; dx++) {
    for (let dy = -radius; dy <= radius; dy++) {
      sim.set_cell(x + dx, y + dy, material);
    }
  }
}

function paintCircle(x: number, y: number, radius: number, material: number, aeration: number = 0.9) {
  for (let dx = -radius; dx <= radius; dx++) {
    for (let dy = -radius; dy <= radius; dy++) {
      if (dx * dx + dy * dy <= radius * radius && Math.random() > aeration) {
        sim.set_cell(x + dx, y + dy, material);
      }
    }
  }
}

let isPainting = false;
let lastPaintPos: { x: number; y: number } | null = null;

function getCanvasCoords(e: MouseEvent) {
  const rect = canvas.getBoundingClientRect();
  const x = Math.floor(((e.clientX - rect.left) / rect.width) * W);
  const y = Math.floor(((e.clientY - rect.top) / rect.height) * H);
  return { x, y };
}

canvas.addEventListener("mousedown", (e) => {
  isPainting = true;
  const { x, y } = getCanvasCoords(e);
  paintCircle(x, y, 10, currentMaterial);
  lastPaintPos = { x, y };
});

canvas.addEventListener("mouseup", () => {
  isPainting = false;
  lastPaintPos = null;
});

canvas.addEventListener("mouseleave", () => {
  isPainting = false;
  lastPaintPos = null;
});

canvas.addEventListener("mousemove", (e) => {
  if (isPainting) {
    const { x, y } = getCanvasCoords(e);
    paintCircle(x, y, 10, currentMaterial);
    lastPaintPos = { x, y };
  }
});

// Paint at last position if mouse is held down and not moving
function brushWhileHeld() {
  if (isPainting && lastPaintPos) {
    paintCircle(lastPaintPos.x, lastPaintPos.y, 10, currentMaterial);
  }
  requestAnimationFrame(brushWhileHeld);
}
brushWhileHeld();

window.addEventListener("keydown", (e) => {
  if (e.key === "1") currentMaterial = 0; // Empty
  if (e.key === "2") currentMaterial = 1; // Sand
  if (e.key === "3") currentMaterial = 2; // Water
});

// Animation loop
let lastFrameTime = performance.now();

let minFps = Infinity;
let maxFps = 0;

function frame() {
  sim.step();

  const pixels = sim.pixels_view();
  const imageData = new ImageData(new Uint8ClampedArray(pixels), W, H);
  ctx.putImageData(imageData, 0, 0);

  frameCounter.textContent = sim.frame().toString();
  const now = performance.now();
  const fps = 1000 / (now - lastFrameTime);

  minFps = Math.min(minFps, fps);
  maxFps = Math.max(maxFps, fps);

  fpsCounter.textContent = `FPS: ${Math.round(fps)}`;
  minFrameCounter.textContent = `Min FPS: ${Math.round(minFps)}`;
  maxFrameCounter.textContent = `Max FPS: ${Math.round(maxFps)}`;
  lastFrameTime = now;
  emptyCountSpan.textContent = sim.count_mat(0).toString();
  sandCountSpan.textContent = sim.count_mat(1).toString();
  waterCountSpan.textContent = sim.count_mat(2).toString();

  requestAnimationFrame(frame);
}

requestAnimationFrame(frame);
