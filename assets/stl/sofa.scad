$fn = 180;
SOFA_WIDTH = 30;

SOFA_RADIUS = 40;
SOFA_DEPTH = 8;

SEAT_HEIGHT = 4.2;
SEAT_DEPTH = 5.5;

REST_HEIGHT = 3.8;
REST_TILT_DEPTH = 2;

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

A = 15;

rotate([90, 0, 0]) {
    translate([- 0.5 * SOFA_WIDTH, 0, -SOFA_RADIUS]) {
        rotate([0, -45, 0]) {
            intersection() {
                rotate([0, -45, 0]) {
                    translate([SOFA_RADIUS, 0, 0]){

                        #cube([2* SOFA_RADIUS, 2* SOFA_HEIGHT,  SOFA_WIDTH], center = true);
                    }
                }
                round_sofa();
            }
        }
    }
}
