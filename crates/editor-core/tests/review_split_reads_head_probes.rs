//! Reviewer probes for PR 7458 that need the head's API
//! (`Doc::free_reads`, `RefusalReason::ReadsNoVariable`). `#[ignore]`d;
//! see `review_split_reads_probes.rs` for the run line.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::fmt::Write as _;

use editor_core::{DocEdit, FreeVar, NodeResult, ProfileDoc, VarId};
use geom_core::Tol;

fn eval_f64(doc: &ProfileDoc) -> editor_core::Evaluation<f64> {
    editor_core::evaluate::<f64>(
        doc,
        None,
        &editor_core::CancelToken::new(),
        &editor_core::EvalOptions::default(),
        Tol::witness(),
    )
}

/// A node's outcome, as far as the f64 lane can tell: its content key
/// when it builds, its typed failure's variant when it fails, poisoned.
fn status(ev: &editor_core::Evaluation<f64>, node: editor_core::RecipeNodeId) -> String {
    match ev.nodes.get(&node) {
        Some(NodeResult::Ok(v)) => format!("ok {:?}", v.content_key),
        Some(NodeResult::Failed(e)) => {
            let k = format!("{:?}", e.kind);
            format!("failed {}", k.split(['{', '(']).next().unwrap_or(&k))
        }
        Some(NodeResult::Poisoned { .. }) => "poisoned".to_owned(),
        None => "absent".to_owned(),
    }
}

/// **The author's differential, widened**: three magnitudes of move
/// (a part in 2^20, 2%, 30%), no prior (every node rebuilt), and FAILURES
/// compared too (a node that fails at either value is compared by its
/// error's variant). Any node whose outcome moves while its read set
/// lacks the moved variable is written out. Also: every node of every
/// evaluation's order is a key of `free_reads`.
#[test]
#[ignore = "reviewer probe: writes to REVIEW_OUT"]
fn head_read_set_differential_with_failures() {
    let mut s = String::new();
    let (mut compared, mut moved, mut failing_pairs, mut skipped) = (0usize, 0usize, 0usize, 0);
    for d in crate::corpus::documents() {
        let reads = d.doc.free_reads();
        let nominal = eval_f64(&d.doc);
        for n in &nominal.order {
            if !reads.contains_key(n) {
                let _ = writeln!(s, "ORDER-NOT-IN-READS {} {}", d.name, d.doc.spoken(*n));
            }
        }
        let vars: Vec<(VarId, f64)> = d
            .doc
            .free_vars()
            .filter_map(|(id, f)| match f {
                FreeVar::Continuous { value, .. } => Some((id, *value)),
                FreeVar::Count { .. } => None,
            })
            .collect();
        for (var, value) in vars {
            for rel in [1.0 / f64::from(1u32 << 20), 0.02, 0.3] {
                let to = value + value.abs().max(1.0) * rel;
                let Ok(o) = editor_core::apply(
                    &d.doc,
                    &DocEdit::SetVarValue {
                        var: editor_core::VarRef::Id(var),
                        value: editor_core::FreeValue::Continuous(to),
                    },
                    Tol::witness(),
                ) else {
                    skipped += 1;
                    continue;
                };
                let doc = o.doc;
                let Ok(there) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    eval_f64(&doc)
                })) else {
                    let _ = writeln!(
                        s,
                        "PANIC {} rel={rel:e} {} = {to:?} (nominal {value:?})",
                        d.name,
                        d.doc.spoken_var(var)
                    );
                    continue;
                };
                for &node in &nominal.order {
                    let (a, b) = (status(&nominal, node), status(&there, node));
                    compared += 1;
                    if a.starts_with("failed") || b.starts_with("failed") {
                        failing_pairs += 1;
                    }
                    if a == b {
                        continue;
                    }
                    moved += 1;
                    if !reads.get(&node).is_some_and(|set| set.contains(&var)) {
                        let _ = writeln!(
                            s,
                            "MISSED {} rel={rel:e} {} moved {}: {a} -> {b}",
                            d.name,
                            d.doc.spoken_var(var),
                            d.doc.spoken(node)
                        );
                    }
                }
            }
        }
    }
    let _ = writeln!(
        s,
        "compared {compared} moved {moved} failing-pairs {failing_pairs} edits-refused {skipped}"
    );
    let dir = std::env::var("REVIEW_OUT").unwrap();
    std::fs::write(format!("{dir}/differential.txt"), &s).unwrap();
}

/// **Claim 2**: two undecided nodes, the first's read set at its depth
/// bound and the second's not. E6: of the deciding node's variables with
/// depth left, none, so `Budget(Depth)`; the second node's variable is
/// never split.
#[test]
#[ignore = "reviewer probe"]
fn head_first_node_spent_second_fresh_refuses_depth() {
    use editor_core::analysis::{AnalysisPolicy, analyzed_box};
    use editor_core::drive::{BudgetKind, DriveConfig, RefusalReason, drive};
    use editor_core::{
        Dimension, Distribution, Formula, LoopProgram, Node, ProfileProgram, UnitSym, VarName,
    };
    let eps = Tol::witness().eps();
    let mut r = crate::fixture::Recorder::new();
    for n in ["a", "b"] {
        r.push(DocEdit::DeclareVar {
            name: VarName::from_static(n),
            def: editor_core::VarDecl::Free(FreeVar::Continuous {
                dim: Dimension::Length,
                value: 20.0 * eps,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: Some(Distribution::Uniform {
                    lo: -40.0 * eps,
                    hi: 40.0 * eps,
                }),
            }),
        });
    }
    let len = crate::fixture::len;
    for n in ["a", "b"] {
        let at = |x: f64, y: f64| {
            editor_core::ProgramStep::LineTo(editor_core::ProgramTarget::Point([len(x), len(y)]))
        };
        let profile = r.insert(Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Chain(vec![
                editor_core::ProgramStep::At([len(0.0), len(0.0)]),
                at(1.0, 0.0),
                at(1.0, 1.0),
                at(0.0, 1.0),
                editor_core::ProgramStep::LineTo(editor_core::ProgramTarget::Start),
            ])],
            ids: Vec::new(),
            on: None,
        }));
        r.insert(Node::Extrude {
            profile: profile.into(),
            distance: Formula::named(VarName::from_static(n), Dimension::Length),
        });
    }
    let analyzed = analyzed_box(&r.doc, &AnalysisPolicy::default());
    let v = drive(
        &r.doc,
        &analyzed,
        &DriveConfig {
            max_depth: 1,
            max_leaves: 64,
            ..DriveConfig::default()
        },
        &editor_core::Stop::default(),
        Tol::witness(),
    )
    .unwrap();
    let b = r.doc.var_named("b").unwrap();
    let reasons: Vec<_> = v.refused().iter().map(|l| l.reason.clone()).collect();
    eprintln!("{:?} {reasons:?}", v.receipt());
    let a = r.doc.var_named("a").unwrap();
    assert!(
        reasons
            .iter()
            .all(|r| *r == RefusalReason::Budget(BudgetKind::Depth { max_depth: 1 }))
    );
    // The half of `a` that still reaches zero: the first slab is undecided
    // there with `a` spent, so it refuses Depth and `b` stays whole.
    let low = v
        .refused()
        .iter()
        .find(|l| l.box_.get(a).unwrap().span().0 == v.root().get(a).unwrap().span().0)
        .expect("the low half of a refuses");
    assert_eq!(low.box_.get(b).unwrap().span(), v.root().get(b).unwrap().span());
}
