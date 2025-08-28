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
    pixels: Vec<u8>,             // RGBA for display
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
            pixels: vec![0; len * 4],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_cell(&mut self, x: u32, y: u32, material: MatID) {
        if x < self.width && y < self.height {
            self.materials_front[idx(self.width, x, y)] = material;
        }
    }

    pub fn step(&mut self) {
        // Clear the back buffer
        self.materials_back.fill(EMPTY);

        // Process each cell from bottom to top (for gravity)
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let mat_id = self.get(x, y);
                if mat_id != EMPTY {
                    // Call update for the material
                    (MATERIALS[mat_id as usize].update)(x, y, self);
                }
            }
        }

        // Swap buffers
        std::mem::swap(&mut self.materials_front, &mut self.materials_back);

        // Convert to pixels for display
        self.render();
    }

    pub fn pixels_view(&self) -> Uint8Array {
        unsafe { Uint8Array::view(&self.pixels) }
    }

    // --- Helper functions for materials to use ---

    /// Reads the material at (x,y) in the front buffer (current frame)
    pub(crate) fn get(&self, x: u32, y: u32) -> MatID {
        if x < self.width && y < self.height {
            self.materials_front[idx(self.width, x, y)]
        } else {
            EMPTY
        }
    }

    /// Reads the material at (x,y) in the back buffer (next frame)
    pub(crate) fn get_back(&self, x: u32, y: u32) -> MatID {
        if x < self.width && y < self.height {
            self.materials_back[idx(self.width, x, y)]
        } else {
            EMPTY
        }
    }

    /// Sets the material at (x,y) in the back buffer (next frame)
    pub(crate) fn set_next(&mut self, x: u32, y: u32, mat: MatID) {
        if x < self.width && y < self.height {
            self.materials_back[idx(self.width, x, y)] = mat;
        }
    }

    // Try to move a particle from (x,y) to (nx,ny)
    pub(crate) fn try_move(&mut self, x: u32, y: u32, nx: u32, ny: u32) -> bool {
        // Check if target is empty in both buffers
        if self.get_back(nx, ny) == EMPTY
            && self.get(nx, ny) == EMPTY
            && nx < self.width
            && ny < self.height
        {
            self.set_next(nx, ny, self.get(x, y));
            true
        } else {
            false
        }
    }

    // Keep particle in place
    pub(crate) fn stay(&mut self, x: u32, y: u32) {
        self.set_next(x, y, self.get(x, y));
    }

    fn render(&mut self) {
        for i in 0..self.materials_front.len() {
            let mat = &MATERIALS[self.materials_front[i] as usize];
            let p = i * 4;
            self.pixels[p] = mat.color[0];
            self.pixels[p + 1] = mat.color[1];
            self.pixels[p + 2] = mat.color[2];
            self.pixels[p + 3] = mat.color[3];
        }
    }
}
