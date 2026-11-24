//! **A parametric living-in-it story**: one user, one sitting, one
//! little lighthouse — three coaxial drums, each pierced a few
//! millimetres into the one below — built through the session's op
//! vocabulary and then LIVED IN parametrically: every proportion hangs
//! off four document variables, and the session edits the part by
//! moving those numbers rather than by touching geometry again.
//!
//! The walk covers the whole parametric surface as one continuous
//! session: the create/replace param partition (`DeclareVar` refuses
//! a taken name, `SetVariable` refuses an absent variable — both in the
//! edit door's words),
//! expressions driving slots (`SetSlotExpression`) and the
//! refuse-with-affordance on a drag over a driven slot (G4), display
//! units as a separate door from values (`SetSlotUnit`), slider
//! gestures over a slot and over a variable (previews against
//! scratch, exactly one undo step, a cancel leaving no trace), a
//! `range?` run (`ProveRange`) and its discard on the next edit, param edits rippling through evaluation against a closed
//! form, tree-shaped undo history (an edit after an undo mints a
//! sibling), and a save/reopen round trip that keeps the expressions,
//! the variables and the written units.
//!
//! One test, deliberately, following `assembly_walk`: the walk's
//! readability is part of what it asserts — each stage is a numbered
//! block whose assertions say what the stage claims.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use viewer::session::BooleanSpec;

use core::f64::consts::PI;

use common::{body_volume, circle_at, len, near, session_insert};
use pncad::analysis::RangeRefusal;
use pncad::document::{
    Dimension, Doc, DocEdit, EditError, FreeVar, ProfileProgram, RecipeNodeId, SlotId, StepArg,
    VarName,
};
use pncad::geom_core::Tol;
use pncad::prelude::MM;
use viewer::props::{self, SlotDriver, SlotValue, in_written, rendering_unit};
use viewer::session::{DocSession, RangeTarget, Refusal, SessionOp};
use viewer::tree::RowStatus;

/// The lighthouse's proportions, as the variables are first declared:
/// base radius, tier ratio, tier height, and how deep each drum sinks
/// into the one below (what keeps every union seam transversal).
const BASE_R: f64 = 0.05;
const TAPER: f64 = 0.6;
const HEIGHT: f64 = 0.03;
const EMBED: f64 = 0.005;
/// The lamp room's height — the one dimension of the part that is NOT
/// variable-driven, kept literal so the walk has a slot that carries
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

/// **Drives the lift of the moved copy `placement`** by `text`, a
/// height up the copy's own z that today is `lift`.
///
/// The lift is the placement's slide along its space's `z`, a value of
/// its bundle, so its variable holds `lift` and is defined as the
/// expression.
fn drive_lift(session: &mut DocSession, placement: RecipeNodeId, lift: f64, text: &str) {
    use pncad::document::{Coord, Node, TargetAxis};
    let doc = session.committed_doc();
    let Some(Node::Place { constraints, .. }) = doc.node(placement) else {
        panic!("{placement} is a placement");
    };
    let slid = constraints
        .iter()
        .filter_map(|c| c.value())
        .find_map(|value| (value.coord == Coord::slide(TargetAxis::Z)).then_some(value.by))
        .unwrap_or_else(|| panic!("{placement} slides along its space's z"));
    assert_eq!(
        props::variable_value(doc, slid),
        Some(SlotValue::Continuous(lift)),
        "the lift stands where the move put it"
    );
    let outcome = session.perform(SessionOp::SetVariableText {
        var: slid,
        text: text.to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{text:?}: {:?}", outcome.refusal);
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

/// A declared variable's stored value.
fn variable_value(doc: &Doc<ProfileProgram>, name: &VarName) -> SlotValue {
    props::variable_rows(doc)
        .into_iter()
        .find(|row| row.label.name() == Some(name))
        .expect("the variable is declared")
        .value
}

/// The variable `doc` names `name`, as the document speaks it.
fn spoken(doc: &Doc<ProfileProgram>, name: &VarName) -> pncad::document::SpokenVar {
    doc.spoken_var(common::var_of(doc, name.as_str()))
}

/// One drum: a circle profile on its own plane driven by `radius_expr`,
/// extruded up by a literal the caller may re-drive afterwards.
///
/// Stacked extruded drums rather than one revolved silhouette: this
/// scene predates the path verbs, which can now spell one.
fn drum(
    session: &mut DocSession,
    radius_expr: &str,
    distance: f64,
) -> (RecipeNodeId, RecipeNodeId) {
    let profile = session_insert(
        session,
        SessionOp::AddProfile {
            // A positive placeholder radius; the expression takes
            // over before anything downstream consumes it.
            loops: vec![circle_at([0.0, 0.0], 0.01)],
        },
    );
    let radius = radius_slot(session.committed_doc(), profile);
    drive(session, profile, radius, radius_expr);
    let extrude = session_insert(
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

    // ── 2. The user declares the proportions FIRST — four variables,
    // each one committed `DeclareVar` edit and one undo step.
    let base_r = VarName::from_static("base_r");
    let taper = VarName::from_static("taper");
    let height = VarName::from_static("height");
    let embed = VarName::from_static("embed");
    for (name, param) in [
        (&base_r, FreeVar::continuous(Dimension::Length, BASE_R)),
        (&taper, FreeVar::continuous(Dimension::Scalar, TAPER)),
        (&height, FreeVar::continuous(Dimension::Length, HEIGHT)),
        (&embed, FreeVar::continuous(Dimension::Length, EMBED)),
    ] {
        let before = session.history().len();
        let outcome = session.perform(SessionOp::DeclareVar {
            name: name.clone(),
            value: param,
        });
        assert!(outcome.refusal.is_none(), "{name:?}: {:?}", outcome.refusal);
        assert_eq!(outcome.committed.len(), 1);
        assert!(
            matches!(outcome.committed.first(), Some(DocEdit::DeclareVar { .. })),
            "the create door authors a declaration"
        );
        assert_eq!(session.history().len(), before + 1, "one undo step");
    }
    assert_eq!(props::variable_rows(session.committed_doc()).len(), 4);
    assert_eq!(
        variable_value(session.committed_doc(), &taper),
        SlotValue::Continuous(TAPER)
    );

    // ── 3. The two doors partition the edit's semantics, typed, and
    // both refusals are the edit door's own, forwarded through
    // `Refusal::Edit`. Create over a declared name is refused by the
    // declare (`VarNameTaken`, naming the variable holding it). A
    // value write to a variable the document does not hold is refused
    // by `DocEdit::SetVarValue`, which carries an existing
    // declaration forward and says so: `EditError::UnknownVar`, naming
    // the variable and the one recourse both doors render
    // (`editor_core::edit::UNKNOWN_VAR_RECOURSE`).
    // Neither commits or mints history.
    let before = session.history().len();
    let taper_var = common::var_of(session.committed_doc(), taper.as_str());
    let outcome = session.perform(SessionOp::DeclareVar {
        name: taper.clone(),
        value: FreeVar::continuous(Dimension::Length, 1.0),
    });
    match outcome.refusal {
        Some(Refusal::Edit(ref error)) => match **error {
            EditError::VarNameTaken {
                ref name,
                ref holder,
            } => {
                assert_eq!(name, &taper);
                assert_eq!(
                    holder.id(),
                    taper_var,
                    "the holder is the standing variable"
                );
            }
            ref other => panic!("expected VarNameTaken, got {other:?}"),
        },
        ref other => panic!("expected the declare door's refusal, got {other:?}"),
    }
    assert!(outcome.committed.is_empty(), "a refusal commits nothing");
    let absent = pncad::document::VarId::new(0, 0x7461_7070_6572);
    let outcome = session.perform(SessionOp::SetVariable {
        var: absent,
        value: SlotValue::Continuous(0.5),
    });
    match outcome.refusal {
        Some(Refusal::Edit(ref error)) => match **error {
            // The door rides along; this row is about the ADDRESS.
            EditError::UnknownVar { ref var, .. } => {
                assert_eq!(*var, absent.into());
            }
            ref other => panic!("expected UnknownVar, got {other:?}"),
        },
        ref other => panic!("expected the edit door's refusal, got {other:?}"),
    }
    assert!(outcome.committed.is_empty());
    assert_eq!(session.history().len(), before, "and mints no history");

    // ── 4. The base drum: a circle profile whose radius is DRIVEN by
    // `base_r`, extruded by `height`. The slot rows say who drives
    // what, and the evaluated body is the closed-form cylinder.
    // The literal 0.03 deliberately equals `height`'s value, so the
    // part is coherent even mid-build; were the expression below to
    // silently not take over, the stage-9/11 variable ripples would
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
            variables: vec![spoken(session.committed_doc(), &base_r)]
        },
        "the radius names its driving variable"
    );
    assert_eq!(
        radius_row.value,
        Ok(viewer::Reading::One(SlotValue::Continuous(BASE_R)))
    );
    let got = body_volume(&mut session, base, tol);
    let want = PI * BASE_R * BASE_R * HEIGHT;
    assert!(near(got, want), "base drum volume {got} vs {want}");

    // ── 5. The tower, twice the tier height and `taper` times the
    // radius, sunk `embed` into the base so the union seam is the
    // tower wall crossing the base's top cap transversally.
    let (_tower_profile, tower) = drum(&mut session, "base_r * taper", 0.055);
    drive(&mut session, tower, SlotId::Distance, "height * 2.0");
    // 0.025 deliberately equals `height - embed` today, so the part is
    // coherent before the drive lands; a literal that stayed driving
    // would be caught by the stage-9/11 ripples.
    let home = session
        .committed_doc()
        .node_space(base)
        .expect("the base's space");
    let tower_up = common::moved_in_space(&mut session, tower, [0.0, 0.0, 0.025], home);
    let hull = session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Union(vec![base.into(), tower_up.into()]),
        },
    );
    // The union places the moved tower into the base's space where it
    // stands; its lift is then driven on that placement.
    drive_lift(&mut session, tower_up, 0.025, "height - embed");
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
            .one()
            .expect("one value")
            .as_f64()
            .to_bits(),
        before_unit
            .value
            .expect("a value")
            .one()
            .expect("one value")
            .as_f64()
            .to_bits(),
        "the notation moved; the canonical value did not"
    );
    assert_eq!(
        in_written(
            after_unit
                .value
                .expect("a value")
                .one()
                .expect("one value")
                .as_f64(),
            rendering_unit(
                after_unit.dimension,
                after_unit.unit,
                props::Notation::DEFAULT
            )
            .expect("a length row"),
        ),
        12.0,
        "shown as twelve millimetres"
    );
    // 0.08 deliberately equals `height * 3 - embed * 2` today (coherent
    // mid-build); a literal that stayed driving would be caught by the
    // stage-9/11 variable ripples.
    let home = session
        .committed_doc()
        .node_space(hull)
        .expect("the hull's space");
    let lamp_up = common::moved_in_space(&mut session, lamp, [0.0, 0.0, 0.08], home);
    let lighthouse = session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Union(vec![hull.into(), lamp_up.into()]),
        },
    );
    drive_lift(&mut session, lamp_up, 0.08, "height * 3.0 - embed * 2.0");
    assert_eq!(
        viewer::world::drawn_nodes(session.committed_doc()),
        [lighthouse],
        "the picture is one body: the finished part, unplaced"
    );
    let got = body_volume(&mut session, lighthouse, tol);
    let want = lighthouse_volume(BASE_R, TAPER, HEIGHT, EMBED, LAMP_H);
    assert!(near(got, want), "lighthouse volume {got} vs {want}");
    let rows = session.tree_rows();
    // Ten: the features, and no placement — the finished part lands
    // unplaced.
    assert_eq!(rows.len(), 10, "ten features and no placement");
    assert!(
        rows.iter().all(|row| row.status == RowStatus::Ok),
        "every row green: {rows:?}"
    );

    // ── 7. Living in it, first touch: a drag on the tower's driven
    // distance refuses with G4's affordance —
    // the typed payload names the driving variable and the value the
    // slot has today, and the refusal commits nothing.
    let before = session.history().len();
    let outcome = session.perform(SessionOp::BeginGesture {
        node: tower,
        slot: SlotId::Distance,
    });
    match outcome.refusal {
        Some(Refusal::DrivenByExpression {
            node,
            slot,
            ref variables,
            current,
            ..
        }) => {
            assert_eq!(node, tower);
            assert_eq!(slot, SlotId::Distance);
            assert_eq!(
                variables,
                &vec![spoken(session.committed_doc(), &height)],
                "the affordance's target"
            );
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
    // `range?` targets the VARIABLE, and on the tower's depth it must:
    // that slot reads `height * 2.0`, a variable a formula defines,
    // which has no interval of its own, and the kernel's admission
    // refuses it in words that send the user to the variables the
    // formula reads. A run on an undeclared variable refuses through
    // the same door. The run is explicit and commits nothing; it is
    // stopped before it builds its witness, since what this walk needs
    // of it is that the next edit drops it.
    let before = session.history().len();
    let taper_var = common::var_of(session.committed_doc(), taper.as_str());
    let outcome = session.perform(SessionOp::ProveRange {
        target: RangeTarget::Variable { var: taper_var },
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(outcome.committed.is_empty(), "a run commits nothing");
    assert_eq!(session.history().len(), before, "and mints no history");
    let outcome = session.perform(SessionOp::ProveRange {
        target: RangeTarget::Slot {
            node: tower,
            slot: SlotId::Distance,
        },
    });
    match outcome.refusal {
        Some(Refusal::Range(ref refusal)) => {
            assert!(
                matches!(**refusal, RangeRefusal::SlotIsDefined { .. }),
                "{refusal:?}"
            );
            assert!(
                refusal
                    .to_string()
                    .contains("certify the variables that formula reads"),
                "{refusal}"
            );
        }
        ref other => panic!("expected the kernel's admission refusal, got {other:?}"),
    }
    let outcome = session.perform(SessionOp::ProveRange {
        target: RangeTarget::Variable {
            var: pncad::document::VarId::new(0, 0x7461_7070_6572),
        },
    });
    assert!(
        matches!(
            outcome.refusal.as_ref(),
            Some(Refusal::Range(refusal))
                if matches!(**refusal, RangeRefusal::NotAContinuousParam { .. })
        ),
        "{:?}",
        outcome.refusal
    );
    let reading = session.range().expect("the run started").clone();
    assert_eq!(
        reading.target,
        RangeTarget::Variable { var: taper_var },
        "the refusals neither set a reading nor cleared the run in flight"
    );
    assert!(reading.proving(), "and it proves until stopped");
    session.perform(SessionOp::StopRange);
    session.pump();
    assert!(
        !session
            .range()
            .expect("a stopped run keeps its reading")
            .proving(),
        "the stop ends it"
    );

    // ── 9. The ripple: one variable moves and the whole part
    // follows. The range's reading — a statement about the OLD
    // document — is dropped by the edit; the volume tracks the closed form;
    // one undo restores the old geometry exactly; redo returns.
    let v_before = body_volume(&mut session, lighthouse, tol);
    let outcome = session.perform(SessionOp::SetVariable {
        var: common::var_of(session.committed_doc(), taper.as_str()),
        value: SlotValue::Continuous(0.7),
    });
    assert_eq!(outcome.committed.len(), 1);
    assert!(matches!(
        outcome.committed.first(),
        Some(DocEdit::SetVarValue { .. })
    ));
    assert!(
        session.range().is_none(),
        "the range's reading dies with the document it was about"
    );
    let v_wide = body_volume(&mut session, lighthouse, tol);
    let want = lighthouse_volume(BASE_R, 0.7, HEIGHT, EMBED, LAMP_H);
    assert!(near(v_wide, want), "widened volume {v_wide} vs {want}");
    assert!(v_wide > v_before, "a fatter taper is more lighthouse");

    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert_eq!(
        variable_value(session.committed_doc(), &taper),
        SlotValue::Continuous(TAPER),
        "undo returns the variable"
    );
    let v_back = body_volume(&mut session, lighthouse, tol);
    assert_eq!(
        v_back.to_bits(),
        v_before.to_bits(),
        "…and the old geometry, exactly"
    );
    assert!(session.perform(SessionOp::Redo).refusal.is_none());
    assert_eq!(
        variable_value(session.committed_doc(), &taper),
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
        let outcome = session.perform(SessionOp::PreviewGesture {
            node: lamp,
            slot: SlotId::Distance,
            value,
        });
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
        Ok(viewer::Reading::One(SlotValue::Continuous(0.013))),
        "the panels show the scratch value"
    );
    assert_eq!(
        row_of(session.committed_doc(), lamp, SlotId::Distance).value,
        Ok(viewer::Reading::One(SlotValue::Continuous(LAMP_H))),
        "the committed document still says the old one"
    );
    let outcome = session.perform(SessionOp::CommitGesture {
        node: lamp,
        slot: SlotId::Distance,
    });
    assert_eq!(outcome.committed.len(), 1, "one edit for the whole drag");
    assert_eq!(session.history().len(), before + 1, "one undo step");
    let landed = row_of(session.committed_doc(), lamp, SlotId::Distance);
    assert_eq!(
        landed.value,
        Ok(viewer::Reading::One(SlotValue::Continuous(0.013)))
    );
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
        variable_value(history.entry(abandoned).doc(), &taper),
        SlotValue::Continuous(0.7),
        "nothing destroyed: the abandoned branch still holds its edit"
    );
    let got = body_volume(&mut session, lighthouse, tol);
    let want = lighthouse_volume(BASE_R, TAPER, HEIGHT, EMBED, 0.013);
    assert!(near(got, want), "taller lamp volume {got} vs {want}");

    // ── 11. A slider drag on a VARIABLE — the affordance's landing
    // spot, and the same gesture machinery through its own door. Mid-
    // drag the preview drives the EVALUATION (the picture follows the
    // scratch document), the document is locked against other edits,
    // and the release commits one `SetVarValue`.
    let before = session.history().len();
    assert!(
        session
            .perform(SessionOp::BeginVariableGesture {
                var: common::var_of(session.committed_doc(), height.as_str())
            })
            .refusal
            .is_none()
    );
    for value in [0.032, 0.04, 0.036] {
        let outcome = session.perform(SessionOp::PreviewVariableGesture {
            var: common::var_of(session.committed_doc(), height.as_str()),
            value,
        });
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
        variable_value(session.committed_doc(), &height),
        SlotValue::Continuous(HEIGHT),
        "the committed variable has not moved"
    );
    assert!(
        matches!(
            session
                .perform(SessionOp::SetVariable {
                    var: common::var_of(session.committed_doc(), embed.as_str()),
                    value: SlotValue::Continuous(0.004),
                })
                .refusal,
            Some(Refusal::GestureInFlight)
        ),
        "other edits refuse typed while the drag holds the document"
    );
    let outcome = session.perform(SessionOp::CommitVariableGesture {
        var: common::var_of(session.committed_doc(), height.as_str()),
    });
    assert_eq!(outcome.committed.len(), 1, "one edit for the whole drag");
    assert!(matches!(
        outcome.committed.first(),
        Some(DocEdit::SetVarValue { .. })
    ));
    assert_eq!(session.history().len(), before + 1, "one undo step");
    assert_eq!(
        variable_value(session.committed_doc(), &height),
        SlotValue::Continuous(0.036),
        "the LAST previewed value is what the commit recorded"
    );
    let got = body_volume(&mut session, lighthouse, tol);
    let want = lighthouse_volume(BASE_R, TAPER, 0.036, EMBED, 0.013);
    assert!(near(got, want), "grown volume {got} vs {want}");
    // One undo returns the whole drag; redo re-lands it.
    session.perform(SessionOp::Undo);
    assert_eq!(
        variable_value(session.committed_doc(), &height),
        SlotValue::Continuous(HEIGHT)
    );
    session.perform(SessionOp::Redo);
    assert_eq!(
        variable_value(session.committed_doc(), &height),
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
    session.perform(SessionOp::PreviewGesture {
        node: lamp,
        slot: SlotId::Distance,
        value: 0.02,
    });
    session.perform(SessionOp::PreviewGesture {
        node: lamp,
        slot: SlotId::Distance,
        value: 0.025,
    });
    assert!(session.perform(SessionOp::CancelGesture).refusal.is_none());
    assert_eq!(session.history().len(), before, "no trace in history");
    assert_eq!(
        row_of(session.doc(), lamp, SlotId::Distance).value,
        Ok(viewer::Reading::One(SlotValue::Continuous(0.013))),
        "the shown document is the committed one again"
    );
    assert!(matches!(
        session.perform(SessionOp::CancelGesture).refusal,
        Some(Refusal::NoGesture)
    ));

    // ── 13. Save, reopen, and everything parametric survives: the
    // document bit-for-bit, the four variables at their final values,
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
            variable_value(session.committed_doc(), name),
            SlotValue::Continuous(want),
            "{name:?} survives the round trip"
        );
    }
    let distance = row_of(session.committed_doc(), tower, SlotId::Distance);
    assert_eq!(
        distance.driver,
        SlotDriver::Expression {
            variables: vec![spoken(session.committed_doc(), &height)]
        },
        "the tower's distance is still height-driven"
    );
    let lamp_row = row_of(session.committed_doc(), lamp, SlotId::Distance);
    assert_eq!(lamp_row.unit, Some(MM.def()), "the mm notation survives");
    assert_eq!(
        lamp_row.value,
        Ok(viewer::Reading::One(SlotValue::Continuous(0.013)))
    );
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
