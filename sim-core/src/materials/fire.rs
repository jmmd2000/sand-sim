use super::Material;
use crate::{Cell, SimAPI};

const EMBER_PASSABLE: &[Material] = &[Material::Empty, Material::Smoke, Material::Steam];

pub(super) fn update_fire(cell: Cell, mut api: SimAPI) {
    const HEAT_EMISSION: u8 = 230; // how hot fire makes its cell; drives diffusion to neighbours
    const MAX_LIFESPAN: u8 = 180; // ticks until fire burns out
    const ASH_CHANCE: u32 = 10; // 1-in-N chance to leave ash instead of vanishing
    const WOOD_SPREAD_RATE: u32 = 4; // 1-in-N chance to ignite adjacent wood each tick
    const SMOKE_SPAWN_RATE: u32 = 100; // 1-in-N chance to emit smoke upward each tick
    const FALL_RATE: u32 = 6; // 1-in-N chance to drip down per tick

    api.set_heat(0, 0, HEAT_EMISSION);

    // Water extinguishes fire
    for (dx, dy) in [(0i32, -1i32), (-1, 0), (1, 0), (0, 1)] {
        if api.get(dx, dy).material == Material::Water {
            api.clear_here();
            return;
        }
    }

    // rb counts lifetime; die into ash or empty
    let life = cell.rb.wrapping_add(1);
    if life > MAX_LIFESPAN {
        let ra = api.rand_u32() as u8;
        let below = api.get(0, 1).material;
        let on_solid = matches!(below, Material::Sand | Material::Stone | Material::Wall | Material::Wood | Material::Obsidian | Material::Ash | Material::Gunpowder | Material::Ice);
        let becomes = if on_solid && api.rand_u32() % ASH_CHANCE == 0 { Material::Ash } else { Material::Empty };
        api.set(0, 0, Cell { material: becomes, ra, rb: 0, clock: 0 });
        return;
    }
    let cell = Cell { rb: life, ..cell };

    // Spread to adjacent wood
    if api.rand_u32() % WOOD_SPREAD_RATE == 0 {
        for (dx, dy) in [(0i32, -1i32), (-1, 0), (1, 0), (0, 1)] {
            if api.get(dx, dy).material == Material::Wood {
                let ra = api.rand_u32() as u8;
                api.set(dx, dy, Cell { material: Material::Fire, ra, rb: 0, clock: 0 });
            }
        }
    }

    // Spawn smoke above
    if api.rand_u32() % SMOKE_SPAWN_RATE == 0 && api.get(0, -1).material == Material::Empty {
        let ra = api.rand_u32() as u8;
        api.set(0, -1, Cell { material: Material::Smoke, ra, rb: 0, clock: 0 });
    }

    if api.rand_u32() % FALL_RATE == 0 {
        if api.try_move_into(0, 1, cell, EMBER_PASSABLE) {
            return;
        }
        let left_first = api.rand_u32() & 1 == 0;
        if left_first {
            if api.try_move_into(-1, 1, cell, EMBER_PASSABLE) {
                return;
            }
            if api.try_move_into(1, 1, cell, EMBER_PASSABLE) {
                return;
            }
        } else {
            if api.try_move_into(1, 1, cell, EMBER_PASSABLE) {
                return;
            }
            if api.try_move_into(-1, 1, cell, EMBER_PASSABLE) {
                return;
            }
        }
    }
    api.set_rb(life);
}

pub(super) fn update_ember(cell: Cell, mut api: SimAPI) {
    const HEAT_EMISSION: u8 = 180; // cooler than fire
    const MIN_LIFESPAN: u8 = 20; // embers burn out fast
    const LIFESPAN_VARIANCE: u8 = 5; // ra/VARIANCE added on top
    const REIGNITE_CHANCE: u32 = 4; // 1-in-N chance to become fire instead of smoke on burnout
    const WOOD_IGNITE_RATE: u32 = 4; // 1-in-N chance to set adjacent wood alight each tick

    api.set_heat(0, 0, HEAT_EMISSION);

    let life = cell.rb.wrapping_add(1);
    let max_life = MIN_LIFESPAN.saturating_add(cell.ra / LIFESPAN_VARIANCE);
    if life > max_life {
        let ra = api.rand_u32() as u8;
        let becomes = if api.rand_u32() % REIGNITE_CHANCE == 0 { Material::Fire } else { Material::Smoke };
        api.set(0, 0, Cell { material: becomes, ra, rb: 0, clock: 0 });
        return;
    }

    let cell = Cell { rb: life, ..cell };

    // Ignite adjacent wood
    if api.rand_u32() % WOOD_IGNITE_RATE == 0 {
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
