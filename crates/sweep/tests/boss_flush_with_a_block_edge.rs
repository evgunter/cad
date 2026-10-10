//! **A boss drawn to the edge of a block's top unions, its two
//! coincidences declared or not.**
//!
//! The block is `40 × 20 × 10` centred on the world xy frame; the boss
//! is `x ∈ [10, 20]`, `y ∈ [−5, 5]`, standing 4 tall on the block's
//! top, so its `+x` wall lies in the block's `+x` wall plane, facing the
//! same way. The two solids touch at two faces: the resting cap pair
//! (`Rest`) and the flush walls (a continuation). Each pair is one
//! carrier by margin, so the union glues it whether or not it is
//! declared: undeclared, with only the cap pair declared, and with
//! every flush finding declared, the union is one body bit for bit
//! (D10), at the closed-form volume `40·20·10 + 10·10·4 = 8400`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use sweep::test_support::{brick, finished};
use topo::{
    AtRestBody, Body, BooleanCoincidence, BooleanDeclarations, BooleanError, BooleanResult,
    FaceKey, FacePairDeclaration,
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

/// Every flush finding between `a` and `b`, declared.
fn every_finding(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> BooleanDeclarations {
    let found = topo::flush::find_flush_candidates(a, b, Tol::witness()).expect("the pair decides");
    topo::flush::declare_all(&found)
}

/// `a ∪ b` under each of `postures` is the union with every flush
/// finding declared, bit for bit, and that union builds at `want`.
fn one_union(
    tag: &str,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    postures: &[(&str, BooleanDeclarations)],
    want: f64,
) {
    let declared = topo::union_with(a, b, &every_finding(a, b), Tol::witness());
    let body = format!("{declared:?}");
    assert_built(&format!("{tag}, declared"), declared, want);
    for (what, d) in postures {
        assert_eq!(
            format!("{:?}", topo::union_with(a, b, d, Tol::witness())),
            body,
            "{tag}, {what}: the declared union"
        );
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

/// **The boss unions at its closed form, declared or not**, in both
/// operand orders: with every flush finding declared (the `+x` walls a
/// continuation, the caps a `Rest`), with only the resting cap pair
/// declared, and undeclared, the union is one body.
#[test]
fn the_boss_unions_at_its_closed_form_declared_or_not() {
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
        assert!(
            every_finding(a, b)
                .coincident_faces
                .iter()
                .any(|d| (d.a, d.b) == (plus_x_wall(a), plus_x_wall(b))
                    && d.class == BooleanCoincidence::Continuation),
            "{order}: the +x walls are a continuation finding"
        );
        one_union(
            order,
            a,
            b,
            &[
                ("undeclared", BooleanDeclarations::none()),
                ("the cap pair declared", caps(a, b, a_is_block)),
            ],
            8400.0,
        );
    }
}

/// **A cylinder through the block, its caps flush with the block's top
/// and bottom, unions to the block**, declared or not: the cylinder
/// lies inside the block, so the union is the block, `40 · 20 · 10 =
/// 8000`, and the undeclared union is the declared one. Its rim
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
            one_union(
                &tag,
                a,
                b,
                &[("undeclared", BooleanDeclarations::none())],
                8000.0,
            );
        }
    }
}
