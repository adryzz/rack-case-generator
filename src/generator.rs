use crate::units::{self, CaseDesign};

pub fn generate(design: &CaseDesign) -> svg::Document {
    
    match design.material {
        units::CaseMaterial::SheetMetal => sheetmetal_generate(design),
        units::CaseMaterial::Wood => todo!(),
    }
}

pub fn sheetmetal_generate(design: &CaseDesign) -> svg::Document {
    svg::Document::new()
}