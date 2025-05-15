module bathroom_cabinet() {
    PLANK_THICKNESS = 0.18;
    CLOSET_HEIGHT = 26.8;
    CLOSED_DEPTH = 4.;
    CLOSET_WIDTH = 6. - PLANK_THICKNESS; // this is to make it like 2 x 30mm cabinets
    MIDDLE_PLANK_DEPTH = CLOSED_DEPTH;
    MIDDLE_VERTICAL_PLANK_HEIGHT = CLOSET_HEIGHT - PLANK_THICKNESS;
    MIDDLE_HORIZONTAL_PLANK_Y = 20.0;
    TOP_HORIZONTAL_DIVIDER_PLANK_Y = MIDDLE_HORIZONTAL_PLANK_Y + 0.5 * (CLOSET_HEIGHT - MIDDLE_HORIZONTAL_PLANK_Y);
    LAUNDRY_BASKET_TOP_Y = 6.0;
    VERTICAL_MIDDLE_PLANK_X = 0.5 * (CLOSET_WIDTH);
    HORIZONTAL_MIDDLE_PLANK_Y = 12.0;

    // top_plank
    translate([PLANK_THICKNESS, MIDDLE_VERTICAL_PLANK_HEIGHT, 0.]) {
        cube([CLOSET_WIDTH - 2. * PLANK_THICKNESS, PLANK_THICKNESS, CLOSED_DEPTH]);
    }
    // top_horizontal_plank
    translate([PLANK_THICKNESS, MIDDLE_HORIZONTAL_PLANK_Y, 0.]) {
        cube([CLOSET_WIDTH - 2. * PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
    }
    // top_horizontal_divider_plank
    translate([PLANK_THICKNESS, TOP_HORIZONTAL_DIVIDER_PLANK_Y, 0.]) {
        cube([CLOSET_WIDTH - 2. * PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
    }
    // laundry_basket_top_plank
    translate([PLANK_THICKNESS, LAUNDRY_BASKET_TOP_Y, 0.]) {
        cube([CLOSET_WIDTH - 2. * PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
    }
        // pułeczka trochę wyżej
    translate([PLANK_THICKNESS, (HORIZONTAL_MIDDLE_PLANK_Y + LAUNDRY_BASKET_TOP_Y)/2, 0.]) {
        cube([CLOSET_WIDTH - 2. * PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
    }
    // middle_horizontal_plank
    translate([PLANK_THICKNESS, HORIZONTAL_MIDDLE_PLANK_Y, 0.]) {
        cube([CLOSET_WIDTH - 2. * PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
    }
    // pułeczka trochę wyżej
    translate([PLANK_THICKNESS, (MIDDLE_HORIZONTAL_PLANK_Y + HORIZONTAL_MIDDLE_PLANK_Y)/2, 0.]) {
        cube([CLOSET_WIDTH - 2. * PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
    }
    side_plank = [PLANK_THICKNESS, CLOSET_HEIGHT, CLOSED_DEPTH];
    // entrance_side_plank
    translate([0, 0, 0]) {
        cube(side_plank);
    }
    // shower_side_plank
    translate([CLOSET_WIDTH - PLANK_THICKNESS, 0., 0.]) {
        cube(side_plank);
    }
    // middle vertical plank
    translate([VERTICAL_MIDDLE_PLANK_X, 0., 0.]) {
        cube([PLANK_THICKNESS, MIDDLE_HORIZONTAL_PLANK_Y, MIDDLE_PLANK_DEPTH]);
    }
    // basket door
    translate([0, 0, CLOSED_DEPTH]) {
        rotate([20, 0, 0]) {
            cube([CLOSET_WIDTH / 2, LAUNDRY_BASKET_TOP_Y, PLANK_THICKNESS]);
        }
    }
    translate([CLOSET_WIDTH, 0, CLOSED_DEPTH + PLANK_THICKNESS]) {
        rotate([-10, 180, 0]) {
            cube([CLOSET_WIDTH / 2, LAUNDRY_BASKET_TOP_Y, PLANK_THICKNESS]);
        }
    }
    // middle doors
    translate([PLANK_THICKNESS, LAUNDRY_BASKET_TOP_Y, CLOSED_DEPTH]) {
        rotate([0, -70, 0]) {
            cube([CLOSET_WIDTH / 2, LAUNDRY_BASKET_TOP_Y, PLANK_THICKNESS]);
        }
    }
    translate([CLOSET_WIDTH, LAUNDRY_BASKET_TOP_Y, CLOSED_DEPTH]) {
        rotate([0, -110, 0]) {
            cube([CLOSET_WIDTH / 2, LAUNDRY_BASKET_TOP_Y, PLANK_THICKNESS]);
        }
    }
    // top middle doors
    translate([PLANK_THICKNESS, HORIZONTAL_MIDDLE_PLANK_Y, CLOSED_DEPTH]) {
        rotate([0, -80, 0]) {
            cube([CLOSET_WIDTH / 2, MIDDLE_HORIZONTAL_PLANK_Y - HORIZONTAL_MIDDLE_PLANK_Y, PLANK_THICKNESS]);
        }
    }
    translate([CLOSET_WIDTH, HORIZONTAL_MIDDLE_PLANK_Y, CLOSED_DEPTH]) {
        rotate([0, -100, 0]) {
            cube([CLOSET_WIDTH / 2, MIDDLE_HORIZONTAL_PLANK_Y - HORIZONTAL_MIDDLE_PLANK_Y, PLANK_THICKNESS]);
        }
    }
    // top top doors
    translate([PLANK_THICKNESS, MIDDLE_HORIZONTAL_PLANK_Y, CLOSED_DEPTH]) {
        rotate([0, -85, 0]) {
            cube([CLOSET_WIDTH / 2, CLOSET_HEIGHT - MIDDLE_HORIZONTAL_PLANK_Y, PLANK_THICKNESS]);
        }
    }
    translate([CLOSET_WIDTH, MIDDLE_HORIZONTAL_PLANK_Y, CLOSED_DEPTH]) {
        rotate([0, -105, 0]) {
            cube([CLOSET_WIDTH / 2, CLOSET_HEIGHT - MIDDLE_HORIZONTAL_PLANK_Y, PLANK_THICKNESS]);
        }
    }
}
bathroom_cabinet();
