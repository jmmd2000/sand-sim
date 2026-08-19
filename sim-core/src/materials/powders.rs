use super::Material;
use crate::{Cell, SimAPI};

pub(super) fn update_gunpowder(_cell: Cell, mut api: SimAPI) {
    const BLAST_RADIUS: i32 = 5;
    const IGNITE_SOURCES: &[Material] = &[Material::Fire, Material::Lava, Material::Ember];

    // check for ignition before velocity
    for (dx, dy) in [(0i32, -1i32), (-1, 0), (1, 0), (0, 1)] {
        if IGNITE_SOURCES.contains(&api.get(dx, dy).material) {
            explode(&mut api, BLAST_RADIUS);
            return;
        }
    }

    api.apply_gravity();
    api.resolve_velocity();
    let cell = api.get(0, 0);

    // slope acceleration
    if cell.vy == 0 {
        let p = super::props(cell.material);
        let left_first = ((api.generation() as u32) ^ api.rand_u32()) & 1 == 0;
        let (first, second) = if left_first { (-1, 1) } else { (1, -1) };

        for dir in [first, second] {
            if api.get(dir, 1).material == Material::Empty {
                let mut sliding = cell;
                let tv = p.terminal_velocity as i16;
                sliding.vx = (sliding.vx as i16 + dir as i16 * p.slide_acceleration as i16).clamp(-tv, tv) as i8;
                sliding.vy = 1;
                api.set(0, 0, sliding);
                return;
            }
        }
    }
}

fn explode(api: &mut SimAPI, radius: i32) {
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            if dx * dx + dy * dy > radius * radius {
                continue;
            }
            let m = api.get(dx, dy).material;
            let ra = api.rand_u32() as u8;
            match m {
                Material::Wall | Material::Obsidian => {}
                Material::Gunpowder => {
                    // Chain detonate — turn into fire to trigger neighbours next tick
                    api.set(dx, dy, Cell { material: Material::Fire, ra, rb: 0, clock: 0, vx: 0, vy: 0 });
                }
                Material::Empty | Material::Smoke | Material::Steam => {
                    if api.rand_u32() % 3 == 0 {
                        api.set(dx, dy, Cell { material: Material::Fire, ra, rb: 0, clock: 0, vx: 0, vy: 0 });
                    }
                }
                _ => {
                    if api.rand_u32() % 2 == 0 {
                        let becomes = if api.rand_u32() % 5 == 0 { Material::Ash } else { Material::Smoke };
                        api.set(dx, dy, Cell { material: becomes, ra, rb: 0, clock: 0, vx: 0, vy: 0 });
                    }
                }
            }
        }
    }
    // Replace the gunpowder itself with fire
    let ra = api.rand_u32() as u8;
    api.set(0, 0, Cell { material: Material::Fire, ra, rb: 0, clock: 0, vx: 0, vy: 0 });
}

pub(super) fn update_sand(_cell: Cell, mut api: SimAPI) {
    // accumulate gravity onto vy, then move along velocity vector
    api.apply_gravity();
    api.resolve_velocity();

    // reread cell at (possibly new) position after resolve
    let cell = api.get(0, 0);

    // slope acceleration: when resting on a surface, vy was zeroed by collision,
    // check if a diagonal below is open and push vx in that direction
    if cell.vy == 0 {
        let p = super::props(cell.material);
        let left_first = ((api.generation() as u32) ^ api.rand_u32()) & 1 == 0;
        let (first, second) = if left_first { (-1, 1) } else { (1, -1) };

        for dir in [first, second] {
            if api.get(dir, 1).material == Material::Empty {
                let mut sliding = cell;
                // accumulate horizontal velocity towards the open diagonal
                let tv = p.terminal_velocity as i16;
                sliding.vx = (sliding.vx as i16 + dir as i16 * p.slide_acceleration as i16).clamp(-tv, tv) as i8;

                sliding.vy = 1;
                api.set(0, 0, sliding);
                return;
            }
        }
    }

    // density displacement: sand sinks through water at 1 cell/tick
    let left_first = ((api.generation() as u32) ^ api.rand_u32()) & 1 == 0;
    if api.try_move_into(0, 1, cell, &[Material::Water]) {
        return;
    }
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
