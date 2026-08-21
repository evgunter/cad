//! M4 PR 6 spec D2 — bit-exact float round-trip, property-tested.
//!
//! Floats persist as ryu shortest-round-trip strings (serde_json's
//! writer); load must reproduce the EXACT bits. Pinned here on the
//! adversarial population: subnormals (min positive included), -0.0
//! (data, not refused), ulp neighbors, and arbitrary finite bit
//! patterns — carried through every float slot the format has (doc
//! params, expression literals, profile geometry, recorded ε, D7
//! metadata).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    Dimension, DocEdit, DocParam, Expr, MetaValue, Node, ParamName, ProfileDoc, load, save,
};
use fixture::desc;
use geom_core::Tol;
use proptest::prelude::*;

/// Round-trips one value through every float slot at once; returns
/// the loaded document for slot-by-slot bit assertions.
fn round_trip(value: f64) -> ProfileDoc {
    let mut doc = ProfileDoc::empty_derived("m4_pr6_floats", Tol::witness());
    let push = |d: &ProfileDoc, e| editor_core::apply(d, &e, Tol::witness()).expect("edit").doc;
    doc = push(
        &doc,
        DocEdit::SetDocParam {
            name: ParamName::new("p"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value,
            },
        },
    );
    // The profile's RAW-float channel is the PLANE PLACEMENT under v4
    // (program args are Exprs, and the VQ9 door validates program
    // GEOMETRY, so arbitrary-magnitude coordinates no longer belong in
    // loops — they ride the placement, which validation passes
    // through as conventional data).
    doc = push(
        &doc,
        DocEdit::InsertNode {
            node: Node::Profile(desc(
                [value, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                vec![vec![(0.0, 0.0), (1.0, 0.0), (0.5, 1.0)]],
            )),
        },
    );
    doc = push(
        &doc,
        DocEdit::InsertNode {
            node: Node::Datum(editor_core::Datum::Point {
                position: [
                    Expr::literal(value, Dimension::Length).expect("finite literal"),
                    Expr::literal(0.0, Dimension::Length).unwrap(),
                    Expr::literal(-0.0, Dimension::Length).unwrap(),
                ],
            }),
        },
    );
    let text = save(&doc, &[], Tol::witness()).expect("save");
    load(&text, Tol::witness()).expect("load").doc
}

fn assert_bits(label: &str, value: f64, loaded: f64) {
    assert_eq!(
        value.to_bits(),
        loaded.to_bits(),
        "{label}: {value:?} ({:#018x}) loaded as {loaded:?} ({:#018x})",
        value.to_bits(),
        loaded.to_bits()
    );
}

fn check_all_slots(value: f64) {
    let doc = round_trip(value);
    let Some(DocParam::Continuous { value: p, .. }) = doc.params().get(&ParamName::new("p")) else {
        panic!("param lost");
    };
    assert_bits("doc param", value, *p);
    let Some(Node::Profile(prof)) = doc.node(editor_core::RecipeNodeId(0)) else {
        panic!("profile lost");
    };
    // The value rides the placement origin (v4's raw-float channel);
    // arbitrary-value EXPRESSION coverage is the datum literal below.
    assert_bits(
        "profile placement",
        value,
        prof.plane.placement.translation.x,
    );
    let Some(Node::Datum(editor_core::Datum::Point { position })) =
        doc.node(editor_core::RecipeNodeId(1))
    else {
        panic!("datum lost");
    };
    let mut bits = Vec::new();
    position[0].literal_bits(&mut bits);
    assert_eq!(bits, vec![value.to_bits()], "expression literal bits");
    position[2].literal_bits(&mut bits);
    assert!(
        bits.contains(&(-0.0f64).to_bits()),
        "-0.0 literal must keep its sign"
    );
}

#[test]
fn special_values_round_trip_bit_exactly() {
    let ulp_up = 1.0f64.next_up();
    let ulp_down = 1.0f64.next_down();
    for v in [
        0.0,
        -0.0,
        1.0,
        -1.0,
        ulp_up,
        ulp_down,
        f64::MIN_POSITIVE,                     // smallest normal
        f64::from_bits(1),                     // smallest subnormal
        f64::from_bits(0x000F_FFFF_FFFF_FFFF), // largest subnormal
        -f64::from_bits(1),
        f64::MAX,
        f64::MIN,
        std::f64::consts::PI,
        0.1,
        1e-323,
        #[allow(clippy::excessive_precision)]
        // deliberately one digit past shortest: the notorious rounding-boundary value
        2.225_073_858_507_201_1e-308,
    ] {
        check_all_slots(v);
    }
}

#[test]
fn epsilon_round_trips_bit_exactly() {
    // ε is recorded in-document (D4); its bits survive save/load.
    // (The in-process reconcile door requires it to MATCH the
    // committed ambient ε, so the probe uses the ambient value.)
    let doc = ProfileDoc::empty_derived("m4_pr6_floats", Tol::witness());
    let text = save(&doc, &[], Tol::witness()).expect("save");
    let loaded = load(&text, Tol::witness()).expect("load");
    assert_bits("epsilon", doc.epsilon(), loaded.doc.epsilon());
}

#[test]
fn metadata_floats_round_trip_bit_exactly() {
    for v in [-0.0f64, f64::from_bits(1), 0.1, f64::MAX] {
        let mut m = std::collections::BTreeMap::new();
        m.insert("v".to_owned(), MetaValue::Int(1));
        m.insert("x".to_owned(), MetaValue::Float(v));
        let value = MetaValue::Map(m);
        // Bit-eq PartialEq (D7): equality on the canonical tree IS
        // bit equality, so assert_eq pins the bits.
        let json = serde_json::to_string(&value).expect("ser");
        let back: MetaValue = serde_json::from_str(&json).expect("de");
        assert_eq!(back, value, "metadata float {v:?} drifted");
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Arbitrary finite bit patterns (subnormals included by
    /// construction: the strategy draws BITS).
    #[test]
    fn arbitrary_finite_bits_round_trip(bits in any::<u64>()) {
        let v = f64::from_bits(bits);
        prop_assume!(v.is_finite());
        check_all_slots(v);
    }

    /// Ulp ladders around exact powers of two and decimal-boundary
    /// values (the shortest-round-trip stress region).
    #[test]
    fn ulp_neighbors_round_trip(exp in -300i32..300, steps in 0u8..8) {
        let mut v = 2.0f64.powi(exp);
        for _ in 0..steps {
            v = v.next_up();
        }
        prop_assume!(v.is_finite());
        check_all_slots(v);
    }
}
