//! **A parametric living-in-it story**: one user, one sitting, one
//! little lighthouse — three coaxial drums, each pierced a few
//! millimetres into the one below — built through the session's op
//! vocabulary and then LIVED IN parametrically: every proportion hangs
//! off four document parameters, and the session edits the part by
//! moving those numbers rather than by touching geometry again.
//!
//! The walk covers the whole parametric surface as one continuous
//! session: the create/replace param partition (`CreateParam` refuses
//! an existing name typed, `SetParam` refuses an absent one),
//! expressions driving slots (`SetSlotExpression`) and the ratified
//! refuse-with-affordance on a numeric write to a driven slot, display
//! units as a separate door from values (`SetSlotUnit`), slider
//! gestures over a slot and over a parameter (previews against
//! scratch, exactly one undo step, a cancel leaving no trace), a
//! locally-valid-range probe (`ProbeBounds`) and its discard on the
//! next edit, param edits rippling through evaluation against a closed
//! form, tree-shaped undo history (an edit after an undo mints a
//! sibling), and a save/reopen round trip that keeps the expressions,
//! the parameters and the written units.
//!
//! One test, deliberately, following `assembly_walk`: the walk's
//! readability is part of what it asserts — each stage is a numbered
//! block whose assertions say what the stage claims.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use core::f64::consts::PI;

use common::{ang, body_volume, insert, len, len3, near, scl3, shape};
use pncad::document::{
    Axis3, BooleanOp, Dimension, Doc, DocEdit, DocParam, ParamName, ProfileProgram, RecipeNodeId,
    SlotId, StepArg,
};
use pncad::geom_core::Tol;
use pncad::prelude::MM;
use viewer::bounds::BoundsProbe;
use viewer::props::{self, SlotDriver, SlotValue, in_written, rendering_unit};
use viewer::session::{BoundsTarget, DocSession, ProfileShape, Refusal, SessionOp};
use viewer::tree::RowStatus;

/// The lighthouse's proportions, as the parameters are first declared:
/// base radius, tier ratio, tier height, and how deep each drum sinks
/// into the one below (what keeps every union seam transversal).
const BASE_R: f64 = 0.05;
const TAPER: f64 = 0.6;
const HEIGHT: f64 = 0.03;
const EMBED: f64 = 0.005;
/// The lamp room's height — the one dimension of the part that is NOT
/// parameter-driven, kept literal so the walk has a slot that carries
/// a display unit and a slider of its own.
const LAMP_H: f64 = 0.012;

/// The closed-form volume of the whole lighthouse. Each drum is a
/// cylinder; each union loses exactly one embed-deep slice of the
/// upper drum (the upper drum is narrower than the drum it sinks
/// into, so the overlap is a cylinder of the upper drum's radius).
fn lighthouse_volume(base_r: f64, taper: f64, height: f64, embed: f64, lamp_h: f64) -> f64 {
    let r0 = base_r;
    let r1 = base_r * taper;
    let r2 = base_r * taper * taper;
    PI * (r0 * r0 * height + r1 * r1 * (2.0 * height) + r2 * r2 * lamp_h
        - r1 * r1 * embed
        - r2 * r2 * embed)
}

/// Replace one slot's expression from source text, asserting the door
/// committed exactly one edit.
fn drive(session: &mut DocSession, node: RecipeNodeId, slot: SlotId, text: &str) {
    let outcome = session.perform(SessionOp::SetSlotExpression {
        node,
        slot,
        text: text.to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{text:?}: {:?}", outcome.refusal);
    assert_eq!(
        outcome.committed.len(),
        1,
        "one committed edit for {text:?}"
    );
}

/// The one slot row `node` carries for `slot`.
fn row_of(doc: &Doc<ProfileProgram>, node: RecipeNodeId, slot: SlotId) -> props::SlotRow {
    props::slot_rows(doc, node)
        .into_iter()
        .find(|row| row.slot == slot)
        .expect("the node carries the slot")
}

/// A circle-template profile's radius slot, found by shape rather than
/// spelled by hand — the finding is itself an assertion that the
/// circle lowering exposes its radius as an editable slot.
fn radius_slot(doc: &Doc<ProfileProgram>, profile: RecipeNodeId) -> SlotId {
    props::slot_rows(doc, profile)
        .into_iter()
        .find_map(|row| match row.slot {
            SlotId::Profile {
                arg: StepArg::Radius,
                ..
            } => Some(row.slot),
            _ => None,
        })
        .expect("a circle profile carries a radius slot")
}

/// A declared parameter's stored value.
fn param_of(doc: &Doc<ProfileProgram>, name: &ParamName) -> SlotValue {
    props::param_rows(doc)
        .into_iter()
        .find(|row| row.name == *name)
        .expect("the parameter is declared")
        .value
}

/// One drum: a circle profile on world XY driven by `radius_expr`,
/// extruded up by a literal the caller may re-drive afterwards.
///
/// Stacked extruded drums rather than one revolved silhouette,
/// because `ProfileShape` spells no revolvable silhouette — the
/// template poverty issue 1457 tracks.
fn drum(
    session: &mut DocSession,
    radius_expr: &str,
    distance: f64,
) -> (RecipeNodeId, RecipeNodeId) {
    let plane = common::xy_frame_in(session);
    let profile = insert(
        session,
        SessionOp::AddProfile {
            plane,
            loops: vec![shape(&ProfileShape::Circle {
                // A positive placeholder; the expression takes over
                // before anything downstream consumes it.
                centre: [0.0, 0.0],
                radius: 0.01,
            })],
        },
    );
    let radius = radius_slot(session.committed_doc(), profile);
    drive(session, profile, radius, radius_expr);
    let extrude = insert(
        session,
        SessionOp::AddExtrude {
            profile,
            distance: len(distance),
        },
    );
    (profile, extrude)
}

#[test]
fn the_parametric_living_walk() {
    let tol = Tol::witness();
    let mut session = DocSession::inline(Doc::empty_derived("parametric-start", tol), tol);

    // ── 1. A fresh document through the New door: whatever was open
    // is gone, and there is nothing to draw yet.
    let outcome = session.perform(SessionOp::NewDocument {
        name: "story-lighthouse".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(session.tree_rows().is_empty(), "an empty document");
    assert_eq!(session.history().len(), 1, "a fresh history: root only");

    // ── 2. The user declares the proportions FIRST — four parameters,
    // each one committed `SetDocParam` edit and one undo step.
    let base_r = ParamName::new("base_r");
    let taper = ParamName::new("taper");
    let height = ParamName::new("height");
    let embed = ParamName::new("embed");
    for (name, param) in [
        (&base_r, DocParam::continuous(Dimension::Length, BASE_R)),
        (&taper, DocParam::continuous(Dimension::Scalar, TAPER)),
        (&height, DocParam::continuous(Dimension::Length, HEIGHT)),
        (&embed, DocParam::continuous(Dimension::Length, EMBED)),
    ] {
        let before = session.history().len();
        let outcome = session.perform(SessionOp::CreateParam {
            name: name.clone(),
            value: param,
        });
        assert!(outcome.refusal.is_none(), "{name:?}: {:?}", outcome.refusal);
        assert_eq!(outcome.committed.len(), 1);
        assert!(
            matches!(outcome.committed.first(), Some(DocEdit::SetDocParam { .. })),
            "the create door authors a declaration"
        );
        assert_eq!(session.history().len(), before + 1, "one undo step");
    }
    assert_eq!(props::param_rows(session.committed_doc()).len(), 4);
    assert_eq!(
        param_of(session.committed_doc(), &taper),
        SlotValue::Continuous(TAPER)
    );

    // ── 3. The two doors partition the edit's semantics, typed.
    // Create over a declared name refuses `ParamExists` carrying what
    // already stands there; a value write to an undeclared name — here
    // a typo — refuses `NoSuchParam`. Neither commits or mints history.
    let before = session.history().len();
    let outcome = session.perform(SessionOp::CreateParam {
        name: taper.clone(),
        value: DocParam::continuous(Dimension::Length, 1.0),
    });
    match outcome.refusal {
        Some(Refusal::ParamExists {
            ref name,
            dimension,
        }) => {
            assert_eq!(name, &taper);
            assert_eq!(
                dimension,
                Dimension::Scalar,
                "the payload carries the EXISTING declaration's dimension"
            );
        }
        ref other => panic!("expected ParamExists, got {other:?}"),
    }
    assert!(outcome.committed.is_empty(), "a refusal commits nothing");
    let outcome = session.perform(SessionOp::SetParam {
        name: ParamName::new("tapper"),
        value: SlotValue::Continuous(0.5),
    });
    match outcome.refusal {
        Some(Refusal::NoSuchParam(ref name)) => assert_eq!(name.0, "tapper"),
        ref other => panic!("expected NoSuchParam, got {other:?}"),
    }
    assert!(outcome.committed.is_empty());
    assert_eq!(session.history().len(), before, "and mints no history");

    // ── 4. The base drum: a circle profile whose radius is DRIVEN by
    // `base_r`, extruded by `height`. The slot rows say who drives
    // what, and the evaluated body is the closed-form cylinder.
    // The literal 0.03 deliberately equals `height`'s value, so the
    // part is coherent even mid-build; were the expression below to
    // silently not take over, the stage-9/11 parameter ripples would
    // catch the literal standing still.
    let (base_profile, base) = drum(&mut session, "base_r", 0.03);
    drive(&mut session, base, SlotId::Distance, "height");
    let radius_row = row_of(
        session.committed_doc(),
        base_profile,
        radius_slot(session.committed_doc(), base_profile),
    );
    assert_eq!(
        radius_row.driver,
        SlotDriver::Expression {
            params: vec![base_r.clone()]
        },
        "the radius names its driving parameter"
    );
    assert_eq!(radius_row.value, Ok(SlotValue::Continuous(BASE_R)));
    let got = body_volume(&mut session, base, tol);
    let want = PI * BASE_R * BASE_R * HEIGHT;
    assert!(near(got, want), "base drum volume {got} vs {want}");

    // ── 5. The tower, twice the tier height and `taper` times the
    // radius, sunk `embed` into the base so the union seam is the
    // tower wall crossing the base's top cap transversally.
    let (_tower_profile, tower) = drum(&mut session, "base_r * taper", 0.055);
    drive(&mut session, tower, SlotId::Distance, "height * 2.0");
    let tower_up = insert(
        &mut session,
        SessionOp::AddTransform {
            input: tower,
            // 0.025 deliberately equals `height - embed` today, so the
            // part is coherent before the drive lands; a literal that
            // stayed driving would be caught by the stage-9/11 ripples.
            translation: len3([0.0, 0.0, 0.025]),
            rotation_axis: scl3([0.0, 0.0, 1.0]),
            rotation_angle: ang(0.0),
        },
    );
    drive(
        &mut session,
        tower_up,
        SlotId::Translation(Axis3::Z),
        "height - embed",
    );
    let hull = insert(
        &mut session,
        SessionOp::AddBoolean {
            op: BooleanOp::Union,
            a: base,
            b: tower_up,
        },
    );
    let r1 = BASE_R * TAPER;
    let got = body_volume(&mut session, hull, tol);
    let want = PI * (BASE_R * BASE_R * HEIGHT + r1 * r1 * (2.0 * HEIGHT) - r1 * r1 * EMBED);
    assert!(near(got, want), "hull volume {got} vs {want}");

    // ── 6. The lamp room on top — its radius driven by the square of
    // the taper, its HEIGHT the part's one literal dimension, authored
    // in millimetres: the unit door changes how the number is WRITTEN
    // and leaves the canonical value bit-identical.
    let (_lamp_profile, lamp) = drum(&mut session, "base_r * taper * taper", LAMP_H);
    let before_unit = row_of(session.committed_doc(), lamp, SlotId::Distance);
    assert_eq!(
        before_unit.unit.map(|u| u.symbol()),
        Some("m"),
        "authored canonically, which is to say IN METRES — said, not left to a reader"
    );
    let outcome = session.perform(SessionOp::SetSlotUnit {
        node: lamp,
        slot: SlotId::Distance,
        unit: MM.def(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1, "a unit change is one edit");
    let after_unit = row_of(session.committed_doc(), lamp, SlotId::Distance);
    assert_eq!(after_unit.unit, Some(MM.def()), "written in mm now");
    assert_eq!(
        after_unit
            .value
            .clone()
            .expect("a value")
            .as_f64()
            .to_bits(),
        before_unit.value.expect("a value").as_f64().to_bits(),
        "the notation moved; the canonical value did not"
    );
    assert_eq!(
        in_written(
            after_unit.value.expect("a value").as_f64(),
            rendering_unit(after_unit.dimension, after_unit.unit).expect("a length row"),
        ),
        12.0,
        "shown as twelve millimetres"
    );
    let lamp_up = insert(
        &mut session,
        SessionOp::AddTransform {
            input: lamp,
            // 0.08 deliberately equals `height * 3 - embed * 2` today
            // (coherent mid-build); a literal that stayed driving would
            // be caught by the stage-9/11 parameter ripples.
            translation: len3([0.0, 0.0, 0.08]),
            rotation_axis: scl3([0.0, 0.0, 1.0]),
            rotation_angle: ang(0.0),
        },
    );
    drive(
        &mut session,
        lamp_up,
        SlotId::Translation(Axis3::Z),
        "height * 3.0 - embed * 2.0",
    );
    let lighthouse = insert(
        &mut session,
        SessionOp::AddBoolean {
            op: BooleanOp::Union,
            a: hull,
            b: lamp_up,
        },
    );
    assert_eq!(
        session.committed_doc().roots(),
        &[lighthouse],
        "one product root: the finished part"
    );
    let got = body_volume(&mut session, lighthouse, tol);
    let want = lighthouse_volume(BASE_R, TAPER, HEIGHT, EMBED, LAMP_H);
    assert!(near(got, want), "lighthouse volume {got} vs {want}");
    let rows = session.tree_rows();
    assert_eq!(rows.len(), 10, "ten features, top to bottom");
    assert!(
        rows.iter().all(|row| row.status == RowStatus::Ok),
        "every row green: {rows:?}"
    );

    // ── 7. Living in it, first touch: a direct number onto the
    // tower's driven distance refuses with the ratified affordance —
    // the typed payload names the driving parameter and the value the
    // slot has today, and the refusal commits nothing.
    let before = session.history().len();
    let outcome = session.perform(SessionOp::SetSlot {
        node: tower,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(0.1),
    });
    match outcome.refusal {
        Some(Refusal::DrivenByExpression {
            node,
            slot,
            ref params,
            current,
        }) => {
            assert_eq!(node, tower);
            assert_eq!(slot, SlotId::Distance);
            assert_eq!(params, &vec![height.clone()], "the affordance's target");
            assert_eq!(
                current,
                Some(SlotValue::Continuous(HEIGHT * 2.0)),
                "and the value the expression computes today"
            );
        }
        ref other => panic!("expected the driven refusal, got {other:?}"),
    }
    assert!(outcome.committed.is_empty(), "a refusal commits nothing");
    assert_eq!(session.history().len(), before, "and mints no history");

    // ── 8. About to move `taper`, the user asks how far it can go.
    // The probe targets the PARAMETER, not a driven slot: on an
    // expression-driven slot the probe lacks the `DrivenByExpression`
    // guard its sibling doors have (issue 1458), so the parameter door
    // is the one that behaves. The probe is explicit, commits nothing,
    // and answers in the session: below, a wall exists (a drum's radius hits zero before
    // taper does — degenerate profile), so the low side is an EDGE
    // strictly inside (0, taper); above, the drums merely re-stack, so
    // the search finds real room past the first reach. A probe on an
    // undeclared parameter refuses through the same typed door as the
    // value write.
    let outcome = session.perform(SessionOp::ProbeBounds {
        target: BoundsTarget::Param {
            name: ParamName::new("tapper"),
        },
    });
    assert!(matches!(outcome.refusal, Some(Refusal::NoSuchParam(_))));
    let before = session.history().len();
    let outcome = session.perform(SessionOp::ProbeBounds {
        target: BoundsTarget::Param {
            name: taper.clone(),
        },
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(outcome.committed.is_empty(), "a probe commits nothing");
    assert_eq!(session.history().len(), before, "and mints no history");
    let (target, bounds) = session.bounds().expect("the probe landed").clone();
    assert_eq!(
        target,
        BoundsTarget::Param {
            name: taper.clone()
        }
    );
    assert_eq!(bounds.origin, TAPER, "searched from where the field is");
    assert!(
        bounds.low.is_edge(),
        "a zero-radius drum walls off the low side: {:?}",
        bounds.low
    );
    let floor = bounds.low.limit();
    assert!(
        floor > 0.0 && floor < TAPER,
        "the furthest valid taper down is strictly inside (0, {TAPER}): {floor}"
    );
    assert!(
        bounds.high.limit() >= TAPER + 1.0,
        "real room upward, at least one canonical seed out: {:?}",
        bounds.high
    );
    assert!(
        bounds.samples > 0 && bounds.samples <= BoundsProbe::MAX_SAMPLES,
        "the cost bound holds: {}",
        bounds.samples
    );

    // ── 9. The ripple: one parameter moves and the whole part
    // follows. The probe's answer — a statement about the OLD document
    // — is discarded by the edit; the volume tracks the closed form;
    // one undo restores the old geometry exactly; redo returns.
    let v_before = body_volume(&mut session, lighthouse, tol);
    let outcome = session.perform(SessionOp::SetParam {
        name: taper.clone(),
        value: SlotValue::Continuous(0.7),
    });
    assert_eq!(outcome.committed.len(), 1);
    assert!(matches!(
        outcome.committed.first(),
        Some(DocEdit::SetDocParamValue { .. })
    ));
    assert!(
        session.bounds().is_none(),
        "the probe's answer dies with the document it was about"
    );
    let v_wide = body_volume(&mut session, lighthouse, tol);
    let want = lighthouse_volume(BASE_R, 0.7, HEIGHT, EMBED, LAMP_H);
    assert!(near(v_wide, want), "widened volume {v_wide} vs {want}");
    assert!(v_wide > v_before, "a fatter taper is more lighthouse");

    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert_eq!(
        param_of(session.committed_doc(), &taper),
        SlotValue::Continuous(TAPER),
        "undo returns the parameter"
    );
    let v_back = body_volume(&mut session, lighthouse, tol);
    assert_eq!(
        v_back.to_bits(),
        v_before.to_bits(),
        "…and the old geometry, exactly"
    );
    assert!(session.perform(SessionOp::Redo).refusal.is_none());
    assert_eq!(
        param_of(session.committed_doc(), &taper),
        SlotValue::Continuous(0.7),
        "redo walks forward along the same branch"
    );
    assert!(session.perform(SessionOp::Undo).refusal.is_none());

    // ── 10. A slider drag on the lamp's height — the mm slot. Several
    // previews evaluate against scratch (the committed document does
    // not move), the release commits EXACTLY one edit and one undo
    // step, and the literal keeps the display unit the drag opened
    // with. Because stage 9 left the cursor one step behind a redo
    // branch, this commit also demonstrates the tree: it mints a
    // SIBLING and destroys nothing.
    let fork = session.history().current();
    let abandoned = session
        .history()
        .entry(fork)
        .active_child()
        .expect("the taper-0.7 branch is still redoable");
    let before = session.history().len();
    assert!(
        session
            .perform(SessionOp::BeginGesture {
                node: lamp,
                slot: SlotId::Distance,
            })
            .refusal
            .is_none()
    );
    let mut previews = 0usize;
    for value in [0.014, 0.016, 0.013] {
        let outcome = session.perform(SessionOp::PreviewGesture { value });
        assert!(outcome.committed.is_empty(), "a preview commits nothing");
        previews += outcome.previewed.len();
        assert_eq!(
            session.history().len(),
            before,
            "history untouched mid-drag"
        );
    }
    assert_eq!(previews, 3);
    assert_eq!(
        row_of(session.doc(), lamp, SlotId::Distance).value,
        Ok(SlotValue::Continuous(0.013)),
        "the panels show the scratch value"
    );
    assert_eq!(
        row_of(session.committed_doc(), lamp, SlotId::Distance).value,
        Ok(SlotValue::Continuous(LAMP_H)),
        "the committed document still says the old one"
    );
    let outcome = session.perform(SessionOp::CommitGesture);
    assert_eq!(outcome.committed.len(), 1, "one edit for the whole drag");
    assert_eq!(session.history().len(), before + 1, "one undo step");
    let landed = row_of(session.committed_doc(), lamp, SlotId::Distance);
    assert_eq!(landed.value, Ok(SlotValue::Continuous(0.013)));
    assert_eq!(
        landed.unit,
        Some(MM.def()),
        "moving the number leaves how it is written alone"
    );

    // The tree-shaped history around that commit: the fork has BOTH
    // children — the abandoned taper-0.7 edit and the gesture's — and
    // the abandoned branch's document is intact.
    let history = session.history();
    assert_eq!(
        history.entry(history.current()).parent(),
        Some(fork),
        "the gesture's edit grew from the fork"
    );
    assert_eq!(history.entry(fork).children().len(), 2, "a real branch");
    assert!(history.entry(fork).children().contains(&abandoned));
    assert_eq!(
        param_of(history.entry(abandoned).doc(), &taper),
        SlotValue::Continuous(0.7),
        "nothing destroyed: the abandoned branch still holds its edit"
    );
    let got = body_volume(&mut session, lighthouse, tol);
    let want = lighthouse_volume(BASE_R, TAPER, HEIGHT, EMBED, 0.013);
    assert!(near(got, want), "taller lamp volume {got} vs {want}");

    // ── 11. A slider drag on a PARAMETER — the affordance's landing
    // spot, and the same gesture machinery through its own door. Mid-
    // drag the preview drives the EVALUATION (the picture follows the
    // scratch document), the document is locked against other edits,
    // and the release commits one `SetDocParamValue`.
    let before = session.history().len();
    assert!(
        session
            .perform(SessionOp::BeginParamGesture {
                name: height.clone()
            })
            .refusal
            .is_none()
    );
    for value in [0.032, 0.04, 0.036] {
        let outcome = session.perform(SessionOp::PreviewGesture { value });
        assert!(outcome.committed.is_empty());
        assert_eq!(outcome.previewed.len(), 1);
    }
    let previewed = body_volume(&mut session, lighthouse, tol);
    // The drag passed 0.04 second and 0.036 last: the picture reads
    // the newest preview's volume, not a stale one's.
    let stale_volume = lighthouse_volume(BASE_R, TAPER, 0.04, EMBED, 0.013);
    assert!(
        near(
            previewed,
            lighthouse_volume(BASE_R, TAPER, 0.036, EMBED, 0.013)
        ) && !near(previewed, stale_volume),
        "the picture follows the newest preview: {previewed}"
    );
    assert_eq!(
        param_of(session.committed_doc(), &height),
        SlotValue::Continuous(HEIGHT),
        "the committed parameter has not moved"
    );
    assert!(
        matches!(
            session
                .perform(SessionOp::SetParam {
                    name: embed.clone(),
                    value: SlotValue::Continuous(0.004),
                })
                .refusal,
            Some(Refusal::GestureInFlight)
        ),
        "other edits refuse typed while the drag holds the document"
    );
    let outcome = session.perform(SessionOp::CommitGesture);
    assert_eq!(outcome.committed.len(), 1, "one edit for the whole drag");
    assert!(matches!(
        outcome.committed.first(),
        Some(DocEdit::SetDocParamValue { .. })
    ));
    assert_eq!(session.history().len(), before + 1, "one undo step");
    assert_eq!(
        param_of(session.committed_doc(), &height),
        SlotValue::Continuous(0.036),
        "the LAST previewed value is what the commit recorded"
    );
    let got = body_volume(&mut session, lighthouse, tol);
    let want = lighthouse_volume(BASE_R, TAPER, 0.036, EMBED, 0.013);
    assert!(near(got, want), "grown volume {got} vs {want}");
    // One undo returns the whole drag; redo re-lands it.
    session.perform(SessionOp::Undo);
    assert_eq!(
        param_of(session.committed_doc(), &height),
        SlotValue::Continuous(HEIGHT)
    );
    session.perform(SessionOp::Redo);
    assert_eq!(
        param_of(session.committed_doc(), &height),
        SlotValue::Continuous(0.036)
    );

    // ── 12. A drag the user thinks better of: previews, then Cancel —
    // no history, no document change, the panels back on the committed
    // value. A second Cancel with nothing in flight refuses typed.
    let before = session.history().len();
    assert!(
        session
            .perform(SessionOp::BeginGesture {
                node: lamp,
                slot: SlotId::Distance,
            })
            .refusal
            .is_none()
    );
    session.perform(SessionOp::PreviewGesture { value: 0.02 });
    session.perform(SessionOp::PreviewGesture { value: 0.025 });
    assert!(session.perform(SessionOp::CancelGesture).refusal.is_none());
    assert_eq!(session.history().len(), before, "no trace in history");
    assert_eq!(
        row_of(session.doc(), lamp, SlotId::Distance).value,
        Ok(SlotValue::Continuous(0.013)),
        "the shown document is the committed one again"
    );
    assert!(matches!(
        session.perform(SessionOp::CancelGesture).refusal,
        Some(Refusal::NoGesture)
    ));

    // ── 13. Save, reopen, and everything parametric survives: the
    // document bit-for-bit, the four parameters at their final values,
    // the drivers still expressions over the same names, the mm
    // notation still on the lamp's height, and the same solid.
    let v_final = body_volume(&mut session, lighthouse, tol);
    let dir = common::tempdir("story-lighthouse");
    let path = dir.join("story-lighthouse.pncad");
    assert!(
        session
            .perform(SessionOp::Save(path.clone()))
            .refusal
            .is_none(),
        "save"
    );
    let authored = session.committed_doc().clone();
    assert!(
        session.perform(SessionOp::Open(path)).refusal.is_none(),
        "open"
    );
    assert!(
        session.committed_doc().bit_eq(&authored),
        "the reopened document is the authored one under D7's comparator"
    );
    for (name, want) in [
        (&base_r, BASE_R),
        (&taper, TAPER),
        (&height, 0.036),
        (&embed, EMBED),
    ] {
        assert_eq!(
            param_of(session.committed_doc(), name),
            SlotValue::Continuous(want),
            "{name:?} survives the round trip"
        );
    }
    let distance = row_of(session.committed_doc(), tower, SlotId::Distance);
    assert_eq!(
        distance.driver,
        SlotDriver::Expression {
            params: vec![height.clone()]
        },
        "the tower's distance is still height-driven"
    );
    let lamp_row = row_of(session.committed_doc(), lamp, SlotId::Distance);
    assert_eq!(lamp_row.unit, Some(MM.def()), "the mm notation survives");
    assert_eq!(lamp_row.value, Ok(SlotValue::Continuous(0.013)));
    let reopened = body_volume(&mut session, lighthouse, tol);
    assert_eq!(
        v_final.to_bits(),
        reopened.to_bits(),
        "same solid after reload"
    );

    // ── 14. The gallery door (`common::story_gallery_dir` states the
    // contract): the finished part, saved through the session's own
    // save door.
    if let Some(gallery) = common::story_gallery_dir() {
        let shot = gallery.join("story-lighthouse.pncad");
        let outcome = session.perform(SessionOp::Save(shot));
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    }
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}
