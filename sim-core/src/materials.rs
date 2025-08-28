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

// Update functions
fn update_empty(_x: u32, _y: u32, _sim: &mut Simulation) {
    // Empty does nothing
}

fn update_sand(x: u32, y: u32, sim: &mut Simulation) {
    // check if it can move down
    let below = y + 1;
    if sim.try_move(x, y, x, below) {
        return; // successfully moved down
    }

    // check if it can move down-left
    let below_left = (x.wrapping_sub(1), y + 1); // wrapping_sub to avoid underflow
    if sim.try_move(x, y, below_left.0, below_left.1) {
        return; // successfully moved down left
    }

    // check if it can move down-right
    let below_right = (x + 1, y + 1);
    if sim.try_move(x, y, below_right.0, below_right.1) {
        return; // successfully moved down right
    }

    // if it can't move, stay in place
    sim.stay(x, y);
}

// Registry of all materials
pub static MATERIALS: &[Material] = &[
    Material {
        name: "Empty",
        color: [0, 0, 0, 255],
        update: update_empty,
    },
    Material {
        name: "Sand",
        color: [194, 178, 128, 255],
        update: update_sand,
    },
];
