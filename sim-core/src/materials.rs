use crate::{Cell, SimAPI};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Material {
    Empty = 0,
    Wall = 1,
    Sand = 2,
    Water = 3,
    Stone = 4, // immovable solid
    Wood = 5,
    Fire = 6,
    Smoke = 7,
    Ash = 8,
}

impl Material {
    #[inline]
    pub fn from_id(id: u8) -> Self {
        match id {
            1 => Material::Wall,
            2 => Material::Sand,
            3 => Material::Water,
            4 => Material::Stone,
            5 => Material::Wood,
            6 => Material::Fire,
            7 => Material::Smoke,
            8 => Material::Ash,
            _ => Material::Empty,
        }
    }
    #[inline]
    pub fn id(self) -> u8 {
        self as u8
    }
}

#[inline]
pub fn color_of(cell: Cell) -> [u8; 4] {
    let base_color: [u8; 4] = match cell.material {
        Material::Empty => return [0, 0, 0, 255],
        Material::Wall => [100, 95, 90, 255],
        Material::Stone => [110, 110, 115, 255],
        Material::Sand => [210, 185, 110, 255],
        Material::Water => [40, 110, 210, 160],
        Material::Wood => [120, 75, 30, 255],
        Material::Fire => [220, 60, 10, 255],
        Material::Smoke => [80, 80, 85, 180],
        Material::Ash => [160, 155, 145, 255],
    };

    let v = cell.ra as i16 - 128;

    // Per-channel divisors: lower = more variation on that channel.
    // Sand biases warm (more R/G), water biases cool (more B).
    let (rd, gd, bd): (i16, i16, i16) = match cell.material {
        Material::Sand => (5, 7, 12),
        Material::Water => (12, 7, 4),
        Material::Stone => (7, 7, 7),
        Material::Wall => (9, 9, 9),
        Material::Wood => (8, 6, 4),
        Material::Fire => (2, 4, 20),
        Material::Smoke => (6, 6, 6),
        Material::Ash => (10, 10, 8),
        _ => (7, 7, 7),
    };

    let r = (base_color[0] as i16 + v / rd).clamp(0, 255) as u8;
    let g = (base_color[1] as i16 + v / gd).clamp(0, 255) as u8;
    let b = (base_color[2] as i16 + v / bd).clamp(0, 255) as u8;

    [r, g, b, base_color[3]]
}

const SMOKE_PASSABLE: &[Material] = &[Material::Empty, Material::Sand, Material::Water, Material::Ash];
const WATER_DISSOLVES: &[Material] = &[Material::Ash];

// Dispatcher for one cell
pub fn update_cell(cell: Cell, api: SimAPI) {
    match cell.material {
        Material::Sand => update_sand(cell, api),
        Material::Water => update_water(cell, api),
        Material::Fire => update_fire(cell, api),
        Material::Smoke => update_smoke(cell, api),
        Material::Ash => update_sand(cell, api),
        _ => { /* Wall, Stone, Wood, Empty - do nothing */ }
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
    if api.try_move_dissolving(0, 1, cell, WATER_DISSOLVES) {
        return;
    }

    let left_first = ((api.generation() as u32) ^ api.rand_u32()) & 1 == 0;

    if left_first {
        if api.try_move_dissolving(-1, 1, cell, WATER_DISSOLVES) {
            return;
        }
        if api.try_move_dissolving(1, 1, cell, WATER_DISSOLVES) {
            return;
        }
    } else {
        if api.try_move_dissolving(1, 1, cell, WATER_DISSOLVES) {
            return;
        }
        if api.try_move_dissolving(-1, 1, cell, WATER_DISSOLVES) {
            return;
        }
    }

    // Scan up to DISPERSION cells in each direction, move to farthest clear cell
    const DISPERSION: i32 = 7;
    let dirs: [i32; 2] = if left_first { [-1, 1] } else { [1, -1] };
    for dir in dirs {
        let mut max = 0;
        for d in 1..=DISPERSION {
            let m = api.get(dir * d, 0).material;
            if m == Material::Ash {
                max = d;
                break;
            }
            if m != Material::Empty {
                break;
            }
            max = d;
        }
        if max > 0 && api.try_move_dissolving(dir * max, 0, cell, WATER_DISSOLVES) {
            return;
        }
    }
}

fn update_fire(cell: Cell, mut api: SimAPI) {
    // Water extinguishes fire
    for (dx, dy) in [(0i32, -1i32), (-1, 0), (1, 0), (0, 1)] {
        if api.get(dx, dy).material == Material::Water {
            api.clear_here();
            return;
        }
    }

    // rb counts lifetime ticks; die into ash when done
    let life = cell.rb.wrapping_add(1);
    if life > 180 {
        let ra = api.rand_u32() as u8;
        if api.rand_u32() % 10 == 0 {
            api.set(0, 0, Cell { material: Material::Ash, ra, rb: 0, clock: 0 });
        } else {
            api.set(0, 0, Cell { material: Material::Empty, ra, rb: 0, clock: 0 });
        }

        return;
    }
    api.set_rb(life);

    // Spread to adjacent wood
    if api.rand_u32() % 4 == 0 {
        for (dx, dy) in [(0i32, -1i32), (-1, 0), (1, 0), (0, 1)] {
            if api.get(dx, dy).material == Material::Wood {
                let ra = api.rand_u32() as u8;
                api.set(dx, dy, Cell { material: Material::Fire, ra, rb: 0, clock: 0 });
            }
        }
    }

    // Spawn smoke above
    if api.rand_u32() % 6 == 0 && api.get(0, -1).material == Material::Empty {
        let ra = api.rand_u32() as u8;
        api.set(0, -1, Cell { material: Material::Smoke, ra, rb: 0, clock: 0 });
    }
}

fn update_smoke(cell: Cell, mut api: SimAPI) {
    let life = cell.rb.wrapping_add(1);
    if life > 120 {
        api.clear_here();
        return;
    }

    let cell = Cell { rb: life, ..cell };

    if api.try_move_into(0, -1, cell, SMOKE_PASSABLE) {
        return;
    }

    let left_first = api.rand_u32() & 1 == 0;
    if left_first {
        if api.try_move_into(-1, -1, cell, SMOKE_PASSABLE) {
            return;
        }
        if api.try_move_into(1, -1, cell, SMOKE_PASSABLE) {
            return;
        }
        if api.try_move_into(-1, 0, cell, SMOKE_PASSABLE) {
            return;
        }
        if api.try_move_into(1, 0, cell, SMOKE_PASSABLE) {
            return;
        }
    } else {
        if api.try_move_into(1, -1, cell, SMOKE_PASSABLE) {
            return;
        }
        if api.try_move_into(-1, -1, cell, SMOKE_PASSABLE) {
            return;
        }
        if api.try_move_into(1, 0, cell, SMOKE_PASSABLE) {
            return;
        }
        if api.try_move_into(-1, 0, cell, SMOKE_PASSABLE) {
            return;
        }
    }

    api.set_rb(life);
}

// fn update_water(cell: Cell, mut api: SimAPI) {
//     // Add some randomness to make water feel more viscous
//     // if api.rand_u32() % 10 == 0 {
//     //     return; // 25% chance to not move this tick
//     // }
//
//     if api.try_move(0, 1, cell) {
//         return;
//     }
//
//     // Check if on the surface
//     let above = api.get(0, -1).material;
//     let is_surface = above == Material::Empty;
//
//     let left_first = ((api.generation() as u32) ^ api.rand_u32()) & 1 == 0;
//
//     if is_surface {
//         // Surface water, prioritize horizontal flow (surface tension)
//         if left_first {
//             if api.try_move(-1, 0, cell) { return; }
//             if api.try_move(1, 0, cell) { return; }
//             if api.try_move(-1, 1, cell) { return; }
//             if api.try_move(1, 1, cell) { return; }
//         } else {
//             if api.try_move(1, 0, cell) { return; }
//             if api.try_move(-1, 0, cell) { return; }
//             if api.try_move(1, 1, cell) { return; }
//             if api.try_move(-1, 1, cell) { return; }
//         }
//     } else {
//         // Submerged water, try diagonal falls first
//         if left_first {
//             if api.try_move(-1, 1, cell) { return; }
//             if api.try_move(1, 1, cell) { return; }
//             if api.try_move(-1, 0, cell) { return; }
//             if api.try_move(1, 0, cell) { return; }
//         } else {
//             if api.try_move(1, 1, cell) { return; }
//             if api.try_move(-1, 1, cell) { return; }
//             if api.try_move(1, 0, cell) { return; }
//             if api.try_move(-1, 0, cell) { return; }
//         }
//     }
//
//     // Check if in a pool (surrounded by water or walls)
//     let below = api.get(0, 1).material;
//     let left = api.get(-1, 0).material;
//     let right = api.get(1, 0).material;
//
//     let is_in_pool = (below == Material::Wall || below == Material::Water)
//         && (left == Material::Wall || left == Material::Water)
//         && (right == Material::Wall || right == Material::Water);
//
//     // Only try horizontal moves if not in a pool
//     if !is_in_pool {
//         if left_first {
//             if api.try_move(-1, 0, cell) { return; }
//             if api.try_move(1, 0, cell) { return; }
//         } else {
//             if api.try_move(1, 0, cell) { return; }
//             if api.try_move(-1, 0, cell) { return; }
//         }
//     }
// }
