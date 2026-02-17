mod fire;
mod gases;
mod liquids;
mod powders;
mod solids;

use crate::{Cell, SimAPI};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Material {
    Empty = 0,
    Wall = 1,
    Sand = 2,
    Water = 3,
    Stone = 4,
    Wood = 5,
    Fire = 6,
    Smoke = 7,
    Ash = 8,
    Lava = 9,
    Steam = 10,
    Obsidian = 11,
    Acid = 12,
    Ember = 13,
}

impl Material {
    #[inline]
    pub fn from_id(id: u8) -> Self {
        match id {
            1 => Material::Wall,
            2 => Material::Sand,
            3 => Material::Water,
            4 => Material::Stone,
            5 => Material::Wood,
            6 => Material::Fire,
            7 => Material::Smoke,
            8 => Material::Ash,
            9 => Material::Lava,
            10 => Material::Steam,
            11 => Material::Obsidian,
            12 => Material::Acid,
            13 => Material::Ember,
            _ => Material::Empty,
        }
    }

    #[inline]
    pub fn id(self) -> u8 {
        self as u8
    }
}

#[inline]
pub fn color_of(cell: Cell) -> [u8; 4] {
    let base: [u8; 4] = match cell.material {
        Material::Empty => return [0, 0, 0, 255],
        Material::Wall => [100, 95, 90, 255],
        Material::Stone => [110, 110, 115, 255],
        Material::Sand => [210, 185, 110, 255],
        Material::Water => [40, 110, 210, 160],
        Material::Wood => [120, 75, 30, 255],
        Material::Fire => [220, 60, 10, 255],
        Material::Smoke => [80, 80, 85, 180],
        Material::Ash => [160, 155, 145, 255],
        Material::Lava => [207, 70, 10, 255],
        Material::Steam => [200, 220, 255, 160],
        Material::Obsidian => [25, 15, 40, 255],
        Material::Acid => [3, 160, 45, 220],
        Material::Ember => [255, 160, 20, 255],
    };

    let v = cell.ra as i16 - 128;
    let (rd, gd, bd): (i16, i16, i16) = match cell.material {
        Material::Sand => (5, 7, 12),
        Material::Water => (12, 7, 4),
        Material::Stone => (7, 7, 7),
        Material::Wall => (9, 9, 9),
        Material::Wood => (8, 6, 4),
        Material::Fire => (2, 4, 20),
        Material::Smoke => (6, 6, 6),
        Material::Ash => (10, 10, 8),
        Material::Lava => (2, 6, 20),
        Material::Steam => (10, 8, 6),
        Material::Obsidian => (12, 12, 8),
        Material::Acid => (4, 12, 7),
        Material::Ember => (2, 5, 20),
        _ => (7, 7, 7),
    };

    let r = (base[0] as i16 + v / rd).clamp(0, 255) as u8;
    let g = (base[1] as i16 + v / gd).clamp(0, 255) as u8;
    let b = (base[2] as i16 + v / bd).clamp(0, 255) as u8;
    [r, g, b, base[3]]
}

#[inline]
pub fn glow_of(cell: Cell) -> [u8; 4] {
    match cell.material {
        Material::Fire => {
            let b = (cell.ra as u16 * 40 / 255) as u8;
            [255, 120 + b, 0, 200]
        }
        Material::Lava => [255, 60, 0, 160],
        Material::Acid => [0, 255, 60, 80],
        Material::Ember => [255, 140, 0, 220],
        _ => [0, 0, 0, 0],
    }
}

pub fn update_cell(cell: Cell, api: SimAPI) {
    match cell.material {
        Material::Sand | Material::Ash => powders::update_sand(cell, api),
        Material::Water => liquids::update_water(cell, api),
        Material::Lava => liquids::update_lava(cell, api),
        Material::Acid => liquids::update_acid(cell, api),
        Material::Smoke => gases::update_smoke(cell, api),
        Material::Steam => gases::update_steam(cell, api),
        Material::Fire => fire::update_fire(cell, api),
        Material::Ember => fire::update_ember(cell, api),

        Material::Stone => solids::update_stone(cell, api),
        _ => {} // Wall, Obsidian, Empty — static
    }
}
