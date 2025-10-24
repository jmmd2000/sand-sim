# Sand Sim

A WIP particle simulation. You can draw sand, water, and other materials and watch them interact with "realistic" physics.

## Tech stack

- Rust compiled to WASM
- TypeScript + Vite
- Canvas for rendering

## Running it

Build the Rust code:

```bash
cd sim-core
wasm-pack build --target web --out-dir ../web-ui/src/wasm
```

Then run the web interface:

```bash
cd ../web-ui
pnpm install
pnpm run dev
```

Go to localhost:5173 in your browser

## How to use

Click and drag to draw materials. Use number keys to switch materials:

- 1 = erase
- 2 = sand
- 3 = water
- 4 = stone
