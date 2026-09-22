//! The whole-curve order-1 jet meter: `eval` + `deriv` at one `t`
//! against `ders1` at the same `t`, on the payload door and on every
//! `Curve3` arm. On the `Nurbs` arm the pair runs two span selections
//! and two basis passes for what one order-1 pass answers; on the
//! analytic arms it builds two azimuthal frames (two `sin_cos`, two
//! `axis × u_ref`) for what one frame answers. This prints how much
//! each costs. Reporting, never gating: runs only under
//! `CAD_R1_BENCH=1`, and wants a release build to mean anything.
//!
//! The analytic table is also the ONLY instrument that can see the
//! `Circle` arm's one-frame property: an arm that built two frames
//! would produce the same bits, so no differential row reds on it,
//! and only this number moves.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::hint::black_box;
use std::sync::Arc;
use std::time::Instant;

use crate::curves::meter_fixture::{DEGREES, curve};
use geom::Curve3;
use geom_core::{Band, Point3, Vec3};

/// Nanoseconds per call of `f` over `REPS` parameters that walk the
/// domain, so the located span changes and nothing hoists.
fn ns_per<R>(reps: usize, f: impl Fn(f64) -> R) -> f64 {
    for i in 0..2_000 {
        black_box(f(black_box(0.05 + (i % 900) as f64 * 1e-3)));
    }
    let t0 = Instant::now();
    for i in 0..reps {
        black_box(f(black_box(0.05 + (i % 900) as f64 * 1e-3)));
    }
    t0.elapsed().as_nanos() as f64 / reps as f64
}

#[test]
fn ders1_meter() {
    if std::env::var("CAD_R1_BENCH").is_err() {
        return;
    }
    const REPS: usize = 200_000;
    println!("degree/interior  NurbsCurve3 eval+deriv  ders1  |  Curve3::Nurbs eval+deriv  ders1");
    for (p, interior) in DEGREES {
        let n = curve(p, interior);
        let c = Curve3::Nurbs(Arc::new(n.clone()));
        let n_pair = ns_per(REPS, |t| (n.eval(t), n.deriv(t)));
        let n_jet = ns_per(REPS, |t| n.ders1(t));
        let c_pair = ns_per(REPS, |t| (c.eval(t), c.deriv(t)));
        let c_jet = ns_per(REPS, |t| c.ders1(t));
        println!(
            "p={p}/{interior}  {n_pair:.0} ns  {n_jet:.0} ns  |  {c_pair:.0} ns  {c_jet:.0} ns"
        );
    }

    // The analytic arms, on one exactly orthonormal tilted frame.
    let center = Point3::new(-0.5, 4.0, 1.25);
    let axis = Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0);
    let u_ref = Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0);
    let arms = [
        (
            "line",
            Curve3::Line {
                origin: center,
                dir: Vec3::new(0.3, -0.4, 1.2),
            },
        ),
        (
            "circle",
            Curve3::Circle {
                center,
                axis,
                radius: 2.5,
                u_ref,
            },
        ),
        (
            "ellipse",
            Curve3::Ellipse {
                center,
                axis,
                major: 2.5,
                minor: 1.0,
                u_ref,
            },
        ),
        (
            "spiric",
            Curve3::spiric(
                center,
                axis,
                u_ref,
                1.2,
                0.225,
                0.05,
                Band::new(1e-9, 1e-8).unwrap(),
            )
            .unwrap(),
        ),
    ];
    println!("arm      Curve3 eval+deriv  ders1");
    for (name, c) in arms {
        let pair = ns_per(REPS, |t| (c.eval(t), c.deriv(t)));
        let jet = ns_per(REPS, |t| c.ders1(t));
        println!("{name:8} {pair:.0} ns  {jet:.0} ns");
    }
}
