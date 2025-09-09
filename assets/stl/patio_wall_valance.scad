$fn = 180;

wall_width = 52.33;
wall_height = 26.8;
wall_thickness = 1.;

window_x = 8.783;

valance_w = 1.5;
valance_d = 0.1;

// Rotate to stand upright
scale([1, -1, 1]) {
    translate([-wall_width, wall_thickness, 0]) {
        rotate([90, 0, 0]) {
            intersection() {
                translate([window_x, wall_height - valance_w, wall_thickness]) {
                    // rounded valance part
                    rotate([-90, 0, 0]) {
                        intersection() {
                            difference() {
                                cylinder(valance_w, valance_w, valance_w);
                                translate([0, 0, -1]) {
                                    cylinder(valance_w + 2, valance_w - valance_d, valance_w - valance_d);
                                }
                            }
                            translate([-valance_w, -valance_w, 0]) {
                                cube([valance_w, valance_w, valance_w]);
                            }
                        }
                    }
                    // straight valance part
                    translate([0, 0, valance_w - valance_d]) {
                        cube([wall_width - window_x, valance_w, valance_d]);
                    }
                }
                translate([0, wall_height - valance_w, wall_thickness + 0.3]) {
                    cube([wall_width, valance_w, wall_thickness * 2]);
                }
            }
        }
    }
}
