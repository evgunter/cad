//! Reviewer R2 probes (PR #4618): a measure whose two references read
//! the two halves of ONE split, each by its port.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{Recorder, len, scl};
use editor_core::{
    CancelToken, Datum, EntityKind, EvalOptions, Evaluation, ExtrudeSide, MeasurePrimitive,
    NamePat, Node, NodeResult, Operand, RecipeNodeId, SegPat, SegTag, Selector, SplitHalf,
    ValuePayload, evaluate, select,
};
use geom_core::Tol;

fn eval(doc: &editor_core::ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// A 1 x 1 x 4 column, split at z = 2.
fn column(r: &mut Recorder) -> RecipeNodeId {
    let p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let ex = r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(4.0),
        side: ExtrudeSide::Along,
    });
    let tool = r.insert(Node::Datum(Datum::Plane {
        origin: [len(0.0), len(0.0), len(2.0)],
        normal: [scl(0.0), scl(0.0), scl(1.0)],
    }));
    r.insert(Node::Split {
        target: ex.into(),
        tool: tool.into(),
    })
}

fn cap_at(ev: &Evaluation<f64>, at: RecipeNodeId, end: editor_core::CapEnd) -> editor_core::StableName {
    let sel =
        Selector::of(NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(end)));
    let mut found = select(ev, at, &sel);
    assert_eq!(found.len(), 1, "one {end:?} cap at the split: {found:?}");
    found.remove(0)
}

fn sel(split: RecipeNodeId, h: SplitHalf, name: editor_core::StableName) -> Operand {
    Operand::Select {
        body: Box::new(Operand::output(split, h.port())),
        names: vec![name],
    }
}

/// The far caps of the two halves are 4 apart. Each reference reads
/// its own half by port.
#[test]
fn r2_a_distance_across_one_splits_two_halves() {
    let mut r = Recorder::new();
    let split = column(&mut r);
    let ev = eval(&r.doc);
    let top = cap_at(&ev, split, editor_core::CapEnd::End);
    let bottom = cap_at(&ev, split, editor_core::CapEnd::Start);
    let m = r.measure(
        &[MeasurePrimitive::Distance { a: 0, b: 1 }],
        &[
            sel(split, SplitHalf::Above, top),
            sel(split, SplitHalf::Below, bottom),
        ],
    );
    let ev = eval(&r.doc);
    match ev.nodes.get(&m.measures[0]) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => {
                assert!((value - 4.0).abs() < 1e-9, "distance {value}, expected 4")
            }
            other => panic!("{}", other.kind_name()),
        },
        other => panic!("the measure did not evaluate: {other:?}"),
    }
}

/// The same pair, each reference read through a one-body node first
/// (a rigid identity transform of each half): control.
#[test]
fn r2_b_distance_across_halves_by_whole_body_clearance() {
    let mut r = Recorder::new();
    let split = column(&mut r);
    let m = r.measure(
        &[MeasurePrimitive::MinClearance {
            a: 0,
            b: 1,
        }],
        &[
            Operand::output(split, SplitHalf::Above.port()),
            Operand::output(split, SplitHalf::Below.port()),
        ],
    );
    let ev = eval(&r.doc);
    let got = ev.nodes.get(&m.measures[0]);
    eprintln!("min_clearance(above, below) = {got:?}");
}

/// Order swapped: the below half's cap first. If the last read of one
/// split overwrites the other, the refusal moves to reference 1.
#[test]
fn r2_c_distance_across_halves_swapped() {
    let mut r = Recorder::new();
    let split = column(&mut r);
    let ev = eval(&r.doc);
    let top = cap_at(&ev, split, editor_core::CapEnd::End);
    let bottom = cap_at(&ev, split, editor_core::CapEnd::Start);
    let m = r.measure(
        &[MeasurePrimitive::Distance { a: 0, b: 1 }],
        &[
            sel(split, SplitHalf::Below, bottom),
            sel(split, SplitHalf::Above, top),
        ],
    );
    let ev = eval(&r.doc);
    let got = ev.nodes.get(&m.measures[0]);
    assert!(
        matches!(got, Some(NodeResult::Ok(_))),
        "the measure did not evaluate: {got:?}"
    );
}

/// Control: the top cap measured against itself on its own half.
#[test]
fn r2_d_one_half_alone_resolves() {
    let mut r = Recorder::new();
    let split = column(&mut r);
    let ev = eval(&r.doc);
    let top = cap_at(&ev, split, editor_core::CapEnd::End);
    let m = r.measure(
        &[MeasurePrimitive::Distance { a: 0, b: 1 }],
        &[
            sel(split, SplitHalf::Above, top.clone()),
            sel(split, SplitHalf::Above, top),
        ],
    );
    let ev = eval(&r.doc);
    let got = ev.nodes.get(&m.measures[0]);
    assert!(
        matches!(got, Some(NodeResult::Ok(_))),
        "the measure did not evaluate: {got:?}"
    );
}
