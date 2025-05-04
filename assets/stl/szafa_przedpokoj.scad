// font size
Size = 20;

// font name, style
Font = "Consolas";

// Kern factor to pull letters tighter or further.
Kern = 0.9;

// sum the prefix of a vector.
function Sum(v, i) = i < 1?0:v[i - 1] + Sum(v, i - 1);

// given a vector of characters and a corresponding vector of character widths, draw them,
// extruding the odd ones 5 and the even ones 4.
module stagger2(chars, lens) {
    for (i = [0:len(chars) - 1]) {
        linear_extrude(4 + (i % 2)) {
            translate([Sum(lens, i), 0])text(chars[i], font = Font, size = Size);
        }
    }
}

// simple call - just pass in the string to render
module stagger(s) {
    stagger2([for (c = s)c], [for (c = s)20 * Kern]);
}

// Example usage
//stagger("Sample");


//// Find the unitary vector with direction v. Fails if v=[0,0,0].
//function unit(v) = norm(v) > 0 ? v / norm(v) : undef;
//// Find the transpose of a rectangular matrix
//function transpose(m) = // m is any rectangular matrix of objects
//    [for (j = [0:len(m[0]) - 1]) [for (i = [0:len(m) - 1]) m[i][j]]];
//// The identity matrix with dimension n
//function identity(n) = [for (i = [0:n - 1]) [for (j = [0:n - 1]) i == j ? 1 : 0]];
//
//
//function rotate_from_to(a, b) =
//let(axis = unit(cross(a, b)))
//        axis * axis >= 0.99 ?
//        transpose([unit(b), axis, cross(axis, unit(b))]) *
//            [unit(a), axis, cross(axis, unit(a))] :
//        identity(3);
//
//module line(p0, p1, diameter = 1) {
//    v = p1 - p0;
//    color([0, 0, 0]) {
//        translate(p0) {
//            // rotate the cylinder so its z axis is brought to direction v
//            multmatrix(rotate_from_to([0, 0, 1], v)) {
//                cylinder(d = diameter, h = norm(v), $fn = 4);
//            }
//        }
//    }
//}


module szafa_przedpokoj() {
    szafa_d = 540;
    szafa_h = 2420;
    szafa_w = 870;
    grubosc_plyty = 18;
    miejsce_na_odkurzacz_h = 105;
    miejsce_na_odkurzacz_d = 250;
    szuflada_h = 160;
    srodkowa_z = 2 * szuflada_h + miejsce_na_odkurzacz_h;
    miejsce_na_kurtki_h = 1600;
    palak_h = 1900;
    palak_odstep_od_sufitu_h = 120;
    color([200 / 255, 200 / 255, 200 / 255]) {
        // plecówka
        cube([szafa_w, 5, szafa_h]);
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
        // dół
        translate([0, miejsce_na_odkurzacz_d, miejsce_na_odkurzacz_h]) {
            cube([szafa_w, szafa_d - miejsce_na_odkurzacz_d, grubosc_plyty]);
        }
        // środek
        translate([0, miejsce_na_odkurzacz_d, srodkowa_z]) {
            cube([szafa_w, szafa_d - miejsce_na_odkurzacz_d, grubosc_plyty]);
        }
        // deseczka środek
        translate([0, szafa_d - grubosc_plyty, srodkowa_z - grubosc_plyty]) {
            cube([szafa_w, grubosc_plyty, 2 * grubosc_plyty]);
        }
        // front szuflad
        translate([0, szafa_d + sin($t * 180) * 100, miejsce_na_odkurzacz_h]) {
            cube([szafa_w, grubosc_plyty, szuflada_h]);
        }
        // deseczka szuflad
        translate([0, szafa_d - grubosc_plyty, miejsce_na_odkurzacz_h + szuflada_h - grubosc_plyty]) {
            cube([szafa_w, grubosc_plyty, 2 * grubosc_plyty]);
        }
    //fron szuflady 2
        translate([0, szafa_d + $t * 100, miejsce_na_odkurzacz_h + szuflada_h]) {
            cube([szafa_w, grubosc_plyty, szuflada_h]);
        }
        //         lewe drzwi
        translate([grubosc_plyty, szafa_d, srodkowa_z]) {
            rotate([0, 0, $t * 90]) {
                cube([szafa_w / 2 - grubosc_plyty, grubosc_plyty, miejsce_na_kurtki_h]);
            }
        }
        //         prawe drzwi
        translate([szafa_w - grubosc_plyty, szafa_d, srodkowa_z]) {
            rotate([0, 0, 180 - $t * 90]) {
                translate([0, -grubosc_plyty, 0]) {
                    cube([szafa_w / 2 - grubosc_plyty, grubosc_plyty, miejsce_na_kurtki_h]);
                }
            }
        }
        // gorna półka
        gorna_polka_gora_z = palak_h + palak_odstep_od_sufitu_h + grubosc_plyty;
        gorne_drzwiczki_h = szafa_h - gorna_polka_gora_z;
        translate([0, 0, gorna_polka_gora_z]) {
            cube([szafa_w, szafa_d, grubosc_plyty]);
        }
        // deseczka półka
        translate([0, szafa_d - grubosc_plyty, gorna_polka_gora_z - grubosc_plyty]) {
            cube([szafa_w, grubosc_plyty, 2 * grubosc_plyty]);
        }
        // lewe drzwiczki
        translate([grubosc_plyty, szafa_d, gorna_polka_gora_z]) {
            rotate([0, 0,  - $t * 90]) {
                cube([szafa_w / 2 - grubosc_plyty, grubosc_plyty, gorne_drzwiczki_h]);
            }
        }
        // prawe drzwiczki
        translate([szafa_w - grubosc_plyty, szafa_d, gorna_polka_gora_z]) {
            rotate([0, 0, 180 - $t * 90]) {
                translate([0, -grubosc_plyty, 0]) {
                    cube([szafa_w / 2 - grubosc_plyty, grubosc_plyty, gorne_drzwiczki_h]);
                }
            }
        }
        // pałąk
        translate([szafa_w / 2, szafa_d / 2, palak_h]) {
            rotate([0, 90, 0]) {
                cylinder(h = szafa_w, d1 = 20, d2 = 20, center = true);
            }
        }
    }
}
//line([0, 0, 0], [5, 23, 42]);
//line([0, 0, 0], [5, 23, 42]);

szafa_przedpokoj();
