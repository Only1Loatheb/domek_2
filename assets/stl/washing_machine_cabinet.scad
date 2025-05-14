module washing_machine_cabinet() {
    szafa_h = 26.8;
    WASHING_MACHINE_HALF_WIDTH = 3.;
    WASHING_MACHINE_HEIGHT = 8.5;
    washing_machine_depth = 2. * WASHING_MACHINE_HALF_WIDTH;
    PLANK_THICKNESS = 0.18;
    three_planks = 3. * PLANK_THICKNESS;
    cabinet_width = washing_machine_depth + 2. * three_planks;
    dryer_plank = 2. * PLANK_THICKNESS;
    h = 2. * WASHING_MACHINE_HEIGHT + dryer_plank;

    // right side
    translate([cabinet_width - PLANK_THICKNESS, 0., 0]) {
        cube([PLANK_THICKNESS, szafa_h, washing_machine_depth]);
    }
    // left side
    translate([0., 0., 0]) {
        cube([PLANK_THICKNESS, szafa_h, washing_machine_depth]);
    }
    // top left door
    translate([PLANK_THICKNESS, h, washing_machine_depth]) {
        rotate([0, -70, 0]) {
            cube([cabinet_width / 2, szafa_h - h, PLANK_THICKNESS]);
        }
    }
    // top right door
    translate([cabinet_width - PLANK_THICKNESS, h, washing_machine_depth]) {
        mirror([1, 0, 0]) {
            rotate([0, -70, 0]) {
                cube([cabinet_width / 2, szafa_h - h, PLANK_THICKNESS]);
            }
        }
    }
    // roof
    translate([0, szafa_h - PLANK_THICKNESS, 0]) {
        cube([cabinet_width, PLANK_THICKNESS, washing_machine_depth]);
    }
    // stash shelf
    translate([0, h, 0]) {
        cube([cabinet_width, PLANK_THICKNESS, washing_machine_depth]);
    }
    // stash shelf
    translate([0, h + (szafa_h - h) / 2, 0]) {
        cube([cabinet_width, PLANK_THICKNESS, washing_machine_depth]);
    }
    // dryer shelf
    translate([0, WASHING_MACHINE_HEIGHT, 0]) {
        cube([cabinet_width, dryer_plank, washing_machine_depth]);
    }
    // sliding doors
    door_distance = PLANK_THICKNESS + 0.1;
    bottom_slide = 2 * washing_machine_depth / 3;
    // bottom left door
    translate([door_distance, 0, bottom_slide]) {
        cube([PLANK_THICKNESS, WASHING_MACHINE_HEIGHT, cabinet_width / 2]);
    }
    // bottom right door
    translate([cabinet_width - door_distance, 0, bottom_slide]) {
        mirror([1, 0, 0]) {
            cube([PLANK_THICKNESS, WASHING_MACHINE_HEIGHT, cabinet_width / 2]);
        }
    }
    middle_slide = 3 * washing_machine_depth / 5;
    // middle left door
    translate([door_distance, WASHING_MACHINE_HEIGHT + dryer_plank, middle_slide]) {
        cube([PLANK_THICKNESS, WASHING_MACHINE_HEIGHT, cabinet_width / 2]);
    }
    // middle right door
    translate([cabinet_width - door_distance, WASHING_MACHINE_HEIGHT + dryer_plank, middle_slide]) {
        mirror([1, 0, 0]) {
            cube([PLANK_THICKNESS, WASHING_MACHINE_HEIGHT, cabinet_width / 2]);
        }
    }
}

washing_machine_cabinet();
