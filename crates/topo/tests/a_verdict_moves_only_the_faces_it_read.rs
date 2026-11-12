//! **A one-carrier verdict decided from values moves only the faces it
//! was read over, and a face's measure is read at the face**
//! (`docs/DESIGN.md` `[coincidence.door]`, `[tolerance.epsilon]`; this
//! crate's README `[topo.one-record]`, and geom-brep's
//! `[props.face-anchored]`).
//!
//! - A face moved onto a record anchored far from it reads its volume
//!   at the face, so a union whose glue moves a face onto such a record
//!   measures within the band times the face's area of its operands'
//!   sum, in both operand orders.
//! - A record also worn by faces the verdict did not decide is the one
//!   kept, so those faces keep their record and the verdict is read
//!   only over the faces it moves: a plate's far fragment and a
//!   bridge's far foot stay on their record and the union builds.
//! - Each glued pair moves its own faces: a face two pairs decide is
//!   moved by the first, and two faces no pair decided one carrier are
//!   never left on one record.
//!
//! Lengths and leans are spelled in ε, so each row is one pose at every
//! ε row.

#![allow(clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol};
use topo::test_support::{brick, finished, mapped_cube, pinned_volume};
use topo::{AtRestBody, BooleanResult};

fn tol() -> Tol {
    Tol::witness()
}

fn vol(what: &str, b: &AtRestBody<f64>) -> f64 {
    pinned_volume(what, b, tol()).volume
}

/// `x ∪ y`, held to tier 3, its body.
fn united(what: &str, x: &AtRestBody<f64>, y: &AtRestBody<f64>) -> AtRestBody<f64> {
    match topo::union(x, y, tol()) {
        Ok(BooleanResult::Body(bb)) => {
            topo::validate_geometric(&bb.body, tol())
                .unwrap_or_else(|e| panic!("{what}: tier 3: {e:?}"));
            bb.body
        }
        other => panic!("{what}: the union builds, got {:?}", other.map(|_| ())),
    }
}

/// `x ∖ y`'s body.
fn cut(what: &str, x: &AtRestBody<f64>, y: &AtRestBody<f64>) -> AtRestBody<f64> {
    match topo::subtract(x, y, tol()) {
        Ok(BooleanResult::Body(bb)) => finished(what, (*bb.body).clone(), tol()),
        other => panic!("{what}: the cut builds, got {:?}", other.map(|_| ())),
    }
}

fn block(what: &str, x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    finished(what, brick::<f64>(x, y, z, tol()), tol())
}

/// The points of `face`'s loops.
fn face_points(body: &topo::Body<f64>, face: topo::FaceKey) -> Vec<Point3<f64>> {
    body.face(face)
        .expect("a live face")
        .loops()
        .filter_map(|(_, l, _)| match l.boundary {
            topo::LoopBoundary::Cycle { first } => body.loop_cycle(first),
            topo::LoopBoundary::Empty { .. } => None,
        })
        .flatten()
        .filter_map(|he| body.half_edge_start_point(he))
        .collect()
}

/// The planar face of `body` facing `+z` (`up`) or `−z` whose boundary
/// lies wholly past `x = x0`, and its record's plane: origin and
/// normal, coordinate by coordinate.
fn plane_beyond(body: &topo::Body<f64>, up: bool, x0: f64) -> [f64; 6] {
    for (f, face) in body.faces() {
        let Some(geom::Surface::Plane { origin, normal, .. }) = body.face_surface(f) else {
            continue;
        };
        let outward = if face.sense { normal.z } else { -normal.z };
        if (outward > 0.9) != up || outward.abs() < 0.9 {
            continue;
        }
        if face_points(body, f).iter().all(|p| p.x > x0) {
            return [origin.x, origin.y, origin.z, normal.x, normal.y, normal.z];
        }
    }
    panic!(
        "a face facing {} past x = {x0}",
        if up { "up" } else { "down" }
    )
}

/// **A unit cube whose top record is anchored far off**: the remnant
/// `[0, 1]³` of a block `[1 − L, 1] × [0, 1]²` whose top leans `lean`
/// about the line `x = 1`, its record anchored at the long block's
/// middle, `L/2` away.
fn far_anchored_cube(lean: f64, l: f64) -> AtRestBody<f64> {
    let long = finished(
        "long",
        mapped_cube::<f64>(
            move |x, y, z| {
                let gx = 1.0 - l + l * x;
                Point3::new(gx, y, z * (1.0 + lean * (gx - 1.0)))
            },
            tol(),
        ),
        tol(),
    );
    let away = block("away", (-l - 1.0, 0.0), (-1.0, 2.0), (-1.0, 2.0));
    cut("the far-anchored cube", &long, &away)
}

/// **R1's corner kiss**: the far-anchored cube, its top leaning
/// `0.1 ε` over `L = 900` (inside the session box), and `B = [1, 2]³` kissing it at its corner
/// edge. The glue moves one top onto the other's record; the volume is
/// the operands' sum to within ε over the unit face, in both orders.
#[test]
fn a_far_anchored_lean_kiss_measures_its_sum() {
    let eps = tol().eps();
    let a = far_anchored_cube(0.1 * eps, 900.0);
    let b = block("B", (1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let sum = vol("A", &a) + vol("B", &b);
    for (order, x, y) in [("A ∪ B", &a, &b), ("B ∪ A", &b, &a)] {
        let v = vol(order, &united(order, x, y));
        assert!(
            (v - sum).abs() <= eps,
            "{order}: {v} against the operands' {sum}, off by {:e} past ε",
            v - sum
        );
    }
}

/// **R2's corner kiss**: the far-anchored cube (`L = 900`, no lean) and
/// a unit cube cornered at `(1, 1, 1)` turned `θ` about `y` there, `θ`
/// at `0.1 ε`, `0.3 ε` and `0.6 ε`: within ε of `2` in both orders.
#[test]
fn a_turned_cube_kissing_a_far_anchored_corner_measures_its_sum() {
    let eps = tol().eps();
    let a = far_anchored_cube(0.0, 900.0);
    for k in [0.1, 0.3, 0.6] {
        let th = k * eps;
        let (s, c) = (th.sin(), th.cos());
        let b = finished(
            "B",
            mapped_cube::<f64>(
                move |x, y, z| Point3::new(1.0 + x * c + z * s, 1.0 + y, 1.0 - x * s + z * c),
                tol(),
            ),
            tol(),
        );
        let sum = vol("A", &a) + vol("B", &b);
        for (order, x, y) in [("A ∪ B", &a, &b), ("B ∪ A", &b, &a)] {
            let what = format!("{order} at θ = {k} ε");
            let v = vol(&what, &united(&what, x, y));
            assert!(
                (v - sum).abs() <= eps,
                "{what}: {v} against the operands' {sum}, off by {:e} past ε",
                v - sum
            );
        }
    }
}

/// **Two flush blocks**: the far-anchored cube, leaning `0.1 ε` over
/// `L = 900`, and `[1, 2] × [0, 1]²` beside it, their tops one carrier,
/// merged into one face. The volume is the operands' sum to within ε
/// over the two tops, in both orders.
#[test]
fn two_flush_blocks_beside_a_far_anchor_measure_their_sum() {
    let eps = tol().eps();
    let a = far_anchored_cube(0.1 * eps, 900.0);
    let b = block("B", (1.0, 2.0), (0.0, 1.0), (0.0, 1.0));
    let sum = vol("A", &a) + vol("B", &b);
    for (order, x, y) in [("A ∪ B", &a, &b), ("B ∪ A", &b, &a)] {
        let v = vol(order, &united(order, x, y));
        assert!(
            (v - sum).abs() <= 2.0 * eps,
            "{order}: {v} against the operands' {sum}, off by {:e} past 2ε",
            v - sum
        );
    }
}

/// **R1's slotted plate**: a plate `[0, 1000] × [0, 1]²` with a slot
/// `x ∈ [10, 990]` cut half through its top, so its top is two faces on
/// one record, and a block `[−2, 0] × [0, 1]²` whose top leans
/// `0.05 ε` about `x = 0`, flush against the plate's near end. The
/// near fragment is in band of the block's top over the two of them,
/// the far one is not. The plate's record is also worn by the far
/// fragment, which no verdict decided, so it is kept: the block's top
/// moves onto it, the far fragment keeps its record, and the union
/// builds in both orders at its operands' sum.
#[test]
fn a_slotted_plates_far_fragment_keeps_its_record() {
    let eps = tol().eps();
    let lean = 0.05 * eps;
    let slab = block("slab", (0.0, 1000.0), (0.0, 1.0), (0.0, 1.0));
    let slot = block("slot", (10.0, 990.0), (-1.0, 2.0), (0.5, 2.0));
    let plate = cut("the slotted plate", &slab, &slot);
    let far = plane_beyond(&plate, true, 900.0);
    let leaning = finished(
        "the leaning block",
        mapped_cube::<f64>(
            move |x, y, z| {
                let gx = -2.0 + 2.0 * x;
                Point3::new(gx, y, z * (1.0 + lean * gx))
            },
            tol(),
        ),
        tol(),
    );
    let sum = vol("plate", &plate) + vol("block", &leaning);
    for (order, x, y) in [("A ∪ P", &leaning, &plate), ("P ∪ A", &plate, &leaning)] {
        let u = united(order, x, y);
        let v = vol(order, &u);
        assert!(
            (v - sum).abs() <= 2.0 * eps,
            "{order}: {v} against the operands' {sum}, off by {:e} past 2ε",
            v - sum
        );
        assert_eq!(
            plane_beyond(&u, true, 900.0),
            far,
            "{order}: the far fragment keeps its record"
        );
    }
}

/// **R2's two-foot bridge**: two feet `x ∈ [0, 1]` and `[L − 1, L]`
/// (`L = 900`, inside the session box)
/// joined above `z = 1`, their bottoms one record, turned `0.3 ε` about
/// `y` and placed with the near foot's low corner at `(1, 1, 1)`,
/// kissing `[0, 1]³` there. The bridge's record is also worn by the far
/// foot, so it is kept: the cube's top moves onto it, the far foot keeps
/// its record, and the union builds in both orders.
#[test]
fn a_bridges_far_foot_keeps_its_record() {
    let eps = tol().eps();
    let l = 900.0;
    let slab = block("slab", (0.0, l), (0.0, 1.0), (0.0, 2.0));
    let gap = block("gap", (1.0, l - 1.0), (-1.0, 2.0), (-1.0, 1.0));
    let bridge = cut("the bridge", &slab, &gap);
    let motion = geom_core::RigidMotion::rotation_then_translation(
        geom_core::UnitVec3::exact_y(),
        0.3 * eps,
        geom_core::Vec3::new(1.0, 1.0, 1.0),
    )
    .expect("a rigid motion");
    let bridge = finished(
        "the turned bridge",
        topo::transform_rigid(&bridge, &motion, tol()).expect("the bridge turns"),
        tol(),
    );
    let far = plane_beyond(&bridge, false, 800.0);
    let cube = block("the cube", (0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let sum = vol("bridge", &bridge) + vol("cube", &cube);
    for (order, x, y) in [
        ("cube ∪ bridge", &cube, &bridge),
        ("bridge ∪ cube", &bridge, &cube),
    ] {
        let u = united(order, x, y);
        let v = vol(order, &u);
        assert!(
            (v - sum).abs() <= eps,
            "{order}: {v} against the operands' {sum}, off by {:e} past ε",
            v - sum
        );
        assert_eq!(
            plane_beyond(&u, false, 800.0),
            far,
            "{order}: the far foot keeps its record"
        );
    }
}

/// **A chain `A1 ~ B1 ~ A2`**: `A` is a base with two posts, their tops
/// `A1` and `A2` coplanar on two records no verdict decided one, and
/// `B` a slab lying across both, its bottom `B1` glued to each. Each
/// pair moves only its own faces, onto a record its verdict was read
/// against: where `A` is written first, the first pair moves `B1` onto
/// `A1`'s record and the second finds it moved and leaves it, so `A2`'s
/// remnant keeps its own record; where `B` is, each pair moves its post
/// top onto `B1`'s record. No face is left on a record no verdict read
/// it against, as a union of the pairs' records would leave `A2` on
/// `A1`'s.
#[test]
fn a_chain_of_glued_pairs_moves_no_undecided_pair() {
    let base = block("base", (0.0, 5.0), (0.0, 1.0), (0.0, 1.0));
    let post = |x0: f64| block("post", (x0, x0 + 1.0), (0.0, 1.0), (1.0, 2.0));
    let a = united("base ∪ post", &base, &post(0.0));
    let a = united("the two posts", &a, &post(4.0));
    let b = block("the slab", (0.5, 4.5), (0.0, 1.0), (2.0, 2.5));
    for (order, x, y, a_first) in [("A ∪ B", &a, &b, true), ("B ∪ A", &b, &a, false)] {
        let u = united(order, x, y);
        // The face at `z = 2` facing `up` (or down) wholly in `[x0, x1]`.
        let at_two = |up: bool, x0: f64, x1: f64| {
            u.faces()
                .find(|&(f, face)| {
                    let Some(geom::Surface::Plane { normal, .. }) = u.face_surface(f) else {
                        return false;
                    };
                    let outward = if face.sense { normal.z } else { -normal.z };
                    (if up { outward > 0.9 } else { outward < -0.9 })
                        && face_points(&u, f)
                            .iter()
                            .all(|p| (p.z - 2.0).abs() < 1e-9 && p.x >= x0 && p.x <= x1)
                })
                .map(|(_, face)| face.surface)
                .unwrap_or_else(|| panic!("{order}: a face at z = 2 in [{x0}, {x1}]"))
        };
        let (a1, a2, b1) = (
            at_two(true, 0.0, 0.5),
            at_two(true, 4.5, 5.0),
            at_two(false, 1.0, 4.0),
        );
        if a_first {
            assert_eq!(b1, a1, "{order}: B1 is moved onto A1's record");
            assert_ne!(a2, a1, "{order}: A2 keeps its own record");
        } else {
            assert_eq!(a1, b1, "{order}: A1 is moved onto B1's record");
            assert_eq!(a2, b1, "{order}: A2 is moved onto B1's record");
        }
    }
}
