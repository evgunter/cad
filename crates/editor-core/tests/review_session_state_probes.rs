//! Reviewer probes for the session-state unit: what a family's value
//! carries of its members' symbolic records, and whether a leaf's
//! receipt is the sum of its nodes' records on a document with a
//! family.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS};
use editor_core::{CancelToken, EvalOptions, Evaluation, NodeResult, ProfileDoc, evaluate};
use geom_core::sym::{DriveMemo, with_session_memo_retry};
use geom_core::{Interval, Sym, SymBudget, SymCounts, SymRetry, SymRules, Tol};

use crate::fixture;

fn budget() -> SymBudget {
    SymBudget {
        max_terms: DEFAULT_SYM_MAX_TERMS,
        max_degree: DEFAULT_SYM_MAX_DEGREE,
    }
}

/// The two-hole plate with a row of three copies of its first hole.
fn plate_with_a_row(tol: Tol) -> (ProfileDoc, editor_core::RecipeNodeId) {
    let p = crate::m10_7_plate::plate_with_holes(5.0e-5, 1.0e-5, tol);
    let (doc, _k, row) = fixture::row(
        p.doc,
        p.holes[0],
        ("k", 3),
        editor_core::TargetAxis::Y,
        5.0e-3,
    );
    (doc, row)
}

fn leaf(doc: &ProfileDoc, tol: Tol) -> (Evaluation<Sym<Interval>>, SymCounts) {
    let root = ParamBox::of(&analyzed_box(doc, &AnalysisPolicy::default()));
    let table = Arc::new(DriveMemo::new(budget(), SymRules::shipped()));
    let opts = EvalOptions {
        param_box: Some(Arc::new(root)),
        ..EvalOptions::default()
    };
    with_session_memo_retry(
        budget(),
        SymRules::shipped(),
        SymRetry::kept_atom(),
        &table,
        || evaluate(doc, None, &CancelToken::new(), &opts, tol),
    )
}

/// What the family's value carries, against what its members carry,
/// and the leaf's receipt against the sum over `ev.order`.
#[test]
fn a_familys_value_and_the_receipt() {
    let tol = Tol::witness();
    let (doc, row) = plate_with_a_row(tol);
    let (ev, session) = leaf(&doc, tol);
    let mut nodes = SymCounts::default();
    let mut members_sum = SymCounts::default();
    let mut member_regs = 0;
    for id in &ev.order {
        let (record, members) = match ev.result(*id) {
            Some(NodeResult::Ok(v)) => (&v.sym, v.members.clone()),
            Some(NodeResult::Failed(e)) => (&e.sym, None),
            other => {
                println!("{id:?}: {}", if other.is_some() { "poisoned" } else { "absent" });
                continue;
            }
        };
        println!(
            "{id:?}{}: counts {:?} registrations {}",
            if *id == row { " (the row)" } else { "" },
            record.counts(),
            record.registrations()
        );
        nodes.absorb(record.counts());
        if let Some(members) = members {
            for (at, m) in &members.members {
                println!(
                    "   member {at:?}: counts {:?} registrations {}",
                    m.sym.counts(),
                    m.sym.registrations()
                );
                members_sum.absorb(m.sym.counts());
                member_regs += m.sym.registrations();
            }
        }
    }
    let session = SymCounts { frozen: 0, ..session };
    println!("session {session:?}\nnodes   {nodes:?}\nmembers {members_sum:?} regs {member_regs}");
    let family = ev.value(row).expect("the row built");
    assert!(members_sum.decisions() > 0, "the members decided: the probe reached its subject");
    assert_eq!(
        family.sym.counts(),
        members_sum,
        "the family's record holds what its members decided"
    );
    assert_eq!(session, nodes, "the leaf's receipt is the sum of its nodes' records");
}

/// A resolver over one in-memory document.
#[derive(Debug)]
struct OneDoc(ProfileDoc);

impl editor_core::PartResolver for OneDoc {
    fn resolve(
        &self,
        _doc_ref: &editor_core::DocRef,
        _tol: Tol,
    ) -> Result<ProfileDoc, editor_core::ResolveFailure> {
        Ok(self.0.clone())
    }
}

/// A cylinder: a circle of radius 1 extruded 1, placed in the world.
/// The document and its extrude.
fn cylinder(label: &str, tol: Tol) -> (ProfileDoc, editor_core::RecipeNodeId) {
    use editor_core::{LoopProgram, Node, ProfileProgram};
    let doc = ProfileDoc::empty(editor_core::DocumentId::derive(label), tol);
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [fixture::len(0.0), fixture::len(0.0)],
                radius: fixture::len(1.0),
            }],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, body) = fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: fixture::len(1.0),
        },
    );
    (fixture::place(doc, body).0, body)
}

fn leaf_with(
    doc: &ProfileDoc,
    resolver: Option<Arc<dyn editor_core::PartResolver>>,
    tol: Tol,
) -> (Evaluation<Sym<Interval>>, SymCounts) {
    let root = ParamBox::of(&analyzed_box(doc, &AnalysisPolicy::default()));
    let table = Arc::new(DriveMemo::new(budget(), SymRules::shipped()));
    let opts = EvalOptions {
        param_box: Some(Arc::new(root)),
        resolver,
        ..EvalOptions::default()
    };
    with_session_memo_retry(
        budget(),
        SymRules::shipped(),
        SymRetry::kept_atom(),
        &table,
        || evaluate(doc, None, &CancelToken::new(), &opts, tol),
    )
}

fn sum_of_records(ev: &Evaluation<Sym<Interval>>) -> (SymCounts, usize) {
    let mut nodes = SymCounts::default();
    let mut registrations = 0;
    for id in &ev.order {
        let record = match ev.result(*id) {
            Some(NodeResult::Ok(v)) => &v.sym,
            Some(NodeResult::Failed(e)) => &e.sym,
            _ => continue,
        };
        println!("{id:?}: counts {:?} registrations {}", record.counts(), record.registrations());
        nodes.absorb(record.counts());
        registrations += record.registrations();
    }
    (nodes, registrations)
}

/// The cylinder evaluated as a document of its own, and the same
/// cylinder instanced as a part: what the nodes' records carry of its
/// decisions and of the identities its extrude registered.
#[test]
fn a_parts_nodes_and_the_receipt() {
    let tol = Tol::witness();
    let (part, _) = cylinder("review-part", tol);
    let (own, own_session) = leaf_with(&part, None, tol);
    println!("-- the cylinder as its own document");
    let (own_nodes, own_registrations) = sum_of_records(&own);
    let own_session = SymCounts { frozen: 0, ..own_session };
    println!("session {own_session:?}\nnodes   {own_nodes:?} registrations {own_registrations}");
    assert_eq!(own_session, own_nodes, "as its own document the receipt is the sum");

    let pin = editor_core::content_pin(&part, tol).expect("the pin computes");
    let doc_ref = editor_core::DocRef { id: part.id(), pin };
    let doc = ProfileDoc::empty(editor_core::DocumentId::derive("review-assembly"), tol);
    let (doc, _instance) = fixture::insert(doc, fixture::mated_instance(doc_ref));
    let doc = fixture::assemble_world_copies(doc);
    let (ev, session) = leaf_with(&doc, Some(Arc::new(OneDoc(part))), tol);
    println!("-- the cylinder instanced");
    for id in &ev.order {
        if let Some(NodeResult::Failed(e)) = ev.result(*id) {
            println!("FAILED {id:?}: {:?}", e.kind);
        }
    }
    let (nodes, registrations) = sum_of_records(&ev);
    let session = SymCounts { frozen: 0, ..session };
    println!("session {session:?}\nnodes   {nodes:?} registrations {registrations}");
    assert!(session.decisions() > 0, "the instanced leaf decided");
    assert_eq!(
        registrations, own_registrations,
        "the identities the part's extrude registered ride some node's record"
    );
    assert_eq!(session, nodes, "the leaf's receipt is the sum of its nodes' records");
}
