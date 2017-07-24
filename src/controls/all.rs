use triangles::all_colored::*;
use triangles::all_outlined::*;
use mkdir;

pub fn all() {
    let _ = mkdir("AppContent/Logo/All");
    all_triangles("AppContent/Logo/All/crstfn-color.png");
    colored_triangle_one("AppContent/Logo/All/triangle1-color.png");
    colored_triangle_two("AppContent/Logo/All/triangle2-color.png");
    colored_triangle_three("AppContent/Logo/All/triangle3-color.png");
    colored_triangle_four("AppContent/Logo/All/triangle4-color.png");
    colored_triangle_five("AppContent/Logo/All/triangle5-color.png");
    colored_triangle_six("AppContent/Logo/All/triangle6-color.png");
    colored_triangle_seven("AppContent/Logo/All/triangle7-color.png");

    all_outlines("AppContent/Logo/All/crstfn-outline.png");
    outline_one("AppContent/Logo/All/triangle1-outline.png");
    outline_two("AppContent/Logo/All/triangle2-outline.png");
    outline_three("AppContent/Logo/All/triangle3-outline.png");
    outline_four("AppContent/Logo/All/triangle4-outline.png");
    outline_five("AppContent/Logo/All/triangle5-outline.png");
    outline_six("AppContent/Logo/All/triangle6-outline.png");
    outline_seven("AppContent/Logo/All/triangle7-outline.png");
}
