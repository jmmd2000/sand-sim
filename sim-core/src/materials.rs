use crate::Simulation;

pub type MatID = u16;

pub struct Material {
    pub name: &'static str,
    pub color: [u8; 4],                        // RGBA
    pub update: fn(u32, u32, &mut Simulation), // How this material behaves
}

// Material IDs
pub const EMPTY: MatID = 0;
pub const SAND: MatID = 1;
pub const WATER: MatID = 2;

// Update functions
fn update_empty(_x: u32, _y: u32, _sim: &mut Simulation) {
    // Empty does nothing
}
#[rustfmt::skip]
fn update_sand(x: u32, y: u32, sim: &mut Simulation) {
    // try straight down
    if sim.try_move(x, y, x, y + 1) {
        return;
    }

    // flip diagonal preference based on position and frame
    let right_first = ((x + y + sim.frame()) & 1) == 0;

    if right_first {
        if sim.try_move(x, y, x + 1, y + 1) {return;}
        if sim.try_move(x, y, x.wrapping_sub(1), y + 1) {return;}
    } else {
        if sim.try_move(x, y, x.wrapping_sub(1), y + 1) {return;}
        if sim.try_move(x, y, x + 1, y + 1) {return;}
    }

    // stay put
    sim.stay(x, y);
}

#[rustfmt::skip]
fn update_water(x: u32, y: u32, sim: &mut Simulation) {
    // try straight down
    if sim.try_move(x, y, x, y + 1) {
        return;
    }

    // flip diagonal preference based on position and frame
    let right_first = ((x + y + sim.frame()) & 1) == 0;
    
    if right_first {
        if sim.try_move(x, y, x + 1, y + 1) { return; } // try down-right
        if x > 0 && sim.try_move(x, y, x.wrapping_sub(1), y + 1) { return; } // try down-left
        if sim.try_move(x, y, x + 1, y){ return; } // try right
        if x > 0 && sim.try_move(x, y, x.wrapping_sub(1), y) { return; } // try left
    } else {
        if x > 0 && sim.try_move(x, y, x.wrapping_sub(1), y + 1) { return; } // try down-left
        if sim.try_move(x, y, x + 1, y + 1) { return; } // try down-right
        if x > 0 && sim.try_move(x, y, x.wrapping_sub(1), y) { return; } // try left
        if sim.try_move(x, y, x + 1, y) { return; } // try right
    }

    // stay put
    sim.stay(x, y);
}

// Registry of all materials
#[rustfmt::skip]
pub static MATERIALS: &[Material] = &[
    Material {name: "Empty",color: [0, 0, 0, 255],      update: update_empty,},
    Material {name: "Sand", color: [194, 178, 128, 255],update: update_sand,},
    Material {name: "Water",color: [64, 164, 223, 255], update: update_water,},
];
