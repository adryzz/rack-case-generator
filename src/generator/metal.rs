use svg::node::element::{path, Group, Path};

use crate::units::{self, CaseDesign, CaseHeight, CaseWidth};

use super::{get_cut_path, get_fold_path, BASE_LENGTH};

pub fn generate(design: &CaseDesign) -> svg::Document {

    let height = units::get_height(design.height);
    let width = units::get_width(design.width);
    let depth = u16::from(design.depth) as f64;

    let body = main_body(design);

    let mut translate = 2.0*(BASE_LENGTH) + depth + height;

    let top = top_cover(design)
    .set("transform", 
    format!("translate({}, {})", height+BASE_LENGTH, translate));

    let ears = ears(design.height).set("transform", 
    format!("translate({}, {})", width + 2.0*(height+BASE_LENGTH), translate + BASE_LENGTH));

    translate += depth + (2.0*BASE_LENGTH);

    let front = front_plate(design)
    .set("transform", 
    format!("translate({}, {})", height, translate));
    
    translate += height + 4.0*BASE_LENGTH;

    let viewboxw = 2.0*height + 8.0*BASE_LENGTH + width;

    svg::Document::new()
        .set("filter", "invert(100%)")
        .set("viewBox", (0, 0, viewboxw, translate))
        .add(body)
        .add(top)
        .add(front)
        .add(ears)
}

fn main_body(design: &CaseDesign) -> Group {

    let height = units::get_height(design.height);
    let width = units::get_width(design.width);
    let depth = u16::from(design.depth) as f64;

    let cut = path::Data::new()
    .move_by((BASE_LENGTH, BASE_LENGTH))

    .move_by((BASE_LENGTH, BASE_LENGTH))
    .move_by((height, height))

    .horizontal_line_by(-(height + BASE_LENGTH))
    .vertical_line_by(depth)

    .horizontal_line_by(BASE_LENGTH)
    .vertical_line_by(-BASE_LENGTH)
    .move_by((0, BASE_LENGTH))
    
    .horizontal_line_by(height)
    .vertical_line_by(-BASE_LENGTH)
    .move_by((0, BASE_LENGTH))

    .horizontal_line_by(width)

    .vertical_line_by(-BASE_LENGTH)
    .move_by((0, BASE_LENGTH))
    .horizontal_line_by(height)

    .vertical_line_by(-BASE_LENGTH)
    .move_by((0, BASE_LENGTH))
    .horizontal_line_by(BASE_LENGTH)

    .vertical_line_by(-depth)
    .horizontal_line_by(-(BASE_LENGTH + height))
    .vertical_line_by(-(height + BASE_LENGTH))

    .horizontal_line_by(-width)

    .vertical_line_by(BASE_LENGTH + height)

    .move_by((-BASE_LENGTH, -BASE_LENGTH))
    .move_by((-height, -height))
    .close();

    let fold = path::Data::new()
    .move_by((BASE_LENGTH, BASE_LENGTH))

    .move_by((BASE_LENGTH, BASE_LENGTH))
    .move_by((height, 0))

    .horizontal_line_by(width)
    .move_by((0, height))
    .horizontal_line_by(-width)

    .vertical_line_by(depth-BASE_LENGTH)
    .move_to((BASE_LENGTH, BASE_LENGTH))
    

    //.vertical_line_by(-(depth-BASE_LENGTH))

    .close();

    Group::new()
    .add(get_cut_path().set("d", cut))
    .add(get_fold_path().set("d", fold))
}

fn top_cover(design: &CaseDesign) -> Group {

    let width = units::get_width(design.width);
    let depth = u16::from(design.depth) as f64;

    let cut = path::Data::new()
    .move_by((BASE_LENGTH, BASE_LENGTH))

    .horizontal_line_by(width)
    .vertical_line_by(depth)
    .horizontal_line_by(-width)
    .vertical_line_by(-depth)

    .close();

    Group::new()
    .add(get_cut_path().set("d", cut))
}

fn front_plate(design: &CaseDesign) -> Group {

    let height = units::get_height(design.height);
    let width = units::get_width(design.width);

    let cut = path::Data::new()
    .move_by((BASE_LENGTH, BASE_LENGTH))
    .move_by((BASE_LENGTH, BASE_LENGTH))

    .horizontal_line_by(-BASE_LENGTH)
    .vertical_line_by(height)
    .horizontal_line_by(BASE_LENGTH)

    .vertical_line_by(BASE_LENGTH)
    .horizontal_line_by(width)
    .vertical_line_by(-BASE_LENGTH)

    .horizontal_line_by(BASE_LENGTH)
    .vertical_line_by(-height)
    .horizontal_line_by(-BASE_LENGTH)

    .vertical_line_by(-BASE_LENGTH)
    .horizontal_line_by(-width)
    .vertical_line_by(BASE_LENGTH)

    .close();

    let fold = path::Data::new()
    .move_by((BASE_LENGTH, BASE_LENGTH))
    .move_by((BASE_LENGTH, BASE_LENGTH))
    .horizontal_line_by(width)
    .vertical_line_by(height)
    .horizontal_line_by(-width)
    .vertical_line_by(-height)
    .close();

    Group::new()
    .add(get_cut_path().set("d", cut))
    .add(get_fold_path().set("d", fold))
}

fn ears(height: CaseHeight) -> Group {
    let ear = ear(height);
    Group::new().add(ear.clone())
    .add(ear.set("transform", 
    format!("translate(0, {})", EAR_LENGTH + BASE_LENGTH)))
}

/// full length of an ear
const EAR_LENGTH: f64 = 60.0;
/// length at which to fold an ear
const EAR_FOLD: f64 = 20.0;

fn ear(height: CaseHeight) -> Group {
    let height = units::get_height(height);
    let cut = path::Data::new()
        .move_by((BASE_LENGTH, BASE_LENGTH))
        .vertical_line_by(EAR_LENGTH)
        .horizontal_line_by(height)
        .vertical_line_by(-EAR_LENGTH)
        .horizontal_line_by(-height)
        .close();


    let fold = path::Data::new()
        .move_by((BASE_LENGTH, BASE_LENGTH+EAR_FOLD))
        .horizontal_line_by(height)
        .close();

    Group::new().add(get_cut_path().set("d", cut))
    .add(get_fold_path().set("d", fold))
}
