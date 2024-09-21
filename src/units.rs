use std::num::{NonZeroU16, NonZeroU8};

pub const RACK_UNIT: f64 = 44.45;
pub const RACK_UNIT_TOLERANCE: f64 = 0.794;

pub const RACK_FULL_WIDTH: f64 = 482.6;
pub const RACK_HALF_WIDTH: f64 = 241.30;

pub const fn get_height(height: CaseHeight) -> f64 {
    (RACK_UNIT * (height as u8) as f64) - RACK_UNIT_TOLERANCE
}

pub const fn get_width(width: CaseWidth) -> f64 {
    match width {
        CaseWidth::Full => RACK_FULL_WIDTH,
        CaseWidth::Half => RACK_HALF_WIDTH,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CaseDesign {
    pub material: CaseMaterial,
    pub thickness: NonZeroU8,
    pub height: CaseHeight,
    pub width: CaseWidth,
    pub depth: NonZeroU16, // mm
    pub ears: bool,
}

impl Default for CaseDesign {
    fn default() -> Self {
        Self {
            material: Default::default(),
            thickness: NonZeroU8::new(2).unwrap(),
            height: Default::default(),
            width: Default::default(),
            depth: NonZeroU16::new(300).unwrap(),
            ears: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum CaseMaterial {
    #[default]
    SheetMetal,
    Wood,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// Case height in rack units
pub enum CaseHeight {
    #[default]
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum CaseWidth {
    #[default]
    Full,
    Half,
}
