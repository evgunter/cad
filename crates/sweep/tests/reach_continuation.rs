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
//! on EITHER operand, and a tangency in the middle of an edge builds in
//! either operand order; and every output is a legal boolean operand.
//! The rows after them: a kiss or a gap is no continuation; an
//! overlapping continuation refuses undeclared and builds declared in
//! every op; and the declared overlaps that still refuse are pinned at
//! their typed refusals, each filed.

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

/// The L: 6 × 6 less its 3 × 3 north-east quarter (area 27), every
/// corner — the concave one included — rounded by `r`.
fn ell_rounded(r: f64) -> ProfileLoop<f64> {
    let t = tol();
    let mut path = Open
        .at(Point2::new(3.0, 0.0))
        .toward(1.0, 0.0, t)
        .expect("south runs east");
    for (corner, (dx, dy)) in [
        (Point2::new(6.0, 1.5), (0.0, 1.0)),
        (Point2::new(4.5, 3.0), (-1.0, 0.0)),
        (Point2::new(3.0, 4.5), (0.0, 1.0)),
        (Point2::new(1.5, 6.0), (-1.0, 0.0)),
        (Point2::new(0.0, 3.0), (0.0, -1.0)),
    ] {
        path = path
            .fillet(r, t)
            .expect("a positive radius")
            .at(corner, t)
            .expect("the fillet fits")
            .toward(dx, dy, t)
            .expect("the next side");
    }
    path.fillet(r, t)
        .expect("a positive radius")
        .to(Start, t)
        .expect("the last fillet fits")
        .into()
}

fn ell_sharp() -> ProfileLoop<f64> {
    ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(6.0, 0.0),
        Point2::new(6.0, 3.0),
        Point2::new(3.0, 3.0),
        Point2::new(3.0, 6.0),
        Point2::new(0.0, 6.0),
    ])
}

/// The area one corner fillet of radius `r` takes off a convex corner
/// (or adds to a concave one): the unit square's corner less a quarter
/// disc.
fn corner(r: f64) -> f64 {
    (1.0 - core::f64::consts::FRAC_PI_4) * r * r
}

/// **A tangency in the middle of an edge builds in either operand
/// order, in every op.** A plate stacked on a rounded one: the upper
/// plate's straight bottom edges pass the lower plate's fillet tangent
/// points mid-span, where the lower plate has a vertex (its flat wall
/// ends there) and the upper one has none. Whichever operand's edges
/// are swept first, the touch is read on the edge's fragments once both
/// directions have split it, and the `Rest` seam along each fillet's rim
/// carries the fillet's arc on both sides.
///
/// The poses: the 6 × 4 sharp plate on its rounded twin, and the 6 × 6
/// sharp L (a 3 × 3 notch: five convex corners and a concave one, whose
/// fillet adds material the sharp L lacks) on its rounded twin, each at
/// r = 0.25, 0.5 and 1; and two rounded plates whose fillets differ
/// (0.3 over 0.5, and 0.5 over 0.3), where the larger fillet's tangent
/// points fall mid-edge on the smaller one's flat walls. Volumes are
/// closed form: a plate's area is 24 − 4c(r) and the L's 27 − 5c(r) +
/// c(r), with c the [`corner`] area. The stacked interiors are
/// disjoint, so the union is the sum, each difference is its minuend,
/// and the intersection is empty. Faces: a sharp plate 6, a rounded one
/// 10; the sharp L 8, the rounded one 14. The unions: on the plate 14
/// (two caps, four corner overhangs, four merged walls, four fillets),
/// on the L 20 (two caps, five corner overhangs, the rounded L's top
/// exposed at the concave fillet, six merged walls, six fillets), and
/// the mismatched pair 18 (two caps, four exposed corners of the
/// mating plane, four merged walls, eight fillets).
#[test]
fn a_tangency_in_the_middle_of_an_edge_builds_in_either_operand_order() {
    let rounded_plate = |r: f64| (W * H - 4.0 * corner(r), 10);
    let mut poses = Vec::new();
    for r in [0.25, 0.5, 1.0] {
        poses.push((
            format!("plate, r = {r}"),
            plate(sharp(), 1.0),
            plate(rounded(r), 0.0),
            (W * H, 6),
            rounded_plate(r),
            14,
        ));
        poses.push((
            format!("L, r = {r}"),
            plate(ell_sharp(), 1.0),
            plate(ell_rounded(r), 0.0),
            (27.0, 8),
            (27.0 - 4.0 * corner(r), 14),
            20,
        ));
    }
    for (upper, lower) in [(0.3, 0.5), (0.5, 0.3)] {
        poses.push((
            format!("r = {upper} over r = {lower}"),
            plate(rounded(upper), 1.0),
            plate(rounded(lower), 0.0),
            rounded_plate(upper),
            rounded_plate(lower),
            18,
        ));
    }
    for (pose, upper, lower, upper_vf, lower_vf, union_faces) in poses {
        for (order, a, b, (va, fa), (vb, fb)) in [
            ("upper is A", &upper, &lower, upper_vf, lower_vf),
            ("lower is A", &lower, &upper, lower_vf, upper_vf),
        ] {
            let label = format!("{pose}, {order}");
            let (mate, walls) = findings(a, b);
            let ab = with(&mate, &walls);
            let (mate, walls) = findings(b, a);
            let ba = with(&mate, &walls);
            builds(
                &format!("{label}: A ∪ B"),
                topo::union_with(a, b, &ab, tol()),
                va + vb,
                union_faces,
            );
            builds(
                &format!("{label}: A ∖ B"),
                topo::subtract_with(a, b, &ab, tol()),
                va,
                fa,
            );
            builds(
                &format!("{label}: B ∖ A"),
                topo::subtract_with(b, a, &ba, tol()),
                vb,
                fb,
            );
            let meet = topo::intersect_with(a, b, &ab, tol());
            assert!(
                matches!(meet, Ok(BooleanResult::Empty)),
                "{label}: A ∩ B, the interiors are disjoint: {meet:?}"
            );
        }
    }
}

/// **A declared `Tangent` touching a fillet mid-edge builds its
/// subtracts and intersect in either operand order, and refuses its
/// union.** A unit box stands beside the rounded plate, turned 45° so
/// its west wall is tangent to the south-east fillet along the ruling at
/// azimuth −45°: the box's wall edges pass that ruling mid-span and the
/// plate has no edge on it. The pair declared `Tangent` is the cover
/// (C4); undeclared, the graze refuses typed in both orders. The union
/// would have material on both sides of a ruling through the box wall's
/// interior, the unbuilt doubled-slit arm, so it refuses. Volumes are
/// closed form (the box 1, the plate [`area`] of four corners, interiors
/// disjoint); faces: the box 6, the plate 10.
#[test]
fn a_declared_tangent_beside_a_fillet_refuses_its_union_and_builds_the_rest_in_either_order() {
    let s2 = core::f64::consts::FRAC_1_SQRT_2;
    let touch = Point2::new(W - R + R * s2, R - R * s2);
    let at = |along: f64, out: f64| {
        Point2::new(touch.x + (along + out) * s2, touch.y + (along - out) * s2)
    };
    let boxed = plate(
        ProfileLoop::polygon([at(-0.5, 0.0), at(-0.5, 1.0), at(0.5, 1.0), at(0.5, 0.0)]),
        0.0,
    );
    let p = plate(rounded(R), 0.0);
    let wall = boxed
        .faces()
        .map(|(k, _)| k)
        .find(|&f| {
            matches!(
                boxed.get_face(f).and_then(|x| boxed.get_surface(x.surface)),
                Some(geom::Surface::Plane { normal, .. })
                    if (normal.x + s2).abs() < 1e-9 && (normal.y - s2).abs() < 1e-9
            )
        })
        .expect("the box's tangent wall");
    let fillet = p
        .faces()
        .map(|(k, _)| k)
        .find(|&f| {
            matches!(
                p.get_face(f).and_then(|x| p.get_surface(x.surface)),
                Some(geom::Surface::Cylinder { origin, .. }) if origin.x > W / 2.0 && origin.y < H / 2.0
            )
        })
        .expect("the south-east fillet");
    let tangent = |fa, fb| BooleanDeclarations {
        coincident_faces: vec![FacePairDeclaration::new(
            fa,
            fb,
            topo::ContactClass::Tangent,
        )],
        ..BooleanDeclarations::default()
    };
    for (order, a, b, fa, fb, (va, na), (vb, nb)) in [
        (
            "box is A",
            &boxed,
            &p,
            wall,
            fillet,
            (1.0, 6),
            (area(4.0), 10),
        ),
        (
            "plate is A",
            &p,
            &boxed,
            fillet,
            wall,
            (area(4.0), 10),
            (1.0, 6),
        ),
    ] {
        let err = topo::union_with(a, b, &BooleanDeclarations::default(), tol())
            .expect_err("an undeclared graze refuses");
        assert!(
            matches!(err, BooleanError::CurvedPierceUnsupported { .. }),
            "{order}, undeclared: {err:?}"
        );
        let (ab, ba) = (tangent(fa, fb), tangent(fb, fa));
        let box_operand = if order == "box is A" {
            topo::Operand::A
        } else {
            topo::Operand::B
        };
        match topo::union_with(a, b, &ab, tol()) {
            Err(BooleanError::TangentSlitArmUnbuilt { interior, .. }) => {
                assert_eq!(interior, box_operand, "{order}: A ∪ B, the box wall");
            }
            out => panic!(
                "{order}: A ∪ B, the slit arm is unbuilt: {:?}",
                out.map(|_| ())
            ),
        }
        builds(
            &format!("{order}: A ∖ B"),
            topo::subtract_with(a, b, &ab, tol()),
            va,
            na,
        );
        builds(
            &format!("{order}: B ∖ A"),
            topo::subtract_with(b, a, &ba, tol()),
            vb,
            nb,
        );
        let meet = topo::intersect_with(a, b, &ab, tol());
        assert!(
            matches!(meet, Ok(BooleanResult::Empty)),
            "{order}: A ∩ B, the interiors are disjoint: {meet:?}"
        );
    }
}

/// The 45° unit box of
/// [`a_declared_tangent_beside_a_fillet_refuses_its_union_and_builds_the_rest_in_either_order`],
/// its west wall on the south-east fillet's ruling at azimuth −45°,
/// standing from `z0` to `z1`; and that wall and the fillet.
fn box_beside_the_fillet(
    p: &Body<f64>,
    z0: f64,
    z1: f64,
) -> (Body<f64>, topo::FaceKey, topo::FaceKey) {
    let s2 = core::f64::consts::FRAC_1_SQRT_2;
    let touch = Point2::new(W - R + R * s2, R - R * s2);
    let at = |along: f64, out: f64| {
        Point2::new(touch.x + (along + out) * s2, touch.y + (along - out) * s2)
    };
    let boxed = extruded(
        sketch_at(z0),
        vec![ProfileLoop::polygon([
            at(-0.5, 0.0),
            at(-0.5, 1.0),
            at(0.5, 1.0),
            at(0.5, 0.0),
        ])],
        z1 - z0,
        tol(),
    );
    let wall = boxed
        .faces()
        .map(|(k, _)| k)
        .find(|&f| {
            matches!(
                boxed.get_face(f).and_then(|x| boxed.get_surface(x.surface)),
                Some(geom::Surface::Plane { normal, .. })
                    if (normal.x + s2).abs() < 1e-9 && (normal.y - s2).abs() < 1e-9
            )
        })
        .expect("the box's tangent wall");
    let fillet = p
        .faces()
        .map(|(k, _)| k)
        .find(|&f| {
            matches!(
                p.get_face(f).and_then(|x| p.get_surface(x.surface)),
                Some(geom::Surface::Cylinder { origin, .. }) if origin.x > W / 2.0 && origin.y < H / 2.0
            )
        })
        .expect("the south-east fillet");
    (boxed, wall, fillet)
}

/// The comb: two 2 × 3 blocks at x 0 to 2 and 8 to 10 bridged above
/// y = 1.5, and between them a tooth whose 90° tip, rounded r = 0.5,
/// touches y = 0 at x = 5 in the middle of its fillet. Unit thick.
fn comb() -> Body<f64> {
    let t = tol();
    let r = 0.5;
    // The tip corner sits r(√2 − 1) below y = 0, so the fillet's lowest
    // point (azimuth −90°, mid-arc) is on it; each side rises 1.5 − tip.
    let side = 1.5 - r * (1.0 - core::f64::consts::SQRT_2);
    let outline: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(2.0, 0.0), t)
        .expect("the west block's foot")
        .line_to(Point2::new(2.0, 1.5), t)
        .expect("its east side")
        .line_to(Point2::new(5.0 - side, 1.5), t)
        .expect("under the bridge")
        .toward(1.0, -1.0, t)
        .expect("down the tooth")
        .fillet(r, t)
        .expect("the tip fits")
        .toward(1.0, 1.0, t)
        .expect("up the tooth")
        .to(Point2::new(5.0 + side, 1.5), t)
        .expect("the tooth's east side")
        .line_to(Point2::new(8.0, 1.5), t)
        .expect("under the bridge")
        .line_to(Point2::new(8.0, 0.0), t)
        .expect("the east block's west side")
        .line_to(Point2::new(10.0, 0.0), t)
        .expect("its foot")
        .line_to(Point2::new(10.0, 3.0), t)
        .expect("its east side")
        .line_to(Point2::new(0.0, 3.0), t)
        .expect("the top")
        .line_to(Start, t)
        .expect("the west side")
        .into();
    plate(outline, 0.0)
}

/// **A covered touch no vertex splits refuses, typed, in both operand
/// orders and every op.** The deferral widens only what the other
/// operand's vertex puts under a touch; where nothing does, the
/// settled pair answers the frontier it was deferred with. The union
/// refuses first at the door: the declared ruling runs through the
/// box wall's interior, the unbuilt doubled-slit arm.
///
/// - The short box beside the fillet (z 0.25 to 0.75, and 0.25 to 1):
///   its wall edges graze the fillet mid-ruling, where the plate has
///   no vertex. The wall and the fillet are declared `Tangent`.
/// - The comb under a long box (x −1 to 11, y −1 to 0, z 0.25 to 2):
///   the box's wall rests on the blocks' feet (`Rest`, found) and is
///   declared `Tangent` to the tooth's fillet. The box's lower wall
///   edge is split where it crosses the blocks' corner edges, at
///   x = 0, 2, 8 and 10, and the touch at x = 5 is left in the middle
///   fragment, whichever way the edge runs: settling reads every
///   fragment, not the leading one.
#[test]
fn a_covered_touch_no_vertex_splits_refuses_in_both_orders() {
    let p = plate(rounded(R), 0.0);
    let mut poses = Vec::new();
    for (z0, z1) in [(0.25, 0.75), (0.25, 1.0)] {
        let (boxed, wall, fillet) = box_beside_the_fillet(&p, z0, z1);
        poses.push((
            format!("box z {z0} to {z1}"),
            boxed,
            p.clone(),
            wall,
            fillet,
        ));
    }
    let teeth = comb();
    let long = extruded(
        sketch_at(0.25),
        vec![ProfileLoop::polygon([
            Point2::new(-1.0, -1.0),
            Point2::new(11.0, -1.0),
            Point2::new(11.0, 0.0),
            Point2::new(-1.0, 0.0),
        ])],
        1.75,
        tol(),
    );
    let wall = long
        .faces()
        .map(|(k, _)| k)
        .find(|&f| {
            matches!(
                long.get_face(f).and_then(|x| long.get_surface(x.surface)),
                Some(geom::Surface::Plane { normal, .. }) if normal.y > 0.5
            )
        })
        .expect("the long box's north wall");
    let tip = teeth
        .faces()
        .map(|(k, _)| k)
        .find(|&f| is_cylinder(&teeth, f))
        .expect("the tooth's fillet");
    poses.push(("comb".to_string(), long, teeth, wall, tip));
    for (pose, a0, b0, fa, fb) in &poses {
        for (order, a, b, x, y) in [
            ("first is A", a0, b0, *fa, *fb),
            ("second is A", b0, a0, *fb, *fa),
        ] {
            let (rest, cont) = findings(a, b);
            let mut d = with(&rest, &cont);
            d.coincident_faces
                .push(FacePairDeclaration::new(x, y, topo::ContactClass::Tangent));
            let out = topo::union_with(a, b, &d, tol());
            assert!(
                matches!(out, Err(BooleanError::TangentSlitArmUnbuilt { .. })),
                "{pose}, {order}, A ∪ B: the ruling runs through the wall: {:?}",
                out.map(|_| ())
            );
            for (op, out) in [
                ("A ∖ B", topo::subtract_with(a, b, &d, tol())),
                ("A ∩ B", topo::intersect_with(a, b, &d, tol())),
            ] {
                assert!(
                    matches!(out, Err(BooleanError::CurvedPierceUnsupported { .. })),
                    "{pose}, {order}, {op}: {:?}",
                    out.map(|_| ())
                );
            }
        }
    }
}

/// **The pairs the settle stage accepts reach the accepted-pair
/// trace.** The sharp plate as A over the rounded one, every finding
/// declared: in the A → B direction each of A's bottom edges is
/// deferred against the two fillets it touches mid-span, so no fillet
/// face is accepted there by the sweep itself; the settle stage reads
/// the fragments B's vertices left and records each touch, and every
/// fillet face must then be in A → B's accepted channel.
#[test]
fn the_settle_stage_writes_the_accepted_trace() {
    let (a, b) = (plate(sharp(), 1.0), plate(rounded(R), 0.0));
    let (mate, walls) = findings(&a, &b);
    let (ab, _) = topo::sweep_traces_with_pad(
        &a,
        &b,
        &with(&mate, &walls),
        topo::SweepStrategy::Realized,
        None,
        None,
        tol(),
    )
    .expect("the traced sweep runs");
    let fillets: Vec<_> = b
        .faces()
        .map(|(k, _)| k)
        .filter(|&f| is_cylinder(&b, f))
        .collect();
    assert_eq!(fillets.len(), 4, "the rounded plate's four fillets");
    for f in fillets {
        assert!(
            ab.accepted.iter().any(|&(_, g)| g == f),
            "fillet {f:?} is accepted in A → B: {:?}",
            ab.accepted
        );
    }
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

/// One row of `overlapping_continuations_refuse_undeclared_and_build_declared`:
/// label, the two operands, and the expected volume and face count of
/// the union, the subtract and the intersect, in that order (`None`:
/// the result is empty).
type ContinuationRow = (
    &'static str,
    Body<f64>,
    Body<f64>,
    [Option<(f64, usize)>; 3],
);

/// The three ops, by name.
type Op = fn(
    &Body<f64>,
    &Body<f64>,
    &BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<f64>, BooleanError>;
const OPS: [(&str, Op); 3] = [
    ("union", topo::union_with::<f64>),
    ("subtract", topo::subtract_with::<f64>),
    ("intersect", topo::intersect_with::<f64>),
];

/// **An overlapping continuation refuses undeclared and builds
/// declared, in every op.** C4: a continuation is one carrier with
/// aligned senses, whether the two faces abut or overlap. Each pose
/// refuses `UndeclaredCoincidence` on an aligned pair undeclared and
/// with only its `Rest` findings, and builds exact with every finding
/// declared, in union, subtract and intersect alike. The volumes are box
/// arithmetic:
///
/// - overlapping equal-height plates (6 × 4 and 6 × 2 sharing 3 × 2):
///   the tops overlap, as do the bottoms;
/// - a sunk stack: a second plate half sunk into the first, its walls
///   overlapping the first plate's;
/// - a flush pocket: a 2 × 2 × 0.5 block inside the plate, its top
///   flush with the plate's;
/// - a rabbet filled: a block in a rabbet, its top flush with the
///   plate's and its east wall with the step's; their interiors are
///   disjoint, so the intersect is empty.
#[test]
fn overlapping_continuations_refuse_undeclared_and_build_declared() {
    let none = BooleanDeclarations::default();
    let p = brick((0.0, W), (0.0, H), (0.0, 1.0));
    let rows: [ContinuationRow; 4] = [
        (
            "overlapping plates",
            p.clone(),
            brick((3.0, 9.0), (1.0, 3.0), (0.0, 1.0)),
            [Some((30.0, 10)), Some((18.0, 10)), Some((6.0, 6))],
        ),
        (
            "sunk stack",
            p.clone(),
            brick((0.0, W), (0.0, H), (0.5, 1.5)),
            [Some((36.0, 6)), Some((12.0, 6)), Some((12.0, 6))],
        ),
        (
            "flush pocket",
            p.clone(),
            brick((2.0, 4.0), (1.0, 3.0), (0.5, 1.0)),
            [Some((24.0, 6)), Some((22.0, 11)), Some((2.0, 6))],
        ),
        (
            "rabbet filled",
            rabbeted(),
            brick((W - 1.0, W), (0.0, H), (0.5, 1.0)),
            [Some((24.0, 6)), Some((22.0, 8)), None],
        ),
    ];
    for (label, a, b, expect) in rows {
        let (rest, cont) = findings(&a, &b);
        assert!(
            !cont.coincident_faces.is_empty(),
            "{label}: the detector mints the overlapping aligned pairs as continuations"
        );
        for ((op_name, op), expect) in OPS.into_iter().zip(expect) {
            let label = format!("{label}, {op_name}");
            for (posture, d) in [("undeclared", &none), ("Rest only", &rest)] {
                let err = op(&a, &b, d, tol()).expect_err("an aligned pair is undeclared");
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
            let out = op(&a, &b, &with(&rest, &cont), tol());
            match expect {
                Some((volume, faces)) => {
                    builds(&label, out, volume, faces);
                }
                None => assert!(
                    matches!(out, Ok(BooleanResult::Empty)),
                    "{label}: the interiors are disjoint: {out:?}"
                ),
            }
        }
    }
}

/// **A declared continuation across a rabbet's step builds every op.**
/// The rabbeted plate and a block on its east edge that fills the
/// rabbet and overlaps the plate beyond it (the step below, or the top
/// beside it), every finding declared. The union is the 6 × 4 × 1 box,
/// six faces, a legal operand: it used to refuse
/// `Join(UnpairedLooseEnds { count: 6 })`
/// (`work/zip/a-declared-continuation-across-a-rabbet-step-leaves-six-loose-ends.md`),
/// and JOIN-1's locus matching pairs those ends (the section segments
/// along the step are edges of both solids).
#[test]
fn a_declared_continuation_across_a_rabbet_step_builds_every_op() {
    for (label, block, subtract, intersect) in [
        (
            "over the step",
            brick((W - 1.0, W), (0.0, H), (0.0, 1.0)),
            (20.0, 6),
            (2.0, 6),
        ),
        (
            "over the top",
            brick((W - 2.0, W), (0.0, H), (0.5, 1.0)),
            (20.0, 8),
            (2.0, 6),
        ),
    ] {
        let a = rabbeted();
        let (rest, cont) = findings(&a, &block);
        assert_eq!(rest.coincident_faces.len(), 1, "{label}: one Rest pair");
        let d = with(&rest, &cont);
        let union = builds(
            &format!("{label}, union"),
            topo::union_with(&a, &block, &d, tol()),
            W * H,
            6,
        );
        sweep::test_support::assert_legal_operand(label, &union.body, tol());
        builds(
            &format!("{label}, subtract"),
            topo::subtract_with(&a, &block, &d, tol()),
            subtract.0,
            subtract.1,
        );
        builds(
            &format!("{label}, intersect"),
            topo::intersect_with(&a, &block, &d, tol()),
            intersect.0,
            intersect.1,
        );
    }
}

/// **A declared rounded continuation that lies inside the other's wall
/// builds its subtract and intersect, and refuses A ∪ B typed.**
/// The rounded plate and a plate of the same outline half as thick,
/// sunk inside it or flush with its top or bottom, so the thin plate's
/// walls (fillets included) lie inside the thick one's. Undeclared,
/// each op refuses `UndeclaredCoincidence`. With every finding
/// declared, subtract and intersect build at box arithmetic in z over
/// the outline's area `24 − (4 − π)·R²`: half the thick plate's volume
/// each, the sunk subtract as two plates of a quarter unit (twenty
/// faces). A ∪ B refuses `FallbackExtentUnsupported`: no crossing
/// event exists, and that pass exempts no declared pair
/// (`work/reach/rounded-stack-subtract-and-intersect-refuse-fallback-extent.md`).
/// With the declarations keyed for (B, A), B ∖ A, empty (the thin
/// plate lies inside the thick one), refuses as A ∪ B does, while B ∪ A
/// builds the thick plate, its walls left split where the thin plate's
/// lay (18 faces sunk, 14 flush, against the plate's 10): the union
/// refuses in one operand order only.
///
/// Two results here are an operand itself measured through another
/// face order, so their `f64` volumes round a few ulps past the operand
/// they are bounded by: the flush-top intersect (the thin plate) and
/// the sunk B ∪ A (the thick plate). The backstop re-derives each tie
/// in interval arithmetic and builds it.
#[test]
fn declared_rounded_continuations_inside_a_wall_build_subtract_and_intersect() {
    let none = BooleanDeclarations::default();
    let a = plate(rounded(R), 0.0);
    let half = area(4.0) / 2.0;
    for (label, z0, subtract_faces, union_faces) in [
        ("sunk inside", 0.25, 20, 18),
        ("flush top", 0.5, 10, 14),
        ("flush bottom", 0.0, 10, 14),
    ] {
        let b = extruded(sketch_at(z0), vec![rounded(R)], 0.5, tol());
        let (rest, cont) = findings(&a, &b);
        assert!(rest.coincident_faces.is_empty(), "{label}: no Rest pair");
        let d = with(&rest, &cont);
        for (op_name, op) in OPS {
            let undeclared = op(&a, &b, &none, tol()).expect_err("undeclared refuses");
            assert!(
                matches!(
                    undeclared,
                    BooleanError::UndeclaredCoincidence {
                        relation: PlaneRelation::SameOriented,
                        ..
                    }
                ),
                "{label}, {op_name}, undeclared: {undeclared:?}"
            );
        }
        let err = topo::union_with(&a, &b, &d, tol()).expect_err("the union refuses");
        assert!(
            matches!(err, BooleanError::FallbackExtentUnsupported { .. }),
            "{label}, union: {err:?}"
        );
        assert!(
            !err.to_string().contains("kernel"),
            "{label}, union: a legal input is no kernel bug: {err}"
        );
        builds(
            &format!("{label}, subtract"),
            topo::subtract_with(&a, &b, &d, tol()),
            half,
            subtract_faces,
        );
        builds(
            &format!("{label}, intersect"),
            topo::intersect_with(&a, &b, &d, tol()),
            half,
            10,
        );
        let (rest_ba, cont_ba) = findings(&b, &a);
        let d_ba = with(&rest_ba, &cont_ba);
        let err = topo::subtract_with(&b, &a, &d_ba, tol()).expect_err("B ∖ A refuses");
        assert!(
            matches!(err, BooleanError::FallbackExtentUnsupported { .. }),
            "{label}, B ∖ A: {err:?}"
        );
        builds(
            &format!("{label}, B ∪ A"),
            topo::union_with(&b, &a, &d_ba, tol()),
            area(4.0),
            union_faces,
        );
    }
}
