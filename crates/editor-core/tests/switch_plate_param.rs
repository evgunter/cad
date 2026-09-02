//! **The parametric acceptance scene's rows** (LIB-SWITCH §7,
//! PROFILES-V2 §V8).
//!
//! `plate_param` is the corpus's first document whose PROFILE geometry
//! is driven by a document parameter. These rows are the demonstration
//! §V8 asks for — "one scene gains a REAL parameter and demonstrates:
//! edit param → re-evaluate → new geometry; drive it into a refusal →
//! typed error names the step" — plus the two doors §7 adds: the
//! validate ladder as a SECOND refusal class, and the §4d
//! authoring-time door.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use corpus::plate_param::{
    HOLE_CENTRES, HOLE_R, HOLE_R_VALUE, PLATE, PLATE_DEPTH, hole_loop, plate_profile,
};
use corpus::{body_of, eval};
use editor_core::{
    Dimension, DocEdit, DocParam, EvalOutcome, Expr, Node, NodeErrorKind, NodeResult, ParamName,
    ProfileDoc, ProfilePayload, RecipeNodeId, SlotId, StepArg, apply,
};
use geom_core::Tol;
use profile::{ContactKind, PathError, ProfileError, ReplayErrorKind};
use topo::mass_properties;

/// The document, plus the ids the rows address.
struct Scene {
    doc: ProfileDoc,
    profile: RecipeNodeId,
    solid: RecipeNodeId,
}

/// The parametric plate ALONE (no tab, no union): the rows below are
/// about the profile and its parameter, and a boolean between them and
/// the refusal only adds noise. The corpus document keeps the union;
/// this is the same profile on a bare extrude.
fn scene() -> Scene {
    let doc = ProfileDoc::empty_derived("switch_plate_param", Tol::witness());
    let doc = apply(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new(HOLE_R),
            value: DocParam::continuous(Dimension::Length, HOLE_R_VALUE),
        },
        Tol::witness(),
    )
    .expect("the parameter declares")
    .doc;
    let applied = apply(
        &doc,
        &DocEdit::InsertNode {
            node: fixture::xy_frame(),
        },
        Tol::witness(),
    )
    .expect("the sketch frame inserts");
    let plane = applied.record.minted.expect("frame id");
    let applied = apply(
        &applied.doc,
        &DocEdit::InsertNode {
            node: Node::Profile(plate_profile(plane)),
        },
        Tol::witness(),
    )
    .expect("the parametric profile inserts");
    let profile = applied.record.minted.expect("profile id");
    let applied = apply(
        &applied.doc,
        &DocEdit::InsertNode {
            node: Node::Extrude {
                profile,
                distance: Expr::literal(PLATE_DEPTH, Dimension::Length).expect("depth"),
            },
        },
        Tol::witness(),
    )
    .expect("the extrude inserts");
    let solid = applied.record.minted.expect("solid id");
    Scene {
        doc: applied.doc,
        profile,
        solid,
    }
}

/// Re-point `hole_r` — the edit that must NEVER refuse at the door
/// (§4d: `SetDocParam` does not refuse for downstream profile breakage).
fn set_hole_r(doc: &ProfileDoc, value: f64) -> ProfileDoc {
    apply(
        doc,
        &DocEdit::SetDocParam {
            name: ParamName::new(HOLE_R),
            value: DocParam::continuous(Dimension::Length, value),
        },
        Tol::witness(),
    )
    .expect("SetDocParam never refuses for downstream profile breakage")
    .doc
}

/// The failure a node reported, if it failed.
fn node_error(doc: &ProfileDoc, id: RecipeNodeId) -> Option<String> {
    let ev = eval::<f64>(doc);
    match ev.nodes.iter().find(|(n, _)| **n == id).map(|(_, r)| r) {
        Some(NodeResult::Failed(e)) => Some(format!("{:?}", e.kind)),
        _ => None,
    }
}

// ------------------------------------------------------------------
// Row 1 — edit the parameter, re-evaluate, get NEW geometry
// ------------------------------------------------------------------

/// The switch's headline claim, executed: one `SetDocParam` changes a
/// profile's geometry, and changes it by the right amount.
///
/// The oracle is exact in the plate term and analytic in the holes: a
/// plate of `w·h·d` less two cylinders of radius r, so the volume moves
/// by `2·π·(r1² − r0²)·d`. The kernel's cylinder is an analytic
/// carrier, so the agreement is to a tolerance rather than to bits —
/// this row is about the DERIVATIVE being right, not about bit
/// identity (which the corpus round-trip rows already pin).
#[test]
fn a_parameter_edit_re_evaluates_into_new_geometry() {
    let s = scene();
    let (x0, x1, y0, y1) = PLATE;
    let plate = (x1 - x0) * (y1 - y0) * PLATE_DEPTH;

    let volume_at = |r: f64| {
        let doc = set_hole_r(&s.doc, r);
        let ev = eval::<f64>(&doc);
        assert_eq!(ev.outcome, EvalOutcome::Completed, "r = {r} evaluates");
        mass_properties(body_of(&ev, s.solid), Tol::witness())
            .expect("mass properties")
            .volume
    };

    let small = volume_at(HOLE_R_VALUE);
    let large = volume_at(0.4);

    // The geometry really moved.
    assert!(
        large < small,
        "growing the holes must remove material: {large} !< {small}"
    );
    // And it moved by the parametric amount, both holes together.
    let expected = |r: f64| plate - 2.0 * std::f64::consts::PI * r * r * PLATE_DEPTH;
    for (r, got) in [(HOLE_R_VALUE, small), (0.4, large)] {
        let want = expected(r);
        assert!(
            (got - want).abs() < 1e-6,
            "r = {r}: volume {got} vs oracle {want}"
        );
    }
}

/// The parameter is SHARED between the two hole loops, so one edit
/// moves both — the sharing the expression layer exists for. Measured
/// by the volume delta being twice a single hole's.
#[test]
fn one_parameter_drives_both_holes() {
    let s = scene();
    let delta = {
        let a = eval::<f64>(&set_hole_r(&s.doc, 0.2));
        let b = eval::<f64>(&set_hole_r(&s.doc, 0.3));
        mass_properties(body_of(&a, s.solid), Tol::witness())
            .expect("a")
            .volume
            - mass_properties(body_of(&b, s.solid), Tol::witness())
                .expect("b")
                .volume
    };
    let one_hole = std::f64::consts::PI * (0.3 * 0.3 - 0.2 * 0.2) * PLATE_DEPTH;
    assert!(
        (delta - 2.0 * one_hole).abs() < 1e-6,
        "delta {delta} should be two holes' worth ({})",
        2.0 * one_hole
    );
    assert_eq!(HOLE_CENTRES.len(), 2, "the scene has two holes to share");
}

// ------------------------------------------------------------------
// Row 2 — drive it into a REPLAY refusal; the error names the step
// ------------------------------------------------------------------

/// r → 0: the driver refuses `NonpositiveCircleRadius`, and the node's
/// typed error names the LOOP and the STEP (§7's row, verbatim).
///
/// Note which door this is NOT: `SetDocParam` applies cleanly. A
/// program that refuses under the current binding is legal AT REST
/// (V1 class 2) — the refusal is the evaluation's, not the edit's.
#[test]
fn a_zero_radius_refuses_at_replay_naming_the_loop_and_step() {
    let s = scene();
    let doc = set_hole_r(&s.doc, 0.0);
    let ev = eval::<f64>(&doc);

    let failed = ev
        .nodes
        .iter()
        .find(|(n, _)| **n == s.profile)
        .map(|(_, r)| r)
        .expect("the profile node has a result");
    let NodeResult::Failed(err) = failed else {
        panic!("a zero hole radius must refuse: {failed:?}");
    };
    let NodeErrorKind::ProfileReplay { loop_, error } = &err.kind else {
        panic!("expected the replay class, got {:?}", err.kind);
    };

    // The LOOP: hole 0 is loop 1 (the outline is loop 0).
    assert_eq!(*loop_, 1, "the first hole is loop 1");
    // The STEP: a circle program is exactly one step.
    assert_eq!(error.step, 0, "a Circle program's only step");
    // And the refusal is the named one.
    assert!(
        matches!(
            error.kind,
            ReplayErrorKind::Path(PathError::NonpositiveCircleRadius { .. })
        ),
        "expected NonpositiveCircleRadius, got {:?}",
        error.kind
    );
}

// ------------------------------------------------------------------
// Row 3 — drive it into a VALIDATE refusal (a different door)
// ------------------------------------------------------------------

/// r → 0.8: both holes replay fine, but they now overlap each other,
/// so the VALIDATE ladder refuses. Two refusal classes, one parameter
/// — which is the point of pinning both.
#[test]
fn an_overlapping_radius_refuses_at_validate() {
    let s = scene();
    // The separation makes 0.8 an overlap of the holes with each other
    // while both still clear the plate walls.
    let gap = HOLE_CENTRES[1].0 - HOLE_CENTRES[0].0;
    assert!(2.0 * 0.8 > gap, "0.8 must actually overlap the holes");

    let doc = set_hole_r(&s.doc, 0.8);
    let ev = eval::<f64>(&doc);
    let failed = ev
        .nodes
        .iter()
        .find(|(n, _)| **n == s.profile)
        .map(|(_, r)| r)
        .expect("the profile node has a result");
    let NodeResult::Failed(err) = failed else {
        panic!("an overlapping profile must refuse: {failed:?}");
    };

    // The VALIDATE class, typed — and NOT the replay class: both hole
    // programs elaborate fine, it is the assembled profile that is bad.
    let NodeErrorKind::Profile(ProfileError::NonSimple {
        first,
        second,
        kind,
    }) = &err.kind
    else {
        panic!("expected Profile(NonSimple), got {:?}", err.kind);
    };

    // Pin WHICH refusal: the two HOLES crossing EACH OTHER. This is what
    // makes the engineered separation load-bearing — a hole/wall
    // crossing would name loop 0 and pass a weaker assertion.
    assert_eq!(*kind, ContactKind::Crossing);
    let mut loops = [first.loop_index, second.loop_index];
    loops.sort_unstable();
    assert_eq!(
        loops,
        [1, 2],
        "the two hole loops must be the offenders, not the outline (loop 0): \
         {first} × {second}"
    );
}

// ------------------------------------------------------------------
// Row 4 — the §4d authoring door
// ------------------------------------------------------------------

/// The two doors, side by side: writing a refusing radius INTO the
/// program is refused at the edit door, while re-pointing the document
/// parameter to the same effective value is NOT — the asymmetry §4d
/// states and VQ9 ratifies.
#[test]
fn the_authoring_door_refuses_but_set_doc_param_does_not() {
    let s = scene();
    let radius_slot = SlotId::Profile {
        loop_: 1,
        step: 0,
        arg: StepArg::Radius,
    };

    // (a) Writing zero into the slot: refused at the door.
    let refused = apply(
        &s.doc,
        &DocEdit::SetParam {
            node: s.profile,
            slot: radius_slot,
            expr: Expr::literal(0.0, Dimension::Length).expect("zero literal"),
        },
        Tol::witness(),
    );
    assert!(
        refused.is_err(),
        "the authoring door must refuse a zero radius"
    );

    // (b) The same breakage via the document parameter: APPLIES, and
    //     surfaces later, at evaluation.
    let doc = set_hole_r(&s.doc, 0.0);
    assert!(
        node_error(&doc, s.profile).is_some(),
        "the refusal surfaces at evaluation instead"
    );

    // (c) And a GOOD radius goes through the same door cleanly, so (a)
    //     is a refusal about the value, not about the slot.
    let ok = apply(
        &s.doc,
        &DocEdit::SetParam {
            node: s.profile,
            slot: radius_slot,
            expr: Expr::literal(0.3, Dimension::Length).expect("literal"),
        },
        Tol::witness(),
    )
    .expect("a well-formed radius applies");
    let ev = eval::<f64>(&ok.doc);
    assert_eq!(ev.outcome, EvalOutcome::Completed);
}

/// The profile node really does expose the parametric slots (§5 item 3:
/// profile nodes flip from slot-free), and the hole radii are among
/// them.
#[test]
fn the_hole_radii_are_addressable_slots() {
    let s = scene();
    let node = s.doc.node(s.profile).expect("the profile node");
    let Node::Profile(program) = node else {
        panic!("expected a profile node");
    };
    for loop_ in 1..=2 {
        let slot = SlotId::Profile {
            loop_,
            step: 0,
            arg: StepArg::Radius,
        };
        assert!(
            program.slots().contains(&slot),
            "loop {loop_}'s radius should be addressable"
        );
    }
    // Sanity: the helper builds the same loop the document carries.
    assert_eq!(
        program.loops.get(1),
        Some(&hole_loop(HOLE_CENTRES[0])),
        "the hole loop is the shared-parameter circle"
    );
}
