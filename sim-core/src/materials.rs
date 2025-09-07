use crate::{Cell, SimAPI};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Material {
    Empty = 0,
    Wall = 1,
    Sand = 2,
    Water = 3,
    Stone = 4, // immovable solid
}

impl Material {
    #[inline]
    pub fn from_id(id: u8) -> Self {
        match id {
            1 => Material::Wall,
            2 => Material::Sand,
            3 => Material::Water,
            4 => Material::Stone,
            _ => Material::Empty,
        }
    }
    #[inline]
    pub fn id(self) -> u8 {
        self as u8
    }
}

#[inline]
pub fn color_of(s: Material) -> [u8; 4] {
    match s {
        Material::Empty => [0, 0, 0, 255],
        Material::Wall => [120, 120, 120, 255],
        Material::Stone => [90, 90, 90, 255],
        Material::Sand => [216, 180, 90, 255],
        Material::Water => [64, 120, 220, 255],
    }
}

// Dispatcher for one cell
pub fn update_cell(cell: Cell, mut api: SimAPI) {
    match cell.material {
        Material::Sand => update_sand(cell, api),
        Material::Water => update_water(cell, api),
        _ => { /* Wall, Stone, Empty - do nothing */ }
    }
}

fn update_sand(cell: Cell, mut api: SimAPI) {
    if api.try_move(0, 1, cell) {
        return;
    }

    // diagonals
    let left_first = ((api.generation() as u32) ^ api.rand_u32()) & 1 == 0;
    if left_first {
        if api.try_move(-1, 1, cell) {
            return;
        }
        if api.try_move(1, 1, cell) {
            return;
        }
    } else {
        if api.try_move(1, 1, cell) {
            return;
        }
        if api.try_move(-1, 1, cell) {
            return;
        }
    }

    // swap with water below
    let below = api.get(0, 1).material;
    if below == Material::Water {
        // move into water cell and push water up
        // write sand below
        api.set(0, 1, cell);

        api.clear_here();
        let mut water = Cell {
            material: Material::Water,
            ra: 0,
            rb: 0,
            clock: 0,
        };
        // stamp at current position
        water.clock = api.generation().wrapping_add(1);
        // set current cell to water
        // we cannot write "here" through set, so do direct since we already cleared_here and set stamp
        let i = super::idx(api.sim.width, api.x, api.y);
        api.sim.cells[i] = water;
        return;
    }
}

fn update_water(cell: Cell, mut api: SimAPI) {
    if api.try_move(0, 1, cell) {
        return;
    }

    let left_first = ((api.generation() as u32) ^ api.rand_u32()) & 1 == 0;

    if left_first {
        if api.try_move(-1, 1, cell) {
            return;
        }
        if api.try_move(1, 1, cell) {
            return;
        }
        if api.try_move(-1, 0, cell) {
            return;
        }
        if api.try_move(1, 0, cell) {
            return;
        }
    } else {
        if api.try_move(1, 1, cell) {
            return;
        }
        if api.try_move(-1, 1, cell) {
            return;
        }
        if api.try_move(1, 0, cell) {
            return;
        }
        if api.try_move(-1, 0, cell) {
            return;
        }
    }
}
