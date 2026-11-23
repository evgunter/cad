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

/// One leaf of `doc` over `box_` on `table`, from `prior` or from
/// nothing.
fn leaf_from(
    doc: &ProfileDoc,
    box_: &ParamBox,
    table: Option<&Arc<DriveMemo>>,
    prior: Option<&Evaluation<Sym<Interval>>>,
    tol: Tol,
) -> (Evaluation<Sym<Interval>>, SymCounts) {
    let opts = EvalOptions {
        param_box: Some(Arc::new(box_.clone())),
        ..EvalOptions::default()
    };
    let run = || evaluate(doc, prior, &CancelToken::new(), &opts, tol);
    match table {
        Some(table) => with_session_memo_retry(
            budget(),
            SymRules::shipped(),
            SymRetry::kept_atom(),
            table,
            run,
        ),
        None => geom_core::sym::with_session_retry(
            budget(),
            SymRules::shipped(),
            SymRetry::kept_atom(),
            run,
        ),
    }
}

fn texts(ev: &Evaluation<Sym<Interval>>) -> Vec<(String, String)> {
    ev.order
        .iter()
        .map(|id| {
            let text = match ev.result(*id) {
                Some(NodeResult::Ok(v)) => format!(
                    "ok {:?} {:?} | {:?} | {:?} | {:?} | {:?}",
                    v.content_key, v.naming_key, v.verdicts, v.escalations, v.marks, v.sym
                ),
                Some(NodeResult::Failed(e)) => format!("failed {:?} | {:?}", e.kind, e.sym),
                other => format!("{other:?}"),
            };
            (format!("{id:?}"), text)
        })
        .collect()
}

/// **The evaluator's own memo as the store, across two leaves of one
/// drive's table**: the plate over its root box, then over the first
/// half of each varying axis from the root leaf as its prior, against
/// the same half evaluated from nothing. Every node's result is read,
/// and the half's receipt with the reused nodes' records added.
#[test]
fn a_leaf_served_from_an_earlier_leaf_of_the_drive() {
    use editor_core::analysis::BoxAxis;
    let tol = Tol::witness();
    let doc = crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol).0;
    let root = ParamBox::of(&analyzed_box(&doc, &AnalysisPolicy::default()));
    let table = Arc::new(DriveMemo::new(budget(), SymRules::shipped()));
    let (parent, _) = leaf_from(&doc, &root, Some(&table), None, tol);
    for (var, lo, hi) in root.varying() {
        let mut axes = root.axes().clone();
        axes.insert(var, BoxAxis::Varying { lo, hi: 0.5 * (lo + hi) });
        let half = ParamBox::from_axes(axes);
        let (served, served_session) = leaf_from(&doc, &half, Some(&table), Some(&parent), tol);
        let fresh_table = Arc::new(DriveMemo::new(budget(), SymRules::shipped()));
        let (fresh, fresh_session) = leaf_from(&doc, &half, Some(&fresh_table), None, tol);
        let differing: Vec<String> = texts(&served)
            .into_iter()
            .zip(texts(&fresh))
            .filter(|(a, b)| a != b)
            .map(|(a, _)| a.0)
            .collect();
        // The served leaf's receipt, the reused nodes' records added.
        let mut with_served = SymCounts { frozen: 0, ..served_session };
        for id in &served.order {
            let (Some(NodeResult::Ok(now)), Some(NodeResult::Ok(then))) =
                (served.result(*id), parent.result(*id))
            else {
                continue;
            };
            if Arc::ptr_eq(&now.sym, &then.sym) {
                with_served.absorb(now.sym.counts());
            }
        }
        println!(
            "axis {var:?}: reused {} recomputed {}; nodes differing from the fresh leaf: {}; \
             served receipt + reused records {with_served:?}; fresh receipt {fresh_session:?}; \
             served frozen {} fresh frozen {}",
            served.reused,
            served.recomputed,
            differing.len(),
            served_session.frozen,
            fresh_session.frozen
        );
        assert!(served.reused > 0, "the half reuses a node of the root leaf");
        assert!(differing.is_empty(), "served and built agree node by node: {differing:?}");
        assert_eq!(
            with_served,
            SymCounts { frozen: 0, ..fresh_session },
            "the receipt counts what the result rests on, served or built"
        );
    }
}

/// **A prior built outside the drive's table, served into a leaf on
/// it**: what the public evaluation door does with it.
#[test]
fn a_prior_built_under_another_session_served_into_a_drives_leaf() {
    use editor_core::analysis::BoxAxis;
    let tol = Tol::witness();
    let doc = crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol).0;
    let root = ParamBox::of(&analyzed_box(&doc, &AnalysisPolicy::default()));
    // The prior: the root box, in a session on no drive's table.
    let (prior, _) = leaf_from(&doc, &root, None, None, tol);
    for (var, lo, hi) in root.varying() {
        let mut axes = root.axes().clone();
        axes.insert(var, BoxAxis::Varying { lo, hi: 0.5 * (lo + hi) });
        let half = ParamBox::from_axes(axes);
        for (label, table) in [
            ("serving", DriveMemo::new(budget(), SymRules::shipped())),
            ("counting only", DriveMemo::counting_only(budget(), SymRules::shipped())),
        ] {
            let table = Arc::new(table);
            let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let (ev, session) = leaf_from(&doc, &half, Some(&table), Some(&prior), tol);
                (ev.reused, ev.recomputed, session)
            }));
            match outcome {
                Ok((reused, recomputed, session)) => {
                    println!("{var:?} {label}: reused {reused} recomputed {recomputed} {session:?}");
                }
                Err(panic) => {
                    let text = panic
                        .downcast_ref::<String>()
                        .cloned()
                        .or_else(|| panic.downcast_ref::<&str>().map(|s| (*s).to_string()))
                        .unwrap_or_default();
                    println!("{var:?} {label}: PANICKED: {text}");
                }
            }
        }
    }
}

fn peak_rss_mb() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VmHWM:"))
                .and_then(|l| l.split_whitespace().nth(1)?.parse::<u64>().ok())
        })
        .map_or(0, |kb| kb / 1024)
}

/// **The drive's table against the leaf count**, and one thread against
/// the default worker count: the wide bracket and the plate, each
/// driven to a growing leaf budget. Prints; asserts nothing but that
/// the drives ran.
#[test]
fn the_table_against_the_leaf_count() {
    use editor_core::drive::{DriveConfig, drive};
    let tol = Tol::witness();
    let docs = [
        ("bracket 1e3 eps", crate::m10_7_r2_probes_interval::bracket(1.0e3 * tol.eps(), tol).0),
        ("plate", crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol).0),
    ];
    for (label, doc) in &docs {
        let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
        for max_leaves in [2usize, 8, 32] {
            for parallel in [false, true] {
                let t0 = std::time::Instant::now();
                let verdict = drive(
                    doc,
                    &analyzed,
                    &DriveConfig {
                        max_leaves,
                        parallel,
                        ..DriveConfig::default()
                    },
                    &editor_core::Stop::default(),
                    tol,
                )
                .expect("the document drives");
                let leaves = verdict.certified().len() + verdict.refused().len();
                println!(
                    "{label}: max_leaves {max_leaves} parallel {parallel}: {leaves} leaves, \
                     {:.1} s, table {:?}, peak rss {} MB",
                    t0.elapsed().as_secs_f64(),
                    verdict.plain_memo(),
                    peak_rss_mb()
                );
                assert!(leaves > 0);
            }
        }
    }
}
