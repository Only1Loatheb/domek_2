$fn = 180;
SOFA_WIDTH = 30;

SOFA_RADIUS = 40;
SOFA_DEPTH = 8;

SEAT_HEIGHT = 4.2;
SEAT_DEPTH = 5.5;

REST_HEIGHT = 3.8;
REST_TILT_DEPTH = 2;

rounding_r = 1;

HOLE_RADIUS = SOFA_RADIUS - SOFA_DEPTH;
eps = 0.01;
SOFA_HEIGHT = SEAT_HEIGHT + REST_HEIGHT;


module round_sofa() {
//    difference() {
//        rotate([-90, 0, 0]) {
//            cylinder(SEAT_HEIGHT, SOFA_RADIUS, SOFA_RADIUS);
//        }
//        translate([0., -eps, 0.]) {
//            rotate([-90, 0, 0]) {
//                cylinder(SEAT_HEIGHT + 2 * eps, HOLE_RADIUS, HOLE_RADIUS);
//            }
//        }
//    }
    // rest
    translate([0., SEAT_HEIGHT, 0]) {
        difference() {
            rotate([-90, 0, 0]) {
                cylinder(REST_HEIGHT, SOFA_RADIUS, SOFA_RADIUS);
            }
            translate([0., -eps, 0.]) {
                rotate([-90, 0, 0]) {
                    cylinder(REST_HEIGHT + 2 * eps, HOLE_RADIUS + SEAT_DEPTH, HOLE_RADIUS + SEAT_DEPTH + REST_TILT_DEPTH);
                }
            }
        }
    }
}

rotate([90, 0, -90]) {
    translate([-SOFA_RADIUS, 0, 0.5 * SOFA_WIDTH]) {
        intersection() {
            translate([SOFA_RADIUS, 0, 0]) {
                cube([2 * SOFA_RADIUS, 2 * SOFA_HEIGHT, SOFA_WIDTH], center = true);
            }
            round_sofa();
        }
    }
}

module rounder_sofa(angle1, angle2, is_front) {
    translate([-SOFA_WIDTH / 2, SOFA_RADIUS - rounding_r, 0]) {
        rotate([0, 0, 180 + 45 + angle1]) {
            rotate_extrude(angle = angle2, convexity = 10) {
                translate([SOFA_RADIUS - 2 * rounding_r, SEAT_HEIGHT - rounding_r, 0]) {
                    circle(rounding_r);
                }
                translate([SOFA_RADIUS - 2 * rounding_r, rounding_r, 0]) {
                    circle(rounding_r);
                    translate([0, rounding_r, 0]) {
                        square([2 * rounding_r, SEAT_HEIGHT - 2 * rounding_r], center = true) ;
                    }
                }
                if (is_front) {
                    translate([SOFA_RADIUS - 2 * rounding_r, 0, 0]) {
                        square([SEAT_DEPTH, SEAT_HEIGHT]) ;
                    }
                }
            }
        }
    }
}
rounder_sofa(angle1 = 22.5, angle2 = 45, is_front = false);
translate([0, SOFA_DEPTH - 2 * rounding_r, 0]) {
    rounder_sofa(angle1 = 25.5, angle2 = 39, is_front = true);
}
