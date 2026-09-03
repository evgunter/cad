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

use crate::revolve_common;

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

/// **The fixture scale these bodies are built at.** Every body in this
/// suite is a unit revolve of the `revolve_cone` triangle or its
/// trapezoid sibling, so the cone's slant extent is `√2` and no feature
/// is further than about that from the axis. [`away`]'s ceiling is
/// stated against this number, and
/// [`the_clamp_floor_clears_the_apex_escalation_shell`] asserts it
/// rather than leaving it as prose a later fixture edit could falsify.
const CONE_SLANT: f64 = core::f64::consts::SQRT_2;

/// The probe offset, and the two shells it has to clear.
///
/// # The binding constraint is the APEX shell, and it is √ε-sized
///
/// A probe has two jobs — definitely outside the ambiguity band, and
/// definitely inside the body — and the first one is set by the
/// SHARPEST escalation near it, which for a cone is not the linear one.
/// Two shells surround the geometry:
///
/// - the **wall/plane** shell, where a residual compared against `Zero`
///   escalates: linear in ε, about `K·ε` (1e-8 at the default row);
/// - the **apex** shell, where the ray×cone discriminant escalates.
///   `disc` decays QUADRATICALLY as the query approaches the apex — the
///   two roots merge there — so the escalation radius goes like
///   `√(K·ε·v_ext)`, not like `ε`. At the default row that is
///   `√(10·1e-9·√2) ≈ 1.2e-4`, nearly FOUR ORDERS wider than the linear
///   shell, and it is what the floor actually has to clear.
///
/// Measured on this fixture (the on-axis probe at
/// `(0, 1 − d, 0)`, walked inward until the door stops answering
/// definitely — [`apex_shell`]):
///
/// | ε | `away()` | apex shell | `√(K·ε·√2)` | clearance |
/// |---|---|---|---|---|
/// | 1e-12 | 1e-3 (floor) | 7.4e-6 | 3.8e-6 | 135.6× |
/// | 1e-9 (default) | 1e-3 (floor) | 2.4e-4 | 1.2e-4 | **4.2×** |
/// | 1e-6 | 1e-1 (ceiling) | 7.2e-3 | 3.8e-3 | 13.9× |
///
/// So the default row is the tight one and it clears by a **factor of
/// four**, not by five orders — which is what the first draft of this
/// comment claimed, from the linear shell, for a constraint that is not
/// the binding one. The measured shell tracks the law at a constant
/// ≈1.96 across all three decades, and it scales as `√ε` (7.4e-6 →
/// 2.4e-4 → 7.2e-3 is a factor of ≈31 per ε decade, i.e. √1000). It
/// scales as `√K` too, so raising `CAD_AMBIGUITY_K` two decades puts
/// the shell past this floor — measured: at K = 1000 the shell is
/// 2.3e-3 and the clearance is 0.4×, and the guard row below goes red
/// saying so. That is a real dependency, not a hypothetical, and
/// it is GUARDED rather than asserted in prose:
/// [`the_clamp_floor_clears_the_apex_escalation_shell`] measures the
/// shell on every run and goes red if it widens past the derivation.
///
/// # The clamp saturates, so the ε-derivation is mostly decorative
///
/// Stated plainly because the obvious reading of the expression is
/// wrong: `1e6·ε` lands inside `[1e-3, 0.1]` only for
/// `ε ∈ (1e-9, 1e-7)`, and **no row in the matrix draws an ε in that
/// interval**. At all three shipped rows the clamp saturates — floor at
/// 1e-9 and 1e-12, ceiling at 1e-6 — so what governs the offsets these
/// rows actually use is the clamp, not the derivation. The ε-scaling is
/// kept because it is the right law between the bounds, not because any
/// gated run exercises it.
///
/// The ceiling's own job: an offset scaling freely with ε reaches 1.0 at
/// the 1e-6 row, and a probe "just inside the wall" of a body whose
/// slant extent is [`CONE_SLANT`] at that distance is not inside the
/// cone at all — it is out the other side, and the row would be
/// asserting about geometry it did not mean.
fn away() -> f64 {
    // The expression itself lives in `revolve_common::probe_offset`,
    // which carries the part of the argument every containment suite
    // shares (the ε-scaling, the clamp and its saturation). What stays
    // here is the SHELL this suite has to clear, which is this arm's own.
    probe_offset()
}

/// The measured radius of the apex escalation shell on the unit cone:
/// the largest on-axis offset from the apex at which `point_in_solid`
/// still declines to answer definitely.
///
/// Walked multiplicatively inward from a definitely-answered offset, so
/// the result is the OUTER edge of the shell — the number [`away`] has
/// to beat — rather than the first escalation found going outward.
fn apex_shell() -> f64 {
    let body = cone();
    let mut worst = 0.0_f64;
    let mut d = 0.5_f64;
    while d > 1e-11 {
        let answered = matches!(
            point_in_solid(
                &body,
                Point3::new(0.0, 1.0 - d, 0.0),
                band(),
                Tol::witness()
            ),
            Ok(SolidContainment::In)
        );
        if !answered {
            worst = worst.max(d);
        }
        d /= 1.05;
    }
    worst
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
/// has an arm. The SPLINE kinds keep the variant, and keep it as a
/// capability claim about a HEALTHY body — the torus kept it too until
/// issue 1011’s torus half landed the ray×torus arm.
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
        kind: geom_brep::SurfaceKind::Nurbs,
    }
    .to_string();
    assert!(msg.contains("HEALTHY"), "{msg}");
    assert!(!msg.contains("corrupt"), "{msg}");
    assert!(
        !msg.contains("cone"),
        "the recourse must not tell a caller to avoid a kind that has an arm: {msg}"
    );
    // The torus was the other kind this message named until issue
    // 1011's torus half landed its arm; what the variant carries now is
    // the spline kinds.
    assert!(!msg.contains("torus"), "{msg}");
    assert!(msg.contains("spline"), "{msg}");

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

/// **The guard for [`away`]'s derivation** (Q6: a claim about ε
/// behaviour is either mechanically checked or carries the written
/// reason it cannot be).
///
/// Three things are asserted, and each is a way the derivation could go
/// silently wrong:
///
/// 1. the fixture is still the size the ceiling is stated against —
///    prose about "unit-sized bodies" that nothing enforces is exactly
///    what a later fixture edit falsifies;
/// 2. the measured apex shell still obeys `√(K·ε·v_ext)`. If the arm's
///    discriminant metering changes, the shell moves, and the table in
///    [`away`] becomes fiction;
/// 3. the floor still CLEARS that shell — asserted at 3×, against a
///    measured 4.2× at the default row, so the guard has headroom for
///    a small metering change but not for a lost order. It is the
///    assertion that goes red if
///    `CAD_AMBIGUITY_K` is raised: the shell grows like `√K`, so two
///    decades of K put it past a 1e-3 floor. The failure message says
///    so, because the fix is to raise the floor, never to widen the
///    band or move the probe.
#[test]
fn the_clamp_floor_clears_the_apex_escalation_shell() {
    // (1) the fixture-size invariant the ceiling is stated against.
    let body = cone();
    let apex = Point3::new(0.0, 1.0, 0.0);
    let mut extent = 0.0_f64;
    for (_, v) in body.vertices() {
        if let Some(p) = body.get_point(v.point) {
            extent = extent.max(p.distance(apex));
        }
    }
    assert!(
        (extent - CONE_SLANT).abs() < 1e-12,
        "the fixture's slant extent moved to {extent}: away()'s ceiling is stated \
         against CONE_SLANT = {CONE_SLANT}, so re-derive it before changing the body"
    );

    let t = Tol::witness().get();
    let (eps, k) = (t.eps, t.k);
    let shell = apex_shell();
    let law = (k * eps * CONE_SLANT).sqrt();
    println!(
        "apex shell: eps={eps:e} k={k} measured={shell:e} law=sqrt(k*eps*slant)={law:e} away()={:e} clearance={:.1}x",
        away(),
        away() / shell
    );

    // (2) the shell obeys the square-root law, within the constant the
    // table's numbers carry (measured ≈ 1.5·law across three decades).
    assert!(
        shell > 0.0 && shell < 3.0 * law,
        "the apex escalation shell is {shell:e}, outside sqrt(K*eps*slant) = {law:e} \
         by more than the constant away()'s table records — the discriminant's \
         metering moved and that table is now fiction"
    );
    // (3) the floor clears it. Single digits at the default row.
    assert!(
        away() >= 3.0 * shell,
        "away() = {:e} no longer clears the apex escalation shell {shell:e} \
         (K = {k}, eps = {eps:e}). The shell grows like sqrt(K), so this fires \
         when K is raised. The fix is to raise away()'s FLOOR — never to widen \
         the band, and never to move the probe off the geometry it means",
        away()
    );
}
