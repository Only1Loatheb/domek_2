$fn = 180;
SOFA_WIDTH = 35;

SOFA_RADIUS = 35;
SOFA_DEPTH = 12;

SEAT_HEIGHT = 4;
SEAT_DEPTH = 7;

REST_HEIGHT = 6;
REST_TILT_DEPTH = 3;

HOLE_RADIUS = SOFA_RADIUS - SOFA_DEPTH;
eps = 0.01;
SOFA_HEIGHT = SEAT_HEIGHT + REST_HEIGHT;
module round_sofa() {
    difference() {
        rotate([-90, 0, 0]) {
            cylinder(SEAT_HEIGHT, SOFA_RADIUS, SOFA_RADIUS);
        }
        translate([0., -eps, 0.]) {
            rotate([-90, 0, 0]) {
                cylinder(SEAT_HEIGHT + 2 * eps, HOLE_RADIUS, HOLE_RADIUS);
            }
        }
    }
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

intersection() {
    translate([SOFA_RADIUS, 0, 0]) {
        rotate([0, -45, 0]) {
            cube([2 * SOFA_RADIUS, SOFA_HEIGHT, 2 * SOFA_RADIUS]);
        }
    }
    translate([SOFA_RADIUS, 0, 0]) {
        round_sofa();
    }
}
