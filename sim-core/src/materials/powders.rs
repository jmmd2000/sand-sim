use super::Material;
use crate::{Cell, SimAPI};

pub(super) fn update_gunpowder(cell: Cell, mut api: SimAPI) {
    const BLAST_RADIUS: i32 = 5;
    const IGNITE_SOURCES: &[Material] = &[Material::Fire, Material::Lava, Material::Ember];

    for (dx, dy) in [(0i32, -1i32), (-1, 0), (1, 0), (0, 1)] {
        if IGNITE_SOURCES.contains(&api.get(dx, dy).material) {
            explode(&mut api, BLAST_RADIUS);
            return;
        }
    }

    // Falls like sand
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
                    api.set(dx, dy, Cell { material: Material::Fire, ra, rb: 0, clock: 0 });
                }
                Material::Empty | Material::Smoke | Material::Steam => {
                    if api.rand_u32() % 3 == 0 {
                        api.set(dx, dy, Cell { material: Material::Fire, ra, rb: 0, clock: 0 });
                    }
                }
                _ => {
                    if api.rand_u32() % 2 == 0 {
                        let becomes = if api.rand_u32() % 5 == 0 { Material::Ash } else { Material::Smoke };
                        api.set(dx, dy, Cell { material: becomes, ra, rb: 0, clock: 0 });
                    }
                }
            }
        }
    }
    // Replace the gunpowder itself with fire
    let ra = api.rand_u32() as u8;
    api.set(0, 0, Cell { material: Material::Fire, ra, rb: 0, clock: 0 });
}

pub(super) fn update_sand(cell: Cell, mut api: SimAPI) {
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
