//! Blinded-review probe rows for VERBS-CYLCYL PR-B (ordinal 80) —
//! attacks on the conservatism fixes' claims, kept on the review's
//! probe branch only.
//!
//! - The bracket's boundary just past the pinned r = 6: the door must
//!   refuse honestly or meter the closed form EXACTLY — a minted wrong
//!   volume is the failure being hunted (PR-A's 30π lesson).
//! - The clamped span-dip bound, checked against the exact quadratic
//!   minimum across the whole vertex-position range, including a
//!   vertex just inside the span end — and the "never worse than 2x"
//!   prose claim measured.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::Body;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// `bracket.py`'s `rounded_plate`, verbatim from the unit's fixture.
fn rounded_plate(w: f64, h: f64, r: f64, thick: f64) -> Body<f64> {
    let tol = Tol::witness();
    let outline = profile::Open
        .at(p2(w / 2.0, 0.0))
        .toward(1.0, 0.0, tol)
        .unwrap()
        .fillet(r, tol)
        .unwrap()
        .toward(0.0, 1.0, tol)
        .unwrap()
        .to(p2(w, h / 2.0), tol)
        .unwrap()
        .fillet(r, tol)
        .unwrap()
        .toward(-1.0, 0.0, tol)
        .unwrap()
        .to(p2(w / 2.0, h), tol)
        .unwrap()
        .fillet(r, tol)
        .unwrap()
        .toward(0.0, -1.0, tol)
        .unwrap()
        .to(p2(0.0, h / 2.0), tol)
        .unwrap()
        .fillet(r, tol)
        .unwrap()
        .to(profile::Start, tol)
        .unwrap();
    let plane = SketchPlane::new(Affine3::identity());
    let prof = Profile::new(plane, vec![outline.into()])
        .validate(tol)
        .unwrap();
    extrude(&prof, Extrusion::Distance(thick), tol)
        .unwrap()
        .body
}

fn slab(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    let tol = Tol::witness();
    let lp =
        profile::ProfileLoop::polygon([p2(x.0, y.0), p2(x.1, y.0), p2(x.1, y.1), p2(x.0, y.1)]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z.0)));
    let prof = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    extrude(&prof, Extrusion::Distance(z.1 - z.0), tol)
        .unwrap()
        .body
}

/// **The boundary just past the pin.** The unit pins r in {3, 4, 5, 6};
/// the closed form `(80*40 - r^2*(4 - pi))*8 - 20*20*5` stays exactly
/// valid for every r < 10 (the corner rounds' material lives in
/// x, y in [0, r]^2 at each corner, and the pocket floor starts at
/// (8, 10), so the two regions are disjoint until r reaches 10). So at
/// r in {6.5, 7, 8, 9.5} the door has exactly two honest answers: a
/// typed refusal, or a body metering the closed form. A body metering
/// anything ELSE is the wrong-answer path this row hunts.
#[test]
fn past_the_pinned_radius_the_door_refuses_or_meters_exactly() {
    let tol = Tol::witness();
    for r in [6.5_f64, 7.0, 8.0, 9.5] {
        let plate = rounded_plate(80.0, 40.0, r, 8.0);
        let pocket = slab((8.0, 28.0), (10.0, 30.0), (-2.0, 5.0));
        match topo::subtract(&plate, &pocket, tol) {
            Err(e) => {
                // A typed refusal is honest; record which door.
                eprintln!("r = {r}: refused at {e:?}");
            }
            Ok(topo::BooleanResult::Body(bb)) => {
                let v = topo::mass_properties(&bb.body, tol).unwrap().volume;
                let expect =
                    (80.0 * 40.0 - r * r * (4.0 - core::f64::consts::PI)) * 8.0 - 20.0 * 20.0 * 5.0;
                assert!(
                    (v - expect).abs() < 1e-9,
                    "r = {r}: MINTED WRONG VOLUME {v}, closed form {expect}"
                );
            }
            Ok(other) => panic!("r = {r}: the cut cannot empty the plate, got {other:?}"),
        }
    }
}

/// **The tangential contact at r = 10.** The pocket's floor corner
/// (8, 10) touches the corner region's y = 10 rim exactly; the closed
/// form still holds (the overlap is measure-zero). Either honest
/// answer is accepted; a minted wrong volume is not.
#[test]
fn the_tangential_radius_ten_refuses_or_meters_exactly() {
    let tol = Tol::witness();
    let plate = rounded_plate(80.0, 40.0, 10.0, 8.0);
    let pocket = slab((8.0, 28.0), (10.0, 30.0), (-2.0, 5.0));
    match topo::subtract(&plate, &pocket, tol) {
        Err(e) => eprintln!("r = 10: refused at {e:?}"),
        Ok(topo::BooleanResult::Body(bb)) => {
            let v = topo::mass_properties(&bb.body, tol).unwrap().volume;
            let expect = (80.0 * 40.0 - 100.0 * (4.0 - core::f64::consts::PI)) * 8.0 - 2000.0;
            assert!(
                (v - expect).abs() < 1e-9,
                "r = 10: MINTED WRONG VOLUME {v}, closed form {expect}"
            );
        }
        Ok(other) => panic!("r = 10: unexpected {other:?}"),
    }
}

/// **The clamped span-dip bound, against the exact quadratic.** For
/// the exactly-quadratic residual along a line (`f'' = q` constant in
/// normalized span units), the true dip below the lower endpoint is
/// `(q/2 - |m|)^2 / (2q)` when `|m| < q/2` and zero otherwise. The
/// code's charge is `max(0, q/2 - |m|) / 4`. This row:
///
/// - verifies charge >= true dip across the whole vertex range,
///   including the vertex JUST INSIDE the span end (`|m| = q/2 - eps`)
///   and just outside (`|m| = q/2 + eps`), by minimizing the actual
///   quadratic numerically rather than trusting the closed form;
/// - MEASURES the looseness ratio, because the comment in
///   `reduce.rs` claims "never more than a factor two loose between":
///   at `m = 3q/8` the ratio is 4, and it grows without bound as the
///   vertex approaches the span end from inside. The claim is false
///   as written (the bound stays SOUND — only the prose overclaims).
#[test]
fn the_span_dip_charge_bounds_the_true_dip_and_the_2x_prose_is_false() {
    let q = 1.7_f64; // arbitrary positive scale
    let charge = |m: f64| (q / 2.0 - m.abs()).max(0.0) / 4.0;
    // f(s) = (q/2) s^2 + (m - q/2) s on s in [0, 1]; f(0) = 0,
    // f(1) = m, so the dip below min(f(0), f(1)) is
    // -(min_s f - min(0, m)).
    let true_dip = |m: f64| {
        let f = |s: f64| 0.5 * q * s * s + (m - q / 2.0) * s;
        let mut min = f(0.0).min(f(1.0));
        for i in 0..=100_000 {
            min = min.min(f(f64::from(i) / 100_000.0));
        }
        (min - f(0.0).min(f(1.0))).min(0.0).abs()
    };
    // Soundness across the range, both signs of m, vertex straddling
    // the span end.
    for m in [
        0.0,
        0.1 * q,
        0.25 * q,
        0.375 * q,
        0.499 * q,
        0.5 * q - 1e-9,
        0.5 * q,
        0.5 * q + 1e-9,
        0.75 * q,
        2.0 * q,
    ] {
        for m in [m, -m] {
            let (c, t) = (charge(m), true_dip(m));
            assert!(
                c >= t - 1e-12,
                "charge {c} under-covers true dip {t} at m = {m}"
            );
        }
    }
    // The 2x prose claim, measured: at m = 3q/8 the ratio is 4.
    let m = 0.375 * q;
    let ratio = charge(m) / true_dip(m);
    assert!(
        ratio > 3.9,
        "expected ratio ~4 at m = 3q/8 (the 'never worse than 2x' \
         comment is false as written); measured {ratio}"
    );
}
