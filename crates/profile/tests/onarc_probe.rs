//! **The mismatched-r probe** (LIB-ONARC §1.1).
//!
//! The arrival verb emits its own run to the anchor along the TRUE
//! carrier (§2c amendment), so a mismatched `r` is a LEGAL new tangent
//! carrier constructed at the tip — sound by construction for every
//! authored `r`. The chain below emits mutually consistent segments
//! and validates.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tolerance};
use profile::{ArcSide, ArcSweep, Center, Open, Profile, Radius, SketchPlane, Start};
use geom_core::Tol;

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
/// continuation's `Radius` names r = 1.2 where the arrival carrier has
/// r = 1.5 (centre (7, 0)), so the derived centre is (7.3, 0).
#[test]
fn mismatched_radius_continuation() {
    let closed = Open
        .at(p2(5.05, -1.6))
        .toward(2.1_f64, 0.8, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.5,
            Center {
                c: p2(7.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(8.5, 0.0),
            },
            Tol::witness(),
        )
        .unwrap()
        // A different r than the arrival carrier's: a legal NEW tangent
        // carrier, constructed at the anchor.
        .arc_fillet(
            Radius {
                r: 1.2,
                side: ArcSide::Left,
            },
            0.5,
            Tol::witness(),
        )
        .expect("a mismatched r names a sound construction")
        .at(p2(4.05, 1.35), Tol::witness())
        .unwrap()
        .toward(-4.1, 0.3, Tol::witness())
        .unwrap()
        .line(1.0, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .expect("the chain closes");
    let lp = &closed.loop_;
    // The arrival emitted ITS OWN run to the hard anchor (8.5, 0),
    // riding the true carrier (7, 0) r 1.5 …
    let anchor_idx = lp
        .vertices()
        .iter()
        .position(|v| (v.pos() - p2(8.5, 0.0)).norm_squared().sqrt() < 1e-12)
        .expect("the authored anchor is a vertex of the final chain");
    let before = lp.vertices()[anchor_idx - 1];
    let (run_c, run_r) = circle_from_bulge(before.pos(), p2(8.5, 0.0), before.bulge());
    assert!(
        (run_c - p2(7.0, 0.0)).norm_squared().sqrt() < 1e-9 && (run_r - 1.5).abs() < 1e-9,
        "the arrival's run rides the true carrier; got ({}, {}) r {run_r}",
        run_c.x,
        run_c.y
    );
    // … and the mismatched-r continuation departs the anchor on the
    // DERIVED carrier (7.3, 0) r 1.2, tangent there by construction —
    // a declared joint at the anchor.
    let anchor_v = lp.vertices()[anchor_idx];
    let next = lp.vertices()[anchor_idx + 1];
    let (dep_c, dep_r) = circle_from_bulge(p2(8.5, 0.0), next.pos(), anchor_v.bulge());
    assert!(
        (dep_c - p2(7.3, 0.0)).norm_squared().sqrt() < 1e-9 && (dep_r - 1.2).abs() < 1e-9,
        "the continuation rides the derived carrier; got ({}, {}) r {dep_r}",
        dep_c.x,
        dep_c.y
    );
    assert!(
        lp.tangent_joints().contains(&anchor_idx),
        "the constructed tangency at the anchor is declared"
    );
    // The defect class is gone structurally: the loop validates.
    Profile::new(SketchPlane::xy(), vec![lp.clone()])
        .validate(Tol::witness())
        .expect("every authored r is sound under the dissolution");
}

/// **Sharp-after-arc-arrival, restored** (§2c dissolution): the
/// interior arc arrival lands on an ordinary directed point, so an
/// ordinary DIRECTOR — a genuine corner, junction-checked — and a
/// straight leg continue it, the spelling the OnArc state made
/// unrepresentable.
#[test]
fn sharp_after_arc_arrival() {
    let closed = Open
        .at(p2(5.05, -1.6))
        .toward(2.1_f64, 0.8, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.5,
            Center {
                c: p2(7.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(8.5, 0.0),
            },
            Tol::witness(),
        )
        .unwrap()
        // The arrival tip's tangent is +y; 2.6 rad is a genuine corner.
        .angle(2.6, Tol::witness())
        .unwrap()
        .line(1.0, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .expect("the sharp continuation closes");
    let lp = &closed.loop_;
    // The authored anchor is a VERTEX (the hard-anchor rule) and its
    // joint is SHARP: not in the declared-tangency set.
    let anchor_idx = lp
        .vertices()
        .iter()
        .position(|v| (v.pos() - p2(8.5, 0.0)).norm_squared().sqrt() < 1e-12)
        .expect("the authored anchor is a vertex");
    // The declared set is POPULATED on this same chain — the opening
    // fillet declares its own two joints — so the absence below is a
    // statement about a working datum rather than about an empty one.
    let declared = lp.tangent_joints();
    assert_eq!(
        declared.len(),
        2,
        "the opening fillet declares its two joints; got {declared:?}"
    );
    assert!(
        !declared.contains(&anchor_idx),
        "the sharp junction at the anchor is not declared tangent, among {declared:?}"
    );
    // And the continuation is the sharp one that was authored: the leg
    // leaves the anchor on the authored heading of 2.6 rad, where the
    // arrival tangent is +y — a turn of ~1.03 rad, which is the corner
    // the declared set must not contain. (A straightness check on the
    // same segment is not here: `.line()` after `.angle()` emits a zero
    // bulge by construction, so it could only restate the builder.)
    let anchor_v = lp.vertices()[anchor_idx];
    let next = lp.vertices()[(anchor_idx + 1) % lp.vertices().len()];
    let leg = next.pos() - anchor_v.pos();
    let heading = leg.y.atan2(leg.x);
    assert!(
        (heading - 2.6).abs() < 1e-9,
        "the leg departs the anchor on the authored heading; got {heading}"
    );
    Profile::new(SketchPlane::xy(), vec![lp.clone()])
        .validate(Tol::witness())
        .expect("the restored junction validates");
}
