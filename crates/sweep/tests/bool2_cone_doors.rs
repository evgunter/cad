//! Issue 1011, the CONE half: the cone doors of
//! `topo::boolean::point_in_solid`.
//!
//! Before this unit `face_geo` resolved `{Plane, Cylinder, Sphere}` and
//! refused every cone face as `KindUnsupported` — a HEALTHY body and a
//! missing capability, and the refusal every containment door
//! downstream of the pair-scoped operand gate inherited. The arm this
//! unit lands is the ray×cone quadratic plus the cone chart's own trim:
//! an azimuth window (the cylinder arm's cosine construction, now
//! shared rather than copied) and a SLANT window in metres along the
//! generator.
//!
//! Two cone classes are exercised, and both are here because a cone's
//! APEX is a junction the closed-form azimuth walk cannot cross:
//!
//! - the **azimuth-WRAPPED group** — a full revolve's two cone bands,
//!   which have no azimuth boundary between them and the rest of the
//!   body, so the slant window alone trims them and the arms act for
//!   one representative;
//! - the **azimuth-TRIMMED face** — a partial revolve's single cone
//!   sector, whose window the walk does get right, served per face.
//!
//! The rows, and what each one is the only witness for:
//!
//! - **interior / exterior / boundary** on a full cone, whose only
//!   curved faces are cone faces, so nothing else can be answering;
//! - **the pierce arm** across the wall, both sides, at a generic
//!   azimuth no schedule ray runs along;
//! - **the apex**: ON the boundary of an apex-closed cone (it is the
//!   solid's tip) — and the MIRROR NAPPE, whose points the infinite
//!   double cone contains and this solid does not. A quadratic without
//!   the nappe test answers those `In`, and a quadratic without the
//!   slant window answers the mirror nappe's SURFACE `OnBoundary`;
//! - **the slant window**, on a frustum: a point exactly on the cone's
//!   CARRIER but past the face's own extent is outside the solid, not
//!   on its boundary;
//! - **grazing escalates rather than answers**: from a frustum's
//!   VIRTUAL apex every ray meets the carrier in a double root, so the
//!   discriminant is zero for every schedule member and the door
//!   refuses `RayExhausted` instead of guessing a parity;
//! - **the consumer unlock**: a disjoint union whose no-crossings
//!   fallback walks a containment door. Red on main as
//!   `KindUnsupported { kind: Cone }`, green here.
//!
//! ε honesty: every probe offset is derived from the RESOLVED band
//! (`Tol::witness().get().eps`), never a literal, so each row means the
//! same thing in each ε lane. The three-outcome rows say which outcome
//! they pin and why the band cannot reach the other two.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use geom_core::{Point3, Tol, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, SketchPlane};
use revolve_common::*;
use sweep::{Extrusion, Revolution, extrude, revolve};
use topo::{Body, BooleanError, PointInSolidError, SolidContainment, point_in_solid};

/// The `revolve_cone` acceptance's profile: the right triangle
/// (0,0), (1,0), (0,1). The slant edge is the cone (apex at (0,1,0),
/// half-angle π/4), the base edge the disc, the axis edge omitted.
fn triangle() -> ProfileLoop<f64> {
    ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)])
}

/// The full cone: base disc of radius 1 at y = 0, apex at (0, 1, 0).
/// Its cone faces are APEX-CLOSED — their slant window reaches v = 0.
fn cone() -> Body<f64> {
    let vp = validated(vec![triangle()]);
    revolve(&vp, axis_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// A frustum: base disc radius 1 at y = 0, top disc radius 0.5 at
/// y = 1. Its cone faces are clear of the apex — the carrier's apex is
/// the VIRTUAL one at (0, 2, 0), a point of free space above the body.
/// The carrier is `ρ = 1 − y/2`.
fn frustum() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.5, 1.0), p2(0.0, 1.0)]);
    let vp = validated(vec![lp]);
    revolve(&vp, axis_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// A quarter cone: the same triangle through a π/2 revolve. Its ONE
/// cone face is azimuth-trimmed (window `[−π, −π/2]`, the quadrant
/// `x > 0, z < 0`), and it is a legal boolean operand — a full
/// revolve's base disc is two half-discs on ONE plane key, which the
/// maximal-faces precondition refuses before containment is ever asked.
fn quarter_cone() -> Body<f64> {
    revolve(
        &validated(vec![triangle()]),
        axis_y(),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap()
    .body
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(x.0, y.0), p2(x.1, y.0), p2(x.1, y.1), p2(x.0, y.1)]);
    let plane = SketchPlane::new(geom_core::Affine3::translation(Vec3::new(0.0, 0.0, z.0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(z.1 - z.0), Tol::witness())
        .unwrap()
        .body
}

fn band() -> geom_core::Band {
    geom_core::Band::linear(Tol::witness()).unwrap()
}

/// The probe offset: derived from the RESOLVED band, so each ε lane
/// probes at its own scale rather than at a hard-coded distance, and
/// CLAMPED at both ends because a probe has two jobs at once and the
/// bodies here are of unit size.
///
/// The floor keeps the offset definitely outside the ambiguity band on
/// the fine lanes (`escalate` is within an order or two of `eps`, so
/// `1e-3` clears every row in the matrix by five orders). The ceiling
/// keeps it INSIDE the body on the coarse ones: an offset that scales
/// freely with `eps` reaches 1.0 at the 1e-6 row, and a probe "just
/// inside the wall" of a unit cone at that distance is not inside the
/// cone at all — it is out the other side, and the row would be
/// asserting about geometry it did not mean.
fn away() -> f64 {
    (1e6 * Tol::witness().get().eps).clamp(1e-3, 0.1)
}

fn pis(body: &Body<f64>, q: Point3<f64>) -> SolidContainment {
    point_in_solid(body, q, band(), Tol::witness()).unwrap()
}

/// A point on the full cone's wall at height `y` and azimuth `phi`
/// (`ρ = 1 − y` there), and the wall's OUTWARD unit normal at it — the
/// meridian gradient of `ρ + y − 1`, which is `(r̂ + ŷ)/√2` for the
/// π/4 half-angle.
fn on_wall(y: f64, phi: f64) -> (Point3<f64>, Vec3<f64>) {
    let (s, c) = phi.sin_cos();
    let rho = 1.0 - y;
    let n = Vec3::new(c, 1.0, s) / 2.0_f64.sqrt();
    (Point3::new(rho * c, y, rho * s), n)
}

/// Interior, exterior and boundary on a body whose only curved faces
/// are cone faces — so every verdict here is the new arm's.
#[test]
fn cone_door_classifies_the_cone_interior_exterior_and_boundary() {
    let body = cone();
    assert_all_tiers(&body);
    // One plane (the base disc) and one cone, each shared by its two
    // band faces: nothing here has a cylinder or sphere arm to lean on.
    assert_eq!(body.surfaces().count(), 2);

    // Interior, off every axis so no schedule ray runs along a symmetry.
    for q in [
        Point3::new(0.2, 0.3, 0.15),
        Point3::new(-0.1, 0.05, 0.2),
        Point3::new(0.0, 0.5, 0.0),
    ] {
        assert_eq!(pis(&body, q), SolidContainment::In, "interior probe {q:?}");
    }
    // Exterior: outside the wall at mid-height, below the base, and far
    // enough away that some schedule members cross nothing at all and
    // the verdict comes from the at-infinity side.
    for q in [
        Point3::new(0.9, 0.9, 0.0),
        Point3::new(0.2, -away(), 0.1),
        Point3::new(10.0, 7.0, 3.0),
    ] {
        assert_eq!(pis(&body, q), SolidContainment::Out, "exterior probe {q:?}");
    }
    // ON the wall — the boundary pre-pass, whose cone residual is the
    // EXACT perpendicular distance to the nappe, not a linearization.
    for (y, phi) in [(0.25, 0.7), (0.5, 2.9), (0.75, -1.4)] {
        let (p, _) = on_wall(y, phi);
        assert_eq!(
            pis(&body, p),
            SolidContainment::OnBoundary,
            "on-wall probe y = {y}, φ = {phi}"
        );
    }
    // ON the apex — the solid's tip, which the arm reports through the
    // same `None` the ray lane treats as a graze.
    //
    // The base DISC is deliberately not probed here. A point in the
    // interior of a revolved half-disc face is misread by the PLANAR
    // arm, on bodies with no cone in them at all (a revolved cylinder
    // answers `Out` for a point on its own cap), so a row here would be
    // pinning another door's defect through this one.
    assert_eq!(
        pis(&body, Point3::new(0.0, 1.0, 0.0)),
        SolidContainment::OnBoundary,
        "the apex is ON the boundary of an apex-closed cone"
    );
}

/// The pierce arm proper: a pair straddling the wall along its own
/// outward normal, at a generic azimuth. Both queries are definitely
/// off every face, so the pre-pass declines and the verdict is the
/// ray×cone quadratic's.
#[test]
fn cone_pierce_reads_the_material_side_across_the_wall() {
    let body = cone();
    let d = away();
    for (y, phi) in [(0.3, 0.9), (0.6, -2.2)] {
        let (p, n) = on_wall(y, phi);
        assert_eq!(
            pis(&body, p - n * d),
            SolidContainment::In,
            "inside the wall at y = {y}"
        );
        assert_eq!(
            pis(&body, p + n * d),
            SolidContainment::Out,
            "outside the wall at y = {y}"
        );
    }
}

/// **The mirror nappe is not this solid.** The infinite double cone
/// contains the whole axis above the apex and its entire upper sheet;
/// the body does not. These are the rows a quadratic with no nappe test
/// gets wrong — `In` for the axis points, `OnBoundary` for the points
/// on the mirror sheet itself — and they are wrong in the direction
/// that silently corrupts a boolean rather than refusing it.
#[test]
fn the_mirror_nappe_and_the_neighbourhood_of_the_apex() {
    let body = cone();
    let d = away();
    // Just below the apex, on the axis: inside (the wall is `d/√2` off).
    assert_eq!(
        pis(&body, Point3::new(0.0, 1.0 - d, 0.0)),
        SolidContainment::In
    );
    // Just above it, on the axis: the mirror nappe's interior.
    assert_eq!(
        pis(&body, Point3::new(0.0, 1.0 + d, 0.0)),
        SolidContainment::Out,
        "the mirror nappe's axis is outside the solid"
    );
    // ON the mirror sheet (ρ = y − 1), where the carrier's residual
    // vanishes for the WRONG nappe and only the nappe sign tells them
    // apart.
    for (h, phi) in [(0.3_f64, 0.4_f64), (1.0, 2.5)] {
        let (s, c) = phi.sin_cos();
        let q = Point3::new(h * c, 1.0 + h, h * s);
        assert_eq!(
            pis(&body, q),
            SolidContainment::Out,
            "a point ON the mirror nappe is outside, not on the boundary"
        );
    }
}

/// **The slant window trims the carrier, not just the nappe.** A
/// frustum's cone face is a band of its carrier; a point exactly on the
/// carrier but past either end of that band is outside the SOLID, and
/// answering `OnBoundary` there would be the infinite cone's answer,
/// not the face's.
#[test]
fn a_carrier_point_beyond_the_slant_window_is_outside_the_frustum() {
    let body = frustum();
    assert_all_tiers(&body);
    // Controls: interior, and a point ON the trimmed band.
    assert_eq!(pis(&body, Point3::new(0.2, 0.5, 0.1)), SolidContainment::In);
    for (y, phi) in [(0.25_f64, 1.1_f64), (0.8, -0.6)] {
        let (s, c) = phi.sin_cos();
        let rho = 1.0 - y / 2.0;
        assert_eq!(
            pis(&body, Point3::new(rho * c, y, rho * s)),
            SolidContainment::OnBoundary,
            "on the trimmed band at y = {y}"
        );
    }
    // On the carrier, past the top rim (y > 1) and past the base rim
    // (y < 0) — the same surface, outside the face's window.
    for y in [1.0 + away(), 1.5, -away(), -0.5] {
        let rho = 1.0 - y / 2.0;
        let q = Point3::new(rho, y, 0.0);
        assert_eq!(
            pis(&body, q),
            SolidContainment::Out,
            "carrier point beyond the slant window at y = {y}"
        );
    }
}

/// **Grazing escalates rather than answers.** At the frustum's VIRTUAL
/// apex the ray×cone quadratic has `B = C = 0` for every direction, so
/// its discriminant is zero and every schedule member is tangent — the
/// double root is the apex itself. The door exhausts the schedule and
/// says so, typed; it does not fall back to a parity guess, and it does
/// not report the point (which is free space, a unit above the body) as
/// on the boundary.
///
/// The three-outcome reading: the discriminant is EXACTLY zero here,
/// not merely in-band, so the row pins the same outcome in every ε
/// lane — no band can push a zero discriminant to a definite sign.
#[test]
fn every_ray_from_the_virtual_apex_grazes_and_the_door_escalates() {
    let body = frustum();
    let err =
        point_in_solid(&body, Point3::new(0.0, 2.0, 0.0), band(), Tol::witness()).unwrap_err();
    let PointInSolidError::RayExhausted = err else {
        panic!("expected the grazing escalation, got {err:?}");
    };
    let msg = err.to_string();
    assert!(msg.contains("grazed"), "{msg}");
    assert!(msg.contains("ill-conditioned"), "{msg}");
}

/// **The azimuth-trimmed class.** A partial revolve's cone face has a
/// real azimuth window, and the window is what tells the swept quadrant
/// from the three the carrier also passes through. Every probe below is
/// at the SAME radius and height — only the azimuth differs — so
/// nothing but the azimuth trim can be separating them.
///
/// The quadrant is `x > 0, z < 0` (the fan walls are the planes `x = 0`
/// and `z = 0`), which in the cone chart is `u ∈ (−π, −π/2)`: the
/// window is stated in CHART azimuth, and this face lies on the `v < 0`
/// nappe, where the chart's radial is the negation of the physical one.
#[test]
fn the_azimuth_window_selects_the_swept_quadrant() {
    let body = quarter_cone();
    assert_all_tiers(&body);
    assert_eq!(
        pis(&body, Point3::new(0.2, 0.2, -0.15)),
        SolidContainment::In
    );
    for q in [
        Point3::new(0.2, 0.2, 0.15),
        Point3::new(-0.2, 0.2, -0.15),
        Point3::new(-0.2, 0.2, 0.15),
    ] {
        assert_eq!(
            pis(&body, q),
            SolidContainment::Out,
            "an unswept quadrant at the same radius and height: {q:?}"
        );
    }
    // ON the trimmed cone face, inside the window: ρ = 1 − y.
    assert_eq!(
        pis(&body, Point3::new(0.4, 0.5, -0.3)),
        SolidContainment::OnBoundary
    );
    // The same point mirrored into an unswept quadrant is ON the
    // CARRIER and outside the face — outside the solid, not on it.
    assert_eq!(
        pis(&body, Point3::new(-0.4, 0.5, 0.3)),
        SolidContainment::Out,
        "the carrier outside the azimuth window is not this face"
    );
}

/// **The consumer unlock.** A disjoint union has no crossings at all,
/// so the pipeline falls through to the containment door and walks
/// every face of the cone-bearing operand. On main that door refuses
/// `KindUnsupported { kind: Cone }` — the pair-scoped operand gate
/// admits the operation (the boxes are four units apart) and the
/// containment question then cannot be asked. With the arm the union
/// assembles.
///
/// The operand is the QUARTER cone: a full revolve's base disc is two
/// half-discs sharing one plane key, which the maximal-faces
/// precondition (F7) refuses before any containment door is reached —
/// a planar precondition, nothing to do with this arm.
#[test]
fn a_disjoint_union_with_a_cone_face_now_assembles() {
    let a = quarter_cone();
    let b = brick((5.0, 6.0), (0.0, 1.0), (-1.0, 0.0));
    let out = match topo::union(&a, &b, Tol::witness()) {
        Ok(out) => out,
        Err(BooleanError::Containment(e)) => panic!(
            "the containment door still refuses a cone operand — this is the \
             refusal issue 1011's cone half retires: {e}"
        ),
        Err(other) => panic!("unexpected refusal: {other:?}"),
    };
    let result = out.body().expect("a disjoint union is not empty");
    assert_eq!(result.kind, topo::BooleanResultKind::Assembly);
    assert_eq!(topo::validate_closed(&result.body), Ok(()));
    // Both operands' material survives, and the containment door had to
    // answer for each.
    assert_eq!(
        pis(&result.body, Point3::new(0.2, 0.2, -0.15)),
        SolidContainment::In
    );
    assert_eq!(
        pis(&result.body, Point3::new(5.5, 0.5, -0.5)),
        SolidContainment::In
    );
    assert_eq!(
        pis(&result.body, Point3::new(3.0, 0.5, -0.5)),
        SolidContainment::Out
    );
}

/// The refusal that STAYS. The cone arm retires `KindUnsupported` for
/// `Cone` and for nothing else, and the message must stop offering
/// "express it without a cone face" as the recourse for a kind that now
/// has an arm. Torus and NURBS keep the variant, and keep it as a
/// capability claim about a HEALTHY body.
#[test]
fn the_kind_refusal_no_longer_names_the_cone() {
    let body = cone();
    // Nothing in a cone-bearing body reaches the refusal any more.
    for q in [
        Point3::new(0.1, 0.2, 0.1),
        Point3::new(4.0, 4.0, 4.0),
        Point3::new(0.0, 1.0, 0.0),
    ] {
        let r = point_in_solid(&body, q, band(), Tol::witness());
        assert!(
            !matches!(r, Err(PointInSolidError::KindUnsupported { .. })),
            "a cone face must not refuse by kind any more: {r:?}"
        );
    }
    let msg = PointInSolidError::KindUnsupported {
        face: body.faces().next().unwrap().0,
        kind: geom_brep::SurfaceKind::Torus,
    }
    .to_string();
    assert!(msg.contains("HEALTHY"), "{msg}");
    assert!(!msg.contains("corrupt"), "{msg}");
    assert!(
        !msg.contains("cone"),
        "the recourse must not tell a caller to avoid a kind that has an arm: {msg}"
    );
    assert!(msg.contains("torus"), "{msg}");

    // The arm's OWN refusal, for a cone face in neither chart class.
    // No public door mints one today — it wants a ringed cone face or
    // two bands stacked on one cone key — so what is pinned here is the
    // claim the message makes, not a body that reaches it.
    let msg = PointInSolidError::PartialConeFace {
        face: body.faces().next().unwrap().0,
    }
    .to_string();
    assert!(msg.contains("HEALTHY"), "{msg}");
    assert!(msg.contains("Recourse"), "{msg}");
    assert!(
        msg.contains("apex"),
        "the refusal must name the junction it is about: {msg}"
    );
}
