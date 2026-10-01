//! Two stacked plates on one footprint: the mating plane is a `Rest`
//! contact, and each pair of outer walls — one carrier, the same
//! outward sense, meeting only along the mating plane — is a
//! CONTINUATION (`crates/topo/README.md`, C4's continuation clause and
//! the crossing layer's one-sided cover).
//!
//! The rows: an undeclared continuation refuses at the reduction on a
//! plane and on a cylinder; a declared one merges (planar) or ships as
//! the recorded curved skip; a `Rest` on an aligned pair and a
//! continuation on an opposed one are each contradicted; the rounded
//! stack's tangent wall edges are covered through a structural tangency
//! on EITHER operand, while a tangency in the middle of an edge keeps
//! its typed refusal; and every output is a legal boolean operand.
//! Two guard rows close the file: a kiss or a gap is no continuation,
//! and aligned pairs whose interiors overlap are accepted as
//! continuations today (pinned as behaviour, not as a ruling).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Open, ProfileLoop, RawLoop, Start};
use sweep::test_support::{extruded, sketch_at};
use topo::{
    Body, BooleanBody, BooleanCoincidence, BooleanDeclarations, BooleanError, BooleanResult,
    FacePairDeclaration, Operand, PlaneRelation,
};

const W: f64 = 6.0;
const H: f64 = 4.0;
const R: f64 = 0.5;

fn tol() -> Tol {
    Tol::witness()
}

/// The 6×4 outline, its four corners rounded by `r` through the PATHS
/// fillet door.
fn rounded(r: f64) -> ProfileLoop<f64> {
    let t = tol();
    Open.at(Point2::new(W / 2.0, 0.0))
        .toward(1.0, 0.0, t)
        .expect("south runs east")
        .fillet(r, t)
        .expect("a positive radius")
        .at(Point2::new(W, H / 2.0), t)
        .expect("the south-east fillet fits")
        .toward(0.0, 1.0, t)
        .expect("east runs north")
        .fillet(r, t)
        .expect("a positive radius")
        .at(Point2::new(W / 2.0, H), t)
        .expect("the north-east fillet fits")
        .toward(-1.0, 0.0, t)
        .expect("north runs west")
        .fillet(r, t)
        .expect("a positive radius")
        .at(Point2::new(0.0, H / 2.0), t)
        .expect("the north-west fillet fits")
        .toward(0.0, -1.0, t)
        .expect("west runs south")
        .fillet(r, t)
        .expect("a positive radius")
        .to(Start, t)
        .expect("the south-west fillet fits")
        .into()
}

/// The rounded outline cut back to `x ≤ W − r`: the west corners keep
/// their fillets and the east side is a sharp wall standing exactly on
/// the tangent points of the full outline's east fillets.
fn rounded_west(r: f64) -> ProfileLoop<f64> {
    let t = tol();
    Open.at(Point2::new(W / 2.0, 0.0))
        .line_to(Point2::new(W - r, 0.0), t)
        .expect("south runs east")
        .line_to(Point2::new(W - r, H), t)
        .expect("the sharp east side")
        .toward(-1.0, 0.0, t)
        .expect("north runs west")
        .fillet(r, t)
        .expect("a positive radius")
        .at(Point2::new(0.0, H / 2.0), t)
        .expect("the north-west fillet fits")
        .toward(0.0, -1.0, t)
        .expect("west runs south")
        .fillet(r, t)
        .expect("a positive radius")
        .to(Start, t)
        .expect("the south-west fillet fits")
        .into()
}

fn sharp() -> ProfileLoop<f64> {
    ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(W, 0.0),
        Point2::new(W, H),
        Point2::new(0.0, H),
    ])
}

/// A unit-thick plate of `outline`, its bottom at `z0`.
fn plate(outline: ProfileLoop<f64>, z0: f64) -> Body<f64> {
    extruded(sketch_at(z0), vec![outline], 1.0, tol())
}

/// The 6×4 area less what `rounded_corners` fillets of radius `R` take.
fn area(rounded_corners: f64) -> f64 {
    W * H - rounded_corners * (1.0 - core::f64::consts::FRAC_PI_4) * R * R
}

/// The flush detector's findings between `a` and `b`, split by class:
/// the `Rest` contacts and the continuations, each declared.
fn findings(a: &Body<f64>, b: &Body<f64>) -> (BooleanDeclarations, BooleanDeclarations) {
    let found = topo::flush::find_flush_candidates(a, b, tol()).expect("the plates decide");
    let (rest, cont): (Vec<_>, Vec<_>) = found
        .into_iter()
        .partition(|f| f.class == BooleanCoincidence::REST);
    assert!(
        cont.iter()
            .all(|f| f.class == BooleanCoincidence::Continuation
                && f.evidence.relation == PlaneRelation::SameOriented),
        "an aligned finding is a continuation: {cont:?}"
    );
    (
        topo::flush::declare_all(&rest),
        topo::flush::declare_all(&cont),
    )
}

fn with(a: &BooleanDeclarations, b: &BooleanDeclarations) -> BooleanDeclarations {
    let mut d = a.clone();
    d.coincident_faces
        .extend(b.coincident_faces.iter().copied());
    d
}

fn is_cylinder(body: &Body<f64>, f: topo::FaceKey) -> bool {
    matches!(
        body.get_face(f).and_then(|x| body.get_surface(x.surface)),
        Some(geom::Surface::Cylinder { .. })
    )
}

fn is_plane(body: &Body<f64>, f: topo::FaceKey) -> bool {
    matches!(
        body.get_face(f).and_then(|x| body.get_surface(x.surface)),
        Some(geom::Surface::Plane { .. })
    )
}

fn volume(body: &Body<f64>) -> f64 {
    topo::mass_properties(body, tol()).unwrap().volume
}

/// The union, which must build, be additive and valid at tier 3 and 3′.
fn union_honest(
    label: &str,
    a: &Body<f64>,
    b: &Body<f64>,
    d: &BooleanDeclarations,
) -> BooleanBody<f64> {
    let out = topo::union_with(a, b, d, tol()).unwrap_or_else(|e| panic!("{label}: {e:?}"));
    let BooleanResult::Body(bb) = out else {
        panic!("{label}: a stack is not empty");
    };
    let (v, sum) = (volume(&bb.body), volume(a) + volume(b));
    assert!(
        (v - sum).abs() <= 8.0 * f64::EPSILON * sum,
        "{label}: {v} vs {sum}"
    );
    assert_eq!(
        topo::validate_geometric(&bb.body, tol()),
        Ok(()),
        "{label}: tier 3"
    );
    assert_eq!(
        topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()),
        Ok(()),
        "{label}: tier 3′"
    );
    bb
}

/// The undeclared-continuation refusal, A's face then B's.
fn refused_continuation(label: &str, err: &BooleanError) -> (topo::FaceKey, topo::FaceKey) {
    let BooleanError::UndeclaredCoincidence {
        pair: [(Operand::A, fa), (Operand::B, fb)],
        relation: PlaneRelation::SameOriented,
        ..
    } = *err
    else {
        panic!("{label}: an undeclared continuation, A then B: {err:?}");
    };
    (fa, fb)
}

/// **An undeclared continuation refuses at the reduction, on a plane
/// and on a cylinder alike**, naming the pair and its aligned relation.
///
/// Sharp stack, mating plane only: the refused pair is two flat walls.
/// Rounded stack, mating plane and the flat walls declared: what is
/// left undeclared is the corner fillets, and the refused pair is two
/// cylinders — the refusal the crossing layer's `CurvedPierceUnsupported`
/// used to stand in for. Rounded stack, mating plane only: it names
/// the first undeclared wall pair.
#[test]
fn an_undeclared_continuation_refuses_on_a_plane_and_on_a_cylinder() {
    let (p, q) = (plate(sharp(), 0.0), plate(sharp(), 1.0));
    let (mate, walls) = findings(&p, &q);
    assert_eq!(
        (mate.coincident_faces.len(), walls.coincident_faces.len()),
        (1, 4),
        "sharp: the mating plane, and four wall continuations"
    );
    let err = topo::union_with(&p, &q, &mate, tol()).expect_err("the walls are undeclared");
    let (fa, fb) = refused_continuation("sharp", &err);
    assert!(
        is_plane(&p, fa) && is_plane(&q, fb),
        "two flat walls: {err:?}"
    );
    assert!(
        walls
            .coincident_faces
            .iter()
            .any(|d| (d.a, d.b) == (fa, fb)),
        "the refused pair is one the detector offers as a continuation"
    );

    let (p, q) = (plate(rounded(R), 0.0), plate(rounded(R), 1.0));
    let (mate, walls) = findings(&p, &q);
    assert_eq!(
        (mate.coincident_faces.len(), walls.coincident_faces.len()),
        (1, 8),
        "rounded: the mating plane, four flat walls and four fillets"
    );
    let err = topo::union_with(&p, &q, &mate, tol()).expect_err("the walls are undeclared");
    let (fa, fb) = refused_continuation("rounded, mating plane only", &err);
    assert!(
        walls
            .coincident_faces
            .iter()
            .any(|d| (d.a, d.b) == (fa, fb)),
        "the first undeclared wall pair: {err:?}"
    );
    let mut flat = mate.clone();
    flat.coincident_faces.extend(
        walls
            .coincident_faces
            .iter()
            .filter(|d| is_plane(&p, d.a))
            .copied(),
    );
    let err = topo::union_with(&p, &q, &flat, tol()).expect_err("the fillets are undeclared");
    let (fa, fb) = refused_continuation("rounded, fillets undeclared", &err);
    assert!(
        is_cylinder(&p, fa) && is_cylinder(&q, fb),
        "two fillet cylinders: {err:?}"
    );
}

/// **A declared continuation merges a planar pair and ships a curved
/// one as the recorded skip**, and the stack is exact.
///
/// Sharp: six faces, nothing skipped. Rounded: the four flat wall pairs
/// glue, and the four fillet pairs stay two faces each — fourteen
/// faces, four `SkippedMerge` records, the merge's declared rung having
/// no cylinder arm.
#[test]
fn a_declared_continuation_merges_planar_walls_and_records_the_curved_skip() {
    let (p, q) = (plate(sharp(), 0.0), plate(sharp(), 1.0));
    let (mate, walls) = findings(&p, &q);
    let bb = union_honest("sharp", &p, &q, &with(&mate, &walls));
    assert_eq!(bb.body.faces().count(), 6, "sharp: one box");
    assert!(
        bb.naming.merge_skipped.is_empty(),
        "{:?}",
        bb.naming.merge_skipped
    );
    assert_eq!(volume(&bb.body), 2.0 * W * H, "sharp: two plates' worth");

    let (p, q) = (plate(rounded(R), 0.0), plate(rounded(R), 1.0));
    let (mate, walls) = findings(&p, &q);
    let bb = union_honest("rounded", &p, &q, &with(&mate, &walls));
    let cylinders = bb
        .body
        .faces()
        .filter(|&(k, _)| is_cylinder(&bb.body, k))
        .count();
    assert_eq!(
        (bb.body.faces().count(), cylinders),
        (14, 8),
        "rounded: top, bottom, four merged walls, and four fillets in two faces each"
    );
    assert_eq!(
        bb.naming.merge_skipped.len(),
        4,
        "{:?}",
        bb.naming.merge_skipped
    );
    let v = volume(&bb.body);
    let closed = 2.0 * area(4.0);
    assert!((v - closed).abs() <= 1e-12, "rounded: {v} vs {closed}");
}

/// **`Rest` means opposed senses at the boolean's door**, and a
/// continuation aligned ones: each is contradicted by the other's sense
/// bit, typed, naming what was declared.
#[test]
fn rest_on_an_aligned_pair_and_a_continuation_on_an_opposed_pair_are_contradicted() {
    let (p, q) = (plate(sharp(), 0.0), plate(sharp(), 1.0));
    let (mate, walls) = findings(&p, &q);
    let wall = walls.coincident_faces[0];
    let mut lie = with(&mate, &walls);
    lie.coincident_faces[1] = FacePairDeclaration::rest(wall.a, wall.b);
    let err = topo::union_with(&p, &q, &lie, tol()).expect_err("a wall pair is not a Rest");
    let BooleanError::ContactContradicted {
        declaration,
        margin,
        ..
    } = &err
    else {
        panic!("a contradicted Rest: {err:?}");
    };
    assert_eq!((declaration.a, declaration.b), (wall.a, wall.b));
    assert_eq!(declaration.class, topo::ContactClass::Rest);
    assert_eq!(margin.predicate, Some("contact_rest_senses_opposed"));

    let plane = mate.coincident_faces[0];
    let mut lie = with(&mate, &walls);
    lie.coincident_faces[0] = FacePairDeclaration::continuation(plane.a, plane.b);
    let err =
        topo::union_with(&p, &q, &lie, tol()).expect_err("the mating plane is no continuation");
    let BooleanError::ContinuationContradicted { a, b, fact, margin } = &err else {
        panic!("a contradicted continuation: {err:?}");
    };
    assert_eq!((*a, *b), (plane.a, plane.b));
    assert_eq!(*fact, None, "the senses decided it, not the carriers");
    assert_eq!(margin.predicate, Some("continuation_senses_aligned"));
    assert!(err.to_string().contains("continuation"), "{err}");
}

/// **The one-sided cover reaches a tangent wall edge through a
/// structural tangency on EITHER operand.**
///
/// Rounded on rounded: P's flat wall edge ends tangent to Q's fillet
/// cylinder, covered by P's own strut (`TangentIntersection` with P's
/// fillet) and the verified continuation of P's fillet into Q's.
///
/// Rounded under a plate cut back to `x ≤ W − r` (west corners rounded,
/// east side sharp): Q's flat south wall ends exactly at P's south-east
/// tangent point. Q has no strut there; the tangency is P's (wall ↔
/// fillet), reached through the declared continuation of Q's wall into
/// P's. It builds in both operand orders.
#[test]
fn the_tangent_edge_cover_reads_a_strut_on_either_operand() {
    let (p, q) = (plate(rounded(R), 0.0), plate(rounded(R), 1.0));
    let (mate, walls) = findings(&p, &q);
    union_honest("rounded on rounded", &p, &q, &with(&mate, &walls));

    let (p, q) = (plate(rounded(R), 0.0), plate(rounded_west(R), 1.0));
    for (label, a, b) in [("P is A", &p, &q), ("Q is A", &q, &p)] {
        let (mate, walls) = findings(a, b);
        let bb = union_honest(label, a, b, &with(&mate, &walls));
        let closed = area(4.0) + (W - R) * H - 2.0 * (1.0 - core::f64::consts::FRAC_PI_4) * R * R;
        let v = volume(&bb.body);
        assert!((v - closed).abs() <= 1e-12, "{label}: {v} vs {closed}");
    }
}

/// **A tangency in the middle of an edge keeps its typed refusal.** A
/// sharp plate stacked on a rounded one: the sharp plate's straight
/// bottom edges pass the rounded plate's tangent points mid-span, where
/// the cover — which records endpoints only — has nothing to say, so
/// the crossing layer refuses as before. (Taken the other way round the
/// rounded operand's own wall edges are swept first and split the
/// sharp edges at the tangent points, so the touch lands on endpoints
/// and the stack builds; that order is the row below the refusal.)
#[test]
fn a_tangency_in_the_middle_of_an_edge_keeps_its_typed_refusal() {
    let (sharp_q, rounded_p) = (plate(sharp(), 1.0), plate(rounded(R), 0.0));
    let (mate, walls) = findings(&sharp_q, &rounded_p);
    let err = topo::union_with(&sharp_q, &rounded_p, &with(&mate, &walls), tol())
        .expect_err("a mid-edge tangency is a frontier");
    assert!(
        matches!(
            err,
            BooleanError::CurvedPierceUnsupported {
                operand: Operand::A,
                ..
            }
        ),
        "{err:?}"
    );
    let (mate, walls) = findings(&rounded_p, &sharp_q);
    union_honest("rounded is A", &rounded_p, &sharp_q, &with(&mate, &walls));
}

/// **Every output is a legal boolean operand.** Each stack's result is
/// unioned again, both with a plate standing clear of it and with a
/// third plate stacked on its top (the mating plane and the new
/// continuations declared) — so no unmerged planar continuation ships,
/// and the recorded curved skip is the form the next boolean accepts.
#[test]
fn the_stack_is_a_legal_operand() {
    for (label, outline) in [
        ("sharp", sharp as fn() -> ProfileLoop<f64>),
        ("rounded", || rounded(R)),
    ] {
        let (p, q) = (plate(outline(), 0.0), plate(outline(), 1.0));
        let (mate, walls) = findings(&p, &q);
        let stack = union_honest(label, &p, &q, &with(&mate, &walls)).body;
        let clear = plate(outline(), 5.0);
        let BooleanResult::Body(_) = topo::union(&stack, &clear, tol())
            .unwrap_or_else(|e| panic!("{label}: the stack is an operand: {e:?}"))
        else {
            panic!("{label}: not empty");
        };
        let third = plate(outline(), 2.0);
        let (mate, walls) = findings(&stack, &third);
        union_honest(
            &format!("{label}, third plate"),
            &stack,
            &third,
            &with(&mate, &walls),
        );
    }
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    sweep::test_support::brick(x, y, z, tol())
}

/// The rabbeted plate: 6 × 4 × 1 with a 1 × 0.5 step cut along its
/// east edge, swept along y from its xz section (volume 22).
fn rabbeted() -> Body<f64> {
    let section = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(W, 0.0),
        Point2::new(W, 0.5),
        Point2::new(W - 1.0, 0.5),
        Point2::new(W - 1.0, 1.0),
        Point2::new(0.0, 1.0),
    ]);
    let xz = sweep::test_support::sketch_from_axes(
        geom_core::Point3::new(0.0, H, 0.0),
        geom_core::Vec3::new(1.0, 0.0, 0.0),
        geom_core::Vec3::new(0.0, 0.0, 1.0),
        tol(),
    );
    extruded(xz, vec![section], H, tol())
}

/// A boolean that must build, at `expect`, valid at tier 3 and 3′.
fn builds(
    label: &str,
    out: Result<BooleanResult<f64>, BooleanError>,
    expect: f64,
    faces: usize,
) -> BooleanBody<f64> {
    let Ok(BooleanResult::Body(bb)) = out else {
        panic!("{label}: {out:?}");
    };
    let v = volume(&bb.body);
    assert!((v - expect).abs() <= 1e-12, "{label}: {v} vs {expect}");
    assert_eq!(bb.body.faces().count(), faces, "{label}: faces");
    assert_eq!(
        topo::validate_geometric(&bb.body, tol()),
        Ok(()),
        "{label}: tier 3"
    );
    assert_eq!(
        topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()),
        Ok(()),
        "{label}: tier 3′"
    );
    bb
}

/// **The scan does not mistake a touch for a meeting.** Three placements
/// of a second plate beside the 6 × 4 × 1 one, each with tops and
/// bottoms on one plane and each sharing no stretch of boundary with
/// it, union UNDECLARED:
///
/// - corner kiss: the plates share one vertical edge line (x = 6,
///   y = 4) and nothing else;
/// - vertex kiss: lifted a plate's height as well, they share one
///   point;
/// - a real gap: the second plate starts a unit east, so the coplanar
///   tops and bottoms (the detector offers them as continuations: one
///   carrier, aligned) never meet.
///
/// A scan that read box overlap, or a shared point, as "meets" refuses
/// the first two as continuations and a box pad wider than the gap
/// refuses the third.
#[test]
fn a_kiss_or_a_gap_is_no_continuation() {
    let none = BooleanDeclarations::default();
    let p = brick((0.0, W), (0.0, H), (0.0, 1.0));
    for (label, q, offered) in [
        (
            "corner kiss",
            brick((W, 2.0 * W), (H, 2.0 * H), (0.0, 1.0)),
            (2, 2),
        ),
        (
            "vertex kiss",
            brick((W, 2.0 * W), (H, 2.0 * H), (1.0, 2.0)),
            (3, 0),
        ),
        (
            "gap",
            brick((W + 1.0, 2.0 * W), (0.0, H), (0.0, 1.0)),
            (0, 4),
        ),
    ] {
        let (rest, cont) = findings(&p, &q);
        assert_eq!(
            (rest.coincident_faces.len(), cont.coincident_faces.len()),
            offered,
            "{label}: the detector reports carriers, not meetings"
        );
        builds(
            label,
            topo::union_with(&p, &q, &none, tol()),
            volume(&p) + volume(&q),
            12,
        );
    }
}

/// **Aligned pairs whose interiors OVERLAP are minted and accepted as
/// continuations today.** C4 defines a continuation as interiors
/// disjoint; the detector and the door do not ask, and the results are
/// exact. These rows pin that behaviour, not a ruling: whether the
/// definition widens to match is Ev's question, asked separately.
///
/// Each refuses undeclared (and with only its `Rest` findings), naming
/// an aligned pair, and builds exact with every finding declared:
///
/// - overlapping equal-height plates: the tops overlap, as do the
///   bottoms;
/// - a sunk stack: a second plate half sunk into the first, its walls
///   overlapping the first plate's;
/// - a flush pocket: a subtract whose cutter's top is flush with the
///   plate's;
/// - a rabbet filled: a block in a rabbet, its top flush with the
///   plate's and its east wall with the step's.
#[test]
fn overlapping_aligned_pairs_are_accepted_as_continuations() {
    let none = BooleanDeclarations::default();
    let p = brick((0.0, W), (0.0, H), (0.0, 1.0));
    let rows: [(&str, bool, Body<f64>, Body<f64>, f64, usize); 4] = [
        (
            "overlapping plates",
            false,
            p.clone(),
            brick((3.0, 9.0), (1.0, 3.0), (0.0, 1.0)),
            30.0,
            10,
        ),
        (
            "sunk stack",
            false,
            p.clone(),
            brick((0.0, W), (0.0, H), (0.5, 1.5)),
            36.0,
            6,
        ),
        (
            "flush pocket",
            true,
            p.clone(),
            brick((2.0, 4.0), (1.0, 3.0), (0.5, 1.0)),
            22.0,
            11,
        ),
        (
            "rabbet filled",
            false,
            rabbeted(),
            brick((W - 1.0, W), (0.0, H), (0.5, 1.0)),
            24.0,
            6,
        ),
    ];
    for (label, subtract, a, b, expect, faces) in rows {
        let op = |d: &BooleanDeclarations| {
            if subtract {
                topo::subtract_with(&a, &b, d, tol())
            } else {
                topo::union_with(&a, &b, d, tol())
            }
        };
        let (rest, cont) = findings(&a, &b);
        assert!(
            !cont.coincident_faces.is_empty(),
            "{label}: the detector mints the overlapping aligned pairs as continuations"
        );
        for (posture, d) in [("undeclared", &none), ("Rest only", &rest)] {
            let err = op(d).expect_err("an aligned pair is undeclared");
            assert!(
                matches!(
                    err,
                    BooleanError::UndeclaredCoincidence {
                        relation: PlaneRelation::SameOriented,
                        ..
                    }
                ),
                "{label}, {posture}: {err:?}"
            );
        }
        builds(label, op(&with(&rest, &cont)), expect, faces);
    }
}
