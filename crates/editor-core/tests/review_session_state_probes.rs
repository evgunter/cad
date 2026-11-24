//! Reviewer probes for the session-state unit: where a leaf's session
//! counts something no top-level node's record carries.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS};
use editor_core::{
    BodyRead, CancelToken, EvalOptions, Evaluation, NodeResult, Operand, ProfileDoc,
    RecipeNodeId, evaluate,
};
use geom_core::sym::{DriveMemo, with_session_memo_retry};
use geom_core::{Interval, Sym, SymBudget, SymCounts, SymRetry, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: DEFAULT_SYM_MAX_TERMS,
        max_degree: DEFAULT_SYM_MAX_DEGREE,
    }
}

fn leaf(doc: &ProfileDoc, opts: EvalOptions, tol: Tol) -> (Evaluation<Sym<Interval>>, SymCounts) {
    let box_ = ParamBox::of(&analyzed_box(doc, &AnalysisPolicy::default()));
    let opts = EvalOptions {
        param_box: Some(Arc::new(box_)),
        ..opts
    };
    let table = Arc::new(DriveMemo::new(budget(), SymRules::shipped()));
    with_session_memo_retry(
        budget(),
        SymRules::shipped(),
        SymRetry::kept_atom(),
        &table,
        || evaluate(doc, None, &CancelToken::new(), &opts, tol),
    )
}

/// The session's counts (frozen aside), the sum of the top-level nodes'
/// records, and how many registrations those records carry.
fn sums(ev: &Evaluation<Sym<Interval>>, session: SymCounts) -> (SymCounts, SymCounts, usize) {
    let mut nodes = SymCounts::default();
    let mut regs = 0;
    for id in &ev.order {
        let record = match ev.result(*id) {
            Some(NodeResult::Ok(v)) => &v.sym,
            Some(NodeResult::Failed(e)) => &e.sym,
            _ => continue,
        };
        nodes.absorb(record.counts());
        regs += record.registrations();
    }
    // This head's receipt: the nodes', the parts' and (when gathered) the product's.
    for part in &ev.parts_sym {
        nodes.absorb(part.counts());
    }
    (
        SymCounts {
            frozen: 0,
            ..session
        },
        nodes,
        regs,
    )
}

fn report(label: &str, ev: &Evaluation<Sym<Interval>>, session: SymCounts) -> (SymCounts, SymCounts, usize) {
    let (s, n, regs) = sums(ev, session);
    let failed: Vec<_> = ev
        .order
        .iter()
        .filter_map(|id| match ev.result(*id) {
            Some(NodeResult::Failed(e)) => Some(format!("{id:?}: {:?}", e.kind)),
            _ => None,
        })
        .collect();
    println!("{label}: session {s:?}\n{label}: nodes   {n:?}\n{label}: registrations on top records {regs}\n{label}: failed {failed:?}");
    (s, n, regs)
}

/// A family: three blocks in a row, each cut by a hole (the members
/// decide), and a union reading the family whole.
#[test]
fn probe_a_family_leafs_receipt_against_its_nodes_records() {
    let tol = Tol::witness();
    let (doc, _, cut) = crate::g_family_of_booleans::cut_row(|doc, b, _| {
        let (doc, hole) = crate::blocks::block_on(doc, b, (0.25, 0.5), (0.25, 0.75), -1.0, 3.0);
        (doc, BodyRead::plain(Operand::Node(hole)))
    });
    let (doc, _union) = crate::fixture::insert(
        doc,
        editor_core::Node::Union {
            members: editor_core::Bodies::Family(BodyRead::plain(Operand::Node(cut))),
        },
    );
    let (ev, session) = leaf(&doc, EvalOptions::default(), tol);
    let (s, n, _) = report("family", &ev, session);
    assert!(s.decisions() > 0, "the leaf decided");
    assert_eq!(s, n, "family: the receipt is the sum of the nodes' records");
}

/// An assembly instancing the two-hole plate (whose arcs register):
/// the part's nodes decide and register while the part is entered,
/// before any of the top's nodes runs.
#[test]
fn probe_an_assemblys_leaf_receipt_and_its_parts_registrations() {
    let tol = Tol::witness();
    let plate = crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol).0;
    // The plate alone: its own nodes carry registrations.
    let (pev, psession) = leaf(&plate, EvalOptions::default(), tol);
    let (_, _, part_regs) = report("plate alone", &pev, psession);
    let mut store = crate::fixture::resolver::PartStore::new();
    let r = store.insert(plate, tol);
    let doc = ProfileDoc::empty(editor_core::DocumentId::derive("probe-assembly"), tol);
    let (doc, inst): (ProfileDoc, RecipeNodeId) =
        crate::fixture::insert(doc, crate::fixture::mated_instance(r));
    let doc = crate::fixture::assemble_world_copies(doc);
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    let (ev, session) = leaf(&doc, opts, tol);
    let (s, n, regs) = report("assembly", &ev, session);
    println!("instance {inst:?}: {:?}", ev.result(inst).map(|r| matches!(r, NodeResult::Ok(_))));
    assert!(part_regs > 0, "the plate's nodes register");
    assert!(s.decisions() > 0, "the leaf decided");
    // The unplaced plate's product is empty, so the instance refuses here;
    // the registrations reaching an instance are pinned by the unit's own
    // `a_parts_run_is_the_parts_and_its_identities_reach_its_instance`.
    if matches!(ev.result(inst), Some(NodeResult::Ok(_))) {
        assert_eq!(regs, part_regs, "the instance carries its part's registrations");
    }
    assert_eq!(s, n, "assembly: the receipt is the sum of the nodes' records");
}
