//! Review rows for PR 6302 (the profile editor holds the authored
//! program): a differential over every profile node of the viewer
//! corpus, through the public half of the edit door's write-back
//! (`sketch::held_loops`, `HeldStep::step`, `HeldStep::written`,
//! `HeldStep::program`) with the readings and units the door takes
//! from the slot rows (`ProfileEdit::reading`, `edit_door_ui`'s
//! `noted`).

use crate::corpus;
use pncad::document::{
    Doc, Formula, LoopProgram, Node, ProfileProgram, RecipeNodeId, SlotId, StepArg,
};
use pncad::geom_core::{Point2, Tol};
use pncad::profile::{Step, Target};
use viewer::props::{self, Notation};
use viewer::session::{DocSession, SessionOp};
use viewer::sketch::{self, HeldStep};

const MM: Notation = Notation {
    length: pncad::quantity::MM,
    angle: pncad::quantity::DEG,
};

fn profiles(doc: &Doc<ProfileProgram>) -> Vec<RecipeNodeId> {
    doc.ids()
        .into_iter()
        .filter(|id| matches!(doc.node(*id), Some(Node::Profile(_))))
        .collect()
}

fn row_of(rows: &[props::SlotRow], slot: SlotId) -> Option<&props::SlotRow> {
    rows.iter().find(|row| row.slot == slot)
}

fn slot(loop_: usize, step: usize, arg: StepArg) -> SlotId {
    SlotId::Profile {
        loop_: u32::try_from(loop_).unwrap(),
        step: u32::try_from(step).unwrap(),
        arg,
    }
}

/// The written program as a unit-aware, bit-aware string.
fn spelled(loops: &[LoopProgram<Formula>]) -> String {
    format!("{loops:?}")
}

/// Every row's view written back over it, as `write_step` would for a
/// row loaded in place, with `moved` applied to the view first.
fn written_back(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    held: &[Vec<HeldStep>],
    notation: Notation,
    moved: &dyn Fn(Step<f64>) -> Option<Step<f64>>,
) -> (Vec<Vec<HeldStep>>, usize, usize) {
    let rows = props::slot_rows(doc, node);
    let mut back = held.to_vec();
    let (mut unviewed, mut changed) = (0, 0);
    for (l, steps) in held.iter().enumerate() {
        for (r, step) in steps.iter().enumerate() {
            let reading = |arg: StepArg| -> Option<f64> {
                row_of(&rows, slot(l, r, arg))?
                    .value
                    .as_ref()
                    .ok()?
                    .one()
                    .map(|v| v.as_f64())
            };
            let noted = |arg: StepArg| row_of(&rows, slot(l, r, arg))?.unit;
            let Ok(view) = step.step(&reading, Tol::witness()) else {
                unviewed += 1;
                continue;
            };
            let view = match moved(view) {
                Some(m) => {
                    changed += 1;
                    m
                }
                None => view,
            };
            back[l][r] = step
                .written(view, &reading, notation, &noted)
                .expect("finite");
        }
    }
    (back, unviewed, changed)
}

fn programs(held: &[Vec<HeldStep>]) -> Vec<LoopProgram<Formula>> {
    held.iter()
        .map(|s| HeldStep::program(s).expect("shape"))
        .collect()
}

/// **Claim 1**: for every profile in the corpus, every row's view
/// written back unmoved, in either notation, leaves the held program
/// equal to the written one, unit for unit and bit for bit; and the
/// door handed it commits nothing.
#[test]
fn review_6302_every_corpus_profile_round_trips_unmoved_unit_for_unit() {
    let (mut nodes, mut rows_total, mut unviewed_total) = (0, 0, 0);
    for doc in corpus::documents() {
        for node in profiles(&doc.doc) {
            let Some(Node::Profile(program)) = doc.doc.node(node) else {
                unreachable!()
            };
            let base = sketch::written_program(&doc.doc, program);
            let held = sketch::held_loops(&doc.doc, node)
                .unwrap_or_else(|r| panic!("{}: node {node} refused: {r}", doc.name));
            assert_eq!(
                spelled(&programs(&held)),
                spelled(&base.loops),
                "{}: held as written",
                doc.name
            );
            for notation in [Notation::CANONICAL, MM] {
                let (back, unviewed, _) = written_back(&doc.doc, node, &held, notation, &|_| None);
                assert_eq!(
                    spelled(&programs(&back)),
                    spelled(&base.loops),
                    "{}: node {node}: an unmoved write-back rewrote something in {notation:?}",
                    doc.name
                );
                if notation == MM {
                    unviewed_total += unviewed;
                }
            }
            nodes += 1;
            rows_total += held.iter().map(Vec::len).sum::<usize>();
            let mut session = DocSession::inline(doc.doc.clone(), Tol::witness());
            let before = session.committed_doc().clone();
            let out = session.perform(SessionOp::EditProfile {
                node,
                ids: base.kept_in_place(),
                base: base.clone(),
                loops: programs(&held),
            });
            assert!(out.refusal.is_none(), "{}: {:?}", doc.name, out.refusal);
            assert!(
                out.committed.is_empty(),
                "{}: {:?}",
                doc.name,
                out.committed
            );
            assert!(session.committed_doc().bit_eq(&before), "{}", doc.name);
        }
    }
    eprintln!(
        "review_6302: {nodes} profile nodes, {rows_total} rows, {unviewed_total} with no view"
    );
    assert!(nodes > 20, "the corpus was walked: {nodes}");
}

/// **Claims 1/2**: a corner of every corpus profile moved (a literal
/// point `line_to`'s x), written back and applied: every other
/// argument lands as it was written (unit and formula), and the moved
/// one lands in its slot's unit.
#[test]
fn review_6302_a_moved_corner_keeps_every_neighbour_as_written() {
    let mut moved_nodes = 0;
    for doc in corpus::documents() {
        for node in profiles(&doc.doc) {
            let Some(Node::Profile(program)) = doc.doc.node(node) else {
                unreachable!()
            };
            let base = sketch::written_program(&doc.doc, program);
            let held = sketch::held_loops(&doc.doc, node).expect("held");
            let rows_before = props::slot_rows(&doc.doc, node);
            // Move the FIRST literal-x line_to only.
            let target = held.iter().enumerate().find_map(|(l, steps)| {
                steps.iter().enumerate().find_map(|(r, _)| {
                    let row = row_of(&rows_before, slot(l, r, StepArg::TargetX))?;
                    matches!(row.driver, props::SlotDriver::Literal).then_some((l, r))
                })
            });
            let Some((tl, tr)) = target else { continue };
            let mut back = held.clone();
            {
                let rows = &rows_before;
                let reading = |arg: StepArg| -> Option<f64> {
                    row_of(rows, slot(tl, tr, arg))?
                        .value
                        .as_ref()
                        .ok()?
                        .one()
                        .map(|v| v.as_f64())
                };
                let noted = |arg: StepArg| row_of(rows, slot(tl, tr, arg))?.unit;
                let Ok(Step::LineTo(Target::Point(p))) =
                    held[tl][tr].step(&reading, Tol::witness())
                else {
                    continue;
                };
                let moved = Step::LineTo(Target::Point(Point2::new(p.x + 1.0e-3, p.y)));
                back[tl][tr] = held[tl][tr]
                    .written(moved, &reading, Notation::CANONICAL, &noted)
                    .expect("finite");
            }
            let mut session = DocSession::inline(doc.doc.clone(), Tol::witness());
            let out = session.perform(SessionOp::EditProfile {
                node,
                ids: base.kept_in_place(),
                base: base.clone(),
                loops: programs(&back),
            });
            if let Some(refusal) = &out.refusal {
                eprintln!(
                    "review_6302: {} node {node}: moved corner refused: {refusal:?}",
                    doc.name
                );
                continue;
            }
            moved_nodes += 1;
            let rows_after = props::slot_rows(session.committed_doc(), node);
            assert_eq!(rows_before.len(), rows_after.len(), "{}", doc.name);
            for (b, a) in rows_before.iter().zip(&rows_after) {
                assert_eq!(b.slot, a.slot, "{}", doc.name);
                assert_eq!(
                    b.unit,
                    a.unit,
                    "{}: node {node}: {} changed unit (moved {tl},{tr})",
                    doc.name,
                    b.slot.label()
                );
                if b.slot != slot(tl, tr, StepArg::TargetX) {
                    assert_eq!(
                        b.source,
                        a.source,
                        "{}: node {node}: {} changed source",
                        doc.name,
                        b.slot.label()
                    );
                }
            }
        }
    }
    eprintln!("review_6302: {moved_nodes} nodes with a moved corner applied");
    assert!(moved_nodes > 5, "{moved_nodes}");
}
