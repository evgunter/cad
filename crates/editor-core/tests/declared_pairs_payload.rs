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
    NodeErrorKind, ProfileDoc, RecipeNodeId, evaluate, inline, split,
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
/// and only the content key can say the node moved. The union refuses
/// the contacts it no longer declares rather than serving the body it
/// built when it declared them.
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
        matches!(
            failure(&ev, union),
            Some(NodeErrorKind::UndeclaredCoincidence { .. })
        ),
        "the cleared union served its declared body: {:?}",
        failure(&ev, union)
    );
    assert_eq!(
        (ev.recomputed, ev.reused),
        (1, cleared.order().len() - 1),
        "the union alone recomputes"
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
        (1, flipped.order().len() - 1),
        "the re-classed union was served from the memo, or a member moved"
    );
}

/// **Following the refusal one finding at a time builds**: three slabs
/// stacked as a stepped pyramid meet in two resting contacts, and a
/// union of them refuses one contact at a time. `declare` ADDS each
/// refusal's finding to the pairs the union holds, so the loop ends —
/// a whole-list replace would trade one contact for the other forever.
#[test]
fn declaring_each_refusals_finding_converges_on_a_union_that_builds() {
    let doc = ProfileDoc::empty_derived("declared-pairs-converge", Tol::witness());
    let (doc, low) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, mid) = block(doc, (0.5, 2.5), (0.5, 2.5), 1.0, 1.0);
    let (doc, top) = block(doc, (1.0, 2.0), (1.0, 2.0), 2.0, 1.0);
    let (mut doc, union) = insert(
        doc,
        Node::Union {
            members: vec![low, mid, top],
            declare: Vec::new(),
        },
    );
    let mut refusals = 0;
    let ev = loop {
        let ev = run(&doc);
        let Some(NodeErrorKind::UndeclaredCoincidence { finding, .. }) = failure(&ev, union) else {
            break ev;
        };
        refusals += 1;
        assert!(refusals <= 2, "the refusals do not converge");
        doc = editor_core::declare(&doc, union, finding, Tol::witness())
            .expect("the finding declares on its union")
            .doc;
    };
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    assert_eq!(refusals, 2, "one refusal per contact");
    assert_eq!(
        doc.node(union).expect("live").declared_pairs().len(),
        2,
        "each contact declared once"
    );
    let volume = topo::mass_properties(crate::corpus::body_of(&ev, union), Tol::witness())
        .expect("the stack has mass")
        .volume;
    assert_eq!(volume, 9.0 + 4.0 + 1.0);
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
    let whole: BTreeSet<RecipeNodeId> = doc.order().iter().copied().collect();
    let out = split(
        &doc,
        &whole,
        DocumentId::derive("declared-pairs-remap-part"),
        tol,
        None,
    )
    .expect("the whole document splits");
    let sited_at_members = |doc: &ProfileDoc| {
        doc.order().iter().any(|&id| match doc.node(id) {
            Some(Node::Union { members, declare }) => {
                !declare.is_empty()
                    && declare
                        .iter()
                        .flat_map(|((one, two), _)| [one.at, two.at])
                        .all(|at| members.contains(&at))
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
        .order()
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
        .order()
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
