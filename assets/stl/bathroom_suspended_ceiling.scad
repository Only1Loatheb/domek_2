$fn = 180;

flat_height = 26.8;
bedroom_x = 30.;
bedroom_z = 32.4;
ceiling_height = 1.;
sus = 4.;
cabbinet = 6.;
eps = 0.01;

module wall_with_openings() {
    translate([0, flat_height - ceiling_height, 0]) {
        difference() {
            cube([bedroom_x, ceiling_height, bedroom_z]);
            translate([sus, -eps, sus + cabbinet]) {
                cube([bedroom_x - 2. * sus, ceiling_height + 2 * eps, bedroom_z - 2. * sus - cabbinet]);
            }
        }
    }
}

rotate([90, 0, 0]) wall_with_openings();
