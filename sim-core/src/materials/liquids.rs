use super::Material;
use crate::{Cell, SimAPI};

const LAVA_DISSOLVES: &[(Material, u32)] = &[(Material::Stone, 20), (Material::Sand, 10), (Material::Ash, 1)];
const WATER_DISSOLVES: &[(Material, u32)] = &[(Material::Ash, 1)];
const ACID_DISSOLVES: &[(Material, u32)] = &[
    (Material::Ash, 1),
    (Material::Sand, 5),
    (Material::Wood, 10),
    (Material::Stone, 20),
    (Material::Obsidian, 40),
    (Material::Wall, 50),
];

pub(super) fn update_water(cell: Cell, mut api: SimAPI) {
    const BOILING_POINT: u8 = 45;
    const BOILING_RATE: u32 = 20; // higher value means slower boil
    const DISPERSION: i32 = 7;

    // Boil to steam when heat is high enough
    if api.heat_here() > BOILING_POINT && api.rand_u32() % BOILING_RATE == 0 {
        let ra = api.rand_u32() as u8;
        api.set(0, 0, Cell { material: Material::Steam, ra, rb: 0, clock: 0 });
        return;
    }

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

    // Scan sideways up to DISPERSION cells, move to farthest clear spot
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

pub(super) fn update_lava(cell: Cell, mut api: SimAPI) {
    const HEAT_EMISSION: u8 = 254;
    const WOOD_IGNITE_RATE: u32 = 8;
    const EMBER_SPAWN_RATE: u32 = 500; // higher = less likely
    const SMOKE_SPAWN_RATE: u32 = 200;
    const VISCOSITY: u32 = 3; // higher = more viscous

    api.set_heat(0, 0, HEAT_EMISSION);

    // Touching water: become obsidian, burst steam
    for (dx, dy) in [(0i32, -1i32), (-1, 0), (1, 0), (0, 1)] {
        if api.get(dx, dy).material == Material::Water {
            let ra = api.rand_u32() as u8;
            api.set(dx, dy, Cell { material: Material::Steam, ra, rb: 0, clock: 0 });
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

    // Ignite adjacent wood
    if api.rand_u32() % WOOD_IGNITE_RATE == 0 {
        for (dx, dy) in [(0i32, -1i32), (-1, 0), (1, 0), (0, 1)] {
            if api.get(dx, dy).material == Material::Wood {
                let ra = api.rand_u32() as u8;
                api.set(dx, dy, Cell { material: Material::Fire, ra, rb: 0, clock: 0 });
            }
        }
    }

    // Occasionally shoot an ember upward
    if api.rand_u32() % EMBER_SPAWN_RATE == 0 {
        let offsets = [(-1i32, -1i32), (0, -1), (1, -1), (-2, -1), (2, -1)];
        let idx = (api.rand_u32() as usize) % offsets.len();
        let (dx, dy) = offsets[idx];
        if api.get(dx, dy).material == Material::Empty {
            let ra = api.rand_u32() as u8;
            api.set(dx, dy, Cell { material: Material::Ember, ra, rb: 0, clock: 0 });
        }
    }

    // Surface lava emits smoke
    if api.rand_u32() % SMOKE_SPAWN_RATE == 0 && api.get(0, -1).material == Material::Empty {
        let ra = api.rand_u32() as u8;
        api.set(0, -1, Cell { material: Material::Smoke, ra, rb: 0, clock: 0 });
    }

    // Viscous: only move 1 in VISCOSITY ticks
    if api.rand_u32() % VISCOSITY != 0 {
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

pub(super) fn update_acid(cell: Cell, mut api: SimAPI) {
    const VISCOSITY: u32 = 3;

    // Viscous: move 1 in 3 ticks
    if api.rand_u32() % VISCOSITY != 0 {
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
