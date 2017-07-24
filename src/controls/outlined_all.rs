use triangles::all_outlined::*;
use mkdir;

pub fn outlined_all() {
    let _ = mkdir("AppContent/Logo/OutlinedAll");
    all_outlines("AppContent/Logo/OutlinedAll/crstfn-outline.png");
    outline_one("AppContent/Logo/OutlinedAll/triangle1-outline.png");
    outline_two("AppContent/Logo/OutlinedAll/triangle2-outline.png");
    outline_three("AppContent/Logo/OutlinedAll/triangle3-outline.png");
    outline_four("AppContent/Logo/OutlinedAll/triangle4-outline.png");
    outline_five("AppContent/Logo/OutlinedAll/triangle5-outline.png");
    outline_six("AppContent/Logo/OutlinedAll/triangle6-outline.png");
    outline_seven("AppContent/Logo/OutlinedAll/triangle7-outline.png");
}

pub fn outlined_no_logo() {
    let _ = mkdir("AppContent/Logo/OutlinedNoLogo");
    outline_one("AppContent/Logo/OutlinedNoLogo/triangle1-outline.png");
    outline_two("AppContent/Logo/OutlinedNoLogo/triangle2-outline.png");
    outline_three("AppContent/Logo/OutlinedNoLogo/triangle3-outline.png");
    outline_four("AppContent/Logo/OutlinedNoLogo/triangle4-outline.png");
    outline_five("AppContent/Logo/OutlinedNoLogo/triangle5-outline.png");
    outline_six("AppContent/Logo/OutlinedNoLogo/triangle6-outline.png");
    outline_seven("AppContent/Logo/OutlinedNoLogo/triangle7-outline.png");
}
