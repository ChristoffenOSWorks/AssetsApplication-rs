use triangles::all_colored::*;
use mkdir;

pub fn colored_no_logo() {
        let _ = mkdir("AppContent/Logo/ColoredNoLogo");
        colored_triangle_one("AppContent/Logo/ColoredNoLogo/triangle1-color.png");
        colored_triangle_two("AppContent/Logo/ColoredNoLogo/triangle2-color.png");
        colored_triangle_three("AppContent/Logo/ColoredNoLogo/triangle3-color.png");
        colored_triangle_four("AppContent/Logo/ColoredNoLogo/triangle4-color.png");
        colored_triangle_five("AppContent/Logo/ColoredNoLogo/triangle5-color.png");
        colored_triangle_six("AppContent/Logo/ColoredNoLogo/triangle6-color.png");
        colored_triangle_seven("AppContent/Logo/ColoredNoLogo/triangle7-color.png");
}
