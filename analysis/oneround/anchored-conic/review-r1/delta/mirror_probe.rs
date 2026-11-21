//! Reviewer r1 delta probe: an arc-held ellipse, re-seamed, under a
//! reflection mapped as `topo::transform::map_frame` maps a frame at the
//! improper hand (origin and seam by the map, the axis by the map negated).
use geom::{Conic, Ellipse, Frame};
use geom_core::{Point3, Vec3};

#[test]
#[ignore]
fn a_mirrored_re_seamed_arc_held_ellipse_is_the_mirror_image() {
    let n = Vec3::new(0.6, -0.48, 0.64);
    let o = Point3::new(0.3, 0.1, -0.2);
    let mirror_p = |p: Point3<f64>| p - n * (2.0 * (p - o).dot(n));
    let mirror_v = |v: Vec3<f64>| v - n * (2.0 * v.dot(n));
    let e0 = Ellipse::anchored(
        Point3::new(3.5, -1.0, 2.0),
        Vec3::new(2.0, 2.0, 1.0),
        Vec3::new(1.0, -2.0, 2.0),
        1500.0,
        15.0,
    );
    for seam in [0.0, 1.4, 2.9, -1.5] {
        let e = e0.seamed_near(seam);
        let img = e.mapped(|f: &Frame<f64>| {
            Frame::new(mirror_p(f.origin()), -mirror_v(f.axis()), mirror_v(f.u_ref()))
        });
        let mut worst = 0.0_f64;
        for t in [-3.0, -1.2, -2e-3, 0.0, 0.4, 1.55, 2.9] {
            worst = worst.max((img.point_at(t) - mirror_p(e.point_at(t))).norm());
        }
        println!("seam near {seam}: worst |mirrored − mirror image| {worst:e}");
        assert!(worst < 1e-9, "seam near {seam}: {worst:e}");
    }
}
