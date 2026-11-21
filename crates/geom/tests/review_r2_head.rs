//! REVIEW PROBE (reviewer r2, PR 7203), head only. Not part of the suite.
#![allow(clippy::all, clippy::pedantic, missing_docs)]
use geom::{Conic, Curve3, Ellipse, Frame};
use geom_core::{Point3, Vec3};

fn fixture(a: f64, b: f64) -> Ellipse<f64> {
    Ellipse::anchored(
        Point3::new(3.5, -1.0, 2.0),
        Vec3::new(2.0, 2.0, 1.0),
        Vec3::new(1.0, -2.0, 2.0),
        a,
        b,
    )
}

/// `Ellipse::placed`'s doc names "a motion, a reversal" as the maps it
/// takes. A reversal through it keeps the phase's sine, so on a
/// re-seamed anchored ellipse the image is another ellipse. Prints the
/// distance between `placed(Frame::reversed)` at `−t` and the ellipse
/// at `t`; `Ellipse::reversed` reads zero.
#[test]
fn placed_with_a_reversal_on_a_re_seamed_anchored_ellipse() {
    for (a, b) in [(30.0, 41.0), (1500.0, 15.0)] {
        let e = fixture(a, b);
        for seam in [0.3, 1.4, 2.9, -1.5] {
            let r = e.seamed_near(seam);
            let by_placed = r.placed(Frame::reversed);
            let by_door = r.reversed();
            let (mut worst_placed, mut worst_door) = (0.0_f64, 0.0_f64);
            for t in [-3.0, -1.2, 0.0, 0.4, 1.55, 2.9] {
                worst_placed = worst_placed.max((by_placed.point_at(-t) - r.point_at(t)).norm());
                worst_door = worst_door.max((by_door.point_at(-t) - r.point_at(t)).norm());
            }
            println!(
                "PLACED a {a} b {b} seam {seam} phase {:?}: placed(reversed) off by {worst_placed:e}, reversed() off by {worst_door:e}",
                r.phase()
            );
        }
    }
}

/// A `-0.0` anchor coordinate: is the point at zero the anchor bit for bit?
#[test]
fn a_negative_zero_anchor_coordinate() {
    let e = Ellipse::anchored(
        Point3::new(-0.0, 1.0, 2.0),
        Vec3::unit_z(),
        Vec3::unit_x(),
        5.0,
        2.0,
    );
    let p = e.point_at(0.0);
    println!(
        "NEGZERO anchor x bits {:016x} point_at(0) x bits {:016x} zero_point x bits {:016x} Curve3::eval {:016x}",
        (-0.0f64).to_bits(),
        p.x.to_bits(),
        e.zero_point().x.to_bits(),
        Curve3::Ellipse(e).eval(0.0).x.to_bits()
    );
}

/// An un-turned re-seam of an anchored ellipse: same bits?
#[test]
fn an_unturned_re_seam_keeps_the_bits() {
    let e = fixture(1e6, 1e4);
    let r = e.seamed_near(0.3);
    let mut same = true;
    for t in [-1e-7, 0.0, 3e-8, 0.2, 1.5, 3.0] {
        let (p, q) = (e.point_at(t), r.point_at(t));
        same &= p.x.to_bits() == q.x.to_bits() && p.y.to_bits() == q.y.to_bits() && p.z.to_bits() == q.z.to_bits();
    }
    println!("UNTURNED same bits: {same}; same_content: {}", Curve3::Ellipse(e).same_content(&Curve3::Ellipse(r)));
    // Four quarter turns bring the seam home: is it the same content?
    let mut k = e;
    for _ in 0..4 {
        k = k.quarter_turned_where(1.0);
    }
    println!(
        "FOURTURNS phase {:?} same_content {} centre moved {:e} point(1e-8) moved {:e}",
        k.phase(),
        Curve3::Ellipse(e).same_content(&Curve3::Ellipse(k)),
        (k.center() - e.center()).norm(),
        (k.point_at(1e-8) - e.point_at(1e-8)).norm()
    );
}
