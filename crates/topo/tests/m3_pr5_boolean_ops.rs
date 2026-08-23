//! M3 PR 5 acceptance: the public boolean ops end to end. The
//! canonical two-brick trace (TOG Fig. 15.4 analogue — the F3 worked
//! example) with hand-derived censuses pinned bitwise (D9), the
//! Fig. 15.1 coplanar-overlap ∩, the A∖B ≡ A∩revert(B) oracle, voids,
//! disjoint/nested/touching operands, and the F7 merge output stage.
//! Every scenario is generic over `T` and runs at f64 (all ε rows via
//! CI) and on the interval lane.
//!
//! Tier-3 posture (the PR 3 gap, same documented posture): boolean
//! outputs carry chord-line descriptions on seam edges, so tier 3 at
//! rest runs through `describe_as_intersections` (the review
//! helper posture) — the honest upgrade op is a PR 6 obligation.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::prism_z;
use geom_core::Decide;
use geom_core::Tol;
use topo::test_support::{ArenaCounts, arena_counts};
use topo::{
    Body, BooleanBody, BooleanError, BooleanResult, BooleanResultKind, mass_properties, subtract,
    subtract_with, union, union_with, validate, validate_closed, validate_geometric,
};

fn brick<T: Decide + geom_core::Bounds>(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<T> {
    prism_z::<T>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

/// A public declared boolean op as a value (M4 PR 5: the corpus
/// declares its intended flush contacts — recipe intent, test form).
type BoolOp<T> = fn(
    &Body<T>,
    &Body<T>,
    &topo::BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<T>, BooleanError>;

/// Runs one op functionally with the author's flush contacts
/// declared, checking the operands stayed bitwise untouched and the
/// result passes tier 1 + 2.
fn run<T: Decide + geom_core::Bounds>(op: BoolOp<T>, a: &Body<T>, b: &Body<T>) -> BooleanResult<T> {
    let (a0, b0) = (format!("{a:?}"), format!("{b:?}"));
    let out = op(a, b, &common::flush_declarations(a, b), Tol::witness()).unwrap();
    assert_eq!(format!("{a:?}"), a0, "operand A untouched");
    assert_eq!(format!("{b:?}"), b0, "operand B untouched");
    if let BooleanResult::Body(body) = &out {
        assert_eq!(validate(&body.body), Ok(()), "tier 1");
        assert_eq!(validate_closed(&body.body), Ok(()), "tier 2");
    }
    out
}

fn body_of<T: Decide + geom_core::Bounds>(r: &BooleanResult<T>) -> &BooleanBody<T> {
    r.body().expect("non-empty boolean result")
}

/// Volume/area equality at f64 (exact values for the pinned corpus).
fn assert_props(body: &Body<f64>, volume: f64, area: f64) {
    let m = mass_properties(body, Tol::witness()).unwrap();
    assert_eq!(m.volume, volume, "exact volume");
    assert_eq!(m.surface_area, area, "exact area");
}

/// Tier 3 directly at rest (D6, M3 PR 6a): boolean results carry
/// honest `Intersection` descriptions natively.
fn assert_tier3_posture(body: &Body<f64>) {
    assert_eq!(validate_geometric(body, Tol::witness()), Ok(()));
}

// ---------------------------------------------------------------
// Acceptance (1): the canonical two-brick trace, A=[0,2]³ B=[1,3]³.
// Seam = the staircase hexagon (2,1,1)-(2,2,1)-(1,2,1)-(1,2,2)-
// (1,1,2)-(2,1,2). Censuses hand-derived in the PR derivation.
// ---------------------------------------------------------------

/// Generic op-runner for the interval lane: censuses + kinds only
/// (exact-value oracles are the f64 lane's; Interval has no PartialEq
/// by design).
fn generic_scenarios<T: Decide + geom_core::Bounds>() {
    let (a, b) = two_bricks::<T>();
    for (op, faces) in [
        (topo::intersect_with as BoolOp<T>, 6),
        (union_with, 12),
        (subtract_with, 9),
    ] {
        let r = run(op, &a, &b);
        let body = body_of(&r);
        assert_eq!(body.kind, BooleanResultKind::Seamed);
        assert_eq!(body.body.faces().count(), faces);
    }
    // Pocket (the cookie-cutter lane) + void + disjoint.
    let a = brick::<T>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<T>((0.75, 1.25), (0.75, 1.25), (1.5, 2.5));
    let r = run(subtract_with, &a, &b);
    assert_eq!(body_of(&r).kind, BooleanResultKind::Seamed);
    let a = brick::<T>((0.0, 3.0), (0.0, 3.0), (0.0, 3.0));
    let b = brick::<T>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let r = run(subtract_with, &a, &b);
    assert_eq!(body_of(&r).kind, BooleanResultKind::Voided);
    let a = brick::<T>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick::<T>((2.0, 3.0), (2.0, 3.0), (2.0, 3.0));
    assert!(matches!(
        run(topo::intersect_with, &a, &b),
        BooleanResult::Empty
    ));
}

#[test]
fn generic_scenarios_f64() {
    generic_scenarios::<f64>();
}

// ---- Interval lane (the same scenarios at T = Interval). ----
#[cfg(feature = "interval")]
mod interval {
    use super::*;

    #[test]
    fn boolean_ops_interval() {
        generic_scenarios::<geom_core::Interval>();
    }
}

fn two_bricks<T: Decide + geom_core::Bounds>() -> (Body<T>, Body<T>) {
    (
        brick::<T>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
        brick::<T>((1.0, 3.0), (1.0, 3.0), (1.0, 3.0)),
    )
}

#[test]
fn two_bricks_intersect() {
    let (a, b) = two_bricks::<f64>();
    let r = run(topo::intersect_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    // The [1,2]³ cube: 3 A faces + 3 B faces, hexagon seam.
    assert_eq!(
        arena_counts(&body.body),
        ArenaCounts {
            solids: 1,
            shells: 1,
            faces: 6,
            loops: 6,
            half_edges: 24,
            edges: 12,
            vertices: 8
        }
    );
    assert_props(&body.body, 1.0, 6.0);
    assert_tier3_posture(&body.body);
    // D9: byte-identical replay.
    let again = run(topo::intersect_with, &a, &b);
    assert_eq!(
        format!("{:?}", body.body),
        format!("{:?}", body_of(&again).body)
    );
}

#[test]
fn two_bricks_union() {
    let (a, b) = two_bricks::<f64>();
    let r = run(union_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    // 7+7 operand corners + 6 seam; 3 full + 3 L faces per operand.
    assert_eq!(
        arena_counts(&body.body),
        ArenaCounts {
            solids: 1,
            shells: 1,
            faces: 12,
            loops: 12,
            half_edges: 60,
            edges: 30,
            vertices: 20
        }
    );
    assert_props(&body.body, 15.0, 42.0);
    assert_tier3_posture(&body.body);
    let again = run(union_with, &a, &b);
    assert_eq!(
        format!("{:?}", body.body),
        format!("{:?}", body_of(&again).body)
    );
}

// ---------------------------------------------------------------
// Acceptance (2): Fig. 15.1 coplanar overlap — extruded profiles
// sharing both cap planes; the Eq. 15.3 lanes live through join.
// ---------------------------------------------------------------

/// The Fig. 15.1 coplanar-overlap ∩ (the deferred PR 5 acceptance
/// item, landed by PR 5.5): the seam runs partly ALONG the shared cap
/// planes; the angular strut spike order (the sort half of
/// `ssortnulledges`, `bool_strut_order`) nests the corner-site chords
/// so the joining completes and the [1,2]²×[0,1] cube comes out with
/// exact mass properties.
#[test]
fn coplanar_overlap_intersect() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
    let b = brick::<f64>((1.0, 3.0), (1.0, 3.0), (0.0, 1.0));
    let r = run(topo::intersect_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    assert_props(&body.body, 1.0, 6.0);
}

// ---------------------------------------------------------------
// Acceptance (4): voids — cube ∖ inner cube births the two-shell
// body (the voids-born-only-from-booleans moment).
// ---------------------------------------------------------------

#[test]
fn void_birth_cube_minus_inner_cube() {
    let a = brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 3.0));
    let b = brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let r = run(subtract_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Voided);
    // Outer cube shell + reverted inner void shell: tier-2-legal
    // multi-shell (asserted by `run`), exact volume outer − inner.
    assert_eq!(
        arena_counts(&body.body),
        ArenaCounts {
            solids: 1,
            shells: 2,
            faces: 12,
            loops: 12,
            half_edges: 48,
            edges: 24,
            vertices: 16
        }
    );
    assert_props(&body.body, 26.0, 60.0);
    assert!(body.contacts.vv.is_empty());
}

// ---------------------------------------------------------------
// Acceptance (5): disjoint and nested operands.
// ---------------------------------------------------------------

#[test]
fn disjoint_operands() {
    let a = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick::<f64>((2.0, 3.0), (2.0, 3.0), (2.0, 3.0));
    // ∪: the typed disjoint union — one solid, two shells.
    let r = run(union_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Assembly);
    assert_eq!(
        arena_counts(&body.body),
        ArenaCounts {
            solids: 1,
            shells: 2,
            faces: 12,
            loops: 12,
            half_edges: 48,
            edges: 24,
            vertices: 16
        }
    );
    assert_props(&body.body, 2.0, 12.0);
    // ∩: the typed empty success.
    assert!(matches!(
        run(topo::intersect_with, &a, &b),
        BooleanResult::Empty
    ));
    // ∖: A untouched.
    let r = run(subtract_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::OperandA);
    assert_eq!(arena_counts(&body.body), arena_counts(&a));
    assert_props(&body.body, 1.0, 6.0);
}

#[test]
fn nested_operands() {
    let a = brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 3.0));
    let b = brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    // B_inside ∖ A = ∅ (typed success).
    assert!(matches!(run(subtract_with, &b, &a), BooleanResult::Empty));
    // A ∩ B_inside = B.
    let r = run(topo::intersect_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::OperandB);
    assert_eq!(arena_counts(&body.body), arena_counts(&b));
    assert_props(&body.body, 1.0, 6.0);
    // A ∪ B_inside = A.
    let r = run(union_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::OperandA);
    assert_props(&body.body, 27.0, 54.0);
}

// ---------------------------------------------------------------
// Acceptance (6): touching-only operands — declared 3′ contacts
// carried through; tier 2 passes structurally (position injectivity
// is tier 3 — PR 6's tier-3′ validator is the landing zone).
// ---------------------------------------------------------------

#[test]
fn corner_kiss_operands() {
    let a = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    // ∪: a touching assembly with the vv contact carried, remapped to
    // live result keys.
    let r = run(union_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Assembly);
    assert_eq!(
        arena_counts(&body.body),
        ArenaCounts {
            solids: 1,
            shells: 2,
            faces: 12,
            loops: 12,
            half_edges: 48,
            edges: 24,
            vertices: 16
        }
    );
    assert_props(&body.body, 2.0, 12.0);
    assert_eq!(body.contacts.vv.len(), 1);
    let c = body.contacts.vv[0];
    assert!(body.body.get_vertex(c.a).is_some());
    assert!(body.body.get_vertex(c.b).is_some());
    // The kiss point is one position, two distinct vertices (3′
    // touching via distinct entities — F2's representable class).
    assert_ne!(c.a, c.b);
    // ∩: regularized empty. ∖: A, contacts dropped (B not in result).
    assert!(matches!(
        run(topo::intersect_with, &a, &b),
        BooleanResult::Empty
    ));
    let r = run(subtract_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::OperandA);
    assert!(body.contacts.vv.is_empty());
}

// ---------------------------------------------------------------
// The cookie-cutter family (consumer finding, binding addition):
// seam rings that close INSIDE single faces — pocket, boss,
// through-pillar, inset-leg union. Each pinned with an exact volume
// oracle (the family that exposed the unanchored strut-side parity).
// ---------------------------------------------------------------

#[test]
fn pocket_subtract() {
    // Pillar into the top face: blind pocket (dyadic coordinates —
    // the volume/area oracles are EXACT).
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((0.75, 1.25), (0.75, 1.25), (1.5, 2.5));
    let r = run(subtract_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    assert_props(&body.body, 8.0 - 0.125, 24.0 + 1.0);
    assert_tier3_posture(&body.body);
}

#[test]
fn boss_union() {
    // The same pillar, added: a boss standing on the top face.
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((0.75, 1.25), (0.75, 1.25), (1.5, 2.5));
    let r = run(union_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    assert_props(&body.body, 8.0 + 0.125, 24.0 + 1.0);
    assert_tier3_posture(&body.body);
}

/// The double-ring single-face-seam family, landed by PR 5.5's seam
/// discipline (derived senses + per-solid ring-lane role rule): the
/// through-pillar tunnel — a pillar piercing BOTH caps — subtracts to
/// the genus-1 body with exact mass properties.
#[test]
fn through_pillar_subtract() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((0.75, 1.25), (0.75, 1.25), (-0.5, 2.5));
    let r = run(subtract_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    assert_eq!(body.body.shells().count(), 1);
    // vol = 8 − 0.5·0.5·2; area = 24 − 2·0.25 + tunnel walls 4·(0.5·2).
    assert_props(&body.body, 7.5, 27.5);
}

/// Same family as [`through_pillar_subtract`]: the inset-leg union.
#[test]
fn inset_leg_union() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (1.0, 1.5));
    let b = brick::<f64>((0.5, 1.0), (0.5, 1.0), (0.0, 1.25));
    let r = run(union_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    // vol = 2·2·0.5 + 0.5·0.5·1.0; area = slab 2·4 + rim 4 − foot .25
    // + leg walls 4·(0.5·1.0) + leg bottom .25.
    assert_props(&body.body, 2.25, 14.0);
}

// ---------------------------------------------------------------
// Acceptance (3): the A∖B ≡ A∩revert(B) executable oracle
// (Problem 15.9) across the corpus: census + volume + area equality
// (bitwise replay equality is NOT claimed across the two routes —
// they take different pipeline paths; documented in the PR report).
// ---------------------------------------------------------------

#[test]
fn subtract_equals_intersect_revert_oracle() {
    let corpus: Vec<(&str, Body<f64>, Body<f64>)> = vec![
        (
            "two-brick",
            brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
            brick((1.0, 3.0), (1.0, 3.0), (1.0, 3.0)),
        ),
        (
            "pocket",
            brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
            brick((0.75, 1.25), (0.75, 1.25), (1.5, 2.5)),
        ),
        (
            "disjoint",
            brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
            brick((2.0, 3.0), (2.0, 3.0), (2.0, 3.0)),
        ),
        (
            "nested",
            brick((0.0, 3.0), (0.0, 3.0), (0.0, 3.0)),
            brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0)),
        ),
        (
            "nested-inverted",
            brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0)),
            brick((0.0, 3.0), (0.0, 3.0), (0.0, 3.0)),
        ),
        (
            "corner-kiss",
            brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
            brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0)),
        ),
        // PR 5.5's newly-working seam families.
        (
            "plus-x-pocket",
            brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
            brick((1.5, 2.5), (0.75, 1.25), (0.75, 1.25)),
        ),
        (
            "through-pillar",
            brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
            brick((0.75, 1.25), (0.75, 1.25), (-0.5, 2.5)),
        ),
    ];
    for (name, a, b) in corpus {
        let direct = subtract(&a, &b, Tol::witness()).unwrap();
        let via_revert = topo::intersect(&a, &b.revert().unwrap(), Tol::witness()).unwrap();
        match (&direct, &via_revert) {
            (BooleanResult::Empty, BooleanResult::Empty) => {}
            (BooleanResult::Body(d), BooleanResult::Body(r)) => {
                assert_eq!(
                    arena_counts(&d.body),
                    arena_counts(&r.body),
                    "{name}: census equality"
                );
                let md = mass_properties(&d.body, Tol::witness()).unwrap();
                let mr = mass_properties(&r.body, Tol::witness()).unwrap();
                assert_eq!(md.volume, mr.volume, "{name}: volume equality");
                assert_eq!(md.surface_area, mr.surface_area, "{name}: area equality");
            }
            _ => panic!("{name}: oracle kinds diverge: {direct:?} vs {via_revert:?}"),
        }
    }
}

// ---------------------------------------------------------------
// F7 merge output stage: the declared rung must actually fire on
// shared-recipe coplanar walls (stacked bricks whose wall planes are
// the SAME literal Surface value on both operands). The default
// Newell construction (independent centroids ⇒ bit-different planes)
// pins the no-numeric-rung side: nothing merges without declaration.
// ---------------------------------------------------------------

#[test]
fn merge_ladder_fires_only_on_declared_planes() {
    // Full-overlap stacked bricks: the whole seam runs ALONG existing
    // operand edges (boundary-on-boundary coincidence). UNDECLARED,
    // the coincidence ladder refuses — no numeric rung, nothing
    // merges without declaration. DECLARED, the union builds since
    // M5 S1 (the REST zip), and — per this test's own extension
    // instruction — the declared-rung merge census holds (F7): the
    // four declared same-plane side pairs merge, leaving exactly the
    // (0..2)²×(0..4) brick's six maximal faces.
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((0.0, 2.0), (0.0, 2.0), (2.0, 4.0));
    let undeclared = union(&a, &b, Tol::witness());
    assert!(
        undeclared.is_err(),
        "the undeclared stacked-full union must keep refusing typed \
         (coincidence is declared, never value-inferred)"
    );
    let r = run(union_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    assert_eq!(body.body.faces().count(), 6, "declared-rung merge census");
    assert_props(&body.body, 16.0, 40.0);
}

#[test]
fn tangential_rest_operands() {
    // Full-face coplanar rest (PR 4: ∩/∖ classify to contacts only).
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((0.0, 2.0), (0.0, 2.0), (2.0, 4.0));
    assert!(matches!(
        run(topo::intersect_with, &a, &b),
        BooleanResult::Empty
    ));
    let r = run(subtract_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::OperandA);
    assert_eq!(arena_counts(&body.body), arena_counts(&a));
    assert_props(&body.body, 8.0, 24.0);
}

#[test]
fn two_bricks_subtract() {
    let (a, b) = two_bricks::<f64>();
    let r = run(subtract_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    // A's union side (7 corners, 3 L + 3 full faces) + reverted BinA
    // (3 squares, corner (1,1,1), 3 split-edge remnants).
    assert_eq!(
        arena_counts(&body.body),
        ArenaCounts {
            solids: 1,
            shells: 1,
            faces: 9,
            loops: 9,
            half_edges: 42,
            edges: 21,
            vertices: 14
        }
    );
    assert_props(&body.body, 7.0, 24.0);
    assert_tier3_posture(&body.body);
    let again = run(subtract_with, &a, &b);
    assert_eq!(
        format!("{:?}", body.body),
        format!("{:?}", body_of(&again).body)
    );
}
