mod metal;
use crate::units::{self, CaseDesign};
use svg::node::element::{path, Path};

const CUT: &str = "cut";
const FOLD: &str = "fold";

const BASE_LENGTH: f64 = 20.0;

pub fn generate(design: &CaseDesign) -> svg::Document {
    match design.material {
        units::CaseMaterial::SheetMetal => metal::generate(design),
        units::CaseMaterial::Wood => todo!(),
    }
}

pub fn get_cut_path() -> Path {
    Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        //.set("stroke-width", 1)
}

pub fn get_fold_path() -> Path {
    Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        //.set("stroke-width", 1)
        .set("stroke-dasharray", 4)
}
