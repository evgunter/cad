//! Review probe (standing-rev): every door on the three standings,
//! Debug + Display printed as `PROBE|door|state|debug|display`, so the
//! same file compiled on `main` and on the PR head can be diffed.
//! Compiles against both trees: it names no type the PR added.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, insert, len, minted, on_frame, step};
use editor_core::{
    Attr, CancelToken, Cmp, DocEdit, EntityKind, EvalOptions, Evaluation, GeomPred, NamePat, Node,
    NodePick, ProfileDoc, RecipeNodeId, Resolution, Rgba8, RoleSeg, RunCtx, Selector, SlotId,
    VerdictVector, body_name, denotation, evaluate, product, resolve, select, select_where,
};
use geom_core::Tol;

fn run(doc: &ProfileDoc, cancel: &CancelToken) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, cancel, &EvalOptions::default(), Tol::witness())
}

fn out(door: &str, state: &str, debug: &dyn core::fmt::Debug, display: &str) {
    println!("PROBE|{door}|{state}|{debug:?}|{display}");
}

#[test]
fn standing_rev_every_door_on_every_standing() {
    let (doc, profile) = on_frame(
        ProfileDoc::empty_derived("node_standing", Tol::witness()),
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, failed) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let (doc, poisoned) = insert(
        doc,
        fixture::xform(failed, [2.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0),
    );
    let good = run(&doc, &CancelToken::new());
    // Appearance on both nodes' body names, set while they exist.
    let red = Attr::Color(Rgba8::opaque(200, 30, 30));
    let mut doc = doc;
    for n in [failed, poisoned] {
        let faces = select(&good, n, &Selector::of(NamePat::of_kind(EntityKind::Face)));
        let name = faces[0].clone();
        doc = step(
            doc,
            DocEdit::SetAppearance {
                name,
                attr: red.clone(),
            },
        )
        .0;
    }
    let (doc, _) = step(
        doc,
        DocEdit::SetRoots {
            roots: vec![poisoned],
        },
    );
    let good = run(&doc, &CancelToken::new());
    let pick_f = NodePick::build(&good, failed, 0, 0.1, Tol::witness()).expect("built");
    let pick_p = NodePick::build(&good, poisoned, 0, 0.1, Tol::witness()).expect("built");
    let (doc, _) = step(
        doc,
        DocEdit::SetParam {
            node: failed,
            slot: SlotId::Distance,
            expr: len(0.0),
        },
    );
    let broken = run(&doc, &CancelToken::new());
    let cancel = CancelToken::new();
    cancel.cancel();
    let canceled = run(&doc, &cancel);

    // (state label, eval, node, pick built on that node)
    let cases: [(&str, &Evaluation<f64>, RecipeNodeId, &NodePick); 5] = [
        ("failed", &broken, failed, &pick_f),
        ("poisoned", &broken, poisoned, &pick_p),
        ("unevaluated-canceled-f", &canceled, failed, &pick_f),
        ("unevaluated-canceled-p", &canceled, poisoned, &pick_p),
        ("foreign", &broken, RecipeNodeId(9999), &pick_f),
    ];
    for (state, ev, node, pick) in cases {
        let e = body_name(ev, node, 0).expect_err("hit");
        out("hit", state, &e, &e.to_string());
        let e = NodePick::build(ev, node, 0, 0.1, Tol::witness()).expect_err("build");
        out("pick_build", state, &e, &e.to_string());
        let e = NodePick::build_all(ev, node, 0.1, Tol::witness()).expect_err("build_all");
        out("pick_build_all", state, &e, &e.to_string());
        if state != "foreign" {
            let e = pick.patch_names(ev).expect_err("patch");
            out("patch_names", state, &e, &e.to_string());
            let e = pick.boundary_names(ev).expect_err("boundary");
            out("boundary_names", state, &e, &e.to_string());
        }
        let name = minted(EntityKind::Body, node, RoleSeg::OutputBody);
        let e = denotation(ev, node, &name).expect_err("interrogate");
        out("interrogate", state, &e, &e.to_string());
        let e = select_where(
            ev,
            profile,
            &Selector::of(NamePat::of_kind(EntityKind::Face)),
            &[GeomPred::DatumDistance {
                datum: node,
                cmp: Cmp::Approx,
                value: len(0.0),
            }],
            &doc.param_env::<f64>(),
            Tol::witness(),
        );
        match e {
            Ok(v) => out("select_where_datum", state, &v, "OK(silent)"),
            Err(e) => out("select_where_datum", state, &e, &e.to_string()),
        }
        let names = select(ev, node, &Selector::of(NamePat::of_kind(EntityKind::Face)));
        out("select", state, &names, "silent");
        let v = resolve(RunCtx { doc: &doc, eval: ev }, &name);
        match &v {
            Resolution::Indeterminate(c) => out("resolve", state, c, &c.to_string()),
            other => out("resolve", state, other, "not-indeterminate"),
        }
        let status: Vec<_> = VerdictVector::of(ev)
            .rows
            .iter()
            .filter(|r| r.node == node)
            .map(|r| r.outcome)
            .collect();
        out("runstatus", state, &status, "");
    }
    for (label, ev) in [("broken", &broken), ("canceled", &canceled)] {
        for loss in &ev.appearance.losses {
            out("appearance", label, &loss.cause, "");
        }
        match product(&doc, ev, Tol::witness()) {
            Ok(_) => out("product", label, &"ok", ""),
            Err(e) => out("product", label, &e, &e.to_string()),
        }
    }
}

/// `RunStatus`'s JSON words and the four-outcome key, printed so the
/// value can be taken on `main` and on the head from one input.
#[test]
fn standing_rev_runstatus_key_on_this_tree() {
    use editor_core::{RunStatus, VerdictRow};
    let all = [
        RunStatus::Ok,
        RunStatus::Failed,
        RunStatus::Poisoned,
        RunStatus::Absent,
    ];
    let key = VerdictVector {
        rows: all
            .into_iter()
            .zip(1..)
            .map(|(outcome, id)| VerdictRow {
                node: RecipeNodeId(id),
                outcome,
                verdicts: Vec::new(),
            })
            .collect(),
    }
    .key();
    let words: Vec<String> = all
        .iter()
        .map(|s| serde_json::to_string(s).unwrap())
        .collect();
    println!("PROBE|runstatus_key|{}|{words:?}", key.0);
}

/// **RED probe: the checks registry's root refusal drops the standing.**
/// `checks.rs`'s `connectedness`/`chart_coherence` read each root with
/// `Evaluation::value` and refuse `ChecksError::Root { node }`, whose
/// sentence tells the author to "fix or remove the failing root" — for
/// a POISONED root, whose repair is upstream at `through`. The door is
/// in neither the spec's survey nor the PR's census (it spells no
/// `NodeResult::`), so no row sees it. Red while the refusal cannot
/// name the node the repair is at.
#[test]
fn standing_rev_checks_root_refusal_names_the_repair_node() {
    use editor_core::{ChecksConfig, run_checks};
    let (doc, profile) = on_frame(
        ProfileDoc::empty_derived("standing_rev_checks", Tol::witness()),
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, failed) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(0.0),
        },
    );
    let (doc, poisoned) = insert(
        doc,
        fixture::xform(failed, [2.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0),
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetRoots {
            roots: vec![poisoned],
        },
    );
    let ev = run(&doc, &CancelToken::new());
    let refusal = run_checks(&doc, &ev, &ChecksConfig::default(), Tol::witness())
        .expect_err("a root with no value refuses the registry");
    let text = refusal.to_string();
    println!("PROBE|checks|poisoned|{refusal:?}|{text}");
    assert!(
        text.contains(&format!("node {}", failed.0)),
        "the refusal names the node the repair is at ({}): {text}",
        failed.0
    );
}
