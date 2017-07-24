use std::fs::File;
use cairo::{Context, ImageSurface, Format};

pub fn all_outlines(fname: &str) {
    let outline_all = ImageSurface::create(Format::ARgb32, 421, 410);
    let one = Context::new(&outline_all);

    one.scale (10.0, 10.0); //1
    one.move_to (2.3, 28.1);
    one.line_to (14.3, 3.0);
    one.move_to (14.3, 3.0);
    one.line_to (19.8, 17.6);
    one.move_to (19.8, 17.6);
    one.line_to (2.3, 28.1);

    one.move_to (10.0, 11.8); //2
    one.line_to (27.5, 4.4);
    one.move_to (27.5, 4.4);
    one.line_to (37.7, 10.2);
    one.move_to (37.7, 10.2);
    one.line_to (10.0, 11.8);

    one.move_to (36.3, 19.0); //3
    one.line_to (25.0, 12.6);
    one.move_to (25.0, 12.6);
	one.line_to (30.5, 2.6);
	one.move_to (30.5, 2.6);
	one.line_to (36.3, 19.0);

    one.move_to (29.0, 14.7); //4
    one.line_to (37.6, 13.0);
    one.move_to (37.6, 13.0);
    one.line_to (30.4, 22.2);
    one.move_to (30.4, 22.2);
    one.line_to (29.0, 14.7);

    one.move_to (7.0, 18.4); //5
    one.line_to (24.4, 31.4);
    one.move_to (24.4, 31.4);
    one.line_to (14.8, 37.5);
    one.move_to (14.8, 37.5);
    one.line_to (7.0, 18.4);

    one.move_to (12.8, 33.0); //6
    one.line_to (19.5, 26.0);
    one.move_to (19.5, 26.0);
    one.line_to (33.3, 35.6);
    one.move_to (33.3, 35.6);
    one.line_to (12.8, 33.0);

    one.move_to (25.2, 32.6); //7
    one.line_to (30.4, 27.0);
    one.move_to (30.4, 27.0);
    one.line_to (31.1, 38.0);
    one.move_to (31.1, 38.0);
    one.line_to (25.2, 32.6);

    one.set_line_width (0.1);
    one.stroke ();

    let mut file = File::create(&fname).unwrap();
    let _ = outline_all.write_to_png(&mut file);
}

pub fn outline_one(fname: &str) {
    let outline = ImageSurface::create(Format::ARgb32, 421, 410);
    let one = Context::new(&outline);
    one.scale (10.0, 10.0);
    
    one.move_to (2.3, 28.1);
    one.line_to (14.3, 3.0);
    one.move_to (14.3, 3.0);
    one.line_to (19.8, 17.6);
    one.move_to (19.8, 17.6);
    one.line_to (2.3, 28.1);

    one.set_line_width (0.1);
    one.stroke ();

    let mut file = File::create(&fname).unwrap();
    let _ = outline.write_to_png(&mut file);
}

pub fn outline_two(fname: &str) {
    let outline = ImageSurface::create(Format::ARgb32, 421, 410);
    let one = Context::new(&outline);
    one.scale (10.0, 10.0);

    one.move_to (10.0, 11.8);
    one.line_to (27.5, 4.4);
    one.move_to (27.5, 4.4);
    one.line_to (37.7, 10.2);
    one.move_to (37.7, 10.2);
    one.line_to (10.0, 11.8);

    one.set_line_width (0.1);
    one.stroke ();

    let mut file = File::create(&fname).unwrap();
    let _ = outline.write_to_png(&mut file);
}

pub fn outline_three(fname: &str) {
    let outline = ImageSurface::create(Format::ARgb32, 421, 410);
    let one = Context::new(&outline);
    one.scale (10.0, 10.0);

    one.move_to (36.3, 19.0);
    one.line_to (25.0, 12.6);
    one.move_to (25.0, 12.6);
	one.line_to (30.5, 2.6);
	one.move_to (30.5, 2.6);
	one.line_to (36.3, 19.0);

    one.set_line_width (0.1);
    one.stroke ();

    let mut file = File::create(&fname).unwrap();
    let _ = outline.write_to_png(&mut file);
}

pub fn outline_four(fname: &str) {
    let outline = ImageSurface::create(Format::ARgb32, 421, 410);
    let one = Context::new(&outline);
    one.scale (10.0, 10.0);

    one.move_to (29.0, 14.7); //4
    one.line_to (37.6, 13.0);
    one.move_to (37.6, 13.0);
    one.line_to (30.4, 22.2);
    one.move_to (30.4, 22.2);
    one.line_to (29.0, 14.7);

    one.set_line_width (0.1);
    one.stroke ();

    let mut file = File::create(&fname).unwrap();
    let _ = outline.write_to_png(&mut file);
}

pub fn outline_five(fname: &str) {
    let outline = ImageSurface::create(Format::ARgb32, 421, 410);
    let one = Context::new(&outline);
    one.scale (10.0, 10.0);

    one.move_to (7.0, 18.4); //5
    one.line_to (24.4, 31.4);
    one.move_to (24.4, 31.4);
    one.line_to (14.8, 37.5);
    one.move_to (14.8, 37.5);
    one.line_to (7.0, 18.4);

    one.set_line_width (0.1);
    one.stroke ();

    let mut file = File::create(&fname).unwrap();
    let _ = outline.write_to_png(&mut file);
}

pub fn outline_six(fname: &str) {
    let outline = ImageSurface::create(Format::ARgb32, 421, 410);
    let one = Context::new(&outline);
    one.scale (10.0, 10.0);

    one.move_to (12.8, 33.0); //6
    one.line_to (19.5, 26.0);
    one.move_to (19.5, 26.0);
    one.line_to (33.3, 35.6);
    one.move_to (33.3, 35.6);
    one.line_to (12.8, 33.0);

    one.set_line_width (0.1);
    one.stroke ();

    let mut file = File::create(&fname).unwrap();
    let _ = outline.write_to_png(&mut file);
}

pub fn outline_seven(fname: &str) {
    let outline = ImageSurface::create(Format::ARgb32, 421, 410);
    let one = Context::new(&outline);
    one.scale (10.0, 10.0);

    one.move_to (25.2, 32.6); //7
    one.line_to (30.4, 27.0);
    one.move_to (30.4, 27.0);
    one.line_to (31.1, 38.0);
    one.move_to (31.1, 38.0);
    one.line_to (25.2, 32.6);

    one.set_line_width (0.1);
    one.stroke ();

    let mut file = File::create(&fname).unwrap();
    let _ = outline.write_to_png(&mut file);
}
