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

/// One leaf of `doc` over `box_` (its root box where none is given) on
/// `table`, from `prior` or from nothing; with the document's product
/// gathered inside the session where `product` asks.
fn run(
    doc: &ProfileDoc,
    box_: Option<&ParamBox>,
    resolver: Option<Arc<dyn editor_core::PartResolver>>,
    prior: Option<&Evaluation<Sym<Interval>>>,
    table: &Arc<DriveMemo>,
    product: bool,
    tol: Tol,
) -> (Evaluation<Sym<Interval>>, SymCounts, SymCounts) {
    let root = ParamBox::of(&analyzed_box(doc, &AnalysisPolicy::default()));
    let opts = EvalOptions {
        param_box: Some(Arc::new(box_.cloned().unwrap_or(root))),
        resolver,
        ..EvalOptions::default()
    };
    let ((ev, gathered), session) = with_session_memo_retry(
        budget(),
        SymRules::shipped(),
        SymRetry::kept_atom(),
        table,
        || {
            let ev: Evaluation<Sym<Interval>> = evaluate(doc, prior, &CancelToken::new(), &opts, tol);
            let gathered = if product {
                match editor_core::product::product_recorded(doc, &ev, tol) {
                    Ok(p) => p.sym.counts(),
                    Err(e) => {
                        println!("the product refused: {e:?}");
                        SymCounts::default()
                    }
                }
            } else {
                SymCounts::default()
            };
            (ev, gathered)
        },
    );
    (ev, SymCounts { frozen: 0, ..session }, gathered)
}

fn table() -> Arc<DriveMemo> {
    Arc::new(DriveMemo::new(budget(), SymRules::shipped()))
}

/// The nodes' records and the parts' runs', summed.
fn carried(ev: &Evaluation<Sym<Interval>>) -> SymCounts {
    let mut sum = SymCounts::default();
    for id in &ev.order {
        match ev.result(*id) {
            Some(NodeResult::Ok(v)) => sum.absorb(v.sym.counts()),
            Some(NodeResult::Failed(e)) => sum.absorb(e.sym.counts()),
            _ => {}
        }
    }
    for part in &ev.parts_sym {
        sum.absorb(part.counts());
    }
    sum
}

/// A cylinder: a circle of radius 1 extruded 1, placed in the world.
fn cylinder(label: &str, tol: Tol) -> ProfileDoc {
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
    fixture::place(doc, body).0
}

/// `part` instanced once in an assembly of its own.
fn instanced(part: &ProfileDoc, label: &str, tol: Tol) -> ProfileDoc {
    let pin = editor_core::content_pin(part, tol).expect("the pin computes");
    let doc_ref = editor_core::DocRef { id: part.id(), pin };
    let doc = ProfileDoc::empty(editor_core::DocumentId::derive(label), tol);
    let (doc, _instance) = fixture::insert(doc, fixture::mated_instance(doc_ref));
    fixture::assemble_world_copies(doc)
}

/// **A leaf's receipt is its nodes', its parts' and its product's
/// records, on a family, on two assemblies, with and without the
/// product gathered.**
#[test]
fn the_receipt_is_the_sum_on_a_family_and_two_assemblies() {
    let tol = Tol::witness();
    let family = {
        let p = crate::m10_7_plate::plate_with_holes(5.0e-5, 1.0e-5, tol);
        fixture::row(p.doc, p.holes[0], ("k", 3), editor_core::TargetAxis::Y, 5.0e-3).0
    };
    let cyl = cylinder("review-part", tol);
    let bracket = {
        let part = crate::m10_7_r2_probes_interval::bracket(1.0e1 * tol.eps(), tol).0;
        let body = part
            .ids()
            .iter()
            .copied()
            .find(|id| matches!(part.node(*id), Some(editor_core::Node::Extrude { .. })))
            .expect("the bracket has an extrude");
        fixture::place(part, body).0
    };
    let cases: Vec<(&str, ProfileDoc, Option<Arc<dyn editor_core::PartResolver>>)> = vec![
        ("family", family, None),
        (
            "cylinder instanced",
            instanced(&cyl, "review-assembly", tol),
            Some(Arc::new(OneDoc(cyl))),
        ),
        (
            "bracket instanced",
            instanced(&bracket, "review-bracket-assembly", tol),
            Some(Arc::new(OneDoc(bracket))),
        ),
    ];
    for (label, doc, resolver) in cases {
        for product in [false, true] {
            let (ev, session, gathered) = run(&doc, None, resolver.clone(), None, &table(), product, tol);
            let mut sum = carried(&ev);
            sum.absorb(gathered);
            println!(
                "{label} (product {product}): session {session:?}; nodes+parts+product {sum:?}; \
                 parts {} product {gathered:?}",
                ev.parts_sym.len()
            );
            assert!(session.decisions() > 0, "{label}: the leaf decided");
            assert_eq!(session, sum, "{label} (product {product})");
        }
    }
}

/// **A family of which a leaf rebuilds some members and takes the
/// others from an earlier leaf**: a row of three copies slid
/// `scalar(k)·pitch`, `pitch` toleranced. Copy 0 slides `0·pitch`, so
/// its number is the same over every box of `pitch` and the others' are
/// not. The half box is evaluated from the root leaf as its prior and
/// from nothing, and the family's record read in both.
#[test]
fn a_family_with_a_member_taken_from_an_earlier_leaf() {
    use editor_core::analysis::BoxAxis;
    use editor_core::{Dimension, Distribution, DocEdit, Formula, FreeVar, UnitSym, VarName};
    let tol = Tol::witness();
    let p = crate::m10_7_plate::plate_with_holes(5.0e-5, 1.0e-5, tol);
    let doc = fixture::step(
        p.doc,
        DocEdit::DeclareVar {
            name: VarName::from_static("pitch"),
            def: editor_core::VarDecl::Free(FreeVar::Continuous {
                dim: Dimension::Length,
                value: 5.0e-3,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: Some(Distribution::Uniform {
                    lo: -5.0e-5,
                    hi: 5.0e-5,
                }),
            }),
        },
    )
    .0;
    let pitch_var = doc.var_named("pitch").expect("pitch is declared");
    let (doc, k) = fixture::index(doc, "k", 3);
    let slide = Formula::mul(
        fixture::scalar_of(k),
        Formula::named(VarName::from_static("pitch"), Dimension::Length),
    )
    .expect("a length");
    let (doc, row) = fixture::family_of(
        doc,
        p.holes[0],
        vec![(editor_core::Coord::slide(editor_core::TargetAxis::Y), slide)],
    );
    let root = ParamBox::of(&analyzed_box(&doc, &AnalysisPolicy::default()));
    let (lo, hi) = root
        .varying()
        .find(|(var, _, _)| *var == pitch_var)
        .map(|(_, lo, hi)| (lo, hi))
        .expect("pitch varies");
    let mut axes = root.axes().clone();
    axes.insert(pitch_var, BoxAxis::Varying { lo, hi: 0.5 * (lo + hi) });
    let half = ParamBox::from_axes(axes);

    let shared = table();
    let (parent, _, _) = run(&doc, Some(&root), None, None, &shared, false, tol);
    let (served, served_session, _) = run(&doc, Some(&half), None, Some(&parent), &shared, false, tol);
    let (fresh, fresh_session, _) = run(&doc, Some(&half), None, None, &table(), false, tol);
    println!(
        "prior refused {:?}; reused {} recomputed {}",
        served.prior_refused, served.reused, served.recomputed
    );
    let members = |ev: &Evaluation<Sym<Interval>>| {
        ev.value(row)
            .and_then(|v| v.members.clone())
            .expect("the row is a family")
    };
    let taken: Vec<bool> = members(&served)
        .members
        .iter()
        .zip(&members(&parent).members)
        .map(|((_, now), (_, then))| Arc::ptr_eq(&now.sym, &then.sym))
        .collect();
    println!("members whose record is the root leaf's own: {taken:?}");
    let of = |ev: &Evaluation<Sym<Interval>>| {
        let v = ev.value(row).expect("the row built");
        let mut of_members = SymCounts::default();
        for (_, m) in &members(ev).members {
            of_members.absorb(m.sym.counts());
        }
        (v.sym.counts(), of_members, v.sym.registrations(), v.sym.read())
    };
    let (s, f) = (of(&served), of(&fresh));
    println!("served family: record {:?} | members {:?} | registrations {} read {}", s.0, s.1, s.2, s.3);
    println!("fresh  family: record {:?} | members {:?} | registrations {} read {}", f.0, f.1, f.2, f.3);
    // The served leaf's receipt, the reused results' records added
    // (nodes whose record is the root leaf's own, and members likewise).
    let mut with_served = served_session;
    for id in &served.order {
        if let (Some(NodeResult::Ok(now)), Some(NodeResult::Ok(then))) =
            (served.result(*id), parent.result(*id))
            && Arc::ptr_eq(&now.sym, &then.sym)
        {
            with_served.absorb(now.sym.counts());
        }
    }
    println!("served receipt + reused nodes' records {with_served:?}; fresh receipt {fresh_session:?}");
    assert!(taken.iter().any(|t| *t) && !taken.iter().all(|t| *t), "some members are taken and some rebuilt");
    assert_eq!(s.0, f.0, "the family's record is the same served or built");
}

/// **The evaluator's own memo as the store, across two leaves of one
/// drive's table**: the plate over its root box, then over the first
/// half of each varying axis from the root leaf as its prior, against
/// the same half evaluated from nothing.
#[test]
fn a_leaf_served_from_an_earlier_leaf_of_the_drive() {
    use editor_core::analysis::BoxAxis;
    let tol = Tol::witness();
    let texts = |ev: &Evaluation<Sym<Interval>>| -> Vec<(String, String)> {
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
    };
    for (label, doc) in [
        ("plate", crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol).0),
        ("bracket", crate::m10_7_r2_probes_interval::bracket(1.0e1 * tol.eps(), tol).0),
    ] {
        let root = ParamBox::of(&analyzed_box(&doc, &AnalysisPolicy::default()));
        let shared = table();
        let (parent, _, _) = run(&doc, Some(&root), None, None, &shared, false, tol);
        for (var, lo, hi) in root.varying() {
            let mut axes = root.axes().clone();
            axes.insert(var, BoxAxis::Varying { lo, hi: 0.5 * (lo + hi) });
            let half = ParamBox::from_axes(axes);
            let (served, served_session, _) =
                run(&doc, Some(&half), None, Some(&parent), &shared, false, tol);
            let (fresh, fresh_session, _) = run(&doc, Some(&half), None, None, &table(), false, tol);
            let differing: Vec<String> = texts(&served)
                .into_iter()
                .zip(texts(&fresh))
                .filter(|(a, b)| a != b)
                .map(|(a, _)| a.0)
                .collect();
            let mut with_served = served_session;
            for id in &served.order {
                if let (Some(NodeResult::Ok(now)), Some(NodeResult::Ok(then))) =
                    (served.result(*id), parent.result(*id))
                    && Arc::ptr_eq(&now.sym, &then.sym)
                {
                    with_served.absorb(now.sym.counts());
                }
            }
            println!(
                "{label} axis {var:?}: refused {:?} reused {} recomputed {}; differing {}; \
                 served+reused {with_served:?}; fresh {fresh_session:?}",
                served.prior_refused,
                served.reused,
                served.recomputed,
                differing.len()
            );
            assert!(differing.is_empty(), "{label}: served and built agree node by node: {differing:?}");
            assert_eq!(with_served, fresh_session, "{label}: the receipt counts served or built");
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

