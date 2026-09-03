//! **M6 unit 1 at the recipe layer — the blocker, FLIPPED** (M6-5).
//!
//! # What this file used to pin
//!
//! `Node::Fillet` was every-edge BY DESIGN: no stable edge names meant
//! nothing to go stale under a bump. A pipped cube's edge set includes
//! the cap's MERIDIAN seam edges — two half-cap faces on ONE sphere
//! surface — and a co-surface seam has no dihedral wedge, so the
//! battery honestly refused `TangentialEdge` at exactly zero margin,
//! at any radius. This file pinned that refusal, executed, and said:
//! the day `Node::Fillet` can name a subset, the pin flips and
//! `die_composed` joins `documents()`.
//!
//! # What it pins now
//!
//! That day is M6-5. The node grew a `selection: Vec<StableName>`
//! (ruled #217), the composition surgery grew per-entity birth records
//! and the naming emitter that reads them, and `die_composed` is
//! REGISTERED — so the corpus's standard rows (evaluation at every ε,
//! the interval lane, save/load/replay, latency) carry it for free.
//! What is left here is what membership does not check:
//!
//! - the refusal is still REAL — the two meridians still refuse
//!   `TangentialEdge` at zero margin, and the document builds because
//!   it leaves them OUT, not because anything got looser;
//! - the authored selection is exactly the target's edge set minus
//!   those two names (provenance in `corpus/die_composed.rs`);
//! - the surgery's table is TOTAL and its names are COVARIANT under
//!   the document's own bump — the freeze semantics, executed.
//!
//! The surgery itself is exercised at the kernel layer: the 21-pip
//! ladder in `sweep/tests/m6_surgery.rs`, its interval lane, the F-e
//! and birth-record rows in `sweep/tests/m6_5_fillet_naming.rs`, and
//! the demo tour's `diecomposed` stop.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;

use std::collections::BTreeSet;

use corpus::die_composed;
use editor_core::{
    CancelToken, DocEdit, EntityKind, EvalOptions, Node, NodeErrorKind, NodeResult, RoleSeg,
    StableName, apply, evaluate,
};
use geom_core::Tol;

fn eval(doc: &editor_core::ProfileDoc) -> editor_core::Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn edge_names(
    ev: &editor_core::Evaluation<f64>,
    node: editor_core::RecipeNodeId,
) -> Vec<StableName> {
    let Some(NodeResult::Ok(v)) = ev.nodes.get(&node) else {
        panic!("node {node:?} did not evaluate");
    };
    v.name_table
        .iter()
        .filter(|(n, _)| n.kind == EntityKind::Edge)
        .map(|(n, _)| n.clone())
        .collect()
}

/// The fillet node's stored selection.
fn selection_of(
    doc: &editor_core::ProfileDoc,
    fillet: editor_core::RecipeNodeId,
) -> Vec<StableName> {
    match doc.node(fillet) {
        Some(Node::Fillet { selection, .. }) => selection.clone(),
        other => panic!("expected a fillet node, got {other:?}"),
    }
}

/// The document's one fillet node, and the target it blends.
fn fillet_and_target(
    doc: &editor_core::ProfileDoc,
) -> (editor_core::RecipeNodeId, editor_core::RecipeNodeId) {
    for id in doc.order() {
        if let Some(Node::Fillet { target, .. }) = doc.node(*id) {
            return (*id, *target);
        }
    }
    panic!("the composed die has a fillet node")
}

/// **The blocker, flipped**: the composed-die recipe now EVALUATES —
/// one node, one `fillet_edges` call, the box blends and the pip-rim
/// torus band together (the F-e form, measured in
/// `sweep/tests/m6_5_fillet_naming.rs`).
#[test]
fn the_composed_die_recipe_evaluates() {
    let doc = die_composed::document();
    let ev = eval(&doc.doc);
    let result = doc.result.expect("the document names its result");
    match ev.nodes.get(&result) {
        Some(NodeResult::Ok(_)) => {}
        other => panic!("the composed die must evaluate, got {other:?}"),
    }
}

/// **The refusal is still real, and still typed.** Adding either
/// cavity meridian to the selection reproduces the original blocker
/// verbatim: `TangentialEdge` at a margin of exactly zero. The
/// document builds because it EXCLUDES a co-surface seam, not because
/// the battery got looser.
#[test]
fn adding_a_cavity_meridian_still_refuses_tangential_at_zero_margin() {
    let doc = die_composed::document();
    let (fillet, target) = fillet_and_target(&doc.doc);
    // The ball's names ride the Transform through unchanged, so the
    // `FromB` payloads name the revolve node — recovered here from the
    // target's own table rather than restated.
    let ball = edge_names(&eval(&doc.doc), target)
        .into_iter()
        .find_map(|n| match n.path.first() {
            Some(RoleSeg::FromB(inner)) => Some(inner.node),
            _ => None,
        })
        .expect("the cavity contributes FromB edges");

    let selection = selection_of(&doc.doc, fillet);
    for meridian in die_composed::excluded_meridians(ball, target) {
        // Grown the ONLY way a selection grows: an explicit `Rebind`
        // swapping one selected box edge for the meridian.
        let d = apply(
            &doc.doc,
            &DocEdit::Rebind {
                from: selection[0].clone(),
                to: meridian.clone(),
            },
            Tol::witness(),
        )
        .expect("rebinding a selected edge onto a cavity meridian")
        .doc;
        let ev = eval(&d);
        let Some(NodeResult::Failed(e)) = ev.nodes.get(&fillet) else {
            panic!("selecting {meridian:?} must refuse");
        };
        match &e.kind {
            NodeErrorKind::Blend {
                error: sweep::blend::BlendError::TangentialEdge { margin, .. },
                ..
            } => {
                assert_eq!(*margin, 0.0, "a co-surface seam has exactly no wedge");
            }
            other => panic!("expected the battery's TangentialEdge refusal, got {other:?}"),
        }
    }
}

/// **The selection is exactly the target's edges minus the two
/// meridians** — the authored set checked against the live table, so a
/// drift in the boolean emitter fails HERE instead of silently
/// reselecting.
#[test]
fn the_selection_is_every_target_edge_except_the_cavity_meridians() {
    let doc = die_composed::document();
    let (fillet, target) = fillet_and_target(&doc.doc);
    let ev = eval(&doc.doc);
    let live: BTreeSet<StableName> = edge_names(&ev, target).into_iter().collect();
    let selection = selection_of(&doc.doc, fillet);
    let chosen: BTreeSet<StableName> = selection.iter().cloned().collect();
    assert_eq!(
        selection.len(),
        14,
        "twelve box edges and the rim's two arcs"
    );
    assert!(
        chosen.is_subset(&live),
        "every selected name is a live edge"
    );
    let left_out: Vec<_> = live.difference(&chosen).cloned().collect();
    assert_eq!(left_out.len(), 2, "exactly the two cavity meridians");
    for n in &left_out {
        assert!(
            matches!(n.path.first(), Some(RoleSeg::FromB(inner))
                if matches!(inner.path.first(), Some(RoleSeg::Meridian(..)))),
            "the excluded names are the cavity's meridians, got {n:?}"
        );
    }
    // The canonical form the content key depends on.
    assert!(
        selection.windows(2).all(|w| w[0] < w[1]),
        "the stored selection is sorted and deduplicated"
    );
}

/// **Totality**: the surgery names EVERY boundary entity of its
/// result. The emitter checks this itself (`check_total`), so an
/// unnamed entity would already be a failed node — this row states the
/// claim positively and pins the counts the vocabulary produces.
#[test]
fn the_surgery_names_every_entity_of_the_composed_die() {
    let doc = die_composed::document();
    let ev = eval(&doc.doc);
    let result = doc.result.expect("a result");
    let Some(NodeResult::Ok(v)) = ev.nodes.get(&result) else {
        panic!("the composed die evaluates")
    };
    let editor_core::ValuePayload::Body(body) = &v.payload else {
        panic!("a body")
    };
    let entities = body.faces().count() + body.edges().count() + body.vertices().count();
    assert_eq!(
        v.name_table.len(),
        entities + 1,
        "every face, edge and vertex is named, plus the body row"
    );
    // Every fillet vocabulary segment in use is a fillet segment (no
    // leakage of another op's roles into this node's names).
    let role = |n: &StableName| match n.path.first().expect("a role path") {
        RoleSeg::OutputBody => "body",
        RoleSeg::FromTarget(_) => "survivor",
        RoleSeg::BlendFace(_) => "blend",
        RoleSeg::CornerFace(_) => "octant",
        RoleSeg::TrimEdge { .. } => "trim",
        RoleSeg::FootVertex { .. } => "foot",
        RoleSeg::CornerArc { .. } => "arc",
        RoleSeg::BandFace(_) => "band",
        RoleSeg::BandTrim { .. } => "band trim",
        RoleSeg::BandFoot(_) => "band foot",
        RoleSeg::BandCross(_) => "band cross",
        RoleSeg::BandCut(_) => "band cut",
        RoleSeg::BandSlit(_) => "slit",
        other => panic!("a non-fillet role leaked into the fillet's table: {other:?}"),
    };
    let seen: BTreeSet<&str> = v.name_table.iter().map(|(n, _)| role(n)).collect();
    assert_eq!(
        seen.len(),
        13,
        "every fillet role the composed die can produce is produced, got {seen:?}"
    );
}

/// **The freeze semantics, executed** (the #217 ruling). The corpus
/// bump slides the pip. The bump mints no edge and retires none, so
/// the frozen selection STILL RESOLVES — bit-identically, because
/// every selection name is a function of the target's names and the
/// target's names did not move — and the result is the bumped die.
///
/// This is the covariance claim: the emitter contributes no judgment
/// of its own that could disagree with the target's naming.
#[test]
fn the_selection_survives_the_corpus_bump_and_names_stay_covariant() {
    let doc = die_composed::document();
    let (fillet, target) = fillet_and_target(&doc.doc);
    let before = eval(&doc.doc);
    let bumped = apply(&doc.doc, &doc.bump, Tol::witness())
        .expect("the corpus bump applies")
        .doc;
    let after = eval(&bumped);

    // The bump really recomputes the fillet's cone.
    let result = doc.result.expect("a result");
    match after.nodes.get(&result) {
        Some(NodeResult::Ok(_)) => {}
        other => panic!("the bumped composed die must evaluate, got {other:?}"),
    }
    // The selection is untouched by the bump — a `SetParam` is not a
    // `Rebind`, and only a `Rebind` grows a selection.
    assert_eq!(
        selection_of(&doc.doc, fillet),
        selection_of(&bumped, fillet),
        "the bump does not touch the set"
    );

    // The target's names, and therefore the fillet's, are unchanged.
    let names_of = |ev: &editor_core::Evaluation<f64>, n| {
        let Some(NodeResult::Ok(v)) = ev.nodes.get(&n) else {
            panic!("node {n:?} evaluates")
        };
        v.name_table
            .iter()
            .map(|(k, _)| k.clone())
            .collect::<BTreeSet<_>>()
    };
    assert_eq!(
        names_of(&before, target),
        names_of(&after, target),
        "the pip's motion mints no name at the target"
    );
    assert_eq!(
        names_of(&before, result),
        names_of(&after, result),
        "the fillet's names are a function of the target's — covariant"
    );
    // And every selected name still resolves in the bumped target.
    let Some(NodeResult::Ok(v)) = after.nodes.get(&target) else {
        panic!("the bumped target evaluates")
    };
    for n in selection_of(&bumped, fillet) {
        assert!(
            v.name_table.lookup(&n).is_some(),
            "a selected name stopped resolving under the bump: {n:?}"
        );
    }
}

/// **`Rebind` is the repair path, and only that** (the ruling's other
/// half, stated at its true reach — review F-4). The selection never
/// moves on its own; an explicit rebind rewrites it 1:1 and
/// re-canonicalizes, so rebinding onto an already-selected edge
/// SHRINKS the set rather than duplicating a name. What a rebind
/// cannot do is make the set LARGER: adding an edge means
/// re-authoring the node, which is exactly what freezing is for.
#[test]
fn rebind_repairs_a_selection_and_can_never_grow_it() {
    let doc = die_composed::document();
    let (fillet, _) = fillet_and_target(&doc.doc);
    let before = selection_of(&doc.doc, fillet);
    let (from, to) = (before[0].clone(), before[1].clone());
    let after = apply(
        &doc.doc,
        &DocEdit::Rebind {
            from: from.clone(),
            to: to.clone(),
        },
        Tol::witness(),
    )
    .expect("rebinding one selected edge onto another")
    .doc;
    let grown = selection_of(&after, fillet);
    assert_eq!(grown.len(), before.len() - 1, "the two names merged to one");
    assert!(!grown.contains(&from), "the old name is gone");
    assert!(grown.contains(&to), "the new name is present exactly once");
    assert!(grown.windows(2).all(|w| w[0] < w[1]), "still canonical");
    assert!(
        grown.len() <= before.len(),
        "a 1:1 rewrite plus dedup can only swap or shrink — never extend"
    );
}
