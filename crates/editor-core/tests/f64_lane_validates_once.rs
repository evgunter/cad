//! **At `f64` the op's value is the pre-pass's validated form — the
//! same value a validation from scratch mints.** The
//! `f64` lane is the profile's first pass (`editor_core::ProfileLane`),
//! so it runs no second one. For every Profile node of every corpus
//! document, the node's `f64` value equals what the program replayed at
//! `f64` and validated on the profile's own xy produces: the same
//! loops and every stored scalar the same bits. The evaluator makes the
//! validation decisions once (`kstats_bracket_rows` pins the log, and
//! `m10_p_lift` holds the value against the second pass run at `f64`).
//! The last rows pin what the lane scalar DOES change: a margin an
//! `Interval` validation would escalate on escalates there.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::documents;

use editor_core::{CancelToken, EvalOptions, Node, ValuePayload, evaluate};
use geom_core::Tol;
use profile::test_support::{decided_facts, stored_scalars};
use profile::{SketchPlane, ValidatedProfile};

#[test]
fn the_f64_value_is_the_revalidated_form() {
    let tol = Tol::witness();
    let mut profiles = 0usize;
    for d in documents() {
        let ev = evaluate::<f64>(
            &d.doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            tol,
        );
        let env = d.doc.var_env::<f64>();
        for &id in &ev.order {
            let Some(Node::Profile(program)) = d.doc.node(id) else {
                continue;
            };
            let value = ev
                .value(id)
                .unwrap_or_else(|| panic!("{}: profile {id:?} evaluates", d.name));
            let ValuePayload::Profile(served) = &value.payload else {
                panic!("{}: {id:?} carries a profile", d.name);
            };
            let loops = program
                .resolve(&env)
                .expect("the corpus program resolves at f64")
                .iter()
                .map(|steps| profile::replay(steps, tol).expect("replays at f64"))
                .collect();
            let revalidated = profile::ConstructedProfile::new(SketchPlane::xy(), loops)
                .validate(tol)
                .unwrap_or_else(|e| panic!("{}: {id:?} validates: {e:?}", d.name));
            assert_eq!(
                decided_facts(&served.validated),
                decided_facts(&revalidated),
                "{}: profile {id:?}: structure",
                d.name
            );
            let bits = |vp: &ValidatedProfile<f64>| {
                stored_scalars(vp)
                    .iter()
                    .map(|x| x.to_bits())
                    .collect::<Vec<_>>()
            };
            assert!(
                bits(&served.validated) == bits(&revalidated),
                "{}: profile {id:?}: the served form differs from the re-validated one:\n \
                 served {:?}\n revalidated {:?}",
                d.name,
                stored_scalars(&served.validated),
                stored_scalars(&revalidated)
            );
            profiles += 1;
        }
    }
    assert!(profiles > 20, "the corpus carries profiles: {profiles}");
}

/// **A margin definite at `f64` and indeterminate at `Interval` is
/// decided at `f64` and escalates at `Interval`.** A step of exactly
/// the escalation threshold K·ε: the tiny edge's length is the
/// axis-aligned distance `sqrt(d²)`, which at `f64` is `d` exactly —
/// the lever arm of the junction turn at its far end, whose
/// displacement margin is then exactly K·ε (`|m| ≥ K·ε`: definite by
/// equality) — and at `Interval` an enclosure whose lower bound rounds
/// below `d`, so the same predicate cannot decide. At `f64` the node is
/// served with the first pass's decisions and no escalation; at
/// `Interval` the op's own replay re-verifies the junction and refuses
/// the node with `path_junction_turn` escalated, which is where such a
/// margin is meant to escalate.
#[test]
fn a_margin_definite_at_f64_and_indeterminate_at_interval_escalates_at_interval() {
    use crate::fixture::on_frame;
    use editor_core::{DocumentId, NodeResult, ProfileDoc};
    use geom_core::{Band, Interval};
    let tol = Tol::witness();
    let d = Band::linear(tol).expect("the linear band").escalate();
    let doc = ProfileDoc::empty(DocumentId::derive("eval8-band-edge"), tol);
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, d),
            (2.0, d),
            (2.0, 1.0),
            (0.0, 1.0),
        ]],
    );
    let at_f64 = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    let served = at_f64.value(profile).expect("validates at f64");
    assert!(
        served
            .verdicts
            .iter()
            .any(|v| v.predicate == "path_junction_turn"),
        "the first pass decided the junction at f64"
    );
    assert!(
        served.escalations.is_empty(),
        "nothing escalated: {:?}",
        served.escalations
    );
    let at_interval = evaluate::<Interval>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    let err = at_interval
        .result(profile)
        .and_then(NodeResult::error)
        .expect("the op's validation at Interval refuses");
    let named: Vec<&str> = err.escalations.iter().map(|e| e.predicate()).collect();
    assert_eq!(
        named,
        ["path_junction_turn"],
        "the junction the tiny edge levers escalates at Interval: {}",
        err.kind
    );
}

/// **The default `Interval` evaluation of an extruded arc builds.** A
/// D-shape whose arc carrier, elaborated at `Interval`, has its rim and
/// radius enclose disjointly: the extrude at
/// `Interval` must place that carrier without the sweep claiming its
/// start lies on it, which the exact witness would disprove (an abort,
/// live in release).
#[test]
fn a_default_interval_evaluation_of_an_extruded_copied_arc_builds() {
    use crate::fixture::{Recorder, len, len2, scl};
    use editor_core::{LoopProgram, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget};
    use geom_core::Interval;
    let mut r = Recorder::new();
    let chain = LoopProgram::Chain(vec![
        ProgramStep::At(len2([-79.674_068_761_865_71, -8.743_422_184_344_226])),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(len2([-79.456_229_843_363_16, -7.494_651_585_468_935])),
            b: scl(1.649_230_685_601_469_6),
        }),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let profile = r.insert(Node::Profile(ProfileProgram {
        loops: vec![chain],
        ids: Vec::new(),
        on: None,
    }));
    let solid = r.insert(Node::Extrude {
        profile: profile.into(),
        distance: len(1.0),
    });
    let tol = Tol::witness();
    let at = |doc| {
        (
            evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default(), tol)
                .value(solid)
                .is_some(),
            evaluate::<Interval>(doc, None, &CancelToken::new(), &EvalOptions::default(), tol)
                .value(solid)
                .is_some(),
        )
    };
    assert_eq!(
        at(&r.doc),
        (true, true),
        "the extrude builds at f64 and at Interval"
    );
}
