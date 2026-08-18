//! **The mismatched-r probe** (LIB-ONARC §1.1, executed at the
//! pre-dissolution head): a `Radius` continuation of an interior arc
//! arrival whose `r` differs from the arrival carrier's is UNGUARDED —
//! the verb constructs, and the emitted run's bulge (computed by
//! `bulge_from_center` from angles alone, about the newly derived
//! centre) describes a circle that matches NEITHER the derived carrier
//! nor the true one, contradicting the tangency declared at the fillet
//! arc's end. Nothing refuses until the LATE validate pass
//! (`TangencyContradicted`).
//!
//! Executed finding at this head, pinned below: chain constructs; the
//! run's recovered carrier sits ~0.26 m off the carrier the verb
//! derived; validate refuses `TangencyContradicted` at that joint.

use geom_core::{Point2, Tolerance};
use profile::{
    ArcSide, ArcSweep, Center, Open, Profile, ProfileError, Radius, SketchPlane, Start,
};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// Recover the circle (center, radius) an emitted chord + bulge pair
/// actually describes — the independent decoding of the stored form.
fn circle_from_bulge(t1: Point2<f64>, t2: Point2<f64>, b: f64) -> (Point2<f64>, f64) {
    let (cx, cy) = (t2.x - t1.x, t2.y - t1.y);
    let l = cx.hypot(cy);
    let d = l / 2.0;
    let radius = d * (1.0 + b * b) / (2.0 * b.abs());
    let k = d * (1.0 - b * b) / (2.0 * b);
    let (mx, my) = ((t1.x + t2.x) / 2.0, (t1.y + t2.y) / 2.0);
    let (ux, uy) = (cx / l, cy / l);
    (Point2::new(mx - uy * k, my + ux * k), radius)
}

/// The boss chain of the family.rs module doc, with ONE change: the
/// continuation's `Radius` names r = 1.2 where the tip's carrier has
/// r = 1.5 (centre (7, 0)), so the derived centre is (7.3, 0).
#[test]
fn mismatched_radius_continuation() {
    let closed = Open
        .at(p2(5.05, -1.6))
        .toward(2.1_f64, 0.8)
        .unwrap()
        .fillet_arc(
            0.5,
            Center {
                c: p2(7.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(8.5, 0.0),
            },
        )
        .unwrap()
        // The hole: r = 1.2 on an r = 1.5 carrier is accepted as-is.
        .arc_fillet(
            Radius {
                r: 1.2,
                side: ArcSide::Left,
            },
            0.5,
        )
        .expect("UNGUARDED: the mismatched radius is accepted at the verb")
        .at(p2(4.05, 1.35))
        .unwrap()
        .toward(-4.1, 0.3)
        .unwrap()
        .line(1.0)
        .unwrap()
        .line_to(Start)
        .expect("UNGUARDED: the chain closes without any refusal");
    let lp = &closed.loop_;
    // The run leaves the arrival tangent point v2 for the next trim v3;
    // decode the circle its emitted chord + bulge actually describe.
    let (t2, t1p, b) = (
        lp.vertices()[2].pos(),
        lp.vertices()[3].pos(),
        lp.vertices()[2].bulge(),
    );
    let (rec_c, _rec_r) = circle_from_bulge(t2, t1p, b);
    // The carrier the verb derived from the tip's bits and r = 1.2:
    // centre = (8.5, 0) + 1.2 * n̂(+y) = (7.3, 0). The emitted run is on
    // NEITHER that circle nor the true (7, 0) carrier — a mutually
    // inconsistent (bulge, claimed centre, declared tangency) triple.
    let off_derived = (rec_c - p2(7.3, 0.0)).norm_squared().sqrt();
    let off_true = (rec_c - p2(7.0, 0.0)).norm_squared().sqrt();
    assert!(
        off_derived > 0.1 && off_true > 0.1,
        "recovered centre ({}, {}) — {off_derived} off the derived carrier, \
         {off_true} off the true one",
        rec_c.x,
        rec_c.y
    );
    // Only the LATE validate pass catches the contradiction.
    let err = Profile::new(SketchPlane::xy(), vec![lp.clone()])
        .validate(Tolerance::get())
        .expect_err("the defect is caught only at validation");
    assert!(
        matches!(err, ProfileError::TangencyContradicted { .. }),
        "expected TangencyContradicted, got {err:?}"
    );
}
