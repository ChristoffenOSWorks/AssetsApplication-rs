use triangles::all_colored::*;
use mkdir;

pub fn all_colored() {
    let _ = mkdir("AppContent/Logo/AllColors");
    all_triangles("AppContent/Logo/AllColors/crstfn-color.png");
    colored_triangle_one("AppContent/Logo/AllColors/triangle1-color.png");
    colored_triangle_two("AppContent/Logo/AllColors/triangle2-color.png");
    colored_triangle_three("AppContent/Logo/AllColors/triangle3-color.png");
    colored_triangle_four("AppContent/Logo/AllColors/triangle4-color.png");
    colored_triangle_five("AppContent/Logo/AllColors/triangle5-color.png");
    colored_triangle_six("AppContent/Logo/AllColors/triangle6-color.png");
    colored_triangle_seven("AppContent/Logo/AllColors/triangle7-color.png");
}
