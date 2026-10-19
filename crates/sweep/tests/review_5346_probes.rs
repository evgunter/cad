//! Review probes for PR 5346 (`surgery::arc_window`). Probe P1 mirrors
//! `arc_window`'s two-anchor read over `Curve3::param_near` (the
//! function is `pub(super)`), so it tests the arithmetic the fix rests
//! on, not the call sites; the ladder probe P2 tests the call sites.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom::Curve3;
use geom_core::{Bounds, Interval, Point3, Real, Tol, Vec3};
use sweep::blend::build::{chamfer_edges, fillet_edges};

fn window(
    c: &Curve3<Interval>,
    p0: Point3<Interval>,
    p1: Point3<Interval>,
) -> (Interval, Interval) {
    let at_seam = c.param_near(p0, Interval::zero()).unwrap();
    let t0 = if at_seam.hi() - at_seam.lo() < PI {
        at_seam
    } else {
        c.param_near(p0, Interval::pi()).unwrap()
    };
    (t0, c.param_near(p1, t0).unwrap())
}

fn ivp(p: Point3<f64>) -> Point3<Interval> {
    Point3::new(
        Interval::from_f64(p.x),
        Interval::from_f64(p.y),
        Interval::from_f64(p.z),
    )
}

/// P1: for an ellipse (and a near-circle, and a tiny one) at angles on
/// and beside both cuts, the chosen `t0` is narrow, is a parameter of
/// the same point (its `f64` read differs by a whole number of turns),
/// and the window to a point `Δ` further on reads `Δ` forward.
#[test]
fn p1_the_two_anchor_read_is_narrow_same_point_and_forward() {
    let axis = Vec3::new(2.0, 2.0, 1.0) / 3.0;
    let u_ref = Vec3::new(1.0, -2.0, 2.0) / 3.0;
    let w = axis.cross(u_ref);
    let center = Point3::new(0.5, -1.0, 2.0);
    let mut worst = 0.0f64;
    for (a, b) in [
        (2.0, 1.0),
        (1.0, 1.0 - 1e-9),
        (1.0, 1.0),
        (1e-6, 5e-7),
        (3.0, 0.01),
    ] {
        let fc = Curve3::Ellipse {
            center,
            axis,
            semi_u: a,
            semi_v: b,
            u_ref,
        };
        let ic = Curve3::Ellipse {
            center: ivp(center),
            axis: Vec3::new(
                Interval::from_f64(axis.x),
                Interval::from_f64(axis.y),
                Interval::from_f64(axis.z),
            ),
            semi_u: Interval::from_f64(a),
            semi_v: Interval::from_f64(b),
            u_ref: Vec3::new(
                Interval::from_f64(u_ref.x),
                Interval::from_f64(u_ref.y),
                Interval::from_f64(u_ref.z),
            ),
        };
        let at = |th: f64| center + u_ref * (a * th.cos()) + w * (b * th.sin());
        for th in [
            PI,
            PI - 1e-15,
            PI + 1e-15,
            PI - 1e-9,
            PI + 1e-9,
            0.0,
            1e-15,
            -1e-15,
            PI / 2.0,
            -PI / 2.0,
            3.0,
            -3.0,
            0.1,
        ] {
            for d in [0.3, 1.0, 2.5] {
                let (p0, p1) = (at(th), at(th + d));
                let (t0, t1) = window(&ic, ivp(p0), ivp(p1));
                let f0 = fc.param_near(p0, 0.0).unwrap();
                let what = format!("a {a} b {b} θ {th} Δ {d}: t0 {t0:?} t1 {t1:?} f64 {f0}");
                let wd = t0.hi() - t0.lo();
                worst = worst.max(wd);
                assert!(wd < 1e-6, "{what}: t0 narrow");
                let mid = 0.5 * (t0.lo() + t0.hi());
                let k = (mid - f0) / TAU;
                assert!(
                    (k - k.round()).abs() < 1e-6 && k.round().abs() <= 1.0,
                    "{what}: same point"
                );
                let span = t1 - t0;
                // Parameter Δ on the ellipse's own angle: atan2 of the
                // scaled coordinates is the eccentric anomaly, so Δ is
                // read exactly (up to rounding).
                assert!(
                    span.lo() > 0.0 && (0.5 * (span.lo() + span.hi()) - d).abs() < 1e-6,
                    "{what}: forward by Δ, got {span:?}"
                );
            }
        }
    }
    println!("P1 widest t0: {worst:e}");
}

/// P2: the PR's shear ladder, every row's `Interval` outcome printed
/// (Ok / the refusal) and nothing asserted beyond "f64 builds" — run it
/// under a mutant to see which rows reach the cut.
#[test]
#[ignore = "diagnostic: cargo nextest run -p sweep --run-ignored all -E 'test(p2_)' --no-capture"]
fn p2_which_ladder_rows_reach_the_cut() {
    let tol = Tol::witness();
    for s in [-0.6, -0.3, -0.1, 0.1, 0.3, 0.6] {
        for convex in [true, false] {
            let (ibody, iedge) =
                crate::band_planar_cut_off_interval::sheared::<Interval>(s, convex, tol);
            for round in [false, true] {
                let r = if round {
                    fillet_edges(&ibody, &[iedge], Interval::from_f64(0.1), tol)
                } else {
                    chamfer_edges(&ibody, &[iedge], Interval::from_f64(0.1), tol)
                };
                println!(
                    "P2 shear {s} convex {convex} round {round}: {}",
                    match r {
                        Ok(_) => "Ok".to_string(),
                        Err(e) => format!("{:?}", e.error).chars().take(140).collect(),
                    }
                );
            }
        }
    }
}

/// P1c: P1's read on a circle carrier (the perpendicular end's arc),
/// whose `param_near` reads through `ders1(near)` rather than the
/// ellipse's scaled frame: at `T::pi()` the anchor's frame is itself an
/// enclosure.
#[test]
fn p1c_the_two_anchor_read_on_a_circle() {
    let axis = Vec3::new(2.0, 2.0, 1.0) / 3.0;
    let u_ref = Vec3::new(1.0, -2.0, 2.0) / 3.0;
    let w = axis.cross(u_ref);
    let center = Point3::new(0.5, -1.0, 2.0);
    let iv3 = |v: Vec3<f64>| {
        Vec3::new(
            Interval::from_f64(v.x),
            Interval::from_f64(v.y),
            Interval::from_f64(v.z),
        )
    };
    for r in [1.0, 1e-6, 1e3] {
        let fc = Curve3::Circle {
            center,
            axis,
            radius: r,
            u_ref,
        };
        let ic = Curve3::Circle {
            center: ivp(center),
            axis: iv3(axis),
            radius: Interval::from_f64(r),
            u_ref: iv3(u_ref),
        };
        let at = |th: f64| center + u_ref * (r * th.cos()) + w * (r * th.sin());
        for th in [PI, PI - 1e-15, PI + 1e-15, 0.0, 1e-15, -1e-15, 3.0, -3.0] {
            let d = 1.0;
            let (t0, t1) = window(&ic, ivp(at(th)), ivp(at(th + d)));
            let f0 = fc.param_near(at(th), 0.0).unwrap();
            let what = format!("r {r} θ {th}: t0 {t0:?} t1 {t1:?} f64 {f0}");
            assert!(t0.hi() - t0.lo() < 1e-6, "{what}: narrow");
            let k = (0.5 * (t0.lo() + t0.hi()) - f0) / TAU;
            assert!((k - k.round()).abs() < 1e-6, "{what}: same point");
            let span = t1 - t0;
            assert!(
                span.lo() > 0.0 && (0.5 * (span.lo() + span.hi()) - d).abs() < 1e-6,
                "{what}: forward"
            );
        }
    }
}
