use std::fs::File;
use cairo::{Context, ImageSurface, Format};

pub fn all_triangles(fname: &str) {
    let all_color = ImageSurface::create(Format::ARgb32, 421, 410);
    let one = Context::new(&all_color);
    let two = Context::new(&all_color);
    let three = Context::new(&all_color);
    let four = Context::new(&all_color);
    let five = Context::new(&all_color);
    let six = Context::new(&all_color);
    let seven = Context::new(&all_color);

    one.scale(1.0, 1.0);
    one.set_source_rgba(0.7, 0.20, 0.17, 0.8,);
    one.line_to (143.0, 30.0);
    one.line_to (198.0, 176.0);
    one.line_to (23.0, 281.0);
    one.close_path();
    one.fill_preserve ();
    one.set_source_rgba (0.98, 0.38, 0.09, 0.8);
    one.stroke ();

    two.scale(1.0, 1.0);
    two.set_source_rgba(0.0, 0.63, 0.90, 0.8);
    two.line_to (275.0, 44.0);
    two.line_to (377.0, 102.0);
    two.line_to (100.0, 118.0);
    two.close_path();
    two.fill_preserve ();
    two.set_source_rgba (0.0, 0.99, 0.93, 0.8);
    two.stroke ();

    three.scale(1.0, 1.0);
    three.set_source_rgba(0.78, 0.38, 0.09, 0.8);
    three.line_to (363.0, 190.0);
    three.line_to (250.0, 126.0);
    three.line_to (305.0, 26.0);
    three.close_path();
    three.fill_preserve ();
    three.set_source_rgba (0.98, 0.20, 0.17, 0.8);
    three.stroke ();

    four.scale(1.0, 1.0);
    four.set_source_rgba(0.19, 0.80, 0.19, 0.8);
    four.line_to (376.0, 130.0);
    four.line_to (304.0, 222.0);
    four.line_to (290.0, 147.0);
    four.close_path();
    four.fill_preserve ();
    four.set_source_rgba (0.48, 0.98, 0.0, 0.8);
    four.stroke ();

    five.scale(1.0, 1.0);
    five.set_source_rgba(0.0, 0.63, 0.90, 0.8);
    five.line_to (244.0, 314.0);
    five.line_to (148.0, 375.0);
    five.line_to (70.0, 184.0);
    five.close_path();
    five.fill_preserve ();
    five.set_source_rgba (0.0, 0.99, 0.93, 0.8);
    five.stroke ();

    six.scale(1.0, 1.0);
    six.set_source_rgba(0.78, 0.38, 0.09, 0.8);
    six.line_to (195.0, 260.0);
    six.line_to (333.0, 356.0);
    six.line_to (128.0, 330.0);
    six.close_path();
    six.fill_preserve ();
    six.set_source_rgba (0.98, 0.20, 0.17, 0.8);
    six.stroke ();

    seven.scale(1.0, 1.0);
    seven.set_source_rgba(0.19, 0.80, 0.19, 0.8);
    seven.line_to (304.0, 270.0);
    seven.line_to (311.0, 380.0);
    seven.line_to (252.0, 326.0);
    seven.close_path();
    seven.fill_preserve ();
    seven.set_source_rgba (0.48, 0.98, 0.0, 0.8);
    seven.stroke ();

    let mut file = File::create(&fname).unwrap();
    let _ = all_color.write_to_png(&mut file);
}

pub fn colored_triangle_one(fname: &str) {
    let surface = ImageSurface::create(Format::ARgb32, 421, 410);

    let one = Context::new(&surface);
        one.scale(1.0, 1.0);
        one.set_source_rgba(0.7, 0.20, 0.17, 0.8,);
        one.line_to (143.0, 30.0);
        one.line_to (198.0, 176.0);
        one.line_to (23.0, 281.0);
        one.close_path();
        one.fill_preserve ();
        one.set_source_rgba (0.98, 0.38, 0.09, 0.8);
        one.stroke ();

    let mut file = File::create(&fname).unwrap();
    let _ = surface.write_to_png(&mut file);
}

pub fn colored_triangle_two(fname: &str) {
    let surface = ImageSurface::create(Format::ARgb32, 421, 410);

    let two = Context::new(&surface);
        two.scale(1.0, 1.0);
        two.set_source_rgba(0.0, 0.63, 0.90, 0.8);
        two.line_to (275.0, 44.0);
        two.line_to (377.0, 102.0);
        two.line_to (100.0, 118.0);
        two.close_path();
        two.fill_preserve ();
        two.set_source_rgba (0.0, 0.99, 0.93, 0.8);
        two.stroke ();

    let mut file = File::create(&fname).unwrap();
    let _ = surface.write_to_png(&mut file);
}

pub fn colored_triangle_three(fname: &str) {
    let surface = ImageSurface::create(Format::ARgb32, 421, 410);

    let three = Context::new(&surface);
        three.scale(1.0, 1.0);
        three.set_source_rgba(0.78, 0.38, 0.09, 0.8);
        three.line_to (363.0, 190.0);
        three.line_to (250.0, 126.0);
        three.line_to (305.0, 26.0);
        three.close_path();
        three.fill_preserve ();
        three.set_source_rgba (0.98, 0.20, 0.17, 0.8);
        three.stroke ();

    let mut file = File::create(&fname).unwrap();
    let _ = surface.write_to_png(&mut file);
}

pub fn colored_triangle_four(fname: &str) {
    let surface = ImageSurface::create(Format::ARgb32, 421, 410);

    let four = Context::new(&surface);
        four.scale(1.0, 1.0);
        four.set_source_rgba(0.19, 0.80, 0.19, 0.8);
        four.line_to (376.0, 130.0);
        four.line_to (304.0, 222.0);
        four.line_to (290.0, 147.0);
        four.close_path();
        four.fill_preserve ();
        four.set_source_rgba (0.48, 0.98, 0.0, 0.8);
        four.stroke ();

    let mut file = File::create(&fname).unwrap();
    let _ = surface.write_to_png(&mut file);
}

pub fn colored_triangle_five(fname: &str) {
    let surface = ImageSurface::create(Format::ARgb32, 421, 410);

    let five = Context::new(&surface);
        five.scale(1.0, 1.0);
        five.set_source_rgba(0.0, 0.63, 0.90, 0.8);
        five.line_to (244.0, 314.0);
        five.line_to (148.0, 375.0);
        five.line_to (70.0, 184.0);
        five.close_path();
        five.fill_preserve ();
        five.set_source_rgba (0.0, 0.99, 0.93, 0.8);
        five.stroke ();

    let mut file = File::create(&fname).unwrap();
    let _ = surface.write_to_png(&mut file);
}

pub fn colored_triangle_six(fname: &str) {
    let surface = ImageSurface::create(Format::ARgb32, 421, 410);

    let six = Context::new(&surface);
        six.scale(1.0, 1.0);
        six.set_source_rgba(0.78, 0.38, 0.09, 0.8);
        six.line_to (195.0, 260.0);
        six.line_to (333.0, 356.0);
        six.line_to (128.0, 330.0);
        six.close_path();
        six.fill_preserve ();
        six.set_source_rgba (0.98, 0.20, 0.17, 0.8);
        six.stroke ();

    let mut file = File::create(&fname).unwrap();
    let _ = surface.write_to_png(&mut file);
}

pub fn colored_triangle_seven(fname: &str) {
    let surface = ImageSurface::create(Format::ARgb32, 421, 410);

    let seven = Context::new(&surface);
        seven.scale(1.0, 1.0);
        seven.set_source_rgba(0.19, 0.80, 0.19, 0.8);
        seven.line_to (304.0, 270.0);
        seven.line_to (311.0, 380.0);
        seven.line_to (252.0, 326.0);
        seven.close_path();
        seven.fill_preserve ();
        seven.set_source_rgba (0.48, 0.98, 0.0, 0.8);
        seven.stroke ();

    let mut file = File::create(&fname).unwrap();
    let _ = surface.write_to_png(&mut file);
}
