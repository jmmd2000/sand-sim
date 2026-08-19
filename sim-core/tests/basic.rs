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

    let cell = Cell { material: Material::Sand, ra: 128, rb: 0, clock: 0 };
    let c = color_of(cell);
    assert_eq!(c, [210, 185, 110, 255]);

    let cell = Cell { material: Material::Empty, ra: 128, rb: 0, clock: 0 };
    let c = color_of(cell);
    assert_eq!(c, [0, 0, 0, 255]);
}

#[wasm_bindgen_test]
fn glow_of_matches_expected() {
    use sim_core::{Cell, Material, glow_of};

    let cell = Cell { material: Material::Lava, ra: 128, rb: 0, clock: 0 };
    assert_eq!(glow_of(cell), [255, 60, 0, 160]);

    let cell = Cell { material: Material::Sand, ra: 128, rb: 0, clock: 0 };
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
