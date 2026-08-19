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

#[wasm_bindgen_test]
fn velocity_props_populated() {
    use sim_core::{Material, props};

    let sand = props(Material::Sand);
    assert_eq!(sand.gravity, 1);
    assert_eq!(sand.terminal_velocity, 10);
    assert_eq!(sand.slide_acceleration, 1);

    let smoke = props(Material::Smoke);
    assert_eq!(smoke.gravity, -1);
    assert_eq!(smoke.terminal_velocity, 1);

    let stone = props(Material::Stone);
    assert_eq!(stone.gravity, 0);
    assert_eq!(stone.terminal_velocity, 0);

    let ice = props(Material::Ice);
    assert_eq!(ice.surface_slipperiness, 0.98);
}

#[wasm_bindgen_test]
fn gravity_direction_correct() {
    use sim_core::{Material, props};

    let falling = [Material::Sand, Material::Water, Material::Ash, Material::Lava, Material::Acid, Material::Oil, Material::Gunpowder];
    let rising = [Material::Smoke, Material::Steam, Material::Ember];
    let stationary = [Material::Empty, Material::Wall, Material::Stone, Material::Wood, Material::Obsidian, Material::Ice, Material::Fire];

    for mat in falling {
        assert_eq!(props(mat).gravity, 1, "{:?} should fall", mat);
    }
    for mat in rising {
        assert_eq!(props(mat).gravity, -1, "{:?} should rise", mat);
    }
    for mat in stationary {
        assert_eq!(props(mat).gravity, 0, "{:?} should be stationary", mat);
    }
}

#[wasm_bindgen_test]
fn sand_preserved_after_falling() {
    let mut sim = Simulation::new(20, 40);
    sim.paint_circle(10, 5, 2, 2);
    let initial = sim.count_mat(2);
    assert!(initial > 0);
    sim.step(30);
    assert_eq!(sim.count_mat(2), initial);
}

#[wasm_bindgen_test]
fn water_preserved_after_falling() {
    let mut sim = Simulation::new(20, 40);
    sim.paint_circle(10, 5, 2, 3);
    let initial = sim.count_mat(3);
    assert!(initial > 0);
    sim.step(30);
    assert_eq!(sim.count_mat(3), initial);
}

#[wasm_bindgen_test]
fn smoke_fades_over_time() {
    let mut sim = Simulation::new(20, 20);
    sim.paint_circle(10, 10, 3, 7);
    let initial = sim.count_mat(7);
    assert!(initial > 0);
    sim.step(200);
    assert_eq!(sim.count_mat(7), 0);
}
