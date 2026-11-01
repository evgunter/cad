//! **DOCM-1 under the Interval lane** — DM1c's rows: a boss placed on a
//! face is placed at the lane scalar, and the floor of a
//! plain-`Interval` extrude of a widened height.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::corpus;
use crate::fixture::{self, Recorder};

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::{
    CancelToken, Dimension, Distribution, DocEdit, EvalOptions, Evaluation, Formula, FreeVar, Node,
    ProfileDoc, UnitSym, VarName, evaluate,
};
use geom_core::{Bounds, Interval, Tol};
use topo::validate_closed;

fn run(
    doc: &ProfileDoc,
    prior: Option<&Evaluation<Interval>>,
    opts: &EvalOptions,
) -> Evaluation<Interval> {
    evaluate::<Interval>(doc, prior, &CancelToken::new(), opts, Tol::witness())
}

/// **The corpus document's boss is placed at the lane scalar**: the
/// boss's copy, placed on the box's top face, evaluates green at
/// `Interval`, and its bottom encloses the cap's height — where the face
/// it names lands, not a placeholder.
#[test]
fn a_boss_placed_on_a_face_is_placed_at_the_lane_scalar() {
    let cd = corpus::face_sketch::document();
    let whole = cd.result.expect("the union");
    let Some(Node::Union { members, .. }) = cd.doc.node(whole) else {
        panic!("the result is the union")
    };
    let boss = members
        .reads()
        .map(|m| cd.doc.operation_of(m.read).expect("a live member"))
        .find(|&m| matches!(cd.doc.node(m), Some(Node::Place { .. })))
        .expect("the boss's placement");
    let ev = run(&cd.doc, None, &EvalOptions::default());
    assert!(
        corpus::failures(&ev).is_empty(),
        "{:?}",
        corpus::failures(&ev)
    );
    let body = corpus::body_of(&ev, boss);
    assert_eq!(validate_closed(&body), Ok(()));
    let bottom = body
        .points()
        .map(|(_, p)| p.z.lo())
        .fold(f64::INFINITY, f64::min);
    assert!(
        (bottom - 1.0).abs() <= 1e-9,
        "the boss stands on the cap: {bottom}"
    );
}

/// **The measurement of record for a plain-`Interval` extrude of a
/// WIDENED height** (no frame anywhere): certifies at ε/10 and every
/// narrower width, refuses at ε/8 — a floor near ε/16, at every ε
/// row. The ladder is printed in full; the assertion is the floor.
#[test]
fn an_interval_extrude_of_a_widened_height() {
    let e = Tol::witness().eps();
    let at = |width: f64| -> bool {
        let mut r = Recorder::new();
        r.push(DocEdit::DeclareVar {
            name: VarName::from_static("hh"),
            def: editor_core::VarDecl::Free(FreeVar::Continuous {
                dim: Dimension::Length,
                value: 1.0,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: if width > 0.0 {
                    Some(Distribution::Uniform {
                        lo: -width,
                        hi: width,
                    })
                } else {
                    None
                },
            }),
        });
        let profile = r.profile(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![fixture::square(0.0, 0.0, 1.0)],
        );
        r.insert(Node::Extrude {
            profile: profile.into(),
            distance: Formula::named(VarName::from_static("hh"), Dimension::Length),
        });
        let doc = r.doc;
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let opts = EvalOptions {
            param_box: Some(Arc::new(ParamBox::of(&analyzed))),
            ..EvalOptions::default()
        };
        let ev = run(&doc, None, &opts);
        let fails = corpus::failures(&ev);
        println!(
            "widened extrude height: eps={e:e} width={width:e} green={} {}",
            fails.is_empty(),
            fails.first().map(String::as_str).unwrap_or("")
        );
        fails.is_empty()
    };
    for div in [8.0, 10.0, 30.0, 100.0, 300.0, 1.0e3, 1.0e4, 1.0e6] {
        let green = at(e / div);
        if div >= 10.0 {
            assert!(
                green,
                "a widened extrude height of ε/{div} certifies on plain Interval"
            );
        } else {
            assert!(
                !green,
                "a widened extrude height of ε/{div} refuses on plain Interval"
            );
        }
    }
    assert!(at(0.0), "the exact height certifies");
}
