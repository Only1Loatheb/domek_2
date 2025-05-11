module washing_machine_cabinet() {
    szafa_h = 26.8;
    WASHING_MACHINE_HALF_WIDTH = 3.;
    WASHING_MACHINE_HEIGHT = 8.5;
    TILE_PLUS_GLUE = 0.08 + 0.07;
    washing_machine_width = 2. * WASHING_MACHINE_HALF_WIDTH;
    PLANK_THICKNESS = 0.18;
    three_planks = 3. * PLANK_THICKNESS;
    h = 2. * WASHING_MACHINE_HEIGHT + 2. * PLANK_THICKNESS;

    translate([0., 0., -washing_machine_width - 2. * three_planks - TILE_PLUS_GLUE]) {
        cube([washing_machine_width, szafa_h, PLANK_THICKNESS]);
    }

    translate([0., 0., -PLANK_THICKNESS - TILE_PLUS_GLUE]) {
        cube([washing_machine_width, szafa_h, PLANK_THICKNESS]);
    }

    translate([0., h, -washing_machine_width - 2. * three_planks - TILE_PLUS_GLUE]) {
        cube([PLANK_THICKNESS, szafa_h - h, washing_machine_width + 2. * three_planks]);
    }
}

washing_machine_cabinet();
