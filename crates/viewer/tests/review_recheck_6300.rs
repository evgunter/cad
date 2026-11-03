//! **Re-check probes, PR 6300 (lane `placing-recheck-6300`)**, at
//! `ef9206924a`. The fix pass carries a loaded turn onto a new turn
//! freedom where its evaluated number equals ± the measured one
//! (`matetool::carried_turn`). These rows ask whether the carried value
//! still means what it meant: the copy turns under the variable as it
//! did before the change of freedom.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::asm;
use pncad::document::{
    Coord, Dimension, Formula, Frame, Node, RecipeNodeId, SlotId, TargetAxis, VarKind, VarName,
};
use pncad::geom_core::Tol;
use viewer::matetool::{MateChoice, MateTool};
use viewer::session::{DocSession, SessionOp};

fn authored(
    session: &DocSession,
    placement: RecipeNodeId,
) -> Vec<pncad::document::Constraint<Formula>> {
    let doc = session.committed_doc();
    match doc.node(placement).map(|node| node.authored(doc)) {
        Some(Node::Place { constraints, .. } | Node::InstantiatePart { constraints, .. }) => {
            constraints
        }
        other => panic!("{placement:?} is a placement: {other:?}"),
    }
}

/// Each value of the bundle, as the panel reads it.
fn read(
    session: &DocSession,
    placement: RecipeNodeId,
) -> Vec<(Coord<pncad::document::Operand>, String)> {
    authored(session, placement)
        .iter()
        .filter_map(|c| {
            c.value().map(|v| {
                (
                    v.coord.clone(),
                    viewer::matetool::value_text(session.committed_doc(), &v.by).replace(' ', ""),
                )
            })
        })
        .collect()
}

fn id_on(
    session: &DocSession,
    placement: RecipeNodeId,
    coord: Coord<pncad::document::Operand>,
) -> pncad::document::ConstraintId {
    authored(session, placement)
        .iter()
        .find_map(|c| c.value().filter(|v| v.coord == coord).map(|_| c.id))
        .unwrap_or_else(|| panic!("a value on {coord:?}"))
}

fn rotation(session: &DocSession, placement: RecipeNodeId, tol: Tol) -> [[f64; 3]; 3] {
    let (doc, _) = session.landed_pair().expect("landed");
    let at: Frame = common::solve(session, doc, tol)
        .placement(doc, placement)
        .expect("a pose");
    at.columns
}

fn close(a: &[[f64; 3]; 3], b: &[[f64; 3]; 3]) -> bool {
    a.iter()
        .flatten()
        .zip(b.iter().flatten())
        .all(|(x, y)| (x - y).abs() < 1e-9)
}

fn rest() -> MateChoice {
    MateChoice {
        kind: VarKind::Plane,
        flip: true,
        values: Vec::new(),
    }
}

fn declare(session: &mut DocSession, name: &str, at: f64) {
    let outcome = session.perform(SessionOp::DeclareVar {
        name: VarName::new(name).expect("a name"),
        value: pncad::document::FreeVar::continuous(Dimension::Angle, at),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
}

fn set(session: &mut DocSession, name: &str, at: f64) {
    let var = common::var_of(session.committed_doc(), name);
    let outcome = session.perform(SessionOp::SetVariable {
        var,
        value: viewer::props::SlotValue::Continuous(at),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
}

fn spin_reads(
    session: &mut DocSession,
    placement: RecipeNodeId,
    coord: Coord<pncad::document::Operand>,
    text: &str,
) {
    let constraint = id_on(session, placement, coord);
    let outcome = session.perform(SessionOp::SetSlotExpression {
        node: placement,
        slot: SlotId::ValueSpin { constraint },
        text: text.to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
}

/// R2's removal row with the variable at `0` — an angle a person
/// declares to drive later. The rest's spin reads `pa`; removing the
/// rest must leave `pa` turning the copy as it did, about the shelf's
/// normal. At `0` every chain turn measures `0`, so the carry's
/// equality test (`± 0 = 0`) matches the first turn freedom it meets
/// and takes the `+` sign first.
#[test]
fn rc_removing_a_rest_whose_spin_reads_a_variable_at_zero_keeps_what_it_turns() {
    let tol = Tol::witness();
    let bench = asm::bench("rc-remove-zero", tol);
    let mut session = asm::open_bench(&bench, tol);
    let (post_top, shelf_bottom) = asm::seat_picks(&session, &bench);
    let mut tool = MateTool::new();
    tool.pick(session.doc(), post_top);
    tool.pick(session.doc(), shelf_bottom);
    let proposal = common::proposed(&session, &tool, rest()).expect("the rest proposes");
    let mate = common::commit_mate(&mut session, proposal.op());
    declare(&mut session, "pa", 0.0);
    spin_reads(&mut session, bench.post_b, Coord::SPIN, "pa");
    set(&mut session, "pa", 0.3);
    let rested = rotation(&session, bench.post_b, tol);
    set(&mut session, "pa", 0.0);
    let outcome = session.perform(SessionOp::RemoveConstraint { mate });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let bundle = read(&session, bench.post_b);
    set(&mut session, "pa", 0.3);
    let freed = rotation(&session, bench.post_b, tol);
    std::fs::remove_dir_all(&bench.dir).expect("removable");
    assert!(
        close(&freed, &rested),
        "pa turns the freed copy to {freed:?}, where the rest turned it to {rested:?}; \
         the freed bundle reads {bundle:?}"
    );
}

/// R2's adding row with two variables at `0`: post_b unmated, its turn
/// about `x` reading `qa` and its turn about `z` (the shelf's normal's
/// line) reading `pa`. The rest leaves the spin about the normal; the
/// value carried onto it should be `pa`'s, the turn about that line —
/// not `qa`'s, a turn about another line whose number happens to agree.
#[test]
fn rc_adding_a_rest_carries_the_turn_about_its_normal_not_one_that_agrees_in_number() {
    let tol = Tol::witness();
    let bench = asm::bench("rc-add-two", tol);
    let mut session = asm::open_bench(&bench, tol);
    declare(&mut session, "qa", 0.0);
    declare(&mut session, "pa", 0.0);
    spin_reads(
        &mut session,
        bench.post_b,
        Coord::Spin(Some(TargetAxis::X)),
        "qa",
    );
    spin_reads(
        &mut session,
        bench.post_b,
        Coord::Spin(Some(TargetAxis::Z)),
        "pa",
    );
    let before = read(&session, bench.post_b);
    let (post_top, shelf_bottom) = asm::seat_picks(&session, &bench);
    let mut tool = MateTool::new();
    tool.pick(session.doc(), post_top);
    tool.pick(session.doc(), shelf_bottom);
    let proposal = common::proposed(&session, &tool, rest()).expect("the rest proposes");
    let dropped: Vec<String> = proposal
        .dropped
        .iter()
        .map(|row| format!("{:?} = {}", row.coord, row.by))
        .collect();
    common::commit_mate(&mut session, proposal.op());
    let after = read(&session, bench.post_b);
    std::fs::remove_dir_all(&bench.dir).expect("removable");
    let spin = after
        .iter()
        .find(|(coord, _)| *coord == Coord::SPIN)
        .map(|(_, text)| text.clone())
        .expect("the rest leaves its spin");
    assert!(
        spin.trim_start_matches('-') == "pa",
        "the spin about the shelf's normal reads {spin:?}; was {before:?}, now {after:?}, \
         said dropped {dropped:?}"
    );
}
