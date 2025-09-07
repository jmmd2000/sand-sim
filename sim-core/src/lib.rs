use wasm_bindgen::prelude::*;

mod materials;
use materials::{Material, color_of, update_cell};

#[wasm_bindgen]
pub struct Simulation {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    pixels: Vec<u8>, // RGBA for display
    generation: u8,
    rng: u64,
    frame: u32, // Frame counter
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Cell {
    pub material: Material,
    pub ra: u8,
    pub rb: u8,
    pub clock: u8,
}

impl Cell {
    #[inline]
    pub fn empty_with_clock(clock: u8) -> Self {
        Self {
            material: Material::Empty,
            ra: 0,
            rb: 0,
            clock,
        }
    }
}

#[inline]
fn idx(width: u32, x: i32, y: i32) -> usize {
    (y as u32 * width + x as u32) as usize
}

impl Simulation {
    #[inline]
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as u32) < self.width && (y as u32) < self.height
    }

    #[inline]
    fn rng_next(&mut self) -> u32 {
        let mut x = self.rng;
        x ^= x << 12;
        x ^= x >> 25;
        x ^= x << 27;
        self.rng = x;
        (x.wrapping_mul(2685821657736338717) >> 32) as u32
    }

    #[inline]
    fn write_pixels(&mut self) {
        let w = self.width as usize;
        let h = self.height as usize;

        for y in 0..h {
            let row = y * w;
            for x in 0..w {
                let i = row + x;
                let p = i * 4;
                let color = color_of(self.cells[i].material);
                self.pixels[p] = color[0];
                self.pixels[p + 1] = color[1];
                self.pixels[p + 2] = color[2];
                self.pixels[p + 3] = color[3];
            }
        }
    }

    #[inline]
    fn update_at(&mut self, x: i32, y: i32) {
        let i = idx(self.width, x, y);
        let cell = self.cells[i];

        // skip cells already update this tick
        if cell.clock.wrapping_sub(self.generation) == 0 {
            return;
        }

        // skip empty cells
        if cell.material == Material::Empty {
            return;
        }

        let api = SimAPI { x, y, sim: self };
        update_cell(cell, api);
    }
}

pub struct SimAPI<'a> {
    pub x: i32,
    pub y: i32,
    pub sim: &'a mut Simulation,
}

impl<'a> SimAPI<'a> {
    #[inline]
    pub fn get(&self, dx: i32, dy: i32) -> Cell {
        let nx = self.x + dx;
        let ny = self.y + dy;

        if !self.sim.in_bounds(nx, ny) {
            return Cell {
                material: Material::Wall,
                ra: 0,
                rb: 0,
                clock: self.sim.generation,
            };
        }

        self.sim.cells[idx(self.sim.width, nx, ny)]
    }

    #[inline]
    pub fn set(&mut self, dx: i32, dy: i32, mut v: Cell) {
        let nx = self.x + dx;
        let ny = self.y + dy;

        if !self.sim.in_bounds(nx, ny) {
            return;
        }

        let di = idx(self.sim.width, nx, ny);
        v.clock = self.sim.generation.wrapping_add(1);
        self.sim.cells[di] = v;
    }

    #[inline]
    pub fn clear_here(&mut self) {
        let i = idx(self.sim.width, self.x, self.y);
        // mark cell as empty and updated
        self.sim.cells[i] = Cell::empty_with_clock(self.sim.generation.wrapping_add(1));
    }

    /// Move cell into target if Empty
    /// Clears current cell if successful
    #[inline]
    pub fn try_move(&mut self, dx: i32, dy: i32, cell: Cell) -> bool {
        if self.get(dx, dy).material == Material::Empty {
            self.set(dx, dy, cell);
            self.clear_here();
            true
        } else {
            false
        }
    }

    /// Move cell into target if it's one of the allowed materials
    /// Clears current cell if successful
    #[inline]
    pub fn try_move_into(&mut self, dx: i32, dy: i32, cell: Cell, allowed_materials: &[Material]) -> bool {
        let target = self.get(dx, dy);
        
        // Check if target material is in the allowed list
        if allowed_materials.contains(&target.material) {
            // Store the target cell to put in current position
            let mut target_cell = target;
            target_cell.clock = self.sim.generation.wrapping_add(1);
            
            // Move our cell to target position
            self.set(dx, dy, cell);
            
            // Put target cell in current position
            let i = idx(self.sim.width, self.x, self.y);
            self.sim.cells[i] = target_cell;
            
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn rand_u32(&mut self) -> u32 {
        self.sim.rng_next()
    }

    #[inline]
    pub fn generation(&self) -> u8 {
        self.sim.generation
    }
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Simulation {
        let len = (width * height) as usize;
        let mut sim = Simulation {
            width,
            height,
            cells: vec![
                Cell {
                    material: Material::Empty,
                    ra: 0,
                    rb: 0,
                    clock: 0
                };
                len
            ],
            pixels: vec![0; len * 4],
            generation: 0,
            rng: 0xA5A5_1234_89AB_CDEF,
            frame: 0,
        };
        sim.write_pixels();
        sim
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Pointer to the RGBA pixel buffer main.ts
    #[inline]
    pub fn pixels_ptr(&self) -> *const u8 {
        self.pixels.as_ptr()
    }

    /// Step the simulation 'ticks' amount of steps
    pub fn step(&mut self, ticks: u32) {
        for _ in 0..ticks {
            self.generation = self.generation.wrapping_add(1);

            let w = self.width as i32;
            let h = self.height as i32;
            let lr = (self.generation & 1) == 0;

            for y in (0..h).rev() {
                if lr {
                    for x in 0..w {
                        self.update_at(x, y);
                    }
                } else {
                    for x in (0..w).rev() {
                        self.update_at(x, y);
                    }
                }
            }
        }
        self.frame += 1;
        self.write_pixels();
    }

    /// Count how many cells of a given material are present
    pub fn count_mat(&self, material_id: u8) -> usize {
        self.cells
            .iter()
            .filter(|&&m| m.material.id() == material_id)
            .count()
    }

    /// Set a single cell to a material ID
    pub fn set_cell(&mut self, x: u32, y: u32, material_id: u8) {
        if x >= self.width || y >= self.height {
            return;
        }

        let i = idx(self.width, x as i32, y as i32);
        let material = Material::from_id(material_id);

        // mark updated
        self.cells[i] = Cell {
            material,
            ra: 0,
            rb: 0,
            clock: self.generation.wrapping_add(1),
        };

        let p = i * 4;
        let c = color_of(material);
        self.pixels[p] = c[0];
        self.pixels[p + 1] = c[1];
        self.pixels[p + 2] = c[2];
        self.pixels[p + 3] = c[3];
    }

    /// Paint a filled circle
    pub fn paint_circle(&mut self, cx: u32, cy: u32, radius: u32, material_id: u8) {
        let r = radius as i32;
        let m = Material::from_id(material_id);
        let cx = cx as i32;
        let cy = cy as i32;
        let r2 = r * r;

        for dy in -r..=r {
            for dx in -r..=r {
                if dx * dx + dy * dy <= r2 {
                    let x = cx + dx;
                    let y = cy + dy;
                    if self.in_bounds(x, y) {
                        let i = idx(self.width, x, y);
                        self.cells[i] = Cell {
                            material: m,
                            ra: 0,
                            rb: 0,
                            clock: self.generation.wrapping_add(1),
                        };
                    }
                }
            }
        }
        self.write_pixels();
    }

    /// Clear the simulation
    pub fn clear(&mut self) {
        for c in &mut self.cells {
            *c = Cell::empty_with_clock(self.generation.wrapping_add(1));
        }
        self.write_pixels();
    }

    /// View into the RGBA pixels
    /// need to refresh the view if WASM memory grows.
    #[inline]
    pub fn pixels(&self) -> js_sys::Uint8Array {
        // The view is valid while `self` is alive and memory hasn't grown.
        unsafe { js_sys::Uint8Array::view(&self.pixels) }
    }
}
