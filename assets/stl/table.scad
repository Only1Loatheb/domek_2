// Chair Parameters (in cm)
seat_width = 45;
seat_depth = 45;
seat_thickness = 8;
seat_height_floor = 45;
back_height = 50;
back_tilt = 8; // Degrees

// Leg Parameters
leg_top_size = 4;
leg_bot_size = 2.5;
leg_spread = 2; // How much the legs flare out

module chair_base() {
    color("SaddleBrown") {
        // Wooden frame under seat
        translate([-seat_width / 2, -seat_depth / 2, seat_height_floor - seat_thickness - 4])
            cube([seat_width, seat_depth, 4]);

        // Four tapered legs
        leg_pos = (seat_width / 2) - 4;
        for (x = [-leg_pos, leg_pos]) {
            for (y = [-leg_pos, leg_pos]) {
                translate([x, y, 0])
                    tapered_leg(x, y);
            }
        }
    }
}

module tapered_leg(x_dir, y_dir) {
    // Determine flare direction based on position
    flare_x = (x_dir > 0) ? leg_spread : -leg_spread;
    flare_y = (y_dir > 0) ? leg_spread : -leg_spread;

    hull() {
        // Top of leg
        translate([0, 0, seat_height_floor - seat_thickness - 4])
            cube([leg_top_size, leg_top_size, 0.1], center = true);
        // Bottom of leg
        translate([flare_x, flare_y, 0])
            cube([leg_bot_size, leg_bot_size, 0.1], center = true);
    }
}

module upholstery() {
    color("SlateGray") {
        // Seat Cushion
        translate([-seat_width / 2, -seat_depth / 2, seat_height_floor - seat_thickness])
            cube([seat_width, seat_depth, seat_thickness]);

        // Backrest
        translate([0, -seat_depth / 2, seat_height_floor - 1])
            rotate([back_tilt, 0, 0])
                translate([-seat_width / 2, 0, 0])
                    cube([seat_width, 6, back_height]);
    }
}

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
    // Render
    translate([table_top_dia / 2, 0, 0]) {
        rotate([0, 0, 90]) {
            chair_base();
            upholstery();
        }
    }
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
