use crate::{Cell, SimAPI};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Material {
    Empty = 0,
    Wall = 1,
    Sand = 2,
    Water = 3,
    Stone = 4,
    Wood = 5,
    Fire = 6,
    Smoke = 7,
    Ash = 8,
    Lava = 9,
    Steam = 10,
    Obsidian = 11,
    Acid = 12,
    Ember = 13,
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
            9 => Material::Lava,
            10 => Material::Steam,
            11 => Material::Obsidian,
            12 => Material::Acid,
            13 => Material::Ember,
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
        Material::Lava => [207, 70, 10, 255],
        Material::Steam => [200, 220, 255, 160],
        Material::Obsidian => [25, 15, 40, 255],
        Material::Acid => [3, 160, 45, 220],
        Material::Ember => [255, 160, 20, 255],
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
        Material::Lava => (2, 6, 20),
        Material::Steam => (10, 8, 6),
        Material::Obsidian => (12, 12, 8),
        Material::Acid => (4, 12, 7),
        Material::Ember => (2, 5, 20),
        _ => (7, 7, 7),
    };

    let r = (base_color[0] as i16 + v / rd).clamp(0, 255) as u8;
    let g = (base_color[1] as i16 + v / gd).clamp(0, 255) as u8;
    let b = (base_color[2] as i16 + v / bd).clamp(0, 255) as u8;

    [r, g, b, base_color[3]]
}

#[inline]
pub fn glow_of(cell: Cell) -> [u8; 4] {
    match cell.material {
        Material::Fire => {
            let b = (cell.ra as u16 * 40 / 255) as u8;
            [255, 120 + b, 0, 200]
        }
        Material::Lava => [255, 60, 0, 160],
        Material::Acid => [0, 255, 60, 80],
        Material::Ember => [255, 140, 0, 220],
        _ => [0, 0, 0, 0],
    }
}

const SMOKE_PASSABLE: &[Material] = &[Material::Empty, Material::Sand, Material::Water, Material::Ash, Material::Steam, Material::Lava, Material::Acid];
const STEAM_PASSABLE: &[Material] = &[Material::Empty, Material::Smoke, Material::Water, Material::Ash, Material::Smoke, Material::Lava, Material::Acid];
const EMBER_PASSABLE: &[Material] = &[Material::Empty, Material::Smoke, Material::Steam];
const LAVA_DISSOLVES: &[(Material, u32)] = &[
    (Material::Stone, 20), // 1/20 chance - melts slowly
    (Material::Sand, 10),  // 1/10 chance - melts faster
];
const WATER_DISSOLVES: &[(Material, u32)] = &[
    (Material::Ash, 1), // always - water washes ash away instantly
];
const ACID_DISSOLVES: &[(Material, u32)] = &[
    (Material::Ash, 1),       // instant
    (Material::Sand, 5),      // fast
    (Material::Wood, 10),     // medium
    (Material::Stone, 20),    // slow
    (Material::Obsidian, 40), // very slow
    (Material::Wall, 50),     // very slow
];

// Dispatcher for one cell
pub fn update_cell(cell: Cell, api: SimAPI) {
    match cell.material {
        Material::Sand => update_sand(cell, api),
        Material::Water => update_water(cell, api),
        Material::Fire => update_fire(cell, api),
        Material::Smoke => update_smoke(cell, api),
        Material::Ash => update_sand(cell, api),
        Material::Lava => update_lava(cell, api),
        Material::Steam => update_steam(cell, api),
        Material::Acid => update_acid(cell, api),
        Material::Ember => update_ember(cell, api),
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
    let max_life = 80u8.saturating_add(cell.ra / 4); // random lifespan 80-143
    if life > max_life {
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

fn update_lava(cell: Cell, mut api: SimAPI) {
    // Lava touching water: become obsidian, water becomes steam, burst nearby steam
    for (dx, dy) in [(0i32, -1i32), (-1, 0), (1, 0), (0, 1)] {
        if api.get(dx, dy).material == Material::Water {
            let ra = api.rand_u32() as u8;
            api.set(dx, dy, Cell { material: Material::Steam, ra, rb: 0, clock: 0 });
            // Burst extra steam into surrounding empty cells
            for (sdx, sdy) in [(-1i32, -2i32), (0, -2), (1, -2), (-1, -1), (1, -1), (-2, 0), (2, 0)] {
                if api.get(sdx, sdy).material == Material::Empty {
                    let ra = api.rand_u32() as u8;
                    api.set(sdx, sdy, Cell { material: Material::Steam, ra, rb: 0, clock: 0 });
                }
            }
            let ra2 = api.rand_u32() as u8;
            api.set(0, 0, Cell { material: Material::Obsidian, ra: ra2, rb: 0, clock: 0 });
            return;
        }
    }

    // Lava ignites adjacent wood
    if api.rand_u32() % 8 == 0 {
        for (dx, dy) in [(0i32, -1i32), (-1, 0), (1, 0), (0, 1)] {
            if api.get(dx, dy).material == Material::Wood {
                let ra = api.rand_u32() as u8;
                api.set(dx, dy, Cell { material: Material::Fire, ra, rb: 0, clock: 0 });
            }
        }
    }

    // Shoot ember particles upward occasionally
    if api.rand_u32() % 500 == 0 {
        let offsets = [(-1i32, -1i32), (0, -1), (1, -1), (-2, -1), (2, -1)];
        let idx = (api.rand_u32() as usize) % offsets.len();
        let (dx, dy) = offsets[idx];
        if api.get(dx, dy).material == Material::Empty {
            let ra = api.rand_u32() as u8;
            api.set(dx, dy, Cell { material: Material::Ember, ra, rb: 0, clock: 0 });
        }
    }

    // Surface lava (nothing directly above) slowly emits smoke
    if api.rand_u32() % 200 == 0 && api.get(0, -1).material == Material::Empty {
        let ra = api.rand_u32() as u8;
        api.set(0, -1, Cell { material: Material::Smoke, ra, rb: 0, clock: 0 });
    }

    // // Cool to obsidian over time (rb counts up slowly)
    // let life = if api.rand_u32() % 30 == 0 { cell.rb.wrapping_add(1) } else { cell.rb };
    // if life > 220 {
    //     let ra = api.rand_u32() as u8;
    //     api.set(0, 0, Cell { material: Material::Obsidian, ra, rb: 0, clock: 0 });
    //     return;
    // }

    // Viscous: only move 1 in 3 ticks
    if api.rand_u32() % 3 != 0 {
        return;
    }

    if api.try_move_dissolving(0, 1, cell, LAVA_DISSOLVES) {
        return;
    }

    let left_first = ((api.generation() as u32) ^ api.rand_u32()) & 1 == 0;
    if left_first {
        if api.try_move_dissolving(-1, 1, cell, LAVA_DISSOLVES) {
            return;
        }
        if api.try_move_dissolving(1, 1, cell, LAVA_DISSOLVES) {
            return;
        }
        if api.try_move_dissolving(-1, 0, cell, LAVA_DISSOLVES) {
            return;
        }
        if api.try_move_dissolving(1, 0, cell, LAVA_DISSOLVES) {
            return;
        }
    } else {
        if api.try_move_dissolving(1, 1, cell, LAVA_DISSOLVES) {
            return;
        }
        if api.try_move_dissolving(-1, 1, cell, LAVA_DISSOLVES) {
            return;
        }
        if api.try_move_dissolving(1, 0, cell, LAVA_DISSOLVES) {
            return;
        }
        if api.try_move_dissolving(-1, 0, cell, LAVA_DISSOLVES) {
            return;
        }
    }
}

fn update_steam(cell: Cell, mut api: SimAPI) {
    // Increment life slowly (1/20 chance) so steam lingers 20x longer - ~7-17s at 60tps
    let life = if api.rand_u32() % 20 == 0 { cell.rb.wrapping_add(1) } else { cell.rb };
    let max_life = 80u8.saturating_add(cell.ra / 2); // random lifespan 80-207 effective ticks
    if life > max_life {
        // Condense into water in place; it will fall naturally
        let ra = api.rand_u32() as u8;
        api.set(0, 0, Cell { material: Material::Water, ra, rb: 0, clock: 0 });
        return;
    }

    let cell = Cell { rb: life, ..cell };

    if api.try_move_into(0, -1, cell, STEAM_PASSABLE) {
        return;
    }

    let left_first = api.rand_u32() & 1 == 0;
    if left_first {
        if api.try_move_into(-1, -1, cell, STEAM_PASSABLE) {
            return;
        }
        if api.try_move_into(1, -1, cell, STEAM_PASSABLE) {
            return;
        }
        if api.try_move_into(-1, 0, cell, STEAM_PASSABLE) {
            return;
        }
        if api.try_move_into(1, 0, cell, STEAM_PASSABLE) {
            return;
        }
    } else {
        if api.try_move_into(1, -1, cell, STEAM_PASSABLE) {
            return;
        }
        if api.try_move_into(-1, -1, cell, STEAM_PASSABLE) {
            return;
        }
        if api.try_move_into(1, 0, cell, STEAM_PASSABLE) {
            return;
        }
        if api.try_move_into(-1, 0, cell, STEAM_PASSABLE) {
            return;
        }
    }

    api.set_rb(life);
}

fn update_acid(cell: Cell, mut api: SimAPI) {
    // Viscous: move 1 in 3 ticks (same as lava)
    if api.rand_u32() % 3 != 0 {
        return;
    }

    // Probabilistically dissolve one adjacent material per tick
    for (dx, dy) in [(0i32, 1i32), (-1, 0), (1, 0), (0, -1)] {
        let target = api.get(dx, dy).material;
        if let Some(&(_, rate)) = ACID_DISSOLVES.iter().find(|&&(m, _)| m == target) {
            if rate == 1 || api.rand_u32() % rate == 0 {
                let ra = api.rand_u32() as u8;
                api.set(dx, dy, Cell { material: Material::Empty, ra, rb: 0, clock: 0 });
                break;
            }
        }
    }

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
    } else {
        if api.try_move(1, 1, cell) {
            return;
        }
        if api.try_move(-1, 1, cell) {
            return;
        }
    }

    // Disperse sideways
    const DISPERSION: i32 = 2;
    let dirs: [i32; 2] = if left_first { [-1, 1] } else { [1, -1] };
    for dir in dirs {
        let mut max = 0;
        for d in 1..=DISPERSION {
            if api.get(dir * d, 0).material != Material::Empty {
                break;
            }
            max = d;
        }
        if max > 0 && api.try_move(dir * max, 0, cell) {
            return;
        }
    }
}

fn update_ember(cell: Cell, mut api: SimAPI) {
    let life = cell.rb.wrapping_add(1);
    let max_life = 20u8.saturating_add(cell.ra / 5); // random lifespan 20-71
    if life > max_life {
        // Burn out into smoke, occasionally fire
        let ra = api.rand_u32() as u8;
        let becomes = if api.rand_u32() % 4 == 0 { Material::Fire } else { Material::Smoke };
        api.set(0, 0, Cell { material: becomes, ra, rb: 0, clock: 0 });
        return;
    }

    let cell = Cell { rb: life, ..cell };

    // Ignite adjacent wood
    if api.rand_u32() % 4 == 0 {
        for (dx, dy) in [(0i32, -1i32), (-1, 0), (1, 0), (0, 1)] {
            if api.get(dx, dy).material == Material::Wood {
                let ra = api.rand_u32() as u8;
                api.set(dx, dy, Cell { material: Material::Fire, ra, rb: 0, clock: 0 });
            }
        }
    }

    // Rise upward, drifting slightly sideways
    if api.try_move_into(0, -1, cell, EMBER_PASSABLE) {
        return;
    }

    let left_first = api.rand_u32() & 1 == 0;
    if left_first {
        if api.try_move_into(-1, -1, cell, EMBER_PASSABLE) {
            return;
        }
        if api.try_move_into(1, -1, cell, EMBER_PASSABLE) {
            return;
        }
    } else {
        if api.try_move_into(1, -1, cell, EMBER_PASSABLE) {
            return;
        }
        if api.try_move_into(-1, -1, cell, EMBER_PASSABLE) {
            return;
        }
    }

    api.set_rb(life);
}
