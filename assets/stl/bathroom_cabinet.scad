module bathroom_cabinet() {
    PLANK_THICKNESS = 0.18;
    CLOSET_HEIGHT = 26.8;
    CLOSED_DEPTH = 4.;
    CLOSET_WIDTH = 6. - PLANK_THICKNESS;
    MIDDLE_PLANK_DEPTH = CLOSED_DEPTH;
    MIDDLE_VERTICAL_PLANK_HEIGHT = CLOSET_HEIGHT - PLANK_THICKNESS;
    MIDDLE_HORIZONTAL_PLANK_Y = 20.0;
    TOP_HORIZONTAL_DIVIDER_PLANK_Y = MIDDLE_HORIZONTAL_PLANK_Y + 0.5 * (CLOSET_HEIGHT - MIDDLE_HORIZONTAL_PLANK_Y);
    LAUNDRY_BASKET_TOP_Y = 6.0;
    VERTICAL_MIDDLE_PLANK_X = 0.5 * (CLOSET_WIDTH);
    HORIZONTAL_MIDDLE_PLANK_X = 12.0;
    
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
    // middle_horizontal_plank
    translate([PLANK_THICKNESS, HORIZONTAL_MIDDLE_PLANK_X, 0.]) {
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
    // broom_compartment_and_drawers_divider_plank
    translate([VERTICAL_MIDDLE_PLANK_X, 0., 0.]) {
        cube([PLANK_THICKNESS, MIDDLE_HORIZONTAL_PLANK_Y, MIDDLE_PLANK_DEPTH]);
    }
}
bathroom_cabinet();
