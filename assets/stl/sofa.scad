$fn = 180;
SOFA_WIDTH = 30;

SOFA_RADIUS = 20;
SOFA_DEPTH = 8;

SEAT_HEIGHT = 4.2;
SEAT_DEPTH = 5.5;

REST_HEIGHT = 3.8;
REST_TILT_DEPTH = 2;

rounding_r = 1;

HOLE_RADIUS = SOFA_RADIUS - SOFA_DEPTH;
eps = 0.01;
SOFA_HEIGHT = SEAT_HEIGHT + REST_HEIGHT;

module rounder_sofa_base(angle1, angle2) {
    translate([-SOFA_WIDTH / 2, SOFA_RADIUS + SEAT_DEPTH - rounding_r, 0]) {
        rotate([0, 0, 180 + 45 + angle1]) {
            rotate_extrude(angle = angle2, convexity = 10) {
                // front
                translate([SOFA_RADIUS - 2 * rounding_r, SEAT_HEIGHT - rounding_r, 0]) {
                    circle(rounding_r);
                }
                translate([SOFA_RADIUS - 2 * rounding_r, rounding_r, 0]) {
                    circle(rounding_r);
                    translate([0, rounding_r, 0]) {
                        square([2 * rounding_r, SEAT_HEIGHT - 2 * rounding_r], center = true) ;
                    }
                }
                // middle
                translate([SOFA_RADIUS - 2 * rounding_r, 0, 0]) {
                    square([SEAT_DEPTH, SEAT_HEIGHT]) ;
                }
                // back
                translate([SOFA_RADIUS + SEAT_DEPTH - 2 * rounding_r, SEAT_HEIGHT + rounding_r, 0]) {
                    circle(rounding_r);
                }
                translate([SOFA_RADIUS + SEAT_DEPTH - 2 * rounding_r, rounding_r, 0]) {
                    circle(rounding_r);
                    translate([-rounding_r, 0, 0]) {
                        square([2 * rounding_r, SEAT_HEIGHT]) ;
                    }
                }
            }
        }
    }
}

module round_sofa_rest() {
    rotate([90, 0, -90]) {
        translate([-SOFA_RADIUS - 0.4, 0, 0.5 * SOFA_WIDTH]) {
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
    }
}

angle1 = 7;
angle2 = 76;
rounder_sofa_base(angle1, angle2);

intersection() {
    translate([0, 0, SEAT_HEIGHT]) {
        rounder_sofa_base(angle1, angle2);
    }
    round_sofa_rest();
}
