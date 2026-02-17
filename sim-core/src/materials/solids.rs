use super::Material;
use crate::{Cell, SimAPI};

pub(super) fn update_ice(_cell: Cell, mut api: SimAPI) {
    const MELT_HEAT: u8 = 30;
    const MELT_RATE: u32 = 50; // 1-in-N chance to melt per tick when hot enough
    const FREEZE_RATE: u32 = 200; // 1-in-N chance to freeze an adjacent water cell

    if api.heat_here() > MELT_HEAT && api.rand_u32() % MELT_RATE == 0 {
        let ra = api.rand_u32() as u8;
        api.set(0, 0, Cell { material: Material::Water, ra, rb: 0, clock: 0 });
        return;
    }

    // Slowly spread ice to adjacent water
    if api.rand_u32() % FREEZE_RATE == 0 {
        for (dx, dy) in [(0i32, -1i32), (-1, 0), (1, 0), (0, 1)] {
            if api.get(dx, dy).material == Material::Water {
                let ra = api.rand_u32() as u8;
                api.set(dx, dy, Cell { material: Material::Ice, ra, rb: 0, clock: 0 });
                break;
            }
        }
    }
}

pub(super) fn update_stone(_cell: Cell, mut api: SimAPI) {
    const MELT_HEAT: u8 = 55; // needs to be basically submerged in lava to reach this
    const MELT_RATE: u32 = 300; // very rare even when hot enough

    if api.heat_here() > MELT_HEAT && api.rand_u32() % MELT_RATE == 0 {
        let ra = api.rand_u32() as u8;
        api.set(0, 0, Cell { material: Material::Lava, ra, rb: 0, clock: 0 });
    }
}
