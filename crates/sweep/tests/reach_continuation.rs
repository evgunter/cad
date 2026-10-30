//! Two stacked plates on one footprint: the mating plane is a `Rest`
//! contact, and each pair of outer walls — one carrier, the same
//! outward sense, meeting only along the mating plane — is a
//! CONTINUATION (`crates/topo/README.md`, C4's continuation clause and
//! the crossing layer's one-sided cover).
//!
//! The rows: a continuation merges, planar and curved alike; the rounded
//! stack's tangent wall edges are covered through a structural tangency
//! on EITHER operand, and a tangency in the middle of an edge builds in
//! either operand order; and every output is a legal boolean operand.
//! The rows after them: a kiss or a gap is no continuation; an
//! overlapping continuation builds in every op; and the overlaps that
//! still refuse are pinned at their typed refusals, each filed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use geom_core::{RigidMotion, Turns, UnitVec3};
use profile::SketchPlane;
use profile::{Open, ProfileLoop, RawLoop, Start};
use sweep::test_support::{extruded, finished, sketch_at};
use topo::{AtRestBody, Body, BooleanBody, BooleanError, BooleanOp, BooleanResult};

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
fn plate(outline: ProfileLoop<f64>, z0: f64) -> AtRestBody<f64> {
    finished(
        "the plate",
        extruded(sketch_at(z0), vec![outline], 1.0, tol()),
        tol(),
    )
}

/// The 6×4 area less what `rounded_corners` fillets of radius `R` take.
fn area(rounded_corners: f64) -> f64 {
    W * H - rounded_corners * (1.0 - core::f64::consts::FRAC_PI_4) * R * R
}

fn is_cylinder(body: &Body<f64>, f: topo::FaceKey) -> bool {
    matches!(
        body.get_face(f).and_then(|x| body.get_surface(x.surface)),
        Some(geom::Surface::Cylinder { .. })
    )
}

fn volume(body: &Body<f64>) -> f64 {
    topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished("the measured body", topo::Body::clone(body), tol()),
        tol(),
    )
    .volume
}

/// The union, which must build, be additive and valid at tier 3 and 3′.
fn union_honest(label: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> BooleanBody<f64> {
    let out = topo::union(a, b, tol()).unwrap_or_else(|e| panic!("{label}: {e:?}"));
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

/// **A continuation merges, a planar pair and a curved one alike**, and the stack is exact.
///
/// Sharp: six faces, nothing skipped. Rounded: the four flat wall pairs
/// and the four fillet pairs glue — ten faces, nothing skipped, the
/// merge reading each fillet pair on the carrier ladder.
#[test]
fn a_continuation_merges_planar_and_curved_walls() {
    let (p, q) = (plate(sharp(), 0.0), plate(sharp(), 1.0));
    let bb = union_honest("sharp", &p, &q);
    assert_eq!(bb.body.faces().count(), 6, "sharp: one box");
    assert!(
        bb.naming.merge_skipped.is_empty(),
        "{:?}",
        bb.naming.merge_skipped
    );
    assert_eq!(volume(&bb.body), 2.0 * W * H, "sharp: two plates' worth");

    let (p, q) = (plate(rounded(R), 0.0), plate(rounded(R), 1.0));
    let bb = union_honest("rounded", &p, &q);
    let cylinders = bb
        .body
        .faces()
        .filter(|&(k, _)| is_cylinder(&bb.body, k))
        .count();
    assert_eq!(
        (bb.body.faces().count(), cylinders),
        (10, 4),
        "rounded: top, bottom, four merged walls, and four merged fillets"
    );
    assert!(
        bb.naming.merge_skipped.is_empty(),
        "{:?}",
        bb.naming.merge_skipped
    );
    let v = volume(&bb.body);
    let closed = 2.0 * area(4.0);
    assert!((v - closed).abs() <= 1e-12, "rounded: {v} vs {closed}");
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
/// fillet), reached through the verified continuation of Q's wall into
/// P's. It builds in both operand orders.
#[test]
fn the_tangent_edge_cover_reads_a_strut_on_either_operand() {
    let (p, q) = (plate(rounded(R), 0.0), plate(rounded(R), 1.0));
    union_honest("rounded on rounded", &p, &q);

    let (p, q) = (plate(rounded(R), 0.0), plate(rounded_west(R), 1.0));
    for (label, a, b) in [("P is A", &p, &q), ("Q is A", &q, &p)] {
        let bb = union_honest(label, a, b);
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
/// carries the fillet's arc on both sides. The join pairs the fillet's
/// germs: the straight bottom edge passing the tangent point reads
/// along the germ to first order, but its far end touches nothing, so
/// only the arc carries the segment, which lies in the plate's bottom
/// face: the join reads the germ in that face, across the edge its
/// sector is bounded by, and builds the union.
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
            let join = topo::test_support::boolean_join_refusal(BooleanOp::Union, a, b, tol());
            assert!(
                join.as_ref().is_ok_and(Option::is_none),
                "{label}: A ∪ B, the join reads each germ tangent to the straight edge in the \
                 face the fillet's rim turns into, and connects: {join:?}"
            );
            builds(
                &format!("{label}: A ∪ B"),
                topo::union(a, b, tol()),
                va + vb,
                union_faces,
            );
            builds(
                &format!("{label}: A ∖ B"),
                topo::subtract(a, b, tol()),
                va,
                fa,
            );
            builds(
                &format!("{label}: B ∖ A"),
                topo::subtract(b, a, tol()),
                vb,
                fb,
            );
            let meet = topo::intersect(a, b, tol());
            assert!(
                matches!(meet, Ok(BooleanResult::Empty)),
                "{label}: A ∩ B, the interiors are disjoint: {meet:?}"
            );
        }
    }
}

/// **A `Tangent` touching a fillet mid-edge builds every op in either
/// operand order.** A unit box stands beside the rounded plate, turned
/// 45° so its west wall is tangent to the south-east fillet along the
/// ruling at azimuth −45°: the box's wall edges pass that ruling
/// mid-span and the plate has no edge on it. The boolean verifies the
/// tangency by its witness, and the verified pair is the cover (C4).
/// The union is two solids touching face–face along the ruling through
/// the box wall's and the fillet's interiors, one record. Volumes are
/// closed form (the box 1, the plate [`area`] of four corners, interiors
/// disjoint); faces: the box 6, the plate 10.
#[test]
fn a_tangent_beside_a_fillet_builds_every_op_in_either_order() {
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
    for (order, a, b, (va, na), (vb, nb)) in [
        ("box is A", &boxed, &p, (1.0, 6), (area(4.0), 10)),
        ("plate is A", &p, &boxed, (area(4.0), 10), (1.0, 6)),
    ] {
        match topo::union(a, b, tol()) {
            Ok(topo::BooleanResult::Body(r)) => {
                assert_eq!(r.body.solids().count(), 2, "{order}: A ∪ B, two solids");
                let v = volume(&r.body);
                assert!((v - va - vb).abs() < 1e-9, "{order}: A ∪ B, volume {v}");
                let kinds: Vec<_> = r
                    .contacts
                    .face_face()
                    .map(|(x, y)| {
                        [x, y].map(|f| {
                            let face = r.body.get_face(f).unwrap();
                            r.body.get_surface(face.surface).unwrap().kind()
                        })
                    })
                    .collect();
                assert!(
                    kinds == [[geom::SurfaceKind::Plane, geom::SurfaceKind::Cylinder]]
                        || kinds == [[geom::SurfaceKind::Cylinder, geom::SurfaceKind::Plane]],
                    "{order}: A ∪ B, the box wall and the fillet's face–face record: {kinds:?}"
                );
            }
            out => panic!("{order}: A ∪ B: {:?}", out.map(|_| ())),
        }
        builds(
            &format!("{order}: A ∖ B"),
            topo::subtract(a, b, tol()),
            va,
            na,
        );
        builds(
            &format!("{order}: B ∖ A"),
            topo::subtract(b, a, tol()),
            vb,
            nb,
        );
        let meet = topo::intersect(a, b, tol());
        assert!(
            matches!(meet, Ok(BooleanResult::Empty)),
            "{order}: A ∩ B, the interiors are disjoint: {meet:?}"
        );
    }
}

/// The 45° unit box of
/// [`a_tangent_beside_a_fillet_builds_every_op_in_either_order`],
/// its west wall on the south-east fillet's ruling at azimuth −45°,
/// standing from `z0` to `z1`.
fn box_beside_the_fillet(z0: f64, z1: f64) -> AtRestBody<f64> {
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
    finished("the box", boxed, tol())
}

/// The comb: two 2 × 3 blocks at x 0 to 2 and 8 to 10 bridged above
/// y = 1.5, and between them a tooth whose 90° tip, rounded r = 0.5,
/// touches y = 0 at x = 5 in the middle of its fillet. Unit thick.
fn comb() -> AtRestBody<f64> {
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
/// settled pair answers the frontier it was deferred with.
///
/// - The short box beside the fillet (z 0.25 to 0.75, and 0.25 to 1):
///   its wall edges graze the fillet mid-ruling, where the plate has
///   no vertex. The wall is tangent to the fillet.
/// - The comb under a long box (x −1 to 11, y −1 to 0, z 0.25 to 2):
///   the box's wall rests on the blocks' feet (`Rest`) and is tangent
///   to the tooth's fillet. The box's lower wall
///   edge is split where it crosses the blocks' corner edges, at
///   x = 0, 2, 8 and 10, and the touch at x = 5 is left in the middle
///   fragment, whichever way the edge runs: settling reads every
///   fragment, not the leading one.
#[test]
fn a_covered_touch_no_vertex_splits_refuses_in_both_orders() {
    let p = plate(rounded(R), 0.0);
    let mut poses = Vec::new();
    for (z0, z1) in [(0.25, 0.75), (0.25, 1.0)] {
        let boxed = box_beside_the_fillet(z0, z1);
        poses.push((format!("box z {z0} to {z1}"), boxed, p.clone()));
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
    let long = finished("the long box", long, tol());
    poses.push(("comb".to_string(), long, teeth));
    for (pose, a0, b0) in &poses {
        for (order, a, b) in [("first is A", a0, b0), ("second is A", b0, a0)] {
            for (op, out) in [
                ("A ∪ B", topo::union(a, b, tol())),
                ("A ∖ B", topo::subtract(a, b, tol())),
                ("A ∩ B", topo::intersect(a, b, tol())),
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
/// trace.** The sharp plate as A over the rounded one: in the A → B
/// direction each of A's bottom edges is
/// deferred against the two fillets it touches mid-span, so no fillet
/// face is accepted there by the sweep itself; the settle stage reads
/// the fragments B's vertices left and records each touch, and every
/// fillet face must then be in A → B's accepted channel.
#[test]
fn the_settle_stage_writes_the_accepted_trace() {
    let (a, b) = (plate(sharp(), 1.0), plate(rounded(R), 0.0));
    let (ab, _) =
        topo::sweep_traces_with_pad(&a, &b, topo::SweepStrategy::Realized, None, None, tol())
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
/// third plate stacked on its top — so no unmerged planar continuation ships,
/// and the recorded curved skip is the form the next boolean accepts.
#[test]
fn the_stack_is_a_legal_operand() {
    for (label, outline) in [
        ("sharp", sharp as fn() -> ProfileLoop<f64>),
        ("rounded", || rounded(R)),
    ] {
        let (p, q) = (plate(outline(), 0.0), plate(outline(), 1.0));
        let stack = union_honest(label, &p, &q).body;
        let clear = plate(outline(), 5.0);
        let BooleanResult::Body(_) = topo::union(&stack, &clear, tol())
            .unwrap_or_else(|e| panic!("{label}: the stack is an operand: {e:?}"))
        else {
            panic!("{label}: not empty");
        };
        let third = plate(outline(), 2.0);
        union_honest(&format!("{label}, third plate"), &stack, &third);
    }
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    finished(
        "the brick",
        sweep::test_support::brick(x, y, z, tol()),
        tol(),
    )
}

/// The rabbeted plate: 6 × 4 × 1 with a 1 × 0.5 step cut along its
/// east edge, swept along y from its xz section (volume 22).
fn rabbeted() -> AtRestBody<f64> {
    let section = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(W, 0.0),
        Point2::new(W, 0.5),
        Point2::new(W - 1.0, 0.5),
        Point2::new(W - 1.0, 1.0),
        Point2::new(0.0, 1.0),
    ]);
    let xz = SketchPlane::from_frame(RigidMotion::turned(
        geom_core::Point3::new(0.0, H, 0.0),
        UnitVec3::exact_x(),
        Turns::quarters(1),
    ));
    finished(
        "the rabbeted plate",
        extruded(xz, vec![section], H, tol()),
        tol(),
    )
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
/// it, union:
///
/// - corner kiss: the plates share one vertical edge line (x = 6,
///   y = 4) and nothing else;
/// - vertex kiss: lifted a plate's height as well, they share one
///   point;
/// - a real gap: the second plate starts a unit east, so the coplanar
///   tops and bottoms (one carrier, aligned) never meet.
///
/// A scan that read box overlap, or a shared point, as "meets" refuses
/// the first two as continuations and a box pad wider than the gap
/// refuses the third.
#[test]
fn a_kiss_or_a_gap_is_no_continuation() {
    let p = brick((0.0, W), (0.0, H), (0.0, 1.0));
    for (label, q) in [
        ("corner kiss", brick((W, 2.0 * W), (H, 2.0 * H), (0.0, 1.0))),
        ("vertex kiss", brick((W, 2.0 * W), (H, 2.0 * H), (1.0, 2.0))),
        ("gap", brick((W + 1.0, 2.0 * W), (0.0, H), (0.0, 1.0))),
    ] {
        builds(
            label,
            topo::union(&p, &q, tol()),
            volume(&p) + volume(&q),
            12,
        );
    }
}

/// One row of `overlapping_continuations_build_in_every_op`:
/// label, the two operands, and the expected volume and face count of
/// the union, the subtract and the intersect, in that order (`None`:
/// the result is empty).
type ContinuationRow = (
    &'static str,
    AtRestBody<f64>,
    AtRestBody<f64>,
    [Option<(f64, usize)>; 3],
);

/// The three ops, by name.
type Op = fn(&AtRestBody<f64>, &AtRestBody<f64>, Tol) -> Result<BooleanResult<f64>, BooleanError>;
const OPS: [(&str, Op); 3] = [
    ("union", topo::union::<f64>),
    ("subtract", topo::subtract::<f64>),
    ("intersect", topo::intersect::<f64>),
];

/// **An overlapping continuation builds in every op.** C4: a
/// continuation is one carrier with aligned senses, whether the two
/// faces abut or overlap. Each pose builds exact, in union, subtract
/// and intersect alike. The volumes are box arithmetic:
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
fn overlapping_continuations_build_in_every_op() {
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
        for ((op_name, op), expect) in OPS.into_iter().zip(expect) {
            let label = format!("{label}, {op_name}");
            let out = op(&a, &b, tol());
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

/// **A continuation across a rabbet's step builds every op.**
/// The rabbeted plate and a block on its east edge that fills the
/// rabbet and overlaps the plate beyond it (the step below, or the top
/// beside it). The union is the 6 × 4 × 1 box,
/// six faces, a legal operand: it used to refuse
/// `Join(UnpairedLooseEnds { count: 6 })`
/// (`work/zip/a-declared-continuation-across-a-rabbet-step-leaves-six-loose-ends.md`),
/// and JOIN-1's locus matching pairs those ends (the section segments
/// along the step are edges of both solids).
#[test]
fn a_continuation_across_a_rabbet_step_builds_every_op() {
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
        let union = builds(
            &format!("{label}, union"),
            topo::union(&a, &block, tol()),
            W * H,
            6,
        );
        sweep::test_support::assert_legal_operand(label, &union.body, tol());
        builds(
            &format!("{label}, subtract"),
            topo::subtract(&a, &block, tol()),
            subtract.0,
            subtract.1,
        );
        builds(
            &format!("{label}, intersect"),
            topo::intersect(&a, &block, tol()),
            intersect.0,
            intersect.1,
        );
    }
}

/// **Three bricks folded in any order build the L.** `a` = (0,1)³, `c`
/// = x∈(0.5,1.5), y∈(−1,0), `b` = x∈(0.5,1.5), y∈(0,1), all z∈(0,1),
/// united pairwise. In
/// `(a ∪ c) ∪ b` the block `b` overlaps the stepped `a ∪ c`'s `a` arm
/// (x∈(0.5,1)) and rests on its `c` arm, at the rabbet's reflex step;
/// it used to refuse
/// `Join(UnpairedLooseEnds { count: 6 })`
/// (`work/zip/a-declared-continuation-across-a-rabbet-step-leaves-six-loose-ends.md`).
/// Each step builds at box arithmetic, tiers 3 and 3′; the L has eight
/// faces and is a legal operand.
#[test]
fn three_bricks_with_a_reflex_step_fold_in_every_order() {
    let part = |n: char| match n {
        'a' => brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
        'b' => brick((0.5, 1.5), (0.0, 1.0), (0.0, 1.0)),
        _ => brick((0.5, 1.5), (-1.0, 0.0), (0.0, 1.0)),
    };
    for (order, first) in [
        ("acb", (2.0, 10)),
        ("cab", (2.0, 10)),
        ("abc", (1.5, 6)),
        ("bac", (1.5, 6)),
        ("bca", (2.0, 6)),
        ("cba", (2.0, 6)),
    ] {
        let n: Vec<char> = order.chars().collect();
        let (p, q, r) = (part(n[0]), part(n[1]), part(n[2]));
        let step = builds(
            &format!("{order}: step 1"),
            topo::union(&p, &q, tol()),
            first.0,
            first.1,
        );
        let l = builds(
            &format!("{order}: step 2"),
            topo::union(&step.body, &r, tol()),
            2.5,
            8,
        );
        sweep::test_support::assert_legal_operand(order, &l.body, tol());
    }
}

/// **A rounded continuation that lies inside the other's wall
/// builds every op.** The rounded plate and a plate of the same outline
/// half as thick, sunk inside it or flush with its top or bottom, so the
/// thin plate's walls (fillets included) lie inside the thick one's.
/// Subtract and intersect build at box arithmetic in z over the
/// outline's area `24 − (4 − π)·R²`: half the thick plate's volume each,
/// the sunk subtract as two plates of a quarter unit (twenty faces).
/// A ∪ B is the thick plate (its 10 faces) and B ∖ A is nothing: no
/// crossing event exists, the section pass defers the settled wall
/// pairs, and no edge of the thick plate runs inside the thin plate's
/// walls, so the thin plate's `In` stands. B ∪ A builds the thick plate
/// too, its walls glued back whole where the thin plate's lay.
///
/// Two results here are an operand itself measured through another
/// face order, so their `f64` volumes round a few ulps past the operand
/// they are bounded by: the flush-top intersect (the thin plate) and
/// the sunk B ∪ A (the thick plate). The backstop re-derives each tie
/// in interval arithmetic and builds it.
#[test]
fn rounded_continuations_inside_a_wall_build_subtract_and_intersect() {
    let a = plate(rounded(R), 0.0);
    let half = area(4.0) / 2.0;
    for (label, z0, subtract_faces) in [
        ("sunk inside", 0.25, 20),
        ("flush top", 0.5, 10),
        ("flush bottom", 0.0, 10),
    ] {
        let b = finished(
            "the thin plate",
            extruded(sketch_at(z0), vec![rounded(R)], 0.5, tol()),
            tol(),
        );
        builds(
            &format!("{label}, A ∪ B"),
            topo::union(&a, &b, tol()),
            area(4.0),
            10,
        );
        builds(
            &format!("{label}, subtract"),
            topo::subtract(&a, &b, tol()),
            half,
            subtract_faces,
        );
        builds(
            &format!("{label}, intersect"),
            topo::intersect(&a, &b, tol()),
            half,
            10,
        );
        let out = topo::subtract(&b, &a, tol());
        assert!(
            matches!(out, Ok(BooleanResult::Empty)),
            "{label}, B ∖ A: nothing: {out:?}"
        );
        builds(
            &format!("{label}, B ∪ A"),
            topo::union(&b, &a, tol()),
            area(4.0),
            10,
        );
    }
}

/// **A sharp plate offset over a rounded one unions, in either operand
/// order.** The 6 × 4 sharp plate on the rounded one (`r = 0.5`), moved
/// rigidly in its own plane by `(dx, dy)`: its bottom edges cross the
/// fillets transversally (`dy > 0`), miss them (`dy < 0`), or overhang
/// two of them (`dx ≠ 0`); only the mating plane's `Rest` survives the
/// move. The union builds, additive,
/// at tiers 3 and 3′. A pose `1e-7` off may instead escalate where the
/// band is fine enough to see its crossing in band (`ε = 1e-12`), and
/// nothing else.
#[test]
fn a_sharp_plate_offset_over_a_rounded_one_unions_in_either_order() {
    let lower = plate(rounded(R), 0.0);
    for (dx, dy) in [
        (0.0, 1e-7),
        (0.0, -1e-7),
        (0.0, 1e-3),
        (0.0, -1e-3),
        (0.25, 0.0),
        (-0.25, 0.0),
    ] {
        let shift = geom_core::Affine3::translation(geom_core::Vec3::new(dx, dy, 0.0));
        let moved = topo::transform_rigid(
            &plate(sharp(), 1.0),
            &geom_core::test_support::rigid(&shift, tol()),
            tol(),
        )
        .unwrap();
        let upper = finished("the offset plate", moved, tol());
        for (order, a, b) in [
            ("upper ∪ lower", &upper, &lower),
            ("lower ∪ upper", &lower, &upper),
        ] {
            let label = format!("({dx}, {dy}), {order}");
            if dy.abs() == 1e-7
                && let Err(e) = topo::union(a, b, tol())
            {
                assert!(
                    matches!(e, BooleanError::Escalated { .. }),
                    "{label}: a near pose refuses only in band: {e:?}"
                );
                continue;
            }
            union_honest(&label, a, b);
        }
    }
}

/// Review probe (PR 6070): the rounded two-plate stack of
/// `work/reachhold/rounded-stack-subtract-and-intersect-refuse-fallback-extent.md`,
/// z 0..1 and 1..2, subtract and intersect in both orders. Prints what
/// each returns; fails only on a wrong body.
#[test]
fn review_6070_the_rounded_stack_subtracts_and_intersects() {
    let (p, q) = (plate(rounded(R), 0.0), plate(rounded(R), 1.0));
    for (order, a, b) in [("P, Q", &p, &q), ("Q, P", &q, &p)] {
        let s = topo::subtract(a, b, tol());
        let i = topo::intersect(a, b, tol());
        eprintln!(
            "RV6070 stack {order}: ∖ {:?}",
            s.as_ref().map(|r| r.body().map(|b| b.kind))
        );
        eprintln!(
            "RV6070 stack {order}: ∩ {:?}",
            i.as_ref().map(|r| r.body().map(|b| b.kind))
        );
        if let Ok(BooleanResult::Body(bb)) = s {
            let v = topo::test_support::pinned_volume("d", &bb.body, tol()).volume;
            assert!(
                (v - area(4.0)).abs() <= 1e-9 * area(4.0),
                "{order}: ∖ volume {v}"
            );
        }
        assert!(
            !matches!(i, Ok(BooleanResult::Body(_))),
            "{order}: ∩ is nothing"
        );
    }
}
