//! **A chain's shape draws what its replay draws.** For every chain of
//! the census corpus and a set of fillet arrangements (every fillet verb
//! among the chains compared whole), the pieces
//! `chain_shape` says the structure draws are the replay record's, in
//! order, where every fillet's runs fit `Positive`; where one fits
//! `Zero` the record draws a subset, and what it leaves out is that
//! fillet's run in or run out.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::coverage_corpus;
use geom_core::{Point2, Tol};
use profile::{ArcData, PieceRole, ReplayStructure, Step, Target, chain_shape, replay_recording};
use std::f64::consts::FRAC_PI_2;

fn arrangements() -> Vec<(&'static str, Vec<Step<f64>>)> {
    let p = Point2::new;
    let lt = |x, y| Step::LineTo(Target::Point(p(x, y)));
    let fillet = |r| Step::Fillet { radius: r };
    let up = Step::Toward { dx: 0.0, dy: 1.0 };
    let east = Step::Toward { dx: 1.0, dy: 0.0 };
    vec![
        // The far-end anchor's own segment is the run out.
        (
            "far end",
            vec![
                Step::At(p(0.0, 0.0)),
                east,
                fillet(0.5),
                up,
                Step::FarEndTo(p(2.0, 2.0)),
                lt(0.0, 2.0),
                Step::LineTo(Target::Start),
            ],
        ),
        // Both runs fit Zero: neither is drawn, nor is the far end's leg.
        (
            "far end, exact fit",
            vec![
                Step::At(p(0.0, 0.0)),
                east,
                fillet(2.0),
                up,
                Step::FarEndTo(p(2.0, 2.0)),
                lt(0.0, 2.0),
                Step::LineTo(Target::Start),
            ],
        ),
        // A riding arrival: the next straight leg is the run out.
        (
            "rides on",
            vec![
                Step::At(p(0.0, 0.0)),
                Step::Angle(0.0),
                fillet(1.0),
                Step::At(p(10.0, 10.0)),
                Step::Angle(FRAC_PI_2),
                Step::Line(3.0),
                lt(0.0, 13.0),
                Step::LineTo(Target::Start),
            ],
        ),
        // The arrival bound by `line_to` from its anchor: that leg is the run out.
        (
            "line_to arrival",
            vec![
                Step::At(p(0.0, 0.0)),
                Step::Angle(0.0),
                fillet(1.0),
                Step::At(p(10.0, 5.0)),
                lt(10.0, 10.0),
                lt(0.0, 10.0),
                Step::LineTo(Target::Start),
            ],
        ),
        // Two fillets in a row: the second's run in is the first's run out.
        (
            "two fillets",
            vec![
                Step::At(p(0.0, 0.0)),
                Step::Angle(0.0),
                fillet(1.0),
                Step::At(p(10.0, 5.0)),
                Step::Angle(FRAC_PI_2),
                fillet(1.0),
                Step::At(p(0.0, 10.0)),
                Step::Angle(core::f64::consts::PI),
                Step::Line(1.0),
                Step::LineTo(Target::Start),
            ],
        ),
        // A fillet on an arc leg's end runs in along its tangent ray.
        (
            "arc leg, fillet",
            vec![
                Step::At(p(0.0, 0.0)),
                Step::ArcTo(ArcData::Bulge {
                    target: Target::Point(p(10.0, 0.0)),
                    b: 0.2,
                }),
                fillet(1.0),
                up,
                Step::FarEndTo(p(12.0, 10.0)),
                lt(0.0, 10.0),
                Step::LineTo(Target::Start),
            ],
        ),
        // A fillet at the seam is the closing segment; it draws no run out.
        (
            "seam",
            vec![
                Step::At(p(4.0, 0.0)),
                lt(10.0, 0.0),
                lt(10.0, 10.0),
                lt(5.0, 5.0),
                fillet(1.0),
                Step::CloseTo,
            ],
        ),
        (
            "continue",
            vec![
                Step::At(p(0.0, 0.0)),
                lt(10.0, 0.0),
                Step::ContinueTo(Target::Point(p(20.0, 0.0))),
                lt(20.0, 10.0),
                Step::LineTo(Target::Start),
            ],
        ),
    ]
}

/// Whether every fillet the record resolved fits with length to spare
/// on both sides.
fn generic(record: &ReplayStructure) -> bool {
    record
        .fillets
        .iter()
        .all(|f| f.fit_in == geom_core::Sign::Positive && f.fit_out == geom_core::Sign::Positive)
}

#[test]
fn a_chains_shape_draws_what_its_replay_draws() {
    let mut chains: Vec<(String, Vec<Step<f64>>)> = coverage_corpus()
        .into_iter()
        .enumerate()
        .map(|(i, closed)| (format!("corpus {i}"), closed.program))
        .collect();
    chains.extend(arrangements().into_iter().map(|(n, s)| (n.to_owned(), s)));
    let (mut compared, mut subsets) = (0, 0);
    let mut whole: Vec<profile::Verb> = Vec::new();
    for (name, steps) in &chains {
        let forms: Vec<profile::StepForm> = steps.iter().map(Step::form).collect();
        let shape =
            chain_shape(&forms).unwrap_or_else(|f| panic!("{name}: the shape refuses {f:?}"));
        let (_, record) = replay_recording(steps, Tol::witness())
            .unwrap_or_else(|e| panic!("{name} does not replay: {e}"));
        let drawn: Vec<_> = record.pieces.clone();
        if generic(&record) {
            assert_eq!(
                drawn, shape.pieces,
                "{name}: the shape's pieces are the record's"
            );
            compared += 1;
            whole.extend(steps.iter().map(Step::verb));
            continue;
        }
        subsets += 1;
        let mut rest = shape.pieces.iter();
        for piece in &drawn {
            assert!(
                rest.any(|p| p == piece),
                "{name}: the record draws {piece:?}, which the shape does not, or out of order: \
                 {drawn:?} against {:?}",
                shape.pieces,
            );
        }
        for piece in &shape.pieces {
            assert!(
                drawn.contains(piece) || matches!(piece.role, PieceRole::RunIn | PieceRole::RunOut),
                "{name}: the record leaves out {piece:?}, which no fit suppresses",
            );
        }
    }
    assert!(
        compared >= 10 && subsets >= 1,
        "{compared} compared whole, {subsets} as subsets"
    );
    for verb in [
        profile::Verb::Fillet,
        profile::Verb::FilletArc,
        profile::Verb::ArcFillet,
        profile::Verb::ArcFilletArc,
    ] {
        assert!(
            whole.contains(&verb),
            "a chain compared whole holds {verb:?}"
        );
    }
}
