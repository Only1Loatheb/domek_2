module drawer_column(num_drawers, drawer_width, height, drawer_depth, spacing) {
    drawer_height = height / num_drawers - spacing;
    for (i = [0 : num_drawers - 1]) {
        translate([0, 0, i * (drawer_height + spacing)]) {
            cube([drawer_width, drawer_depth, drawer_height]);
        }
    }
}

module szafa_biuro() {
    grubosc_plyty = 0.18;

    szafa_w = 20.399998;
    szafa_h = 26.8;
    szafa_d = 6.;

    plecowka_d = 0.05;
    inner_d = szafa_d;
    drawer_w = szafa_w / 4 - grubosc_plyty; // grubosc_plyty odjęta przy każdej szufladzie
    drawers_h = 5;
    scale([-1., 1., 1.]) {
        // plecówka
        cube([szafa_w, plecowka_d, szafa_h]);
        // lewa
        cube([grubosc_plyty, szafa_d, szafa_h]);
        // prawa
        translate([szafa_w - grubosc_plyty, 0, 0]) {
            cube([grubosc_plyty, szafa_d, szafa_h]);
        }
        // góra
        translate([0, 0, szafa_h - grubosc_plyty]) {
            cube([szafa_w, szafa_d, grubosc_plyty]);
        }
        // szuflady
        echo(str("szafa_w = ", szafa_w));
        translate([szafa_w / 4, 0, 0]) {
            drawer_column(num_drawers = 2, drawer_width = szafa_w / 2, height = drawers_h, drawer_depth = inner_d, spacing = 0.1);
        }
        // półki lewe
        for (i = [0 : 1]) {
            translate([grubosc_plyty + i * (grubosc_plyty + drawer_w), 0, drawers_h]) {
                // magic spacing to make it look like shelves
                drawer_column(num_drawers = 5, drawer_width = drawer_w, height = szafa_h - drawers_h, drawer_depth = inner_d, spacing = 4.2);
            }
        }
        // lewy pionowy środek półek
        translate([szafa_w / 4, 0, 0]) {
            cube([grubosc_plyty, inner_d, szafa_h]);
        }
        translate([0, szafa_d, 0]) {
            rotate([0, 0, -10]) {
                cube([grubosc_plyty, inner_d, szafa_h]);
            }
        }
        //---
        // pionowy środek
        translate([szafa_w / 2, 0, drawers_h]) {
            cube([grubosc_plyty, inner_d, szafa_h - drawers_h]);
        }
        translate([szafa_w / 4, szafa_d, drawers_h]) {
            rotate([0, 0, -10]) {
                cube([grubosc_plyty, inner_d, szafa_h - drawers_h]);
            }
        }
        translate([3 * szafa_w / 4, szafa_d, drawers_h]) {
            rotate([0, 0, 10]) {
                cube([grubosc_plyty, inner_d, szafa_h - drawers_h]);
            }
        }
        //---
        // półki prawe
        translate([szafa_w / 2, 0, 0]) {
            for (i = [0 : 1]) {
                translate([grubosc_plyty + i * (grubosc_plyty + drawer_w), 0, drawers_h]) {
                    // magic spacing to make it look like shelves
                    drawer_column(num_drawers = 5, drawer_width = drawer_w, height = szafa_h - drawers_h, drawer_depth = inner_d, spacing = 4.2);
                }
            }
            // prawy pionowy środek półek
            translate([szafa_w / 4, 0, 0]) {
                cube([grubosc_plyty, inner_d, szafa_h]);
            }
        }
        door_h = 20.;
        bonus_size = [9.2 + .69, szafa_d, szafa_h - door_h];
        translate([szafa_w, 0., door_h]) {
            translate(bonus_size / 2) {
                difference() {
                    cube(bonus_size, center = true);
                    cube(bonus_size - 2 * grubosc_plyty * [1, -1, 1], center = true);
                }
            }
            translate([grubosc_plyty, bonus_size.y, 0]) {
                rotate([0, 0, 70]) {
                    cube([bonus_size.x / 2, grubosc_plyty, bonus_size.z]);
                }
                translate([bonus_size.x - grubosc_plyty, 0, 0]) {
                    rotate([0, 0, 110]) {
                        cube([bonus_size.x / 2, grubosc_plyty, bonus_size.z]);
                    }
                }
            }
        }
        // without doors for a walkway
        //        translate([szafa_w - grubosc_plyty, szafa_d, 0]) {
        //            rotate([0, 0, 10]) {
        //                cube([grubosc_plyty, inner_d, szafa_h]);
        //            }
        //        }
        //        translate([7 * szafa_w / 8, 1.2 * szafa_d, szafa_h/2]) {
        //            rotate([0, 90, 0]) {
        //                scale([0.1, 0.1, 0.1]) {
        //                    text("brak drzwi ↓");
        //                }
        //            }
        //        }
    }
}
szafa_biuro();
