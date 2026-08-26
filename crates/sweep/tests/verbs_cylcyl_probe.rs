//! **The opening probe** for the cylinder×cylinder germ lane: which
//! door refuses #347's cases, measured rather than assumed, on bodies
//! this suite authors through the public extrude door.
//!
//! Two families, and they meet the SAME door from opposite sides:
//!
//! - #347's cylinder unions (coaxial, parallel, Steinmetz) refuse at
//!   `CurvedPierceUnsupported` — the curved sweep arm's frontier, the
//!   door D3's containment is built to open.
//! - #347's bracket bound (`r ≤ 4` passes, `r ≥ 5` refuses) is the
//!   same frontier reached through the LINE row: the corner round's
//!   face box is the whole carrier slab (`FaceBoxRule::CylinderSlab`,
//!   `x ∈ [0, 2r]`), so the pocket's `x = 8` edge becomes a candidate
//!   exactly when `2r > 8`, and the span-dip clearance bound then
//!   cannot prove the miss it would have to prove.
//!
//! The last row is the **no-crossings silence**: a cylinder pair whose
//! walls cross in one closed loop touching no edge of either operand
//! reaches the vertex probe with no extent certificate.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A circle-derived cylinder: circle (cx, cy) of radius r, extruded
/// from z0 to z1 — exactly #347's "`circle`-derived cylinder".
fn cyl(cx: f64, cy: f64, r: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(p2(cx, cy), r, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(z1 - z0), tol)
        .unwrap()
        .body
}

fn union_err(a: &Body<f64>, b: &Body<f64>) -> BooleanError {
    topo::union(a, b, Tol::witness()).expect_err("this pair has no arm yet")
}

/// #347's "two `circle`-derived cylinders refuse to union at all
/// (coaxial or not)": every crossing pose meets the CURVED SWEEP ARM's
/// frontier — the pierce door, not a kind gate and not a join refusal.
#[test]
fn cylinder_unions_refuse_at_the_curved_pierce_door() {
    let turned = topo::transform_rigid(
        &cyl(0.0, 0.0, 1.0, -2.0, 2.0),
        &Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            PI / 2.0,
        ),
        Tol::witness(),
    )
    .unwrap();
    let rows: [(&str, BooleanError); 4] = [
        // Coaxial, equal radius, overlapping heights: B's rim circles
        // lie ON A's wall carrier, so the circle row's residual
        // extremes are Zero and the incidence is undeclared.
        (
            "coaxial-equal-r",
            union_err(&cyl(0.0, 0.0, 1.0, 0.0, 2.0), &cyl(0.0, 0.0, 1.0, 1.0, 3.0)),
        ),
        // Coaxial, equal radius, stacked cap-to-cap.
        (
            "coaxial-stacked",
            union_err(&cyl(0.0, 0.0, 1.0, 0.0, 2.0), &cyl(0.0, 0.0, 1.0, 2.0, 4.0)),
        ),
        // Parallel axes, definitely crossing walls.
        (
            "parallel-equal-r",
            union_err(&cyl(0.0, 0.0, 1.0, 0.0, 2.0), &cyl(1.2, 0.0, 1.0, 0.0, 2.0)),
        ),
        // Perpendicular axes, equal radius (the Steinmetz pair).
        (
            "steinmetz",
            union_err(&cyl(0.0, 0.0, 1.0, -2.0, 2.0), &turned),
        ),
    ];
    for (name, err) in rows {
        assert!(
            matches!(err, BooleanError::CurvedPierceUnsupported { .. }),
            "{name}: expected the curved pierce door, got {err:?}"
        );
    }
}

/// The COAXIAL UNEQUAL-radius pose (a boss on a shaft) is refused by a
/// DIFFERENT door — the both-split point lane, which needs a `Line`
/// carrier — so it is not evidence about the pierce door and is pinned
/// separately rather than folded into the row above.
#[test]
fn the_coaxial_boss_refuses_at_the_point_split_carrier_door() {
    let err = union_err(&cyl(0.0, 0.0, 1.0, 0.0, 2.0), &cyl(0.0, 0.0, 0.5, 1.0, 3.0));
    assert!(
        matches!(err, BooleanError::PointSplitCarrierUnsupported { .. }),
        "expected the point-split carrier door, got {err:?}"
    );
}

/// #347's measured bound, reproduced through the Rust doors: the
/// bracket's pocket cut passes at `r ≤ 4 mm` and refuses at `r ≥ 5 mm`,
/// which is exactly `2r > 8` — the corner round's CARRIER slab reaching
/// the pocket's `x = 8` wall. The refusal names a LINE edge of the
/// pocket against the round's cylinder face.
#[test]
fn the_bracket_bound_is_the_carrier_slab_not_the_arc() {
    let tol = Tol::witness();
    for r_mm in [3.0_f64, 4.0] {
        let plate = rounded_plate(80.0, 40.0, r_mm, 8.0);
        let pocket = slab((8.0, 28.0), (10.0, 30.0), (-2.0, 5.0));
        topo::subtract(&plate, &pocket, tol)
            .unwrap_or_else(|e| panic!("r = {r_mm} mm must cut: {e:?}"));
    }
    for r_mm in [5.0_f64, 6.0] {
        let plate = rounded_plate(80.0, 40.0, r_mm, 8.0);
        let pocket = slab((8.0, 28.0), (10.0, 30.0), (-2.0, 5.0));
        let err = topo::subtract(&plate, &pocket, tol)
            .expect_err("the carrier-slab candidate refuses today");
        assert!(
            matches!(err, BooleanError::CurvedPierceUnsupported { .. }),
            "r = {r_mm} mm: {err:?}"
        );
    }
}

/// **The executed no-crossings silence.** The pair below genuinely
/// interpenetrates, yet no vertex of either operand is inside the
/// other, so the containment fallback keeps both shells whole and the
/// union is metered as two DISJOINT solids: its volume is the SUM of
/// the operands', with the shared lens counted twice. This is the one
/// path that yields a WRONG ANSWER rather than a refusal.
#[test]
fn a_fully_crossing_cylinder_pair_with_no_edge_event_is_answered_wrong() {
    let tol = Tol::witness();
    let (a, b) = crossing_pair_without_edge_events();
    let va = topo::mass_properties(&a, tol).unwrap().volume;
    let vb = topo::mass_properties(&b, tol).unwrap().volume;
    let topo::BooleanResult::Body(out) = topo::union(&a, &b, tol).unwrap() else {
        panic!("the fallback answers rather than refusing today");
    };
    let v = topo::mass_properties(&out.body, tol).unwrap().volume;
    assert_eq!(out.body.shells().count(), 2, "two shells, kept whole");
    assert!(
        (v - (va + vb)).abs() < 1e-9,
        "the overlap is counted twice: {v} vs {va} + {vb}"
    );
}

/// The pair whose walls cross in ONE closed loop that reaches no edge
/// of either operand — A's seam meridians sit at y = 0, outside the
/// loop's θ band, and B's at z = 4 and z = 6, clear of A entirely. The
/// reduction therefore finds no crossing at all and the operation falls
/// through to the containment fallback with the boundaries genuinely
/// meeting: the S12-silence shape, for a cylinder pair.
pub(crate) fn crossing_pair_without_edge_events() -> (Body<f64>, Body<f64>) {
    let tol = Tol::witness();
    let a = cyl(0.0, 0.0, 1.0, 0.0, 10.0);
    let rod = cyl(0.0, 0.0, 1.0, -10.0, 10.0);
    let turn = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        PI / 2.0,
    );
    let lie = topo::transform_rigid(&rod, &turn, tol).unwrap();
    let b =
        topo::transform_rigid(&lie, &Affine3::translation(Vec3::new(0.0, 1.5, 5.0)), tol).unwrap();
    (a, b)
}

/// `bracket.py`'s `rounded_plate`, in millimetres.
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

/// `bracket.py`'s `slab`, in millimetres.
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
