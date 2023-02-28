$fn=64;
epsilon=0.01;

module plate(h, plate_r=50) {
  cylinder(r=plate_r, h=h, center=true);
}

module enclosure(r=53, h=10) {
  translate([0, 0, h/2]) difference() {
    hull() {
      cylinder(r1=r, r2=r-3, h=h, center=true);
      translate([22, -66, -h/2+5]) cylinder(r=5, h=10, center=true);
      translate([-22, -66, -h/2+5]) cylinder(r=5, h=10, center=true);
    }

    rotate([35, 0, 0])
      translate([24, -50, 20])
      cylinder(r=1, h=h*2, center=true);

    rotate([35, 0, 0])
      translate([24, -50, 18])
      cylinder(r=3.2, h=h, center=true);

    rotate([35, 0, 0])
      translate([-24, -50, 20])
      cylinder(r=1, h=h*2, center=true);

    rotate([35, 0, 0])
      translate([-24, -50, 18])
      cylinder(r=3.2, h=h, center=true);

    hull() {
      translate([-4, 10, -h/2+6])
        rotate([90, 0, 0])
        cylinder(r=2, h=100, center=true);
      translate([4, 10, -h/2+6])
        rotate([90, 0, 0])
        cylinder(r=2, h=100, center=true);
    }

    rotate([35, 0, 0])
      translate([0, -49, 28])
      cube(size=[26, 15, 30], center=true);

    rotate([35, 0, 0])
      translate([0, -48, 21.5])
      cube(size=[42.4, 25, 30], center=true);

    translate([0, 0, -5]) 
      plate(h=h+epsilon);

    cylinder(r1=48, r2=49, h=h+epsilon, center=true);
  }
}

enclosure(h=25);

