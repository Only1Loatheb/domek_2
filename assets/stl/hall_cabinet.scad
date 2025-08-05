$fn=90;
module hall_cabinet() {
    PLANK_THICKNESS = 0.18;
    CLOSET_HEIGHT = 26.8;
    CLOSED_DEPTH = 6.8;
    VENT_DEPTH = 8.;
    KITCHEN_WALL_LENGTH = 9.;
    CLOSET_WIDTH = VENT_DEPTH + KITCHEN_WALL_LENGTH;
    BROOM_COMPARTMENT_WIDTH = 1.5;
    DRAWERS_WIDTH = 5.;
    SLIDING_DOORS_DEPTH = 1.;
    MIDDLE_PLANK_DEPTH = CLOSED_DEPTH - SLIDING_DOORS_DEPTH;
    MIDDLE_VERTICAL_PLANK_HEIGHT = CLOSET_HEIGHT - PLANK_THICKNESS;
    MIDDLE_HORIZONTAL_PLANK_Y = 20.0;
    TOP_HORIZONTAL_DIVIDER_PLANK_Y = MIDDLE_HORIZONTAL_PLANK_Y + 0.5 * (CLOSET_HEIGHT - MIDDLE_HORIZONTAL_PLANK_Y);
    HANGER_ROD_Y = 19.0;
    SHOES_DRAWER_TOP_Y = 5.0;
    SHOES_DRAWER_MIDDLE_Y = 2.5;
    HANGER_SPACE_W = CLOSET_WIDTH - 3. * PLANK_THICKNESS - BROOM_COMPARTMENT_WIDTH - DRAWERS_WIDTH;
    BROOM_X = CLOSET_WIDTH - 2. * PLANK_THICKNESS - BROOM_COMPARTMENT_WIDTH;
    MAX_DRAWER_Y = 11.;
    eps = 0.1;

    difference() {
        union() {
            // top plank
            translate([PLANK_THICKNESS, MIDDLE_VERTICAL_PLANK_HEIGHT, 0.]) {
                cube([CLOSET_WIDTH - 2. * PLANK_THICKNESS, PLANK_THICKNESS, CLOSED_DEPTH]);
            }
            // over the rod plank
            translate([PLANK_THICKNESS, MIDDLE_HORIZONTAL_PLANK_Y, 0.]) {
                cube([CLOSET_WIDTH - 2. * PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
            }
            // top storech horizontal plank
            translate([PLANK_THICKNESS, TOP_HORIZONTAL_DIVIDER_PLANK_Y, 0.]) {
                cube([CLOSET_WIDTH - 2. * PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
            }
            // the rod
            translate([PLANK_THICKNESS, HANGER_ROD_Y, 0.5 * CLOSED_DEPTH]) {
                cube([HANGER_SPACE_W, PLANK_THICKNESS, PLANK_THICKNESS]);
            }
            // over the boot draweds under the couat space
            translate([PLANK_THICKNESS, SHOES_DRAWER_TOP_Y, 0.]) {
                cube([BROOM_X, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
            }
            // split the boots with horizontal plank
            translate([PLANK_THICKNESS, SHOES_DRAWER_MIDDLE_Y, 0.]) {
                cube([BROOM_X, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
            }
            // shoes drawer bottom left
            translate([PLANK_THICKNESS, 0.3, 2.0]) {
                cube([HANGER_SPACE_W - PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
            }
//            // shoes drawer bottom right
//            translate([HANGER_SPACE_W + PLANK_THICKNESS, 0.3, 2.0]) {
//                cube([BROOM_X - HANGER_SPACE_W - PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
//            }
            // shoes drawer top left
            translate([PLANK_THICKNESS, SHOES_DRAWER_MIDDLE_Y + 0.3, 2.0]) {
                cube([HANGER_SPACE_W - PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
            }
//            // shoes drawer top right
//            translate([HANGER_SPACE_W + PLANK_THICKNESS, SHOES_DRAWER_MIDDLE_Y + 0.3, 2.0]) {
//                cube([BROOM_X - HANGER_SPACE_W - PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
//            }
            // pułki
            for (i = [3 : 6]) {
                translate([HANGER_SPACE_W + PLANK_THICKNESS, i * SHOES_DRAWER_MIDDLE_Y, 0]) {
                    cube([BROOM_X - HANGER_SPACE_W - PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH]);
                }
            }
            // left side plank
            translate([0., 0., 0.]) {
                cube([PLANK_THICKNESS, CLOSET_HEIGHT, CLOSED_DEPTH]);
            }
            // right side plank
            translate([CLOSET_WIDTH - PLANK_THICKNESS, 0., 0.]) {
                cube([PLANK_THICKNESS, CLOSET_HEIGHT, CLOSED_DEPTH]);
            }
            // broom compartment and drawers divider
            translate([BROOM_X, 0., 0.]) {
                cube([PLANK_THICKNESS, MIDDLE_HORIZONTAL_PLANK_Y, MIDDLE_PLANK_DEPTH]);
            }
            // drawers and hanging space divider
            translate([HANGER_SPACE_W, 0., 0.]) {
                cube([PLANK_THICKNESS, MIDDLE_VERTICAL_PLANK_HEIGHT, MIDDLE_PLANK_DEPTH]);
            }
        }
        translate([CLOSET_WIDTH - CLOSED_DEPTH, -0.5 * eps, 0]) {
            difference() {
                cube([CLOSED_DEPTH +eps, CLOSET_HEIGHT + eps, CLOSED_DEPTH+ eps]);
                rotate([-90, 0, 0]) {
                    cylinder(CLOSET_HEIGHT + eps, CLOSED_DEPTH + eps, CLOSED_DEPTH + eps);
                }
            }
        }
    }
}

hall_cabinet();
