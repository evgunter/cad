//! M5 S12 acceptance: curved `revert` is wired, and curved
//! subtract/intersect are live on the classes that have a seam lane.
//!
//! S10 ratified `Face::sense` (a face's outward normal is
//! `sense_sign · chart_normal`); S11 made the constructors write the
//! bit honestly; S12 — this unit — makes `revert` FLIP it, makes
//! splitting's `mef`/`mfkrh` re-mints INHERIT the parent's bit, and
//! narrows the wholesale curved ∖/∩ front door to the classes that
//! still lack a join lane.
//!
//! Rows, in the order the unit's three parts land:
//!
//! - **revert**: the two exclusive encodings (a `Plane` negates its
//!   stored normal; every other class flips the face bit), the bitwise
//!   involution and determinism rows on a genuinely curved body, the
//!   tier-2/`NegativeVolume` class, and the flip-not-stamp row on a
//!   body whose incoming bits are MIXED (S11's concave arc wall).
//! - **∖ and ∩ end-to-end**: a slab minus a cylinder boss (blind
//!   hole), the through hole, the two-shell complement, the ∩ twin and
//!   the `V(A∖B) + V(A∩B) = V(A)` additivity oracle — exact
//!   closed-form volumes, tier 3, and bit-identical results under both
//!   sweep strategies (the idealized/realized door).
//! - **the mixed-sense split**: the S11 banked hazard's live shape — a
//!   boolean that SPLITS a `sense: false` wall — with the fragments'
//!   inherited bits read back off the result.
//! - **the S13 flips**: the die-pips shape and the poking-ball ∪
//!   finding, both flipped from refusal/defect pins to construction
//!   rows when M5 S13 wired the (Plane, Sphere) germ arm and the
//!   extent-certified fallback re-cut.
//!
//! **Tolerance shape.** Everything this unit adds is exact structure:
//! the sense flip is a `bool` negation, the inheritance rule is a
//! surface-KEY equality, and the front door is an arena scan of surface
//! kinds. None of the three has an in-band twin, none moves with ε, and
//! the involution/determinism rows are compared BITWISE. The volume
//! assertions are ordinary metric checks against closed forms and are
//! slacked from the resolved band rather than from a literal, so the
//! rows mean the same thing in each ε lane.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use profile::RawLoop;

use geom::Surface;
use geom_core::{Affine3, Point2, Point3, Tolerance, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations};
use geom_core::Tol;

// ---------------------------------------------------------------------
// Fixtures and helpers.
// ---------------------------------------------------------------------

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// Volume slack: derived from the RESOLVED band, never a literal, so
/// each ε lane compares at its own scale (floor at the f64 lane's
/// historical 1e-9).
fn slack() -> f64 {
    (1e3 * Tol::witness().get().eps).max(1e-9)
}

fn vol(body: &Body<f64>) -> f64 {
    topo::mass_properties(body).unwrap().volume
}

fn rect(w: f64, h: f64) -> ProfileLoop<f64> {
    ProfileLoop::new(
        [(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    )
}

/// The 3 × 3 × 0.8 plate (the PR 9 boss consumer's own dimensions).
fn plate() -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![rect(3.0, 3.0)])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(0.8)).unwrap().body
}

const R: f64 = 0.35;

/// A cylindrical boss of radius `R` at (1.2, 1.7), authored as `n`
/// equal arcs, sketched at `z0` and extruded by `len`.
fn boss(n: usize, z0: f64, len: f64) -> Body<f64> {
    let theta = 2.0 * PI / n as f64;
    let bulge = (theta / 4.0).tan();
    let at = |i: usize| {
        let th = theta * i as f64;
        p2(1.2 + R * th.cos(), 1.7 + R * th.sin())
    };
    let lp = ProfileLoop::new((0..n).map(|i| ProfileVertex::new(at(i), bulge)).collect());
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(len)).unwrap().body
}

/// A 3 × 3 × 1 plate with a CONCAVE semicircular notch (radius 0.5)
/// bitten out of its `x = 3` wall — S11's `sense: false` arc wall, and
/// the operand that makes the mixed-sense split reachable.
fn notched() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(3.0, 0.0), 0.0),
        ProfileVertex::new(p2(3.0, 1.0), -1.0),
        ProfileVertex::new(p2(3.0, 2.0), 0.0),
        ProfileVertex::new(p2(3.0, 3.0), 0.0),
        ProfileVertex::new(p2(0.0, 3.0), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0)).unwrap().body
}

/// The notch's own volume debit: a half-disc of radius 0.5 through the
/// full height.
const NOTCH: f64 = PI * 0.25 / 2.0;

/// The unit ball of the PR 9c acceptance: two half-sphere bands on ONE
/// sphere surface, translated to `centre`.
fn ball_at(centre: Vec3<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), 1.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    let ball = revolve(&vp, axis, Revolution::Full).unwrap().body;
    topo::transform_rigid(&ball, &Affine3::translation(centre)).unwrap()
}

/// A body's face senses in arena order.
fn senses(body: &Body<f64>) -> Vec<bool> {
    body.faces().map(|(_, f)| f.sense).collect()
}

/// Runs an op through BOTH sweep strategies and requires bit-identical
/// results (the PERF-PLAN §4.4 idealized/realized door), returning the
/// body.
fn both_lanes(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let decls = BooleanDeclarations::none();
    let realized = boolean_op_with(op, a, b, &decls, SweepStrategy::Realized)
        .unwrap_or_else(|e| panic!("{op:?} (realized): {e}"));
    let idealized = boolean_op_with(op, a, b, &decls, SweepStrategy::Idealized)
        .unwrap_or_else(|e| panic!("{op:?} (idealized): {e}"));
    let rb = realized.body().expect("a body").body.clone();
    let ib = idealized.body().expect("a body").body.clone();
    assert_eq!(
        format!("{rb:?}"),
        format!("{ib:?}"),
        "{op:?}: the two sweep strategies must produce bit-identical results"
    );
    if let Err(errs) = topo::validate_geometric(&rb) {
        panic!("{op:?} result is not tier-3 valid: {errs:?}");
    }
    rb
}

// =====================================================================
// Part (a): revert writes the curved arm.
// =====================================================================

/// The two exclusive encodings, on one body that has both kinds of
/// face: the boss's planar caps negate their stored plane normal (the
/// M3 arm, untouched) and its cylindrical wall segments flip
/// `Face::sense` (the S12 arm). No face is flipped twice — pinned by
/// metering the volume, which is exactly negated rather than
/// unchanged.
#[test]
fn revert_uses_the_plane_normal_for_planes_and_the_sense_bit_for_charts() {
    let body = boss(3, 0.0, 1.0);
    let before = format!("{body:?}");
    let v = vol(&body);
    assert!(v > 0.0);

    let planar: Vec<_> = body
        .faces()
        .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Plane { .. })))
        .map(|(k, f)| (k, f.sense, f.surface))
        .collect();
    let curved: Vec<_> = body
        .faces()
        .filter(|(_, f)| !matches!(body.get_surface(f.surface), Some(Surface::Plane { .. })))
        .map(|(k, f)| (k, f.sense, f.surface))
        .collect();
    assert_eq!(planar.len(), 2, "two caps");
    assert_eq!(curved.len(), 3, "three cylinder wall segments");

    let rev = body.revert().expect("S12: curved revert is wired");
    assert_eq!(format!("{body:?}"), before, "revert mutated its operand");

    // Planar faces: the BIT is untouched, the stored normal negates.
    for &(k, sense, sk) in &planar {
        assert_eq!(rev.get_face(k).unwrap().sense, sense, "plane bit moved");
        let (Some(Surface::Plane { normal: n0, .. }), Some(Surface::Plane { normal: n1, .. })) =
            (body.get_surface(sk), rev.get_surface(sk))
        else {
            panic!("plane surfaces stay planes");
        };
        assert_eq!(format!("{:?}", *n1), format!("{:?}", -*n0));
    }
    // Curved faces: the BIT flips, the chart is untouched.
    for &(k, sense, sk) in &curved {
        assert_eq!(rev.get_face(k).unwrap().sense, !sense, "chart bit stuck");
        assert_eq!(
            format!("{:?}", rev.get_surface(sk).unwrap()),
            format!("{:?}", body.get_surface(sk).unwrap()),
            "a non-plane chart must not be perturbed by revert (D1: exact structure)"
        );
    }

    // The M3 validity contract, now on a curved body.
    assert_eq!(topo::validate(&rev), Ok(()));
    assert_eq!(topo::validate_closed(&rev), Ok(()));
    assert_eq!(
        topo::validate_geometric(&rev),
        Err(vec![topo::ValidationError::NegativeVolume]),
        "a reverted body bounds the complement: tier 3 is exactly NegativeVolume"
    );
    assert_eq!(vol(&rev).to_bits(), (-v).to_bits(), "volume is bit-negated");
}

/// Involution and determinism on a curved body, both BITWISE — the D1
/// amendment's "orientation reversal is exact structure, never a
/// decide" made executable. A `bool` negation and an IEEE sign flip are
/// each involutions, so nothing widens and nothing drifts.
#[test]
fn curved_revert_is_a_bitwise_involution_and_deterministic() {
    for body in [boss(3, 0.0, 1.0), boss(2, 0.0, 1.0), notched()] {
        let original = format!("{body:?}");
        let once = body.revert().unwrap();
        assert_eq!(format!("{:?}", once.revert().unwrap()), original);
        assert_eq!(
            format!("{:?}", once.revert().unwrap().revert().unwrap()),
            format!("{once:?}"),
        );
        // Determinism: the same input maps to the same output, bit for
        // bit, on every call.
        assert_eq!(format!("{:?}", body.revert().unwrap()), format!("{once:?}"));
    }
}

/// **Flip, not stamp.** The notched plate carries S11's honest
/// `sense: false` on its concave arc wall, so its incoming bits are
/// MIXED. Reverting must flip each face's own bit — a writer that
/// stamped a constant would pass every all-`true` fixture and fail
/// exactly here.
#[test]
fn revert_flips_a_mixed_sense_body_face_by_face() {
    let body = notched();
    let before = senses(&body);
    assert!(
        before.contains(&false) && before.contains(&true),
        "the S11 notch fixture must carry MIXED bits, got {before:?}"
    );
    let rev = body.revert().unwrap();
    let after = senses(&rev);
    // Only the CURVED face's bit is the one revert writes; the planar
    // faces keep theirs (their reversal is in the normal).
    for ((k, f), was) in body.faces().zip(before.iter()) {
        let now = rev.get_face(k).unwrap().sense;
        let curved = !matches!(body.get_surface(f.surface), Some(Surface::Plane { .. }));
        assert_eq!(now, if curved { !*was } else { *was }, "face {k:?}");
    }
    assert_ne!(after, before, "something must have moved");
    assert_eq!(senses(&rev.revert().unwrap()), before, "involution on bits");
}

// =====================================================================
// Part (c): subtract and intersect, live on the plane×cylinder class.
// =====================================================================

/// The bread-and-butter drilled BLIND hole — the row that was pinned at
/// the front-door refusal since M5 PR 9 (`review_m5_pr9_boss_probe`'s
/// `my_boss_subtract_makes_a_blind_hole_honestly`) and is now a
/// construction row. Exact closed form, tier 3, both sweep lanes, and
/// both authorings of the disc (3 arcs and 2 semicircles).
#[test]
fn subtract_drills_a_blind_hole_with_the_exact_closed_form_volume() {
    for n in [3, 2] {
        let a = plate();
        let b = boss(n, 0.3, 1.0);
        let out = both_lanes(BooleanOp::Subtract, &a, &b);
        // The boss enters at z = 0.3 and leaves through the top at
        // z = 0.8: a blind pocket 0.5 deep.
        let expect = 3.0 * 3.0 * 0.8 - PI * R * R * 0.5;
        assert!(
            (vol(&out) - expect).abs() < slack(),
            "{n}-arc blind hole: {} vs {expect}",
            vol(&out)
        );
        assert_eq!(out.shells().count(), 1, "a blind pocket is one shell");
    }
}

/// The THROUGH hole (the boss spans the whole thickness and beyond),
/// and — the same pair the other way round — the two-shell complement:
/// boss ∖ plate is the two stubs sticking out above and below.
#[test]
fn subtract_makes_a_through_hole_and_a_two_shell_complement() {
    let a = plate();
    let b = boss(3, -0.2, 1.2);
    let holed = both_lanes(BooleanOp::Subtract, &a, &b);
    let expect = 3.0 * 3.0 * 0.8 - PI * R * R * 0.8;
    assert!(
        (vol(&holed) - expect).abs() < slack(),
        "through hole: {} vs {expect}",
        vol(&holed)
    );

    let stubs = both_lanes(BooleanOp::Subtract, &b, &a);
    // The boss spans z ∈ [−0.2, 1.0]; the plate takes z ∈ [0, 0.8], so
    // 0.2 is left below and 0.2 above — two disjoint shells.
    let expect = PI * R * R * 0.4;
    assert!(
        (vol(&stubs) - expect).abs() < slack(),
        "stubs: {} vs {expect}",
        vol(&stubs)
    );
    assert_eq!(stubs.shells().count(), 2, "two disjoint stubs");
}

/// The ∩ twin, plus the additivity oracle that ties the two ops
/// together without either closed form: `V(A∖B) + V(A∩B) = V(A)` on the
/// same operand pair.
#[test]
fn intersect_is_the_subtract_twin_and_the_two_are_additive() {
    let a = plate();
    let b = boss(3, 0.3, 1.0);
    let met = both_lanes(BooleanOp::Intersect, &a, &b);
    let expect = PI * R * R * 0.5;
    assert!(
        (vol(&met) - expect).abs() < slack(),
        "intersect: {} vs {expect}",
        vol(&met)
    );
    let cut = both_lanes(BooleanOp::Subtract, &a, &b);
    assert!(
        (vol(&cut) + vol(&met) - vol(&a)).abs() < slack(),
        "A∖B + A∩B must meter A: {} + {} vs {}",
        vol(&cut),
        vol(&met),
        vol(&a)
    );
}

// =====================================================================
// Part (b): the mixed-sense split, made reachable by part (c).
// =====================================================================

/// **The S11 banked hazard's live shape.** Until S12, splitting a
/// `sense: false` face was unreachable (curved ∖/∩ refused at the front
/// door), and every `mef` re-mint stamped `sense: true` — so the day
/// such a split became reachable it would have silently reset the bit
/// on the fragments. This row is that day: a slab crossing the notched
/// plate's CONCAVE arc wall splits it, through all three ops.
///
/// The proof is in two independent places. Structurally: the result
/// keeps a face on the notch's cylinder carrying `sense: false` — the
/// inherited bit. Metrically: the volumes are the exact closed forms,
/// which they would not be if a fragment claimed the wrong material
/// side (the notch would be metered as a boss).
#[test]
fn a_boolean_that_splits_a_reversed_wall_inherits_the_parent_bit() {
    let a = notched();
    let sq = ProfileLoop::new(
        [(2.0, 0.5), (4.0, 0.5), (4.0, 2.5), (2.0, 2.5)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    );
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.3)));
    let sp = Profile::new(plane, vec![sq])
        .validate(Tol::witness())
        .unwrap();
    let b = extrude(&sp, Extrusion::Distance(0.4)).unwrap().body;

    let v_a = 9.0 - NOTCH; // the notched plate, height 1
    let v_b = 2.0 * 2.0 * 0.4;
    // The slab's footprint covers x ∈ [2, 3] of the plate over the full
    // notch span, so the meet is (1 × 2 − the half-disc) × 0.4.
    let v_meet = (2.0 - NOTCH) * 0.4;

    for (op, expect) in [
        (BooleanOp::Union, v_a + v_b - v_meet),
        (BooleanOp::Subtract, v_a - v_meet),
        (BooleanOp::Intersect, v_meet),
    ] {
        let out = both_lanes(op, &a, &b);
        assert!(
            (vol(&out) - expect).abs() < slack(),
            "{op:?} across the reversed wall: {} vs {expect}",
            vol(&out)
        );
        // The inherited bit, read back off the result: the notch's
        // cylinder still bounds material on its INSIDE.
        let reversed_curved = out
            .faces()
            .filter(|(_, f)| !matches!(out.get_surface(f.surface), Some(Surface::Plane { .. })))
            .filter(|(_, f)| !f.sense)
            .count();
        assert!(
            reversed_curved > 0,
            "{op:?}: every fragment of the reversed wall came back sense: true — \
             the mef re-mint did not inherit the parent bit"
        );
    }
}

// =====================================================================
// What stays refused, and the finding that made the door structural.
// =====================================================================

/// **CONSTRUCTION row, flipped from the S12 refusal pin** (S9 pattern;
/// the retired pin was
/// `the_die_pip_sphere_shape_still_refuses_typed_at_the_narrowed_door`,
/// itself flipped from PR 9c's
/// `curved_subtract_front_door_quotes_the_same_finding`). The blocker
/// chain it recorded is retired in order: `revert` (S10–S12), then the
/// JOIN lane — M5 S13 wires the `(Plane, Sphere)` germ arm (the exact
/// C5 Circle, no fitted chord) and the extent-certified fallback
/// re-cut, so the die-pip shape now CUTS. The ball half-buried in the
/// slab pokes out of BOTH faces with no edge crossings (the S12
/// finding's shape), so both ops exercise the re-cut end to end:
///
/// - `∖` = slab minus the barrel zone `= 16 − 11π/12`, one shell, the
///   zone wall's sphere fragments REVERSED (`sense: false` — the S12
///   guard row's audited-answer arm, live for the sphere class);
/// - `∩` = the zone itself `= 11π/12`, one shell;
/// - additivity `V(∖) + V(∩) = V(slab)` exactly.
#[test]
fn the_die_pip_sphere_shape_now_cuts_at_the_opened_door() {
    let slab = Profile::new(SketchPlane::xy(), vec![rect(4.0, 4.0)])
        .validate(Tol::witness())
        .unwrap();
    let a = extrude(&slab, Extrusion::Distance(1.0)).unwrap().body;
    let b = ball_at(Vec3::new(2.0, 2.0, 0.5));

    let zone = 11.0 * PI / 12.0;
    let cut = both_lanes(BooleanOp::Subtract, &a, &b);
    assert!((vol(&cut) - (16.0 - zone)).abs() < slack(), "{}", vol(&cut));
    assert_eq!(cut.shells().count(), 1, "the barrel tunnel is one shell");
    let reversed_sphere = cut
        .faces()
        .filter(|(_, f)| {
            matches!(cut.get_surface(f.surface), Some(Surface::Sphere { .. })) && !f.sense
        })
        .count();
    assert!(
        reversed_sphere > 0,
        "the cavity wall's sphere fragments must keep the reverted bit"
    );

    let met = both_lanes(BooleanOp::Intersect, &a, &b);
    assert!((vol(&met) - zone).abs() < slack(), "{}", vol(&met));
    assert_eq!(met.shells().count(), 1);
    assert!(
        (vol(&cut) + vol(&met) - 16.0).abs() < slack(),
        "∖/∩ additivity"
    );

    // And the CYLINDER class through the same entry point is still
    // live — S13 opens a class, it does not trade one for another.
    assert!(
        topo::subtract(&plate(), &boss(3, 0.3, 1.0)).is_ok(),
        "the opened door must not re-gate the cylinder class"
    );
}

/// **FINDING (S12, executed; NOT fixed here) — the containment fallback
/// is vertex-probed, so it is unsound for curved boundaries.**
///
/// A unit ball half-buried in a slab pokes out of both faces, yet its
/// only two vertices (the revolve poles, on the axis) sit inside the
/// slab. The reduction finds no crossings for the sphere class, the
/// pipeline falls through to per-shell vertex-in-solid containment, and
/// the ball is classified as WHOLLY CONTAINED. `union` therefore meters
/// the slab alone.
///
/// - True answer: `16 + 2 · (π h²(3r − h)/3)` with `h = 0.5`, i.e.
///   16 + 1.3089969… = 17.3089969…
/// - What this build answers: 16.0 exactly.
///
/// **Reproduced on the merge base** (main at `3ef715e`, S12's parent):
/// the same call returns 16.0 there. This predates S12 and is not
/// caused by it — `union` was never behind the curved front door, and
/// the sphere's faces are `sense: true` in both builds, so neither the
/// revert wiring nor the sense inheritance touches this path.
///
/// S12's response was to REFUSE the class up front for ∖ and ∩ and to
/// pin the ∪ defect at the wrong value so the eventual fix fails
/// loudly. **M5 S13 is that fix, and this is the construction row the
/// pin flipped to** (the pin's own doc comment promised exactly this):
/// the fallback's vertex probe is now backed by the curved-EXTENT scan
/// — the sphere group's true extent (center ± r) is consulted against
/// every face of the other operand — which detects the escape, re-cuts
/// the operand (rigid re-chart about the escape normal), and re-enters
/// the pipeline, where the crossing layer finds the two section
/// circles and the `(Plane, Sphere)` germ arm joins them exactly.
///
/// - union = `16 + 2·(π h²(3r − h)/3)` with `h = 0.5` —
///   **17.30899693899575**, where the finding metered 16.0;
/// - the poking ball ∪ slab is **ONE shell** (both caps join the slab
///   boundary through their seam circles — derived, then pinned);
/// - tier-3 valid, both sweep strategies bit-identical.
#[test]
fn finding_row_flipped_containment_fallback_now_sees_the_curved_extent() {
    let slab = Profile::new(SketchPlane::xy(), vec![rect(4.0, 4.0)])
        .validate(Tol::witness())
        .unwrap();
    let a = extrude(&slab, Extrusion::Distance(1.0)).unwrap().body;
    let b = ball_at(Vec3::new(2.0, 2.0, 0.5));

    // The ball genuinely leaves the slab: its equator reaches z = 1.5.
    let above = Point3::new(2.0, 2.0, 1.4);
    assert_eq!(
        topo::boolean::point_in_solid(&b, above, geom_core::Band::linear(Tol::witness()).unwrap()).unwrap(),
        topo::boolean::SolidContainment::In,
        "the ball really does poke out above the slab"
    );

    let joined = both_lanes(BooleanOp::Union, &a, &b);
    let got = vol(&joined);
    let h = 0.5_f64;
    let truth = 16.0 + 2.0 * (PI * h * h * (3.0 - h) / 3.0);
    assert!(
        (got - truth).abs() < slack(),
        "∪ must meter the poking caps: got {got}, want {truth}"
    );
    assert_eq!(
        joined.shells().count(),
        1,
        "slab ∪ poking ball is one shell"
    );
}
