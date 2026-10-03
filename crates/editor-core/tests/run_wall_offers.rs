//! **A program edit joins or splits a run, and N3 offers across it.**
//! Runs are decided on the values (`names/README.md`, N1 "Swept walls
//! over a run"), so a value edit can move which pieces one wall holds:
//! a station inserted on a side joins its two pieces into one run wall,
//! and bending them apart splits it. The vanished name is offered the
//! wall that covers it, or the walls that cover its pieces — both
//! extrusion senses, since a reversed extrusion lists its walls in the
//! other order. And the run names are the same at both scalar types.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    DocEdit, EntityKind, LoopProgram, Node, PieceRun, ProfileDoc, ProfileEdgeRef, ProfileProgram,
    ProgramStep, ProgramTarget, RecipeNodeId, Resolution, RoleSeg, RunCtx, StableName, StepId,
    apply, resolve,
};
use geom_core::Tol;

use crate::fixture::{frame, insert, len, len2, minted, piece, run};

fn to(x: f64, y: f64) -> ProgramTarget {
    ProgramTarget::Point(len2([x, y]))
}

fn build(steps: Vec<ProgramStep>, d: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("run_wall_offers", Tol::witness());
    let (doc, plane) = insert(doc, frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let (doc, p) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::Chain(steps)],
            ids: Vec::new(),
        }),
    );
    let (doc, ex) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(d),
        },
    );
    (doc, p, ex)
}

fn ids(doc: &ProfileDoc, p: RecipeNodeId) -> Vec<StepId> {
    match doc.node(p) {
        Some(Node::Profile(pp)) => pp.ids[0].clone(),
        other => panic!("not a profile: {other:?}"),
    }
}

fn edit(
    doc: &ProfileDoc,
    p: RecipeNodeId,
    steps: Vec<ProgramStep>,
    ids: Vec<Option<StepId>>,
) -> ProfileDoc {
    apply(
        doc,
        &DocEdit::SetProgram {
            node: p,
            loops: vec![LoopProgram::Chain(steps)],
            ids: vec![ids],
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the program edit applies")
    .doc
}

fn lateral(ex: RecipeNodeId, pieces: Vec<ProfileEdgeRef>) -> StableName {
    minted(
        EntityKind::Face,
        ex,
        RoleSeg::Lateral(PieceRun::new(pieces).unwrap()),
    )
}

/// The offers on `name`'s failed resolution; panics if it resolves.
fn offers(doc: &ProfileDoc, name: &StableName) -> Vec<StableName> {
    let ev = run(doc, &Default::default());
    match resolve(RunCtx { doc, eval: &ev }, name) {
        Resolution::Failed(f) => f.offers,
        other => panic!("{name:?} should vanish, resolved {other:?}"),
    }
}

#[test]
fn a_station_joining_or_splitting_a_run_offers_across_it() {
    for d in [1.0, -1.0] {
        let plain = vec![
            ProgramStep::At(len2([0.0, 0.0])),
            ProgramStep::LineTo(to(2.0, 0.0)),
            ProgramStep::LineTo(to(2.0, 2.0)),
            ProgramStep::LineTo(to(0.0, 2.0)),
            ProgramStep::LineTo(ProgramTarget::Start),
        ];
        let (doc, p, ex) = build(plain, d);
        let held = lateral(ex, vec![piece(&doc, ex, 0, 0)]);
        let i = ids(&doc, p);
        // A station on the bottom side: its two pieces are one run.
        let joined = vec![
            ProgramStep::At(len2([0.0, 0.0])),
            ProgramStep::LineTo(to(1.0, 0.0)),
            ProgramStep::ContinueTo(to(2.0, 0.0)),
            ProgramStep::LineTo(to(2.0, 2.0)),
            ProgramStep::LineTo(to(0.0, 2.0)),
            ProgramStep::LineTo(ProgramTarget::Start),
        ];
        let doc2 = edit(
            &doc,
            p,
            joined,
            vec![
                Some(i[0]),
                Some(i[1]),
                None,
                Some(i[2]),
                Some(i[3]),
                Some(i[4]),
            ],
        );
        let run_wall = lateral(ex, vec![piece(&doc2, ex, 0, 0), piece(&doc2, ex, 0, 1)]);
        assert!(
            offers(&doc2, &held).contains(&run_wall),
            "d={d}: the one-piece wall is offered the run wall that covers it"
        );
        // Bend the station off the line: the run splits.
        let split = vec![
            ProgramStep::At(len2([0.0, 0.0])),
            ProgramStep::LineTo(to(1.0, 0.0)),
            ProgramStep::LineTo(to(2.0, 0.3)),
            ProgramStep::LineTo(to(2.0, 2.0)),
            ProgramStep::LineTo(to(0.0, 2.0)),
            ProgramStep::LineTo(ProgramTarget::Start),
        ];
        let doc3 = edit(
            &doc2,
            p,
            split,
            match doc2.node(p) {
                Some(Node::Profile(pp)) => pp.kept_in_place().remove(0),
                other => panic!("not a profile: {other:?}"),
            },
        );
        let got = offers(&doc3, &run_wall);
        for k in 0..2 {
            let wall = lateral(ex, vec![piece(&doc3, ex, 0, k)]);
            assert!(
                got.contains(&wall),
                "d={d}: the broken run offers piece {k}'s wall"
            );
        }
    }
}

#[test]
fn run_names_agree_across_scalar_types() {
    use geom_core::Interval;
    for (label, steps, revolve) in [
        (
            "oblique run, extruded down",
            vec![
                ProgramStep::At(len2([0.0, 0.0])),
                ProgramStep::LineTo(to(1.0, 0.7)),
                ProgramStep::ContinueTo(to(2.0, 1.4)),
                ProgramStep::LineTo(to(0.5, 3.0)),
                ProgramStep::LineTo(ProgramTarget::Start),
            ],
            false,
        ),
        (
            "run on the axis-normal base, revolved full",
            vec![
                ProgramStep::At(len2([0.0, 0.0])),
                ProgramStep::LineTo(to(0.7, 0.0)),
                ProgramStep::ContinueTo(to(2.0, 0.0)),
                ProgramStep::LineTo(to(2.0, 2.0)),
                ProgramStep::LineTo(to(0.0, 2.0)),
                ProgramStep::LineTo(ProgramTarget::Start),
            ],
            true,
        ),
    ] {
        let doc = ProfileDoc::empty_derived("run_wall_offers_iv", Tol::witness());
        let (doc, plane) = insert(doc, frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
        let (doc, p) = insert(
            doc,
            Node::Profile(ProfileProgram {
                plane,
                loops: vec![LoopProgram::Chain(steps)],
                ids: Vec::new(),
            }),
        );
        let (doc, n) = if revolve {
            let (doc, axis) = insert(
                doc,
                crate::fixture::axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)),
            );
            insert(
                doc,
                Node::Revolve {
                    profile: p,
                    axis,
                    angle: crate::fixture::ang(std::f64::consts::TAU),
                },
            )
        } else {
            insert(
                doc,
                Node::Extrude {
                    profile: p,
                    distance: len(-1.0),
                },
            )
        };
        let ev = run(&doc, &Default::default());
        let evi = editor_core::evaluate::<Interval>(
            &doc,
            None,
            &editor_core::CancelToken::new(),
            &Default::default(),
            Tol::witness(),
        );
        let names = |t: &editor_core::NameTable| {
            let mut v: Vec<String> = t.iter().map(|(k, _)| format!("{k:?}")).collect();
            v.sort();
            v
        };
        let a = names(&ev.value(n).expect("f64 value").name_table);
        let b = names(&evi.value(n).expect("interval value").name_table);
        assert!(
            a.iter()
                .any(|x| x.contains("Lateral([") || x.contains("Band([")),
            "{label}: a run name is minted"
        );
        assert_eq!(a, b, "{label}: the two scalar types mint the same names");
    }
}
