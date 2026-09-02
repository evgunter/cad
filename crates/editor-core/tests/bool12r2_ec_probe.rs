//! **R2 review probes for BOOL-12's wire spelling** (PR #1573, frozen
//! head 50740f96).
//!
//! The unit adds `ProgramTarget::StartArriving` / `WireTarget::StartArriving`
//! and their two-member `Arrival` mirrors, plus content-key tags 43/44.
//! No row in the tree constructs one: `switch_program_vocabulary`'s
//! corpus carries `ProgramTarget::Start` and `ProgramTarget::Point` only
//! (its census is over `Verb::ALL`, and there is no target census), the
//! checked-in `.pncad` documents are byte-identical because none of them
//! authors the new target, and the profile-side coverage corpus never
//! reaches this crate. So `from_target` / `into_target`, the serde
//! derives on `WireArrival`, `arrival` / `arrival_lit` and the two new
//! tags are compiled but never executed.
//!
//! These rows execute them.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{LoopProgram, ProfileProgram, ProgramStep, ProgramTarget};
use geom_core::{Point2, Tol};
use profile::{Open, SketchPlane, Start};
use std::f64::consts::FRAC_PI_2;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// Evan's D-shape, whose closing leg declares BOTH facts.
fn d_shape() -> Vec<profile::Step<f64>> {
    let t = Tol::witness();
    Open.at(p2(0.0, 0.0))
        .angle(FRAC_PI_2, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .arc_to(
            profile::Bulge {
                p: p2(0.0, -2.0),
                b: 1.0,
            },
            t,
        )
        .unwrap()
        .line_to(p2(0.0, -1.0), t)
        .unwrap()
        .continue_to(Start.arrives_tangent(), t)
        .unwrap()
        .program
}

/// The stadium, whose closing cap declares the G1 arrival.
fn stadium() -> Vec<profile::Step<f64>> {
    let t = Tol::witness();
    Open.at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .tangent()
        .tangent_arc_to(p2(2.0, 2.0), t)
        .unwrap()
        .tangent()
        .line(2.0, t)
        .unwrap()
        .tangent()
        .tangent_arc_to(Start.arrives_tangent(), t)
        .unwrap()
        .program
}

/// **The lifting door reaches the new target.** Both chains lift, each
/// carrying the seam's declaration as `ProgramTarget::StartArriving` —
/// which has no payload, because there is one declaration to make.
#[test]
fn r2_the_declared_arrivals_lift_to_the_document() {
    for (name, steps) in [("d-shape", d_shape()), ("stadium", stadium())] {
        let prog = LoopProgram::from_recorded(&steps)
            .unwrap_or_else(|e| panic!("{name}: the declared arrival lifts: {e}"));
        let LoopProgram::Chain(doc) = &prog else {
            panic!("{name}: a chain program")
        };
        let found: Vec<&ProgramStep> = doc
            .iter()
            .filter(|s| {
                matches!(
                    s,
                    ProgramStep::LineTo(ProgramTarget::StartArriving)
                        | ProgramStep::ContinueTo(ProgramTarget::StartArriving)
                        | ProgramStep::TangentArcTo(ProgramTarget::StartArriving)
                )
            })
            .collect();
        assert_eq!(found.len(), 1, "{name}: {doc:?}");
        println!("R2: {name} lifts -> {found:?}");
    }
}

/// **The new target survives the wire**, which
/// `every_document_verb_survives_the_wire` cannot say because the corpus
/// it walks has no `StartArriving` in it. `ProfileProgram`'s `PartialEq`
/// is the D7 bit comparator, so this is bit-identity.
#[test]
fn r2_the_declared_arrivals_survive_the_wire() {
    for (name, steps) in [("d-shape", d_shape()), ("stadium", stadium())] {
        let LoopProgram::Chain(doc) = LoopProgram::from_recorded(&steps).expect("lifts") else {
            panic!("{name}: a chain")
        };
        let before = ProfileProgram {
            plane: SketchPlane::xy(),
            loops: vec![LoopProgram::Chain(doc)],
        };
        let text = serde_json::to_string(&before).expect("serializes");
        println!("R2: {name} wire -> {text}");
        assert!(
            text.contains("StartArriving"),
            "{name}: the arrival reaches the wire: {text}"
        );
        let after: ProfileProgram = serde_json::from_str(&text).expect("deserializes");
        assert_eq!(before, after, "{name}");
    }
}

// NOT probed here, and recorded as unprobed: the two content-key tags
// (43/44 in `eval/mod.rs`'s `feed_step`) are reachable only by
// evaluating a document node that carries the new target, which needs a
// whole `Doc`. Nothing in the tree does that either, so those two arms
// remain compiled and unexecuted.
