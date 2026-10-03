//! M3 PR 5 acceptance: the public boolean ops end to end. The
//! canonical two-brick trace (TOG Fig. 15.4 analogue — the F3 worked
//! example) with hand-derived censuses pinned bitwise (D9), the
//! Fig. 15.1 coplanar-overlap ∩, the A∖B ≡ A∩revert(B) oracle, voids,
//! disjoint/nested/touching operands, and the F7 merge output stage.
//! Every scenario is generic over `T` and runs at f64 (all ε rows via
//! CI) and on the interval lane.
//!
//! Boolean outputs carry honest descriptions on their seam edges, so
//! tier 3 runs on them directly.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished};
use geom_core::Decide;
use geom_core::Tol;
use topo::test_support::{ArenaCounts, arena_counts};
use topo::{
    AtRestBody, Body, BooleanBody, BooleanError, BooleanResult, BooleanResultKind, mass_properties,
    subtract, subtract_with, union, union_with, validate, validate_closed, validate_geometric,
};

/// A public declared boolean op as a value (M4 PR 5: the corpus
/// declares its intended flush contacts — recipe intent, test form).
type BoolOp<T> = fn(
    &AtRestBody<T>,
    &AtRestBody<T>,
    &topo::BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<T>, BooleanError>;

/// Runs one op functionally with the author's flush contacts
/// declared, checking the operands stayed bitwise untouched and the
/// result passes tier 1 + 2.
fn run<T: Decide + geom_core::Bounds>(
    op: BoolOp<T>,
    a: &AtRestBody<T>,
    b: &AtRestBody<T>,
) -> BooleanResult<T> {
    let (a0, b0) = (format!("{a:?}"), format!("{b:?}"));
    let out = op(
        a,
        b,
        &common::flush_declarations(a, b, Tol::witness()),
        Tol::witness(),
    )
    .unwrap();
    assert_eq!(format!("{a:?}"), a0, "operand A untouched");
    assert_eq!(format!("{b:?}"), b0, "operand B untouched");
    if let BooleanResult::Body(body) = &out {
        assert_eq!(validate(&body.body), Ok(()), "tier 1");
        assert_eq!(validate_closed(&body.body), Ok(()), "tier 2");
    }
    out
}

/// The brick `x × y × z`, finished.
fn finished_brick<T: Decide + topo::AtRestPolicy>(
    x: (f64, f64),
    y: (f64, f64),
    z: (f64, f64),
) -> AtRestBody<T> {
    let tol = Tol::witness();
    finished("brick", brick::<T>(x, y, z, tol), tol)
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
fn generic_scenarios<T: Decide + geom_core::Bounds + topo::AtRestPolicy>() {
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
    let a = finished_brick::<T>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = finished_brick::<T>((0.75, 1.25), (0.75, 1.25), (1.5, 2.5));
    let r = run(subtract_with, &a, &b);
    assert_eq!(body_of(&r).kind, BooleanResultKind::Seamed);
    let a = finished_brick::<T>((0.0, 3.0), (0.0, 3.0), (0.0, 3.0));
    let b = finished_brick::<T>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let r = run(subtract_with, &a, &b);
    assert_eq!(body_of(&r).kind, BooleanResultKind::Voided);
    let a = finished_brick::<T>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = finished_brick::<T>((2.0, 3.0), (2.0, 3.0), (2.0, 3.0));
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
mod interval {
    use super::*;

    #[test]
    fn boolean_ops_interval() {
        generic_scenarios::<geom_core::Interval>();
    }
}

fn two_bricks<T: Decide + geom_core::Bounds + topo::AtRestPolicy>() -> (AtRestBody<T>, AtRestBody<T>)
{
    (
        finished_brick::<T>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
        finished_brick::<T>((1.0, 3.0), (1.0, 3.0), (1.0, 3.0)),
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
    let a = finished_brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
    let b = finished_brick::<f64>((1.0, 3.0), (1.0, 3.0), (0.0, 1.0));
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
    let a = finished_brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 3.0));
    let b = finished_brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
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
    let a = finished_brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = finished_brick::<f64>((2.0, 3.0), (2.0, 3.0), (2.0, 3.0));
    // ∪: the typed disjoint union — two solids, one per piece.
    let r = run(union_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Assembly);
    assert_eq!(
        arena_counts(&body.body),
        ArenaCounts {
            solids: 2,
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
    let a = finished_brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 3.0));
    let b = finished_brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
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
    let a = finished_brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = finished_brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    // ∪: a touching assembly with the vv contact carried, remapped to
    // live result keys. Pieces that only touch are distinct solids.
    let r = run(union_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Assembly);
    assert_eq!(
        arena_counts(&body.body),
        ArenaCounts {
            solids: 2,
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
    let a = finished_brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = finished_brick::<f64>((0.75, 1.25), (0.75, 1.25), (1.5, 2.5));
    let r = run(subtract_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    assert_props(&body.body, 8.0 - 0.125, 24.0 + 1.0);
    assert_tier3_posture(&body.body);
}

#[test]
fn boss_union() {
    // The same pillar, added: a boss standing on the top face.
    let a = finished_brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = finished_brick::<f64>((0.75, 1.25), (0.75, 1.25), (1.5, 2.5));
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
    let a = finished_brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = finished_brick::<f64>((0.75, 1.25), (0.75, 1.25), (-0.5, 2.5));
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
    let a = finished_brick::<f64>((0.0, 2.0), (0.0, 2.0), (1.0, 1.5));
    let b = finished_brick::<f64>((0.5, 1.0), (0.5, 1.0), (0.0, 1.25));
    let r = run(union_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    // vol = 2·2·0.5 + 0.5·0.5·1.0; area = slab 2·4 + rim 4 − foot .25
    // + leg walls 4·(0.5·1.0) + leg bottom .25.
    assert_props(&body.body, 2.25, 14.0);
}

// ---------------------------------------------------------------
// Acceptance (3), Problem 15.9's ∖/∩ duality through the door: A∖B and
// A∩B partition A, each through the public door with finished operands.
// The door's own A∖B ≡ A∩revert(B) route is internal: revert(B) is a
// complement, which is not a finished body (tier 3's +V invariant), so
// it refuses at the at-rest gate, naming its solid.
// ---------------------------------------------------------------

#[test]
fn subtract_and_intersect_partition_a_and_a_complement_is_unfinished() {
    let corpus: Vec<(&str, AtRestBody<f64>, AtRestBody<f64>)> = vec![
        (
            "two-brick",
            finished_brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
            finished_brick((1.0, 3.0), (1.0, 3.0), (1.0, 3.0)),
        ),
        (
            "pocket",
            finished_brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
            finished_brick((0.75, 1.25), (0.75, 1.25), (1.5, 2.5)),
        ),
        (
            "disjoint",
            finished_brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
            finished_brick((2.0, 3.0), (2.0, 3.0), (2.0, 3.0)),
        ),
        (
            "nested",
            finished_brick((0.0, 3.0), (0.0, 3.0), (0.0, 3.0)),
            finished_brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0)),
        ),
        (
            "nested-inverted",
            finished_brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0)),
            finished_brick((0.0, 3.0), (0.0, 3.0), (0.0, 3.0)),
        ),
        (
            "corner-kiss",
            finished_brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
            finished_brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0)),
        ),
        // PR 5.5's newly-working seam families.
        (
            "plus-x-pocket",
            finished_brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
            finished_brick((1.5, 2.5), (0.75, 1.25), (0.75, 1.25)),
        ),
        (
            "through-pillar",
            finished_brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
            finished_brick((0.75, 1.25), (0.75, 1.25), (-0.5, 2.5)),
        ),
    ];
    for (name, a, b) in corpus {
        partition_and_complement(name, &a, &b);
    }
}

/// The duality row's two claims on one pair: `vol(A∖B) + vol(A∩B) =
/// vol(A)` exactly (the corpus is dyadic), and `revert(B)` refused at
/// the at-rest gate with `NegativeVolume` alone.
pub(crate) fn partition_and_complement(name: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>) {
    let volume = |r: BooleanResult<f64>| {
        r.body().map_or(0.0, |bb| {
            mass_properties(&bb.body, Tol::witness()).unwrap().volume
        })
    };
    let minus = volume(subtract(a, b, Tol::witness()).unwrap());
    let meet = volume(topo::intersect(a, b, Tol::witness()).unwrap());
    let whole = mass_properties(a, Tol::witness()).unwrap().volume;
    assert_eq!(minus + meet, whole, "{name}: A∖B and A∩B partition A");
    let refused = AtRestBody::validate(b.revert().unwrap(), Tol::witness())
        .expect_err("a complement is not a finished body");
    assert!(
        !refused.is_empty()
            && refused
                .iter()
                .all(|e| matches!(e, topo::ValidationError::NegativeVolume { .. })),
        "{name}: the complement refuses on its +V sign alone, got {refused:?}"
    );
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
    let a = finished_brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = finished_brick::<f64>((0.0, 2.0), (0.0, 2.0), (2.0, 4.0));
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

/// `body` with its one face on `plane` re-charted onto that plane's
/// reversal and its sense flipped: the same outward side on the
/// opposite chart, through the describing door with every stranded
/// edge description carried. Returns the body and the face. `plane` picks a face by its chart's
/// origin and normal and its sense, read off the description, not
/// through any outward-normal door.
fn with_reversed_face(
    body: &Body<f64>,
    plane: impl Fn(geom_core::Point3<f64>, geom_core::Vec3<f64>, bool) -> bool,
) -> (AtRestBody<f64>, topo::FaceKey) {
    let picked: Vec<_> = body
        .faces()
        .filter_map(|(k, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Plane {
                origin,
                normal,
                u_ref,
            }) if plane(*origin, *normal, f.sense) => Some((
                k,
                geom::Surface::Plane {
                    origin: *origin,
                    normal: -*normal,
                    u_ref: *u_ref,
                },
                !f.sense,
            )),
            _ => None,
        })
        .collect();
    let [(face, reversed, sense)] = &picked[..] else {
        panic!("exactly one face on the plane: {picked:?}")
    };
    let mut out = body.clone();
    let charts = vec![topo::Rechart::new(reversed.clone(), *face, *sense)];
    let specs = out.carried_redescriptions(&charts).unwrap();
    out.set_face_surfaces_describing(charts, &specs, Tol::witness())
        .unwrap();
    (finished("the re-charted body", out, Tol::witness()), *face)
}

/// The full-overlap stacked bricks of
/// [`merge_ladder_fires_only_on_declared_planes`], with one face of the
/// upper brick reversed onto its plane's opposite chart (the outward
/// side kept), unioned with the flush pairs declared: the result is the
/// one the unreversed pair gives. Returns the operands and the reversed
/// face.
fn stacked_union_with_a_reversed_face(
    plane: impl Fn(geom_core::Point3<f64>, geom_core::Vec3<f64>, bool) -> bool,
) -> (AtRestBody<f64>, AtRestBody<f64>, topo::FaceKey) {
    let a = finished_brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let (b, reversed) = with_reversed_face(
        &finished_brick::<f64>((0.0, 2.0), (0.0, 2.0), (2.0, 4.0)),
        plane,
    );
    let r = run(union_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    assert_eq!(body.body.faces().count(), 6, "declared-rung merge census");
    assert_props(&body.body, 16.0, 40.0);
    (a, b, reversed)
}

/// **The REST lane reads the contact face's sense.** The upper brick's
/// bottom (z = 2, outward -z) is charted +z with `sense: false`, so the
/// contact's carriers face apart only through the bit: the flush
/// detector must class the pair a rest, and the declaration door
/// verifies a rest by the senses it reads off the two carriers. A
/// carrier door that drops the bit reads the faces facing one way and
/// classes the contact a continuation.
#[test]
fn stacked_union_rests_on_a_contact_face_reversed_onto_its_opposite_chart() {
    let (a, b, bottom) = stacked_union_with_a_reversed_face(|o, n, sense| {
        o.z == 2.0 && n.z != 0.0 && !sense == (n.z > 0.0)
    });
    let top = a
        .faces()
        .find(|(_, f)| {
            matches!(a.get_surface(f.surface), Some(geom::Surface::Plane { origin, normal, .. })
                if origin.z == 2.0 && f.sense == (normal.z > 0.0))
        })
        .map(|(k, _)| k)
        .expect("the lower brick's top");
    let decls = common::flush_declarations(&a, &b, Tol::witness());
    let classes: Vec<_> = decls
        .coincident_faces
        .iter()
        .filter(|d| (d.a, d.b) == (top, bottom))
        .map(|d| d.class)
        .collect();
    assert_eq!(
        classes,
        vec![topo::BooleanCoincidence::Contact(topo::ContactClass::Rest)],
        "the contact is declared once, as a rest"
    );
    // The door verifies the rest it is handed against the senses it
    // reads, whatever the detector found: the recipe's rest, stated.
    let mut stated = decls;
    stated
        .coincident_faces
        .retain(|d| (d.a, d.b) != (top, bottom));
    stated.coincident_faces.push(topo::FacePairDeclaration::new(
        top,
        bottom,
        topo::ContactClass::Rest,
    ));
    let r = union_with(&a, &b, &stated, Tol::witness())
        .expect("a rest whose carriers face apart is verified");
    assert_eq!(body_of(&r).kind, BooleanResultKind::Seamed);
}

/// **The merge stage's declared rung reads a side wall's sense.** The
/// upper brick's x = 0 wall (outward -x) is charted +x with
/// `sense: false`; the lower brick's wall on the same plane keeps
/// `sense: true`. The pair reaches the declared rung, and only the bit
/// makes the two outward normals agree: a planar door that drops it
/// refuses the pair as opposite.
#[test]
fn stacked_union_merges_a_side_wall_reversed_onto_its_opposite_chart() {
    stacked_union_with_a_reversed_face(|o, n, sense| {
        o.x == 0.0 && n.x != 0.0 && !sense == (n.x > 0.0)
    });
}

/// **The ring lane winds the island's run about the face's OUTWARD
/// normal.** [`pocket_subtract`] with the block's top (z = 2, outward
/// +z) charted -z with `sense: false`: the pocket's rim is an island in
/// that face, and which side of the run is the outer boundary follows
/// from the run's winding read about the outward normal, so the bit is
/// what keeps the roles. A door that drops it reads the run's winding
/// backwards.
#[test]
fn pocket_subtract_into_a_top_reversed_onto_its_opposite_chart() {
    let (a, _) = with_reversed_face(
        &finished_brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
        |o, n, sense| o.z == 2.0 && n.z != 0.0 && sense == (n.z > 0.0),
    );
    let b = finished_brick::<f64>((0.75, 1.25), (0.75, 1.25), (1.5, 2.5));
    let r = run(subtract_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    assert_props(&body.body, 8.0 - 0.125, 24.0 + 1.0);
    assert_tier3_posture(&body.body);
}

#[test]
fn tangential_rest_operands() {
    // Full-face coplanar rest (PR 4: ∩/∖ classify to contacts only).
    let a = finished_brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = finished_brick::<f64>((0.0, 2.0), (0.0, 2.0), (2.0, 4.0));
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
