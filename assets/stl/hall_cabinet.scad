$fn = 90;
module hall_cabinet() {
    PLANK_THICKNESS = 0.18;
    CLOSET_HEIGHT = 26.8;
    CLOSED_DEPTH = 6.8;
    VENT_DEPTH = 8.;
    KITCHEN_WALL_LENGTH = 9.;
    CLOSET_WIDTH = VENT_DEPTH + KITCHEN_WALL_LENGTH;
    BROOM_COMPARTMENT_WIDTH = 1.5;
    SEAT_WIDTH = 6.; // our has 10.
    SEAT_DEPTH = 4.;
    SEAT_TOP = 4.2;
    SEAT_END = SEAT_WIDTH + PLANK_THICKNESS;
    SLIDING_DOORS_DEPTH = 1.;
    MIDDLE_PLANK_DEPTH = CLOSED_DEPTH - SLIDING_DOORS_DEPTH;
    MIDDLE_VERTICAL_PLANK_HEIGHT = CLOSET_HEIGHT - PLANK_THICKNESS;
    MIDDLE_HORIZONTAL_PLANK_Y = 20.0;
    TOP_HORIZONTAL_DIVIDER_PLANK_Y = MIDDLE_HORIZONTAL_PLANK_Y + 0.5 * (CLOSET_HEIGHT - MIDDLE_HORIZONTAL_PLANK_Y);
    HANGER_ROD_Y = 19.0;
    SHOES_DRAWER_TOP_Y = SEAT_TOP - PLANK_THICKNESS;
    SHOES_DRAWER_MIDDLE_Y = 0.5 * SHOES_DRAWER_TOP_Y;
    HANGER_SPACE_W = CLOSET_WIDTH - 3. * PLANK_THICKNESS - BROOM_COMPARTMENT_WIDTH - SEAT_WIDTH;
    BROOM_X = CLOSET_WIDTH - 2. * PLANK_THICKNESS - BROOM_COMPARTMENT_WIDTH;
    MAX_DRAWER_Y = 11.;
    eps = 0.01;
    rounding_radius = BROOM_COMPARTMENT_WIDTH + 2 * PLANK_THICKNESS;
    SHOES_DROWER_DEPTH_DISPLACEMENT = 0.5;

    difference() {
        union() {
            // BOX
            // top plank
            translate([PLANK_THICKNESS, MIDDLE_VERTICAL_PLANK_HEIGHT, 0.]) {
                cube([CLOSET_WIDTH - 2. * PLANK_THICKNESS, PLANK_THICKNESS, CLOSED_DEPTH]);
            }
            // left side plank
            translate([0., 0., 0.]) {
                cube([PLANK_THICKNESS, CLOSET_HEIGHT, CLOSED_DEPTH]);
            }
            // right side plank
            translate([CLOSET_WIDTH - PLANK_THICKNESS, 0., 0.]) {
                cube([PLANK_THICKNESS, CLOSET_HEIGHT, CLOSED_DEPTH]);
            }
            // HANGER SPACE
            // over the rod hanger plank
            translate([SEAT_END, MIDDLE_HORIZONTAL_PLANK_Y, 0.]) {
                cube([HANGER_SPACE_W, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
            }
            // top storage divider horizontal hanger plank
            translate([SEAT_END, TOP_HORIZONTAL_DIVIDER_PLANK_Y, 0.]) {
                cube([HANGER_SPACE_W, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
            }
            // the rod
            translate([SEAT_END, HANGER_ROD_Y, 0.5 * CLOSED_DEPTH]) {
                cube([HANGER_SPACE_W, PLANK_THICKNESS, PLANK_THICKNESS]);
            }
            // over the boot draweds under the couat space
            translate([SEAT_END, SHOES_DRAWER_TOP_Y, 0.]) {
                cube([HANGER_SPACE_W, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
            }
            // split the boots with horizontal plank
            translate([SEAT_END, SHOES_DRAWER_MIDDLE_Y, 0.]) {
                cube([HANGER_SPACE_W, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
            }
            // shoes drawer bottom
            translate([SEAT_END + PLANK_THICKNESS, 0.3, SHOES_DROWER_DEPTH_DISPLACEMENT]) {
                cube([HANGER_SPACE_W - 2 * PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
            }
            // shoes drawer top
            translate([SEAT_END + PLANK_THICKNESS, SHOES_DRAWER_MIDDLE_Y + 0.3, SHOES_DROWER_DEPTH_DISPLACEMENT]) {
                cube([HANGER_SPACE_W - 2 * PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
            }
            // broom compartment and hanger space divider
            translate([BROOM_X, 0., 0.]) {
                cube([PLANK_THICKNESS, CLOSET_HEIGHT, MIDDLE_PLANK_DEPTH]);
            }
            // HANGER SPACE SLIDING DOORS
            translate([SEAT_END + 0.1 * HANGER_SPACE_W, 0., CLOSED_DEPTH - PLANK_THICKNESS]) {
                cube([0.5 * HANGER_SPACE_W, CLOSET_HEIGHT, PLANK_THICKNESS]);
            }
            translate([SEAT_END + 0.25 * HANGER_SPACE_W, 0., CLOSED_DEPTH - 3 * PLANK_THICKNESS]) {
                cube([0.5 * HANGER_SPACE_W, CLOSET_HEIGHT, PLANK_THICKNESS]);
            }
            // SEAT
            // seat and hanging space divider
            translate([SEAT_END - PLANK_THICKNESS, 0., 0.]) {
                cube([PLANK_THICKNESS, MIDDLE_VERTICAL_PLANK_HEIGHT, CLOSED_DEPTH]);
            }
            // the seat
            translate([PLANK_THICKNESS, 0., 0.]) {
                cube([SEAT_WIDTH, SEAT_TOP, SEAT_DEPTH]);
            }
            seat_shelf_depth = CLOSED_DEPTH - PLANK_THICKNESS;
            // over the rod seat plank
            translate([PLANK_THICKNESS, MIDDLE_HORIZONTAL_PLANK_Y, 0.]) {
                cube([SEAT_WIDTH, PLANK_THICKNESS, seat_shelf_depth]);
            }
            // top storage divider horizontal seat plank
            translate([PLANK_THICKNESS, TOP_HORIZONTAL_DIVIDER_PLANK_Y, 0.]) {
                cube([SEAT_WIDTH, PLANK_THICKNESS, seat_shelf_depth]);
            }
            // top over seat storage doors
            translate([PLANK_THICKNESS, MIDDLE_HORIZONTAL_PLANK_Y, seat_shelf_depth + PLANK_THICKNESS]) {
                rotate([0, 80, 0]) {
                    cube([PLANK_THICKNESS, CLOSET_HEIGHT - MIDDLE_HORIZONTAL_PLANK_Y - PLANK_THICKNESS, SEAT_WIDTH - PLANK_THICKNESS]);
                }
            }
            // rounded door for broom compartment
            translate([CLOSET_WIDTH - rounding_radius, 0., CLOSED_DEPTH - rounding_radius]) {
                difference() {
                    intersection() {
                        cube([rounding_radius, CLOSET_HEIGHT, rounding_radius]);
                        rotate([-90, 0, 0]) {
                            cylinder(CLOSET_HEIGHT + 2 * eps, rounding_radius, rounding_radius);
                        }
                    }
                    translate([0., -2 * eps, 0.]) {
                        rotate([-90, 0, 0]) {
                            cylinder(CLOSET_HEIGHT + eps, rounding_radius - PLANK_THICKNESS, rounding_radius - PLANK_THICKNESS);
                        }
                    }
                }
            }
        }
        translate([CLOSET_WIDTH - rounding_radius, -0.5 * eps, CLOSED_DEPTH - rounding_radius]) {
            difference() {
                cube([rounding_radius + eps, CLOSET_HEIGHT + eps, rounding_radius + eps]);
                rotate([-90, 0, 0]) {
                    cylinder(CLOSET_HEIGHT + eps, rounding_radius + eps, rounding_radius + eps);
                }
            }
        }
    }
}

hall_cabinet();
