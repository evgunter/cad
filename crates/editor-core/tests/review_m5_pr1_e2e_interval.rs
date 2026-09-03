//! **End-to-end consumer check of the M5 PR 1 backend swap.** Promoted
//! from the adversarial review's scratch consumer crate (its F9), which
//! was a standalone binary depending on the kernel by path — i.e. a real
//! external consumer of the public API. Re-hosted here as an integration
//! test so it runs in CI; the body below is the review's, unchanged in
//! substance.
//!
//! Every other suite in this PR tests the scalar, or the crate under the
//! scalar. This one drives the *product*: a document built through the
//! public `ProfileDoc::apply` door, evaluated at `T = Interval` through
//! `evaluate`, and asked the only question a user would — does the
//! certified lane still produce a certified answer?
//!
//! The cutter is **rotated by 0.5 rad**, deliberately and non-specially:
//! a rotation is the shortest path from a document to the transcendental
//! surface the swap replaced (the rigid transform's matrix comes from
//! `sin_cos` of the angle), and an angle that is not a quarter turn
//! keeps the matrix entries genuinely inexact. A translation-only
//! document would exercise none of it.
//!
//! What is asserted:
//! 1. the volume enclosure **brackets the exact rational 7.75** — the
//!    rotation is volume-preserving, so the oracle stays exact even
//!    though the coordinates do not;
//! 2. the residual `vol − 7.75` **certifies as `Zero`** through the
//!    public `Decide` door, and a deliberately shifted baseline still
//!    certifies `Positive` — bracketing is not the same as being able to
//!    *prove* it, and a control is needed or "certifies Zero" could be
//!    satisfied by an enclosure too wide to say anything;
//! 3. a domain violation flows through arithmetic AND transcendentals
//!    into a **refused** verdict carrying `MarginDiag::Invalid`.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    BooleanOp, BooleanValue, CancelToken, EvalOptions, Node, ProfileDoc, ValuePayload, evaluate,
};
use fixture::{ang, insert, len, on_frame, scl};
use geom_core::Tol;
use geom_core::predicate::{Band, Decide, Indeterminate, MarginDiag};
use geom_core::{Bounds, Interval, Real};
use topo::{mass_properties, validate, validate_closed};

#[test]
fn rotated_cutter_boolean_at_interval_certifies_end_to_end() {
    // ---- body 1: dyadic cube minus embedded pip; volume EXACTLY 8 - 1/128.
    let doc = ProfileDoc::empty_derived("review_m5_pr1_e2e_interval", Tol::witness());
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
    // A second, ROTATED cutter well inside the cube: rotation angle 0.5 rad
    // pushes sin/cos (the swapped transcendentals) through the whole
    // evaluation path at T = Interval.
    let (doc, cut_p) = on_frame(
        doc,
        [0.0, 0.0, 0.5],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (-0.25, -0.25),
            (0.25, -0.25),
            (0.25, 0.25),
            (-0.25, 0.25),
        ]],
    );
    let (doc, cut) = insert(
        doc,
        Node::Extrude {
            profile: cut_p,
            distance: len(1.0),
        },
    );
    let (doc, placed) = insert(
        doc,
        Node::Transform {
            input: cut,
            translation: [len(1.0), len(1.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.5),
        },
    );
    let (doc, sub) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: cube,
            b: placed,
            declare: None,
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
    // Interior rotated cuboid 0.5 x 0.5 x 1.0 => volume exactly 8 - 0.25.
    let oracle = 8.0 - 0.25;
    assert!(
        vol.lo() <= oracle && oracle <= vol.hi(),
        "enclosure [{}, {}] must bracket {oracle}",
        vol.lo(),
        vol.hi()
    );
    // ---- certified residual through Decide: |vol - oracle| certified ~ 0
    // at a 1e-6 band, definitely POSITIVE against a shifted baseline.
    let band = Band::new(1e-6, 1e-5).unwrap();
    let residual = vol - Interval::from_f64(oracle);
    match residual.sign_within(band) {
        Ok(s) => assert_eq!(format!("{s:?}"), "Zero", "residual must certify Zero"),
        other => panic!("residual failed to certify: {other:?}"),
    }
    let shifted = vol - Interval::from_f64(oracle - 1.0);
    assert!(matches!(
        shifted.sign_within(band),
        Ok(geom_core::predicate::Sign::Positive)
    ));
    println!(
        "e2e body: volume enclosure [{:.15}, {:.15}] width {:.3e}, residual certified Zero",
        vol.lo(),
        vol.hi(),
        vol.hi() - vol.lo()
    );

    // ---- poison path end-to-end: a domain violation at interval type
    // flows through arithmetic/transcendentals into a REFUSED verdict.
    let poisoned = Interval::from_bounds(-1.0, 4.0).sqrt(); // clamped [0,2] @ Trv
    let chained = (poisoned * Interval::pi()).sin() + Interval::from_f64(10.0);
    match chained.sign_within(band) {
        Err(Indeterminate {
            margin: MarginDiag::Invalid,
            ..
        }) => println!("e2e poison: refused with Invalid margin, as contracted"),
        other => panic!("poison leaked to a verdict: {other:?}"),
    }
    println!("review-e2e: ALL CHECKS PASSED");
}

/// **Beyond the review harness**: the other direction poison can arrive
/// at a document — as a *dimension* rather than as a computed enclosure.
/// A non-finite literal is refused typed at the expression layer, before
/// any scalar sees it: the outermost ring of the same policy the
/// interval scalar implements at the innermost (`from_f64(NaN)` = NaI),
/// and the reason the evaluation pipeline never has to carry a NaN.
#[test]
fn a_non_finite_dimension_cannot_enter_the_document() {
    use editor_core::{Dimension, DimensionError, Expr};
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(
            Expr::literal(bad, Dimension::Length).unwrap_err(),
            DimensionError::NonFiniteLiteral,
            "{bad} must be refused as a length literal"
        );
        assert!(Expr::literal(bad, Dimension::Angle).is_err());
        assert!(Expr::literal(bad, Dimension::Scalar).is_err());
    }
}

/// **Beyond the review harness**: a finite-but-degenerate document must
/// refuse *typed* at evaluation rather than produce a plausible body —
/// the end-to-end refusal path at `T = Interval`. A zero-distance
/// extrude has no volume to be right about; the certified lane must say
/// so instead of guessing.
#[test]
fn a_degenerate_document_refuses_typed_end_to_end() {
    let doc = ProfileDoc::empty_derived("review_m5_pr1_e2e_interval", Tol::witness());
    let (doc, prof) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]],
    );
    let (doc, degenerate) = insert(
        doc,
        Node::Extrude {
            profile: prof,
            distance: len(0.0),
        },
    );
    let ev = evaluate::<Interval>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    assert!(
        ev.value(degenerate).is_none(),
        "a zero-distance extrude must refuse at Interval, got {:?}",
        ev.value(degenerate).map(|v| &v.payload)
    );
    assert!(
        ev.nodes.contains_key(&degenerate),
        "the refusal must be recorded against the node, not silently absent"
    );
}
