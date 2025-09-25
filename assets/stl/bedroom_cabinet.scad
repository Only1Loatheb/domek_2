module drawer_column(num_drawers, drawer_width, height, drawer_depth, spacing) {
    drawer_height = height / num_drawers - spacing;
    for (i = [0 : num_drawers - 1]) {
        translate([0, 0, i * (drawer_height + spacing)]) {
            cube([drawer_width, drawer_depth, drawer_height]);
        }
    }
}

module szafa_przedpokoj() {
    grubosc_plyty = 0.18;

    szafa_w = 18.1 + 1.;
    szafa_h = 26.8 - 1.01;
    szafa_d = 6.;

    plecowka_d = 0.05;
    palak_h = 19.;
    nad_palakiem_z = 1.;
    drzwi_przsowne_d = 1.;
    inner_d = szafa_d - drzwi_przsowne_d;
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
        // pionowy środek
        translate([szafa_w / 2, 0, 0]) {
            cube([grubosc_plyty, inner_d, szafa_h]);
        }
        // pałąk
        translate([szafa_w / 2, szafa_d / 2, palak_h]) {
            rotate([0, 90, 0]) {
                cylinder(h = szafa_w / 2, d1 = 0.2, d2 = 0.2, center = false);
            }
        }
        // półka nad pałąkiem
        translate([szafa_w / 2, 0., palak_h + nad_palakiem_z]) {
            cube([szafa_w / 2, inner_d, grubosc_plyty]);
        }
        // półka nad pałąkiem ii
        translate([szafa_w / 2, 0., (szafa_h - palak_h - nad_palakiem_z) / 2 + palak_h + nad_palakiem_z]) {
            cube([szafa_w / 2, inner_d, grubosc_plyty]);
        }
        // półka pod pałąkiem
        translate([szafa_w / 2, 0., drawers_h]) {
            cube([szafa_w / 2, inner_d, grubosc_plyty]);
        }
        // szuflady
        echo(str("szafa_w = ", szafa_w));
        for (i = [2 : 3]) {
            translate([grubosc_plyty + i * (grubosc_plyty + drawer_w), 0, 0]) {
                drawer_column(num_drawers = 2, drawer_width = drawer_w, height = drawers_h, drawer_depth = inner_d, spacing = 0.1);
            }
        }
        // półki
        for (i = [0 : 1]) {
            translate([grubosc_plyty + i * (grubosc_plyty + drawer_w), 0, drawers_h]) {
                // magic spacing to make it look like shelves
                drawer_column(num_drawers = 5, drawer_width = drawer_w, height = szafa_h - drawers_h, drawer_depth = inner_d, spacing = 4.2);
            }
        }
        // pionowy środek półek
        translate([szafa_w / 4, 0, 0]) {
            cube([grubosc_plyty, inner_d, szafa_h]);
        }
        translate([7 * szafa_w / 8, 1.2 * szafa_d, szafa_h / 2]) {
            rotate([0, 90, 0]) {
                scale([0.1, 0.1, 0.1]) {
                    text("przesuwne drzwi ↓");
                }
            }
        }
    }
}

szafa_przedpokoj();
