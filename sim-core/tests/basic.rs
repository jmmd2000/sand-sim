use sim_core::Simulation;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn creates_simulation() {
    let sim = Simulation::new(10, 10);
    assert_eq!(sim.width(), 10);
    assert_eq!(sim.height(), 10);
}

#[wasm_bindgen_test]
fn step_does_not_panic() {
    let mut sim = Simulation::new(10, 10);
    sim.step(1);
}

#[wasm_bindgen_test]
fn paint_and_count() {
    let mut sim = Simulation::new(10, 10);
    sim.paint_circle(10, 10, 3, 2);
    assert!(sim.count_mat(2) > 0);
}

#[wasm_bindgen_test]
fn props_array_covers_all_materials() {
    for id in 0..17u8 {
        let mat = sim_core::Material::from_id(id);
        let p = sim_core::props(mat);
        if id == 0 {
            assert_eq!(p.group, sim_core::Group::Empty);
        } else {
            assert_ne!(p.group, sim_core::Group::Empty);
        }
    }
}

#[wasm_bindgen_test]
fn color_of_matches_expected() {
    use sim_core::{Cell, Material, color_of};

    let cell = Cell { material: Material::Sand, ra: 128, rb: 0, clock: 0, vx: 0, vy: 0 };
    let c = color_of(cell);
    assert_eq!(c, [210, 185, 110, 255]);

    let cell = Cell { material: Material::Empty, ra: 128, rb: 0, clock: 0, vx: 0, vy: 0 };
    let c = color_of(cell);
    assert_eq!(c, [0, 0, 0, 255]);
}

#[wasm_bindgen_test]
fn glow_of_matches_expected() {
    use sim_core::{Cell, Material, glow_of};

    let cell = Cell { material: Material::Lava, ra: 128, rb: 0, clock: 0, vx: 0, vy: 0 };
    assert_eq!(glow_of(cell), [255, 60, 0, 160]);

    let cell = Cell { material: Material::Sand, ra: 128, rb: 0, clock: 0, vx: 0, vy: 0 };
    assert_eq!(glow_of(cell), [0, 0, 0, 0]);
}

#[wasm_bindgen_test]
fn group_check_covers_solids_and_powders() {
    use sim_core::{Group, Material, props};

    let solids = [Material::Wall, Material::Stone, Material::Wood, Material::Obsidian, Material::Ice];
    let powders = [Material::Sand, Material::Ash, Material::Gunpowder];

    for mat in solids {
        assert_eq!(props(mat).group, Group::Solid);
    }
    for mat in powders {
        assert_eq!(props(mat).group, Group::Powder);
    }
}

#[wasm_bindgen_test]
fn from_id_round_trips() {
    use sim_core::Material;

    for id in 0..17u8 {
        assert_eq!(Material::from_id(id).id(), id);
    }
    assert_eq!(Material::from_id(255).id(), 0);
}

#[wasm_bindgen_test]
fn cell_velocity_defaults_to_zero() {
    use sim_core::{Cell, Material};

    let cell = Cell { material: Material::Sand, ra: 128, rb: 0, clock: 0, vx: 0, vy: 0 };
    assert_eq!(cell.vx, 0);
    assert_eq!(cell.vy, 0);
}

#[wasm_bindgen_test]
fn cell_velocity_can_be_set() {
    use sim_core::{Cell, Material};

    let cell = Cell { material: Material::Sand, ra: 0, rb: 0, clock: 0, vx: -3, vy: 5 };
    assert_eq!(cell.vx, -3);
    assert_eq!(cell.vy, 5);
}

#[wasm_bindgen_test]
fn cell_spread_preserves_velocity() {
    use sim_core::{Cell, Material};

    let cell = Cell { material: Material::Sand, ra: 0, rb: 0, clock: 0, vx: 2, vy: -4 };
    let updated = Cell { rb: 10, ..cell };
    assert_eq!(updated.vx, 2);
    assert_eq!(updated.vy, -4);
    assert_eq!(updated.rb, 10);
}
