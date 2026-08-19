use super::Material;
use crate::{Cell, SimAPI};

const SMOKE_PASSABLE: &[Material] = &[Material::Empty, Material::Sand, Material::Water, Material::Ash, Material::Steam, Material::Lava, Material::Acid];
const STEAM_PASSABLE: &[Material] = &[Material::Empty, Material::Smoke, Material::Water, Material::Ash, Material::Lava, Material::Acid];

pub(super) fn update_smoke(cell: Cell, mut api: SimAPI) {
    const MIN_LIFESPAN: u8 = 80; // ticks before a smoke particle can disappear
    const LIFESPAN_VARIANCE: u8 = 4; // ra divided by this is added on top, so each particle lasts a bit differently

    let life = cell.rb.wrapping_add(1);
    let max_life = MIN_LIFESPAN.saturating_add(cell.ra / LIFESPAN_VARIANCE);
    if life > max_life {
        api.clear_here();
        return;
    }

    // write updated lifteime before velocity moves the cell
    api.set_rb(life);

    api.apply_gravity();
    api.resolve_velocity();
    let cell = api.get(0, 0);

    // if blocked above, try displacement through passable materials
    if cell.vy == 0 {
        if api.try_move_into(0, -1, cell, SMOKE_PASSABLE) {
            return;
        }
    }

    // horizontal drifting
    if api.rand_u32() % 10 == 0 {
        let left_first = api.rand_u32() & 1 == 0;
        if left_first {
            if api.try_move_into(-1, 0, cell, SMOKE_PASSABLE) {
                return;
            }
            if api.try_move_into(1, 0, cell, SMOKE_PASSABLE) {
                return;
            }
        } else {
            if api.try_move_into(1, 0, cell, SMOKE_PASSABLE) {
                return;
            }
            if api.try_move_into(-1, 0, cell, SMOKE_PASSABLE) {
                return;
            }
        }
    }
}

pub(super) fn update_steam(cell: Cell, mut api: SimAPI) {
    const LIFE_INCREMENT_RATE: u32 = 20; // only ages 1-in-N ticks, so steam lingers much longer than smoke
    const MIN_LIFESPAN: u8 = 80; // minimum lifespan before condensing back to water
    const LIFESPAN_VARIANCE: u8 = 2; // ra/VARIANCE added on top - larger divisor = less variance than smoke

    let life = if api.rand_u32() % LIFE_INCREMENT_RATE == 0 { cell.rb.wrapping_add(1) } else { cell.rb };
    let max_life = MIN_LIFESPAN.saturating_add(cell.ra / LIFESPAN_VARIANCE);
    if life > max_life {
        let ra = api.rand_u32() as u8;
        api.set(0, 0, Cell { material: Material::Water, ra, rb: 0, clock: 0, vx: 0, vy: 0 });
        return;
    }

    // write updated lifteime before velocity moves the cell
    api.set_rb(life);

    api.apply_gravity();
    api.resolve_velocity();
    let cell = api.get(0, 0);

    // if blocked above, try displacement through passable materials
    if cell.vy == 0 {
        if api.try_move_into(0, -1, cell, STEAM_PASSABLE) {
            return;
        }
    }

    if api.rand_u32() % 10 == 0 {
        let left_first = api.rand_u32() & 1 == 0;
        if left_first {
            if api.try_move_into(-1, 0, cell, STEAM_PASSABLE) {
                return;
            }
            if api.try_move_into(1, 0, cell, STEAM_PASSABLE) {
                return;
            }
        } else {
            if api.try_move_into(1, 0, cell, STEAM_PASSABLE) {
                return;
            }
            if api.try_move_into(-1, 0, cell, STEAM_PASSABLE) {
                return;
            }
        }
    }
}
