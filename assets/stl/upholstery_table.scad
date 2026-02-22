// Parameters
seat_width = 45;
seat_depth = 45;
seat_thickness = 8;
seat_height_floor = 45;
back_height = 50;
back_tilt = 8;
rounding_rad = 2.5; // Controls how "soft" the cushions look

// Colors based on your wood texture and image
color_wood = [0.82, 0.70, 0.55]; // Light Oak
color_fabric = [0.35, 0.37, 0.42]; // Charcoal Slate

module upholstery() {
    color(color_fabric) {
        // Rounded Seat Cushion
        translate([0, 0, seat_height_floor - (seat_thickness/2)])
            rounded_box(seat_width, seat_depth, seat_thickness, rounding_rad);

        // Rounded Backrest
        translate([0, -seat_depth/2 - 0.3, seat_height_floor + (back_height/2) - 4])
            rotate([back_tilt, 0, 0])
                rounded_box(seat_width, 6, back_height, rounding_rad);
    }
}

// Helper Module: Creates a rounded volume by hulling spheres at corners
module rounded_box(w, d, h, r) {
    hull() {
        for (x = [-w/2 + r, w/2 - r])
            for (y = [-d/2 + r, d/2 - r])
                for (z = [-h/2 + r, h/2 - r])
                    translate([x, y, z])
                        sphere(r = r, $fn = 32);
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
table_color = "BlanchedAlmond";

// Render
scale([0.1, 0.1, 0.1]) {
//    table_top();
    legs();
}
module table_top() {
    color(table_color) {
        translate([0, 0, table_height - top_thickness])
            cylinder(h = top_thickness, r1 = table_top_dia / 2 - 3, r2 = table_top_dia / 2, $fn = 500);
    }
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
//            chair_base();
            upholstery();
        }
    }
//    color(table_color) {
//        translate([0.5, 0, 0])
//            cube([leg_floor_contact_offset_from_table_center, leg_width, leg_thickness]);
//        hull() {
//            // Floor contact
//            translate([leg_floor_contact_offset_from_table_center, 0, 0])
//                cube([leg_thickness, leg_width, 0.1]);
//            // Top contact
//            translate([leg_table_top_contact_offset_from_table_center, 0, table_height - top_thickness])
//                cube([leg_thickness, leg_width, 0.1]);
//        }
//    }
}
