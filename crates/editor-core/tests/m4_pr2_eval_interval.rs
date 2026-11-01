//! The pinned Interval evaluation of a boolean-bearing document (M4
//! PR 2 spec D2/D7): `evaluate<T>` instantiated at the certified
//! interval scalar, through profile validation, extrude, a
//! translation-only rigid transform (re-certification at Interval),
//! and a subtract — the enclosure brackets the exact dyadic oracle.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    BooleanValue, CancelToken, EvalOptions, Node, ProfileDoc, ValuePayload, evaluate,
};
use fixture::{insert, len, on_frame};
use geom_core::Tol;
use geom_core::{Bounds, Interval};
use topo::{validate, validate_closed};

#[test]
fn interval_evaluation_of_a_boolean_doc_brackets_the_oracle() {
    // Cube [0,2]³ minus one pip 0.25×0.25×0.125 embedded in the top
    // face (the die's +z pip, alone): volume exactly 8 − 1/128.
    let doc = ProfileDoc::empty_derived("m4_pr2_eval_interval", Tol::witness());
    let (doc, cube_p) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]],
    );
    let (doc, cube) = insert(
        doc,
        Node::Extrude {
            profile: cube_p.into(),
            distance: len(2.0),
        },
    );
    let (doc, pip_p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 0.125)],
    );
    let (doc, pip) = insert(
        doc,
        Node::Extrude {
            profile: pip_p.into(),
            distance: len(0.125),
        },
    );
    // The pip stands on the cube's top at (1, 1), its own z reversed so
    // it reaches down into the cube.
    let (doc, placed) = fixture::against(
        doc,
        cube,
        pip,
        &fixture::axes_at([1.0, 1.0, 2.0], [1.0, 0.0, 0.0], [0.0, -1.0, 0.0]),
    );
    // The pip's outer cap lies ON the cube's top; the subtract glues it.
    let (doc, sub) = insert(
        doc,
        Node::Subtract {
            from: cube.into(),
            tool: placed.into(),
        },
    );

    let ev = evaluate::<Interval>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let ValuePayload::Boolean(BooleanValue::Body { body, .. }) = &ev
        .value(sub)
        .expect("subtract evaluated at Interval")
        .payload
    else {
        panic!("expected boolean body");
    };
    assert_eq!(validate(body.body()), Ok(()));
    assert_eq!(validate_closed(body.body()), Ok(()));
    let vol = topo::mass_properties(
        &topo::test_support::finished(
            "the measured body",
            topo::Body::clone(body.body()),
            Tol::witness(),
        ),
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .expect("the measured body")
    .met()
    .expect("the measured body")
    .volume;
    let oracle = 8.0 - 0.25 * 0.25 * 0.125;
    assert!(
        vol.lo() <= oracle && oracle <= vol.hi(),
        "enclosure [{}, {}] must bracket {oracle}",
        vol.lo(),
        vol.hi()
    );
    // The enclosure is TIGHT for a dyadic construction.
    assert!(vol.hi() - vol.lo() < 1e-9);
}

#[test]
fn the_die_evaluates_at_interval_and_brackets_the_oracle() {
    let d = fixture::die();
    let ev = evaluate::<Interval>(
        &d.doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let ValuePayload::Boolean(BooleanValue::Body { body, .. }) = &ev
        .value(d.final_node)
        .expect("die evaluated at Interval")
        .payload
    else {
        panic!("expected boolean body");
    };
    assert_eq!(validate(body.body()), Ok(()));
    assert_eq!(validate_closed(body.body()), Ok(()));
    let vol = topo::mass_properties(
        &topo::test_support::finished(
            "the measured body",
            topo::Body::clone(body.body()),
            Tol::witness(),
        ),
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .expect("the measured body")
    .met()
    .expect("the measured body")
    .volume;
    assert!(
        vol.lo() <= fixture::DIE_VOLUME && fixture::DIE_VOLUME <= vol.hi(),
        "enclosure [{}, {}] must bracket the oracle",
        vol.lo(),
        vol.hi()
    );
}
