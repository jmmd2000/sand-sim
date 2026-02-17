use super::Material;
use crate::{Cell, SimAPI};

pub(super) fn update_stone(_cell: Cell, mut api: SimAPI) {
    const MELT_HEAT: u8 = 55; // needs to be basically submerged in lava to reach this
    const MELT_RATE: u32 = 300; // very rare even when hot enough

    if api.heat_here() > MELT_HEAT && api.rand_u32() % MELT_RATE == 0 {
        let ra = api.rand_u32() as u8;
        api.set(0, 0, Cell { material: Material::Lava, ra, rb: 0, clock: 0 });
    }
}
