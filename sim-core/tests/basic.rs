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
