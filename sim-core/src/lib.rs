use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

mod materials;
use materials::{EMPTY, MATERIALS, MatID};

#[wasm_bindgen]
pub struct Simulation {
    width: u32,
    height: u32,
    materials_front: Vec<MatID>, // Current frame
    materials_back: Vec<MatID>,  // Next frame
    colours_front: Vec<[u8; 4]>, // RGBA for front buffer colour variation
    colors_back: Vec<[u8; 4]>,   // RGBA for back buffer colour variation
    pixels: Vec<u8>,             // RGBA for display
    frame: u32,                  // Frame counter
}

// Convert (x,y) to array index
#[inline]
fn idx(width: u32, x: u32, y: u32) -> usize {
    (y * width + x) as usize
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Simulation {
        let len = (width * height) as usize;
        Simulation {
            width,
            height,
            materials_front: vec![EMPTY; len],
            materials_back: vec![EMPTY; len],
            colours_front: vec![[0, 0, 0, 255]; len],
            colors_back: vec![[0, 0, 0, 255]; len],
            pixels: vec![0; len * 4],
            frame: 0,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn frame(&self) -> u32 {
        self.frame
    }

    /// Set the material at (x,y) in the front buffer (immediate effect)
    pub fn set_cell(&mut self, x: u32, y: u32, material: MatID) {
        if self.in_bounds(x, y) {
            let i = idx(self.width, x, y);
            self.materials_front[i] = material;

            // Base color from the material table
            let base = MATERIALS[material as usize].color;
            // variation amount
            let range: i16 = 10;
            // mix x, y and frame into one value
            let mixed = (x as i16 * 13) + (y as i16 * 7) + (self.frame as i16);
            // limit it to 0..range*2
            let wrapped = mixed % (range * 2 + 1);
            // shift to -range..+range
            let v = wrapped - range;
            // apply to each channel
            let mut c = base;
            for k in 0..3 {
                let channel = c[k] as i16 + v;
                c[k] = channel.clamp(0, 255) as u8;
            }
            // set color
            self.colours_front[i] = c;
        }
    }

    /// Advance the simulation by one step
    pub fn step(&mut self) {
        let w = self.width;
        let h = self.height;

        // clear BACK once
        self.materials_back.fill(EMPTY);
        self.colors_back.fill([0, 0, 0, 255]);

        for y in (0..h).rev() {
            // alternate scan direction to avoid left/right bias
            let ltr = ((y + self.frame) & 1) == 0;
            if ltr {
                for x in 0..w {
                    let id = self.get(x, y);
                    if id != EMPTY {
                        (MATERIALS[id as usize].update)(x, y, self);
                    }
                }
            } else {
                let mut x = w;
                while x > 0 {
                    x -= 1;
                    let id = self.get(x, y);
                    if id != EMPTY {
                        (MATERIALS[id as usize].update)(x, y, self);
                    }
                }
            }
        }

        // swap once and render
        std::mem::swap(&mut self.materials_front, &mut self.materials_back);
        std::mem::swap(&mut self.colours_front, &mut self.colors_back);
        self.render();
        self.frame += 1;
    }

    /// Get a view of the pixel buffer for rendering
    pub fn pixels_view(&self) -> Uint8Array {
        unsafe { Uint8Array::view(&self.pixels) }
    }

    /// Count how many cells of a given material are present
    pub fn count_mat(&self, mat: MatID) -> usize {
        self.materials_front.iter().filter(|&&m| m == mat).count()
    }

    // --- Helper functions for materials to use ---

    /// Reads the material at (x,y) in the front buffer (current frame)
    pub(crate) fn get(&self, x: u32, y: u32) -> MatID {
        if self.in_bounds(x, y) {
            self.materials_front[idx(self.width, x, y)]
        } else {
            EMPTY
        }
    }

    /// Reads the material at (x,y) in the back buffer (next frame)
    pub(crate) fn get_back(&self, x: u32, y: u32) -> MatID {
        if self.in_bounds(x, y) {
            self.materials_back[idx(self.width, x, y)]
        } else {
            EMPTY
        }
    }

    /// Sets the material at (x,y) in the back buffer (next frame)
    pub(crate) fn set_next(&mut self, x: u32, y: u32, mat: MatID) {
        if self.in_bounds(x, y) {
            self.materials_back[idx(self.width, x, y)] = mat;
        }
    }

    #[inline]
    #[rustfmt::skip]
    pub fn try_move(&mut self, x: u32, y: u32, nx: u32, ny: u32) -> bool {
        if !self.in_bounds(nx, ny) {
            return false;
        }

        let id = self.get(x, y); // read from FRONT
        if id == EMPTY {return false;}

        // Only block if BACK already has something (claimed this frame)
        if self.get_back(nx, ny) != EMPTY {return false;}

        // Write ID and carry color to destination in BACK
        let src = idx(self.width, x, y);
        let dst = idx(self.width, nx, ny);

        self.set_next(nx, ny, id);
        self.colors_back[dst] = self.colours_front[src];

        true
    }

    // Keep particle in place
    #[inline]
    #[rustfmt::skip]
    pub(crate) fn stay(&mut self, x: u32, y: u32) {
        if !self.in_bounds(x, y) { return; }

        let i = idx(self.width, x, y);
        let id = self.get(x, y); // FRONT
        self.set_next(x, y, id); // ID to BACK
        self.colors_back[i] = self.colours_front[i]; // color to BACK
    }

    fn render(&mut self) {
        for i in 0..self.materials_front.len() {
            let p = i * 4;
            let [r, g, b, a] = self.colours_front[i];
            self.pixels[p] = r;
            self.pixels[p + 1] = g;
            self.pixels[p + 2] = b;
            self.pixels[p + 3] = a;
        }
    }

    #[inline]
    fn in_bounds(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }
}
