$fn = 180;

wall_width = 52.33;
wall_height = 26.8;
wall_thickness = 1.;

window_w = 9.;
window_h = 15.;
window_y = 8.; // distance from bottom to window
window_x = 8.783;

door_w = 18;
door_h = 24;

module wall_with_openings() {
    difference() {
        // main wall
        cube([wall_width, wall_height, wall_thickness]);

        // window cutout
        translate([window_x, window_y, -1]) {
            cube([window_w, window_h, wall_thickness + 2]);
        }

        // door cutout
        translate([24.624, -0.01, -1]) {
            cube([door_w, door_h, wall_thickness + 2]);
        }
    }
}

// Rotate to stand upright
translate([-wall_width, wall_thickness, 0]) {
    rotate([90, 0, 0]) {
        wall_with_openings();
    }
}
