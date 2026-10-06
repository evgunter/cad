//! **A boss drawn to the edge of a block's top unions once its two
//! coincidences are declared.**
//!
//! The block is `40 × 20 × 10` centred on the world xy frame; the boss
//! is `x ∈ [10, 20]`, `y ∈ [−5, 5]`, standing 4 tall on the block's
//! top, so its `+x` wall lies in the block's `+x` wall plane, facing the
//! same way. The two solids touch at two faces: the resting cap pair
//! (`Rest`) and the flush walls (a continuation). Undeclared, the union
//! reports the walls first; each refusal names one pair, and declaring
//! the pairs the refusals name, one at a time, ends in the union at the
//! closed-form volume `40·20·10 + 10·10·4 = 8400`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use sweep::test_support::{brick, finished};
use topo::{
    AtRestBody, Body, BooleanCoincidence, BooleanDeclarations, BooleanError, BooleanResult,
    FaceKey, FacePairDeclaration, Operand, PlaneRelation,
};

/// The block and the boss, finished as operands.
fn scene() -> (AtRestBody<f64>, AtRestBody<f64>) {
    let tol = Tol::witness();
    (
        finished(
            "the block",
            brick((-20.0, 20.0), (-10.0, 10.0), (0.0, 10.0), tol),
            tol,
        ),
        finished(
            "the boss",
            brick((10.0, 20.0), (-5.0, 5.0), (10.0, 14.0), tol),
            tol,
        ),
    )
}

/// The planar face of `body` at `x = 20` facing `+x`.
fn plus_x_wall(body: &Body<f64>) -> FaceKey {
    let hits: Vec<FaceKey> = body
        .faces()
        .map(|(k, _)| k)
        .filter(|&k| {
            matches!(topo::face_carrier(body, k),
                Some(topo::CarrierDesc::Plane { origin, normal })
                    if normal.x > 1.0 - 1e-12 && (origin.x - 20.0).abs() < 1e-12)
        })
        .collect();
    let [f] = hits[..] else {
        panic!("one +x wall at x = 20, got {hits:?}");
    };
    f
}

/// The class a declaration of a refused pair asserts: opposed faces
/// rest, aligned ones carry on.
fn class_of(relation: PlaneRelation) -> BooleanCoincidence {
    match relation {
        PlaneRelation::SameOpposite => BooleanCoincidence::REST,
        PlaneRelation::SameOriented => BooleanCoincidence::Continuation,
        PlaneRelation::Distinct => panic!("a coincidence refusal never names a distinct pair"),
    }
}

/// `a ∪ b`, declaring each pair an `UndeclaredCoincidence` names until
/// the union answers; returns the answer and the relations declared, in
/// the order the refusals named them.
fn offer_loop(
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
) -> (Result<BooleanResult<f64>, BooleanError>, Vec<PlaneRelation>) {
    let mut decls = BooleanDeclarations::none();
    let mut named = Vec::new();
    loop {
        match topo::union_with(a, b, &decls, Tol::witness()) {
            Err(BooleanError::UndeclaredCoincidence { pair, relation, .. }) if named.len() < 4 => {
                let (fa, fb) = by_operand(pair);
                named.push(relation);
                decls
                    .coincident_faces
                    .push(FacePairDeclaration::new(fa, fb, class_of(relation)));
            }
            other => return (other, named),
        }
    }
}

/// A refused pair as `(face of A, face of B)`, whichever order the
/// refusal names them in.
fn by_operand(pair: [(Operand, FaceKey); 2]) -> (FaceKey, FaceKey) {
    match pair {
        [(Operand::A, fa), (Operand::B, fb)] | [(Operand::B, fb), (Operand::A, fa)] => (fa, fb),
        other => panic!("a refused pair names a face of each operand: {other:?}"),
    }
}

/// A built union: its volume against the closed form, one shell, tier 3
/// and the census.
fn assert_built(tag: &str, r: Result<BooleanResult<f64>, BooleanError>, want: f64) {
    let tol = Tol::witness();
    let bb = match r {
        Ok(BooleanResult::Body(bb)) => bb,
        other => panic!("{tag}: the union builds, got {other:?}"),
    };
    let got = topo::mass_properties(&bb.body, tol).unwrap().volume;
    assert!(
        (got - want).abs() <= 1e-12 * want,
        "{tag}: volume {got} vs {want}"
    );
    assert_eq!(bb.body.shells().count(), 1, "{tag}: one shell");
    assert_eq!(
        topo::validate_geometric(&bb.body, tol),
        Ok(()),
        "{tag}: tier 3"
    );
    assert_eq!(
        topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol),
        Ok(()),
        "{tag}: the census"
    );
}

/// **The flush walls are the first contact reported**, in both operand
/// orders: undeclared, and with only the resting cap pair declared.
#[test]
fn the_flush_walls_are_reported_before_and_after_the_cap_pair_is_declared() {
    let (block, boss) = scene();
    let caps = |a: &AtRestBody<f64>, b: &AtRestBody<f64>, a_is_block: bool| {
        let cap = |body: &Body<f64>, up: bool| {
            let hits: Vec<FaceKey> = body
                .faces()
                .map(|(k, _)| k)
                .filter(|&k| {
                    matches!(topo::face_carrier(body, k),
                        Some(topo::CarrierDesc::Plane { origin, normal })
                            if (normal.z > 0.5) == up && normal.z.abs() > 0.5
                                && (origin.z - 10.0).abs() < 1e-12)
                })
                .collect();
            hits[0]
        };
        let mut d = BooleanDeclarations::none();
        d.coincident_faces.push(FacePairDeclaration::new(
            cap(a, a_is_block),
            cap(b, !a_is_block),
            BooleanCoincidence::REST,
        ));
        d
    };
    for (order, a, b, a_is_block) in [
        ("block ∪ boss", &block, &boss, true),
        ("boss ∪ block", &boss, &block, false),
    ] {
        for (what, decls) in [
            ("undeclared", BooleanDeclarations::none()),
            ("the cap pair declared", caps(a, b, a_is_block)),
        ] {
            match topo::union_with(a, b, &decls, Tol::witness()) {
                Err(BooleanError::UndeclaredCoincidence { pair, relation, .. }) => {
                    let (fa, fb) = by_operand(pair);
                    assert_eq!(relation, PlaneRelation::SameOriented, "{order}, {what}");
                    assert_eq!(
                        (fa, fb),
                        (plus_x_wall(a), plus_x_wall(b)),
                        "{order}, {what}: the +x walls are named"
                    );
                }
                other => panic!("{order}, {what}: the flush walls refuse, got {other:?}"),
            }
        }
    }
}

/// **Following the refusals builds the union**: the walls are named
/// first, then the resting caps, and the union of the two declarations
/// is the closed form, in both operand orders.
#[test]
fn declaring_what_the_refusals_name_builds_the_union_at_its_closed_form() {
    let (block, boss) = scene();
    for (order, a, b) in [
        ("block ∪ boss", &block, &boss),
        ("boss ∪ block", &boss, &block),
    ] {
        let (r, named) = offer_loop(a, b);
        assert_eq!(
            named,
            [PlaneRelation::SameOriented, PlaneRelation::SameOpposite],
            "{order}: the walls, then the caps"
        );
        assert_built(order, r, 8400.0);
    }
}

/// **A cylinder through the block, its caps flush with the block's top
/// and bottom, unions** once both cap pairs are declared as the
/// refusals name them (each a continuation): the cylinder lies inside
/// the block, so the union is the block, `40 · 20 · 10 = 8000`. Its rim
/// authored as two arcs and as three, in both operand orders.
#[test]
fn a_cylinder_through_the_block_with_flush_caps_unions_to_the_block() {
    let tol = Tol::witness();
    let block = finished(
        "the block",
        brick((-20.0, 20.0), (-10.0, 10.0), (0.0, 10.0), tol),
        tol,
    );
    for n in [2, 3] {
        let cylinder = finished(
            "the cylinder",
            sweep::test_support::disc_of_arcs(n, 5.0, 10.0, tol),
            tol,
        );
        for (order, a, b) in [
            ("block ∪ cylinder", &block, &cylinder),
            ("cylinder ∪ block", &cylinder, &block),
        ] {
            let tag = format!("{n} arcs, {order}");
            let (r, named) = offer_loop(a, b);
            assert_eq!(
                named,
                [PlaneRelation::SameOriented, PlaneRelation::SameOriented],
                "{tag}: the two cap pairs"
            );
            assert_built(&tag, r, 8000.0);
        }
    }
}
