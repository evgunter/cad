//! The pinned Interval evaluation of a boolean-bearing document (M4
//! PR 2 spec D2/D7): `evaluate<T>` instantiated at the certified
//! interval scalar, through profile validation, extrude, a
//! translation-only rigid transform (re-certification at Interval),
//! and a subtract — the enclosure brackets the exact dyadic oracle.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    BooleanOp, BooleanValue, CancelToken, EvalOptions, Node, ProfileDoc, ValuePayload, evaluate,
};
use fixture::{ang, desc, insert, len, on_frame, scl};
use geom_core::Tol;
use geom_core::{Bounds, Interval};
use topo::{mass_properties, validate, validate_closed};

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
            profile: cube_p,
            distance: len(2.0),
        },
    );
    let (doc, pip_p) = on_frame(
        doc,
        [0.0, 0.0, 2.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 0.125)],
    );
    let (doc, pip) = insert(
        doc,
        Node::Extrude {
            profile: pip_p,
            distance: len(-0.125),
        },
    );
    let (doc, placed) = insert(
        doc,
        Node::Transform {
            input: pip,
            translation: [len(1.0), len(1.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    // The pip's outer cap lies ON the cube's top — declared (M4 PR 5).
    let (doc, decl) = insert(
        doc,
        Node::declare_rest(vec![(
            fixture::fname(cube, editor_core::RoleSeg::Cap(editor_core::CapEnd::Top)),
            fixture::fname(pip, editor_core::RoleSeg::Cap(editor_core::CapEnd::Bottom)),
        )]),
    );
    let (doc, sub) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: cube,
            b: placed,
            declare: Some(decl),
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
    assert_eq!(validate(body), Ok(()));
    assert_eq!(validate_closed(body), Ok(()));
    let vol = mass_properties(body, Tol::witness()).unwrap().volume;
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
    assert_eq!(validate(body), Ok(()));
    assert_eq!(validate_closed(body), Ok(()));
    let vol = mass_properties(body, Tol::witness()).unwrap().volume;
    assert!(
        vol.lo() <= fixture::DIE_VOLUME && fixture::DIE_VOLUME <= vol.hi(),
        "enclosure [{}, {}] must bracket the oracle",
        vol.lo(),
        vol.hi()
    );
}
