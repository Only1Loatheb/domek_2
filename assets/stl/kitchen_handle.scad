$fn = 200;

eps = 0.1;
the_r = 17;
handle_width = 300;
face_height = 30;
lip_radius = 6;
thicckness = 0.3;

negative_r = 50;
negative_r_middle = 10;
middle_cuts_center = 20;
scala = 4.3;

module folded_sheet_handle() {
    intersection() {
        difference() {
            translate([0, -the_r, 0]) {
                // base shape:
                intersection() {
                    difference() {
                        cylinder(r = the_r, h = handle_width, center = true);
                        cylinder(r = the_r - thicckness, h = handle_width + eps, center = true);
                    }
                    translate([the_r / 2, the_r, 0]) {
                        cube([the_r, the_r, handle_width], center = true);
                    }
                }
            }
            // side cuts:
            union() {
                translate([negative_r, 0, handle_width / 2]) {
                    scale([1, 1, scala]) {
                        rotate([90, 0, 0]) {
                            cylinder(r = negative_r, h = handle_width);
                        }
                    }
                }
                translate([negative_r, 0, -handle_width / 2]) {
                    scale([1, 1, scala]) {
                        rotate([90, 0, 0]) {
                            cylinder(r = negative_r, h = handle_width);
                        }
                    }
                }
            }
        }
        // million cuts:
        union() {
            translate([0, 0, middle_cuts_center / 2]) {
                scale([1, 1, scala]) {
                    rotate([90, 0, 0]) {
                        cylinder(r = negative_r_middle, h = handle_width);
                    }
                }
            }
            translate([0, 0, -middle_cuts_center / 2]) {
                scale([1, 1, scala]) {
                    rotate([90, 0, 0]) {
                        cylinder(r = negative_r_middle, h = handle_width);
                    }
                }
            }
            cube([2 * negative_r_middle, handle_width, middle_cuts_center], center = true);
            cube([negative_r / 3, handle_width, handle_width], center = true);
        }
    }
}

//folded_sheet_handle();
cube([10, 10, 600], center = true);
