//! **End-to-end consumer check of the M5 PR 1 backend swap** (adopted
//! from the adversarial review's scratch harness).
//!
//! Every other suite in this PR tests the scalar, or the crate under the
//! scalar. This one drives the *product* — a document built and mutated
//! through the public `ProfileDoc::apply` door, evaluated at
//! `T = Interval` through `evaluate` — and asks the only question a user
//! would: does the certified lane still produce a certified answer?
//!
//! The cutter is **rotated**, deliberately. A rotation is the shortest
//! path from a document to the transcendental surface the swap replaced:
//! the rigid transform's matrix comes from `sin_cos` of the angle, and
//! its re-certification at `Interval` runs on those enclosures. A
//! translation-only document would exercise none of it.
//!
//! What is asserted, in the order a reviewer should care about:
//! 1. the volume enclosure **brackets the exact rational 7.75** — the
//!    rotation is volume-preserving, so the oracle is exact even though
//!    the coordinates are not;
//! 2. the residual `|volume − 7.75|` is **`Decide`-certified** as zero
//!    through the public predicate door — bracketing is not the same as
//!    being able to *prove* it, and the certified lane must be able to;
//! 3. refusal works end to end in both directions poison can arrive:
//!    a non-finite dimension cannot enter the document at all, and a
//!    finite-but-degenerate one refuses typed at evaluation rather than
//!    producing a plausible body;
//! 4. a clamped-but-plausible enclosure is diagnosed `Invalid`, not
//!    merely "too wide" — the distinction the subdivision driver needs.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    BooleanOp, BooleanValue, CancelToken, EvalOptions, Node, ProfileDoc, ValuePayload, evaluate,
};
use fixture::{ang, desc, insert, len, scl};
use geom_core::{Band, Bounds, Decide, Interval, MarginDiag, Real, Sign};
use topo::{mass_properties, validate, validate_closed};

/// Cube 2×2×2 minus a 0.5×0.5×1.0 pocket: exactly 8 − 0.25.
const ORACLE_VOLUME: f64 = 7.75;

/// Builds the document: a 2×2×2 cube with a rotated square pocket cut
/// into its top face. The pocket profile is a square centred on the
/// cube's axis, so the quarter-turn maps it onto itself — the removed
/// volume is exactly 0.25 whatever the rotation does to the endpoints,
/// while the transform itself still goes through `sin_cos` at `Interval`.
fn rotated_cutter_doc() -> (ProfileDoc, editor_core::RecipeNodeId) {
    let doc = ProfileDoc::empty();
    let (doc, cube_p) = insert(
        doc,
        Node::Profile(desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]],
        )),
    );
    let (doc, cube) = insert(
        doc,
        Node::Extrude {
            profile: cube_p,
            distance: len(2.0),
        },
    );
    // The cutter profile sits on the top plane z = 2, centred at the
    // origin of its own sketch so the rotation is about its centre.
    let (doc, cut_p) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, 2.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![fixture::square(0.0, 0.0, 0.25)],
        )),
    );
    let (doc, cutter) = insert(
        doc,
        Node::Extrude {
            profile: cut_p,
            distance: len(-1.0),
        },
    );
    // Quarter turn about +z, then centred on the cube's axis.
    let (doc, placed) = insert(
        doc,
        Node::Transform {
            input: cutter,
            translation: [len(1.0), len(1.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(core::f64::consts::FRAC_PI_2),
        },
    );
    let (doc, decl) = insert(
        doc,
        Node::Declare {
            pairs: vec![(
                fixture::fname(cube, editor_core::RoleSeg::Cap(editor_core::CapEnd::Top)),
                fixture::fname(
                    cutter,
                    editor_core::RoleSeg::Cap(editor_core::CapEnd::Bottom),
                ),
            )],
        },
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
    (doc, sub)
}

#[test]
fn rotated_cutter_boolean_at_interval_brackets_and_certifies_the_oracle() {
    let (doc, sub) = rotated_cutter_doc();
    let ev = evaluate::<Interval>(&doc, None, &CancelToken::new(), &EvalOptions::default());
    let ValuePayload::Boolean(BooleanValue::Body { body, .. }) = &ev
        .value(sub)
        .unwrap_or_else(|| {
            panic!(
                "rotated-cutter subtract must evaluate at Interval; node state: {:?}",
                ev.nodes.get(&sub)
            )
        })
        .payload
    else {
        panic!("expected a boolean body");
    };
    assert_eq!(validate(body), Ok(()));
    assert_eq!(validate_closed(body), Ok(()));

    let vol = mass_properties(body).unwrap().volume;
    // (1) Containment of the exact rational oracle.
    assert!(
        vol.lo() <= ORACLE_VOLUME && ORACLE_VOLUME <= vol.hi(),
        "enclosure [{}, {}] must bracket {ORACLE_VOLUME}",
        vol.lo(),
        vol.hi()
    );

    // (2) The residual is CERTIFIED zero through the public predicate
    // door — the enclosure is not merely correct, it is provably correct
    // at a band a consumer would actually use. This is the assertion the
    // pad widening could have broken and did not.
    let residual = vol - Interval::from_f64(ORACLE_VOLUME);
    let band = Band::new(1e-9, 1e-8).unwrap();
    assert_eq!(
        residual.sign_within(band),
        Ok(Sign::Zero),
        "residual enclosure [{}, {}] must certify as zero",
        residual.lo(),
        residual.hi()
    );
}

/// Poison must not be able to ENTER a document in the first place: a
/// non-finite literal is refused typed at the expression layer, before
/// any scalar sees it. This is the outermost ring of the same policy the
/// interval scalar implements at the innermost (`from_f64(NaN)` = NaI),
/// and it is why the evaluation pipeline never has to carry a NaN
/// dimension.
#[test]
fn a_non_finite_dimension_cannot_enter_the_document() {
    use editor_core::{Dimension, Expr};
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(
            Expr::literal(bad, Dimension::Length).unwrap_err(),
            editor_core::DimensionError::NonFiniteLiteral,
            "{bad} must be refused as a length literal"
        );
        assert!(Expr::literal(bad, Dimension::Angle).is_err());
        assert!(Expr::literal(bad, Dimension::Scalar).is_err());
    }
}

/// A finite-but-degenerate document must refuse **typed** at evaluation
/// rather than producing a plausible body — the end-to-end refusal path
/// at `T = Interval`. A zero-distance extrude has no volume to be right
/// about; the certified lane must say so instead of guessing.
#[test]
fn a_degenerate_document_refuses_typed_end_to_end() {
    let doc = ProfileDoc::empty();
    let (doc, prof) = insert(
        doc,
        Node::Profile(desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]],
        )),
    );
    let (doc, degenerate) = insert(
        doc,
        Node::Extrude {
            profile: prof,
            distance: len(0.0),
        },
    );
    let ev = evaluate::<Interval>(&doc, None, &CancelToken::new(), &EvalOptions::default());
    assert!(
        ev.value(degenerate).is_none(),
        "a zero-distance extrude must refuse at Interval, got {:?}",
        ev.value(degenerate).map(|v| &v.payload)
    );
    // The refusal is TYPED (a recorded node failure), not a silent absence.
    assert!(
        ev.nodes.get(&degenerate).is_some(),
        "the refusal must be recorded against the node"
    );
}

/// The poison-refusal contract at the scalar boundary the document
/// pipeline sits on: a domain-violating enclosure that *looks* healthy
/// must still refuse, and `MarginDiag::Invalid` is how the driver tells
/// "poisoned" from "too wide to call".
#[test]
fn a_clamped_enclosure_refuses_with_the_invalid_diagnosis() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    // sqrt([-1, 4]) clamps to [0, 2]: plausible bounds, poisoned history.
    let clamped = Interval::from_bounds(-1.0, 4.0).sqrt();
    assert_eq!((clamped.lo(), clamped.hi()), (0.0, 2.0));
    match clamped.sign_within(band) {
        Err(e) => assert_eq!(
            e.margin,
            MarginDiag::Invalid,
            "a clamp must be diagnosed as Invalid, not as a wide enclosure"
        ),
        Ok(s) => panic!("clamped enclosure decided {s:?}"),
    }
}
