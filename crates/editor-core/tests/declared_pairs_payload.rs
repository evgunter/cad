//! **A boolean's declared pairs as its own payload**: what the content
//! key, the single-finding declare door and the split/inline remap each
//! owe the pairs now that they live on the node rather than on a node
//! of their own.
//!
//! Each row here is one a mutant survived before it existed: the
//! pairs' feed into the union's content key made a no-op, the declare
//! door replacing the whole list, and the remap passing sites through
//! unmapped all left the suite green.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::{block, declared_union, failure, run};
use crate::fixture;

use std::collections::BTreeSet;
use std::sync::Arc;

use editor_core::{
    BooleanCoincidence, CancelToken, DocEdit, DocumentId, EvalOptions, Evaluation, Node,
    ProfileDoc, RecipeNodeId, evaluate, inline, split,
};
use fixture::resolver::PartStore;
use fixture::{flush_pairs, insert, step};
use geom_core::Tol;

/// `doc` evaluated against the evaluation `prior` of an earlier
/// version, so a node whose content key did not move is served from it.
fn rerun(doc: &ProfileDoc, prior: &Evaluation<f64>) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        Some(prior),
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// Two blocks overlapping in x, so their caps and y-walls carry on
/// flush, under a union declaring those continuations — the declared
/// union `docm7_union_declare` builds to volume 1.5.
fn declared_overlap(id: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived(id, Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let pairs = flush_pairs(&doc, (a, a), (b, b));
    declared_union(doc, &[a, b], pairs)
}

/// **Clearing a live union's declaration recomputes it**, against the
/// evaluation of the declared version: one id, so the memo is reached
/// and only the content key can say the node moved. The cleared union
/// builds undeclared, and its body is the declared one's.
#[test]
fn clearing_a_live_declaration_is_not_served_from_the_memo() {
    let (doc, union) = declared_overlap("declared-pairs-key-clear");
    let prior = run(&doc);
    assert!(
        failure(&prior, union).is_none(),
        "the declared union builds"
    );
    let (cleared, _) = step(
        doc,
        DocEdit::SetDeclare {
            node: union,
            pairs: Vec::new(),
        },
    );
    let ev = rerun(&cleared, &prior);
    assert!(
        failure(&ev, union).is_none(),
        "the cleared union builds undeclared: {:?}",
        failure(&ev, union)
    );
    assert_eq!(
        (ev.recomputed, ev.reused),
        (1, cleared.ids().len() - 1),
        "the cleared union was served from the memo, or a member moved"
    );
    assert_eq!(
        fixture::built_bits(&ev, union),
        fixture::built_bits(&prior, union),
        "the undeclared union is the declared one's body"
    );
}

/// **A class flip on one id recomputes the union, and nothing else**:
/// the same pairs, one of them re-classed, is another declaration, and
/// the key is what says so.
#[test]
fn a_class_flip_on_a_live_union_recomputes_it_alone() {
    let (doc, union) = declared_overlap("declared-pairs-key-flip");
    let prior = run(&doc);
    let mut pairs = doc
        .node(union)
        .expect("the union is live")
        .declared_pairs()
        .to_vec();
    assert_eq!(pairs[0].1, BooleanCoincidence::Continuation);
    pairs[0].1 = BooleanCoincidence::REST;
    let (flipped, _) = step(doc, DocEdit::SetDeclare { node: union, pairs });
    let ev = rerun(&flipped, &prior);
    assert_eq!(
        (ev.recomputed, ev.reused),
        (1, flipped.ids().len() - 1),
        "the re-classed union was served from the memo, or a member moved"
    );
}

/// **`declare` ADDS each finding to the pairs the union holds**: three
/// slabs stacked as a stepped pyramid meet in two resting contacts, the
/// detector reports one finding at each, and declaring them one at a
/// time leaves both declared — a whole-list replace would hold only the
/// last. The undeclared stack builds, and the declared one is its body
/// bit for bit.
#[test]
fn declaring_each_finding_in_turn_adds_it_and_builds_the_undeclared_body() {
    let doc = ProfileDoc::empty_derived("declared-pairs-converge", Tol::witness());
    let (doc, low) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, mid) = block(doc, (0.5, 2.5), (0.5, 2.5), 1.0, 1.0);
    let (doc, top) = block(doc, (1.0, 2.0), (1.0, 2.0), 2.0, 1.0);
    let (mut doc, union) = insert(
        doc,
        Node::Union {
            members: vec![low.into(), mid.into(), top.into()],
            declare: Vec::new(),
        },
    );
    let undeclared = run(&doc);
    assert!(
        failure(&undeclared, union).is_none(),
        "{:?}",
        failure(&undeclared, union)
    );
    let volume = topo::mass_properties(crate::corpus::body_of(&undeclared, union), Tol::witness())
        .expect("the stack has mass")
        .volume;
    assert_eq!(volume, 9.0 + 4.0 + 1.0);
    let mut findings = Vec::new();
    for (a, b) in [(low, mid), (mid, top)] {
        let found = editor_core::find_flush_candidates(&undeclared, a, b, Tol::witness())
            .expect("the detector answers");
        assert_eq!(found.len(), 1, "one resting contact: {found:?}");
        findings.extend(found);
    }
    for finding in &findings {
        doc = editor_core::declare(&doc, union, finding, Tol::witness())
            .expect("the finding declares on its union")
            .doc;
    }
    assert_eq!(
        doc.node(union).expect("live").declared_pairs().len(),
        2,
        "each contact declared once"
    );
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    assert_eq!(
        fixture::built_bits(&ev, union),
        fixture::built_bits(&undeclared, union),
        "the declared stack is the undeclared one's body"
    );
}

/// **A declared union split into a part and inlined back builds**, its
/// pairs' sites following its members through both re-mints: the part
/// is rebuilt from empty with new ids, and the inline splices it into
/// the host with new ids again. A site left on an old id would name
/// no member of the re-minted union.
#[test]
fn a_declared_union_survives_a_split_and_an_inline() {
    let tol = Tol::witness();
    let (doc, union) = declared_overlap("declared-pairs-remap");
    let doc = fixture::place(doc, union).0;
    let whole: BTreeSet<RecipeNodeId> = doc.ids().iter().copied().collect();
    let out = split(
        &doc,
        &whole,
        DocumentId::derive("declared-pairs-remap-part"),
        tol,
        None,
    )
    .expect("the whole document splits");
    let sited_at_members = |doc: &ProfileDoc| {
        doc.ids().iter().any(|&id| match doc.node(id) {
            Some(Node::Union { members, declare }) => {
                !declare.is_empty()
                    && declare
                        .iter()
                        .flat_map(|((one, two), _)| [one.at, two.at])
                        .all(|at| members.iter().any(|&m| doc.operation_of(m) == Some(at)))
            }
            _ => false,
        })
    };
    assert!(
        sited_at_members(&out.part),
        "the part's union reads its pairs at its own members"
    );
    let instance = *out
        .remainder
        .ids()
        .iter()
        .find(|&&id| matches!(out.remainder.node(id), Some(Node::InstantiatePart { .. })))
        .expect("the remainder holds the instance");
    let mut store = PartStore::default();
    store.insert(out.part, tol);
    let resolver: Arc<dyn editor_core::PartResolver> = Arc::new(store);
    let back = inline(&out.remainder, instance, &resolver, tol).expect("the part inlines");
    assert!(
        sited_at_members(&back.doc),
        "the inlined union reads its pairs at its own members"
    );
    let ev = run(&back.doc);
    let spliced = *back
        .doc
        .ids()
        .iter()
        .find(|&&id| matches!(back.doc.node(id), Some(Node::Union { .. })))
        .expect("the union is spliced back");
    assert_ne!(spliced, union, "the inline re-mints the union");
    assert!(
        failure(&ev, spliced).is_none(),
        "{:?}",
        failure(&ev, spliced)
    );
    let volume = topo::mass_properties(crate::corpus::body_of(&ev, spliced), tol)
        .expect("the union has mass")
        .volume;
    assert_eq!(volume, 1.5, "the round trip is the two blocks' union");
}
