//! R1 triage: what does a zero axis yield at Interval, old vs new
//! spelling, at angle = 0 and at angle = 1?

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Bounds, Interval, Mat3, Point3, Real, Vec3};

fn show(tag: &str, e: Interval) {
    println!(
        "{tag}: [{:e}, {:e}] poison={}",
        e.lo(),
        e.hi(),
        e.is_poison()
    );
}

#[test]
fn r1_poison_triage() {
    let z = Vec3::new(Interval::zero(), Interval::zero(), Interval::zero());
    let q = Point3::new(
        Interval::from_f64(1.0),
        Interval::from_f64(2.0),
        Interval::from_f64(3.0),
    ) - Point3::origin();
    for angle in [Interval::zero(), Interval::one()] {
        println!("--- angle {:e}", angle.lo());
        let r = Mat3::rotation_about(z, angle);
        show("R.c0.x", r.c0.x);
        let old = q - r * q;
        show("old tx", old.x);
        let op = Mat3::identity_minus_rotation_about(z, angle);
        show("op.c0.x", op.c0.x);
        show("op.c0.y", op.c0.y);
        let new = op * q;
        show("new tx", new.x);
        show("new ty", new.y);
        show("new tz", new.z);
    }
    // And the normalized zero axis itself.
    let n = z.normalize();
    show("normalize(0).x", n.x);
}
