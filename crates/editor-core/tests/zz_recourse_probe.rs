//! Reviewer probes (recourse-rev); not for commit.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::block;
use crate::fixture;
use editor_core::{
    Dimension, DocEdit, DocParam, DocumentId, Node, ParamName, ProfileDoc, RecipeNodeId,
    SitedRef, UpstreamCause, apply,
};
use fixture::{fname, insert, run, wall};
use geom_core::{Sign, Tol};
use std::collections::BTreeSet;

fn p(name: &'static str) -> ParamName {
    ParamName::from_static(name)
}

fn ok(doc: &ProfileDoc, edit: DocEdit<editor_core::ProfileProgram>) -> ProfileDoc {
    apply(doc, &edit, Tol::witness(), &editor_core::RefusingReach)
        .expect("accepted")
        .doc
}

#[test]
fn probe_count_param_unit_and_distribution_way_through() {
    let doc = ProfileDoc::empty(DocumentId::derive("probe-count"), Tol::witness());
    let doc = ok(
        &doc,
        DocEdit::SetDocParam {
            name: p("n"),
            value: DocParam::Count { value: 4 },
        },
    );
    let unit = apply(
        &doc,
        &DocEdit::SetDocParamUnit {
            name: p("n"),
            unit: editor_core::UnitSym::from_def(&quantity::MM.def()),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect_err("count has no unit");
    println!("PROBE unit door: {unit}");
    // The way through the sentence denies: redeclare it continuous.
    let redeclared = apply(
        &doc,
        &DocEdit::SetDocParam {
            name: p("n"),
            value: DocParam::written_length(quantity::WrittenLength::in_unit(4.0, quantity::MM)),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    println!("PROBE redeclare as length (unreferenced): {:?}", redeclared.as_ref().map(|a| a.doc.params().get(&p("n")).cloned()).map_err(|e| e.to_string()));
    let redeclared_dimless = apply(
        &doc,
        &DocEdit::SetDocParam {
            name: p("n"),
            value: DocParam::continuous(Dimension::Scalar, 4.0),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    println!("PROBE redeclare as scalar (unreferenced): {:?}", redeclared_dimless.as_ref().map(|a| a.doc.params().get(&p("n")).cloned()).map_err(|e| e.to_string()));
    // The value door's sibling offers redeclaration for the same kind change.
    let val = apply(
        &doc,
        &DocEdit::SetDocParamValue {
            name: p("n"),
            value: editor_core::DocParamValue::Continuous(0.004),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    println!("PROBE value door: {}", val.err().map(|e| e.to_string()).unwrap_or_default());
}

/// A Declare rebound to a node inserted after it, then split whole.
#[test]
fn probe_split_forward_declare() {
    let doc = ProfileDoc::empty_derived("probe_split_fwd", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let wa = wall(&doc, a, 0);
    let wb = wall(&doc, b, 0);
    let (doc, _decl) = insert(
        doc,
        Node::declare_rest(vec![(
            SitedRef::new(a, fname(a, wa)),
            SitedRef::new(b, fname(b, wb.clone())),
        )]),
    );
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let wc = wall(&doc, c, 0);
    let doc = ok(
        &doc,
        DocEdit::Rebind {
            from: fname(b, wb),
            to: fname(c, wc),
        },
    );
    let cut: BTreeSet<RecipeNodeId> = doc.order().iter().copied().collect();
    let err = editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("probe-split-fwd-part"),
        Tol::witness(),
        None,
    )
    .err()
    .expect("the split refuses");
    println!("PROBE split: {err}\nPROBE split debug: {err:?}");
}

#[test]
fn probe_flip_predicates_in_real_logs() {
    // A profile + extrude + boolean: every predicate the logs record.
    let doc = ProfileDoc::empty_derived("probe_flips", Tol::witness());
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, m) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
    let (doc, cut) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Subtract,
            a,
            b: m,
            declare: None,
        },
    );
    let ev = run(&doc, &editor_core::EvalOptions::default());
    let mut preds: BTreeSet<&'static str> = BTreeSet::new();
    for id in doc.order() {
        if let Some(v) = ev.value(*id) {
            for verdict in v.verdicts.iter() {
                preds.insert(verdict.predicate);
            }
        }
    }
    let _ = cut;
    for pred in preds {
        let text = UpstreamCause::PredicateFlip {
            predicate: pred,
            at: RecipeNodeId(1),
            from: Sign::Negative,
            to: Sign::Positive,
        }
        .to_string();
        let unnamed = text.starts_with(geom_core::UNNAMED_DECISION);
        let words = profile::decision_subject(pred);
        println!("PROBE flip {pred}: unnamed={unnamed} profile_words={words:?} -> {text}");
    }
}
