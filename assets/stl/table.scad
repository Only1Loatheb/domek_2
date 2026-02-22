// TILL Table Parameters (Units in cm)
table_top_dia = 130;
table_height = 76;
top_thickness = 6;
num_legs = 6;
leg_width = 5;
leg_thickness = 4;

leg_floor_contact_offset_from_table_center = 22;
leg_table_top_contact_offset_from_table_center = 10;

// Render
table_top();
legs();

module table_top() {
    translate([0, 0, table_height - top_thickness])
        cylinder(h = top_thickness, r1 = table_top_dia / 2 - 3, r2 = table_top_dia / 2, $fn = 500);
}

module legs() {
    for (i = [0 : num_legs - 1]) {
        rotate([0, 0, i * (360 / num_legs)])
            translate([0, -leg_width / 2, 0])
                leg_geometry();
    }
}

module leg_geometry() {
    translate([0.5, 0, 0])
        cube([leg_floor_contact_offset_from_table_center, leg_width, leg_thickness]);
    hull() {
        // Floor contact
        translate([leg_floor_contact_offset_from_table_center, 0, 0])
            cube([leg_thickness, leg_width, 0.1]);
        // Top contact
        translate([leg_table_top_contact_offset_from_table_center, 0, table_height - top_thickness])
            cube([leg_thickness, leg_width, 0.1]);
    }
}
