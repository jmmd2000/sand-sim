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

    // If can't fall straight down, try diagonal falls
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

    // try swap down if there is water below
    if api.try_move_into(0, 1, cell, &[Material::Water]) {
        return;
    }

    // If can't fall diagonally, try to move into water
    if left_first {
        if api.try_move_into(-1, 1, cell, &[Material::Water]) {
            return;
        }
        if api.try_move_into(1, 1, cell, &[Material::Water]) {
            return;
        }
    } else {
        if api.try_move_into(1, 1, cell, &[Material::Water]) {
            return;
        }
        if api.try_move_into(-1, 1, cell, &[Material::Water]) {
            return;
        }
    }
}

fn update_water(cell: Cell, mut api: SimAPI) {
    // Add some randomness to make water feel more viscous
    if api.rand_u32() % 4 == 0 {
        return; // 25% chance to not move this tick
    }

    if api.try_move(0, 1, cell) {
        return;
    }

    // Check if on the surface
    let above = api.get(0, -1).material;
    let is_surface = above == Material::Empty;

    let left_first = ((api.generation() as u32) ^ api.rand_u32()) & 1 == 0;

    if is_surface {
        // Surface water, prioritize horizontal flow (surface tension)
        if left_first {
            if api.try_move(-1, 0, cell) {
                return;
            }
            if api.try_move(1, 0, cell) {
                return;
            }
            if api.try_move(-1, 1, cell) {
                return;
            }
            if api.try_move(1, 1, cell) {
                return;
            }
        } else {
            if api.try_move(1, 0, cell) {
                return;
            }
            if api.try_move(-1, 0, cell) {
                return;
            }
            if api.try_move(1, 1, cell) {
                return;
            }
            if api.try_move(-1, 1, cell) {
                return;
            }
        }
    } else {
        // Submerged water, try diagonal falls first
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

    // Check if in a pool (surrounded by water or walls)
    let below = api.get(0, 1).material;
    let left = api.get(-1, 0).material;
    let right = api.get(1, 0).material;
    
    let is_in_pool = (below == Material::Wall || below == Material::Water) &&
                     (left == Material::Wall || left == Material::Water) &&
                     (right == Material::Wall || right == Material::Water);

    // Only try horizontal moves if not in a pool
    if !is_in_pool {
        if left_first {
            if api.try_move(-1, 0, cell) {
                return;
            }
            if api.try_move(1, 0, cell) {
                return;
            }
        } else {
            if api.try_move(1, 0, cell) {
                return;
            }
            if api.try_move(-1, 0, cell) {
                return;
            }
        }
    }
}
