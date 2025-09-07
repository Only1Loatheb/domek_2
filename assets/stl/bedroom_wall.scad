$fn = 180;

wall_width = 30.;
wall_height = 26.8;
wall_thickness = 1.;

window_w = 16;
window_h = 18.;
window_y = 8.; // distance from bottom to window

module wall_with_openings() {
    difference() {
        // main wall
        cube([wall_width, wall_height, wall_thickness]);
        // window cutout
        translate([8.818, window_y, -1]) {
            cube([window_w, window_h, wall_thickness + 2]);
        }
    }
}

// Rotate to stand upright
translate([-wall_width, 0, 0]) {
    rotate([90, 0, 0]) wall_with_openings();
}
