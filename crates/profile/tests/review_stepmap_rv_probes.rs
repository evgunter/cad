//! **Review probes (`stepmap-rv`) for the per-step segment span.**
//!
//! The unit's own lying-span row is authored on a CHAIN. The carrier
//! forms are the case the check's placement at `replay_guided` exists
//! for, and nothing exercised them, so this row does.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{p2, tol};
use profile::{
    Decision, DecisionValue, PathError, ReplayErrorKind, StepSpan, StructureRefusalKind,
    replay_guided, replay_recording,
};

/// **A lying span on a `circle_split` record refuses typed, naming the
/// step** — the carrier form the check was moved out of the guide for.
#[test]
fn a_lying_step_span_on_a_carrier_form_refuses_typed() {
    let program = vec![profile::Step::CircleSplit {
        centre: p2(0.0, 0.0),
        radius: 1.0,
        n: 4,
        phase: 0.0,
    }];
    let (loop_, structure) = replay_recording(&program, tol()).expect("the carrier replays");
    assert_eq!(structure.steps.len(), 1, "one authored step");
    assert_eq!(
        structure.steps[0],
        StepSpan::new(0, loop_.vertices().len()),
        "the carrier's one step reaches every segment"
    );
    let mut lie = structure.clone();
    lie.steps[0] = StepSpan::new(0, loop_.vertices().len() - 1);
    let err = replay_guided(&program, &lie, tol()).expect_err("the span is contradicted");
    let ReplayErrorKind::Path(PathError::Structure(refusal)) = err.kind else {
        panic!("expected a structure refusal, got {:?}", err.kind);
    };
    assert_eq!(refusal.decision, Decision::StepSpan { step: 0 });
    assert!(matches!(
        refusal.kind,
        StructureRefusalKind::Flipped {
            recorded: DecisionValue::Span(_),
            found: DecisionValue::Span(_),
        }
    ));
}
