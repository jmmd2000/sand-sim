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
    Oil = 14,
    Ice = 15,
    Gunpowder = 16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Group {
    Empty,
    Powder,
    Liquid,
    Solid,
    Gas,
    Fire,
}

#[derive(Clone, Copy)]
pub struct MaterialProps {
    pub group: Group,
    pub colour: [u8; 4],
    pub colour_variation: [i16; 3],
    pub glow: [u8; 4],
    pub gravity: i8,
    pub terminal_velocity: i8,
    pub slide_acceleration: i8,
    pub air_drag: f32,
    pub base_friction: f32,
    pub surface_slipperiness: f32,
}

const PROPS: [MaterialProps; 17] = [
    // empty
    MaterialProps {
        group: Group::Empty,
        colour: [0, 0, 0, 255],
        colour_variation: [7, 7, 7],
        glow: [0, 0, 0, 0],
        gravity: 0,
        terminal_velocity: 0,
        slide_acceleration: 0,
        air_drag: 1.0,
        base_friction: 1.0,
        surface_slipperiness: 1.0,
    },
    // wall
    MaterialProps {
        group: Group::Solid,
        colour: [100, 95, 90, 255],
        colour_variation: [9, 9, 9],
        glow: [0, 0, 0, 0],
        gravity: 0,
        terminal_velocity: 0,
        slide_acceleration: 0,
        air_drag: 1.0,
        base_friction: 1.0,
        surface_slipperiness: 0.85,
    },
    // sand
    MaterialProps {
        group: Group::Powder,
        colour: [210, 185, 110, 255],
        colour_variation: [5, 7, 12],
        glow: [0, 0, 0, 0],
        gravity: 1,
        terminal_velocity: 10,
        slide_acceleration: 1,
        air_drag: 0.95,
        base_friction: 0.85,
        surface_slipperiness: 0.82,
    },
    // water
    MaterialProps {
        group: Group::Liquid,
        colour: [40, 110, 210, 160],
        colour_variation: [12, 7, 4],
        glow: [0, 0, 0, 0],
        gravity: 1,
        terminal_velocity: 8,
        slide_acceleration: 3,
        air_drag: 0.95,
        base_friction: 0.95,
        surface_slipperiness: 0.96,
    },
    // stone
    MaterialProps {
        group: Group::Solid,
        colour: [110, 110, 115, 255],
        colour_variation: [7, 7, 7],
        glow: [0, 0, 0, 0],
        gravity: 0,
        terminal_velocity: 0,
        slide_acceleration: 0,
        air_drag: 1.0,
        base_friction: 1.0,
        surface_slipperiness: 0.85,
    },
    // wood
    MaterialProps {
        group: Group::Solid,
        colour: [120, 75, 30, 255],
        colour_variation: [8, 6, 4],
        glow: [0, 0, 0, 0],
        gravity: 0,
        terminal_velocity: 0,
        slide_acceleration: 0,
        air_drag: 1.0,
        base_friction: 1.0,
        surface_slipperiness: 0.82,
    },
    // fire
    MaterialProps {
        group: Group::Fire,
        colour: [220, 60, 10, 255],
        colour_variation: [2, 4, 20],
        glow: [255, 120, 0, 200],
        gravity: 0,
        terminal_velocity: 4,
        slide_acceleration: 0,
        air_drag: 0.90,
        base_friction: 1.0,
        surface_slipperiness: 1.0,
    },
    // smoke
    MaterialProps {
        group: Group::Gas,
        colour: [80, 80, 85, 180],
        colour_variation: [6, 6, 6],
        glow: [0, 0, 0, 0],
        gravity: -1,
        terminal_velocity: 1,
        slide_acceleration: 0,
        air_drag: 0.90,
        base_friction: 1.0,
        surface_slipperiness: 1.0,
    },
    // ash
    MaterialProps {
        group: Group::Powder,
        colour: [160, 155, 145, 255],
        colour_variation: [10, 10, 8],
        glow: [0, 0, 0, 0],
        gravity: 1,
        terminal_velocity: 5,
        slide_acceleration: 1,
        air_drag: 0.95,
        base_friction: 0.88,
        surface_slipperiness: 0.80,
    },
    // lava
    MaterialProps {
        group: Group::Liquid,
        colour: [207, 70, 10, 255],
        colour_variation: [2, 6, 20],
        glow: [255, 60, 0, 160],
        gravity: 1,
        terminal_velocity: 4,
        slide_acceleration: 1,
        air_drag: 0.95,
        base_friction: 0.75,
        surface_slipperiness: 0.70,
    },
    // steam
    MaterialProps {
        group: Group::Gas,
        colour: [200, 220, 255, 160],
        colour_variation: [10, 8, 6],
        glow: [0, 0, 0, 0],
        gravity: -1,
        terminal_velocity: 1,
        slide_acceleration: 0,
        air_drag: 0.92,
        base_friction: 1.0,
        surface_slipperiness: 1.0,
    },
    // obsidian
    MaterialProps {
        group: Group::Solid,
        colour: [25, 15, 40, 255],
        colour_variation: [12, 12, 8],
        glow: [0, 0, 0, 0],
        gravity: 0,
        terminal_velocity: 0,
        slide_acceleration: 0,
        air_drag: 1.0,
        base_friction: 1.0,
        surface_slipperiness: 0.80,
    },
    // acid
    MaterialProps {
        group: Group::Liquid,
        colour: [3, 160, 45, 220],
        colour_variation: [4, 12, 7],
        glow: [0, 255, 60, 80],
        gravity: 1,
        terminal_velocity: 6,
        slide_acceleration: 2,
        air_drag: 0.95,
        base_friction: 0.90,
        surface_slipperiness: 0.92,
    },
    // ember
    MaterialProps {
        group: Group::Fire,
        colour: [255, 160, 20, 255],
        colour_variation: [2, 5, 20],
        glow: [255, 140, 0, 220],
        gravity: -1,
        terminal_velocity: 1,
        slide_acceleration: 0,
        air_drag: 0.88,
        base_friction: 1.0,
        surface_slipperiness: 1.0,
    },
    // oil
    MaterialProps {
        group: Group::Liquid,
        colour: [22, 20, 16, 255],
        colour_variation: [8, 6, 4],
        glow: [0, 0, 0, 0],
        gravity: 1,
        terminal_velocity: 6,
        slide_acceleration: 2,
        air_drag: 0.95,
        base_friction: 0.96,
        surface_slipperiness: 0.95,
    },
    // ice
    MaterialProps {
        group: Group::Solid,
        colour: [160, 216, 240, 255],
        colour_variation: [12, 10, 8],
        glow: [0, 0, 0, 0],
        gravity: 0,
        terminal_velocity: 0,
        slide_acceleration: 0,
        air_drag: 1.0,
        base_friction: 1.0,
        surface_slipperiness: 0.98,
    },
    // gunpowder
    MaterialProps {
        group: Group::Powder,
        colour: [60, 55, 50, 255],
        colour_variation: [8, 8, 8],
        glow: [0, 0, 0, 0],
        gravity: 1,
        terminal_velocity: 10,
        slide_acceleration: 1,
        air_drag: 0.95,
        base_friction: 0.85,
        surface_slipperiness: 0.82,
    },
];

#[inline]
pub fn props(mat: Material) -> &'static MaterialProps {
    &PROPS[mat as usize]
}

impl Material {
    #[inline]
    pub fn from_id(id: u8) -> Self {
        if (id as usize) < PROPS.len() { unsafe { std::mem::transmute(id) } } else { Material::Empty }
    }

    #[inline]
    pub fn id(self) -> u8 {
        self as u8
    }
}

#[inline]
pub fn color_of(cell: Cell) -> [u8; 4] {
    let p = props(cell.material);
    if cell.material == Material::Empty {
        return p.colour;
    }

    let v = cell.ra as i16 - 128;
    let r = (p.colour[0] as i16 + v / p.colour_variation[0]).clamp(0, 255) as u8;
    let g = (p.colour[1] as i16 + v / p.colour_variation[1]).clamp(0, 255) as u8;
    let b = (p.colour[2] as i16 + v / p.colour_variation[2]).clamp(0, 255) as u8;
    [r, g, b, p.colour[3]]
}

#[inline]
pub fn glow_of(cell: Cell) -> [u8; 4] {
    props(cell.material).glow
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
        Material::Ice => solids::update_ice(cell, api),
        Material::Oil => liquids::update_oil(cell, api),
        Material::Gunpowder => powders::update_gunpowder(cell, api),
        _ => {} // Wall, Obsidian, Empty — static
    }
}
