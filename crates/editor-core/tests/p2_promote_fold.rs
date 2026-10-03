//! **`Promote` and `Fold`** (ASSEMBLY.md A4: "A part at a frame of its
//! own is two single-document edits around a split").
//!
//! `Promote` turns an instance's offset into a gauge under its gauge,
//! the instance and the rest of its group on it at the empty chain;
//! `Fold` dissolves a gauge, joining its placement's steps in front of
//! each dependent's own. Neither computes a frame, so each is the
//! other's inverse up to node ids, every evaluation is unchanged, and a
//! log holding them replays with no reach. Promoting a root and cutting
//! the group leaving the gauge behind makes a part at that frame;
//! cutting a gauge's content and folding the gauge left behind gives
//! the instance the gauge's placement as its offset.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use crate::p2_gauges::{literal, parts, placed_pair, seat, set_gauge, set_offset};
use crate::p2_split::{extent, round_trip, same_extent, split};

use editor_core::{
    DocEdit, DocumentId, EditError, Label, Node, Placement, ProfileDoc, RecipeNodeId,
    RefusingReach, apply_replayed,
};
use fixture::resolver::with_resolver;
use fixture::round_trip::{identity, same_up_to_ids};
use fixture::{head, insert, offset_of, solve, step};
use geom_core::Tol;

fn promote(doc: ProfileDoc, instance: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    let (doc, gauge) = step(doc, DocEdit::Promote { instance });
    (doc, gauge.expect("a promote mints its gauge"))
}

fn fold(doc: ProfileDoc, gauge: RecipeNodeId) -> ProfileDoc {
    step(doc, DocEdit::Fold { gauge }).0
}

fn refused(doc: &ProfileDoc, edit: DocEdit<editor_core::ProfileProgram>) -> EditError {
    let error = doc
        .apply(&edit, Tol::witness(), &RefusingReach)
        .map(|_| ())
        .expect_err("the edit refuses");
    assert!(error.to_string().contains("Recourse:"), "{error}");
    error
}

fn gauge_of(doc: &ProfileDoc, id: RecipeNodeId) -> Option<RecipeNodeId> {
    doc.node(id).and_then(Node::gauge_ref)
}

fn label(doc: &ProfileDoc, id: RecipeNodeId) -> Option<String> {
    doc.label(id).map(ToString::to_string)
}

fn labelled(doc: ProfileDoc, node: RecipeNodeId, text: &str) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetLabel {
            node,
            label: Some(Label::new(text).expect("a label")),
        },
    )
    .0
}

/// **`Promote` moves a group's frame onto a gauge, and `Fold` undoes
/// it**: a placed pair on a gauge g, its base at an offset. The promoted
/// gauge sits on g holding the base's offset, ahead of the base in the
/// root list; the base sits on it at the empty chain and the mated top
/// moves with it, so the mate still places; nothing moves. Folding the
/// gauge returns the document up to node ids, the base's label kept,
/// and both edits replay with no reach.
#[test]
fn promote_moves_a_groups_frame_onto_a_gauge_and_fold_undoes_it() {
    let (p, doc, [base, top, mate]) = placed_pair("pf-promote");
    let o = p.opts();
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 8.0, 0.0])));
    let doc = set_gauge(set_gauge(doc, base, Some(g)), top, Some(g));
    let doc = labelled(doc, base, "bracket");

    let (promoted, k) = promote(doc.clone(), base);
    assert_eq!(
        promoted.node(k),
        Some(&Node::gauge(Some(g), literal([4.0, 0.0, 0.0]))),
        "the gauge sits on the instance's gauge, holding its offset"
    );
    assert_eq!(gauge_of(&promoted, base), Some(k));
    assert_eq!(offset_of(&promoted, base), Some(Placement::IDENTITY));
    assert_eq!(gauge_of(&promoted, top), Some(k), "the group moves whole");
    assert_eq!(offset_of(&promoted, top), None);
    let at = |d: &ProfileDoc, id| d.roots().iter().position(|&r| r == id);
    assert_eq!(
        at(&promoted, k).map(|i| i + 1),
        at(&promoted, base),
        "the gauge joins the roots just ahead of the instance"
    );
    assert_eq!(
        solve(&promoted, &o, Tol::witness()).role(mate),
        solve(&doc, &o, Tol::witness()).role(mate),
        "the mate still places"
    );
    let (before, after) = (extent(&doc, &o), extent(&promoted, &o));
    assert_eq!(before.2.to_bits(), after.2.to_bits());
    same_extent(before, after, "a promote moves no material");

    let folded = fold(promoted.clone(), k);
    let (map, steps) = identity(&doc);
    same_up_to_ids(&doc, &folded, &map, &steps)
        .unwrap_or_else(|e| panic!("fold ∘ promote is the identity:\n{e}"));
    assert_eq!(label(&folded, base).as_deref(), Some("bracket"));

    for (from, edit, to) in [
        (&doc, DocEdit::Promote { instance: base }, &promoted),
        (&promoted, DocEdit::Fold { gauge: k }, &folded),
    ] {
        let replayed = apply_replayed(from, &edit, Tol::witness()).expect("no reach");
        assert!(replayed.doc.bit_eq(to), "{edit:?} replays to the edit");
    }
}

/// **`Fold` joins the gauge's steps in front, and `Promote` after it is
/// the identity on a lone dependent**: K on g, one instance on K at the
/// empty chain. Folding K puts the instance on g with K's placement as
/// its offset, step for step, and promoting the instance mints K back up
/// to node ids. A gauge on K gets K's steps ahead of its own, an
/// instance on K with no offset keeps none, and a lone dependent with no
/// label takes K's.
#[test]
fn fold_joins_the_gauges_steps_in_front_and_promote_after_it_is_the_identity() {
    let p = parts("pf-fold");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("pf-fold"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 8.0, 0.0])));
    let (doc, k) = insert(doc, Node::gauge(Some(g), literal([2.0, 0.0, 0.5])));
    let (doc, lone) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, lone, Some(k));

    let folded = fold(doc.clone(), k);
    assert!(folded.node(k).is_none(), "the gauge dissolves");
    assert_eq!(gauge_of(&folded, lone), Some(g));
    assert_eq!(
        offset_of(&folded, lone),
        Some(literal([2.0, 0.0, 0.5])),
        "K's steps in front of the empty chain"
    );
    let (before, after) = (extent(&doc, &o), extent(&folded, &o));
    assert_eq!(before.2.to_bits(), after.2.to_bits());
    same_extent(before, after, "a fold moves no material");
    let (back, k_again) = promote(folded, lone);
    let (mut map, steps) = identity(&doc);
    map.insert(k, k_again);
    same_up_to_ids(&doc, &back, &map, &steps)
        .unwrap_or_else(|e| panic!("promote ∘ fold is the identity:\n{e}"));

    // The label: a lone dependent with none takes K's, one with its
    // own keeps it.
    let named = labelled(doc.clone(), k, "bench");
    assert_eq!(label(&fold(named.clone(), k), lone).as_deref(), Some("bench"));
    let both = labelled(named, lone, "post");
    assert_eq!(label(&fold(both, k), lone).as_deref(), Some("post"));

    // A gauge on K, and an instance on K with no offset.
    let (doc, k2) = insert(doc, Node::gauge(Some(k), literal([0.0, 0.0, 1.0])));
    let (doc, bare) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, bare, Some(k));
    let doc = set_offset(doc, bare, None);
    let folded = fold(doc, k);
    assert_eq!(
        folded.node(k2),
        Some(&Node::gauge(
            Some(g),
            literal([2.0, 0.0, 0.5]).compose(&literal([0.0, 0.0, 1.0]))
        )),
        "a gauge on K gets K's steps ahead of its own"
    );
    assert_eq!(gauge_of(&folded, bare), Some(g));
    assert_eq!(offset_of(&folded, bare), None, "no offset stays none");
    assert_eq!(label(&folded, lone), None, "two dependents take no label");
}

/// **"Make a part at this frame"**: promote a placed group's root, then
/// cut the group leaving the promoted gauge. The part's root sits at the
/// empty chain on its world, the instance on the promoted gauge at the
/// empty offset, the evaluation is unchanged, and the round trip is the
/// promoted document up to node ids.
#[test]
fn a_part_at_this_frame_is_a_promote_and_a_cut_leaving_the_gauge() {
    let (p, doc, [base, top, mate]) = placed_pair("pf-frame");
    let o = p.opts();
    let (promoted, k) = promote(doc.clone(), base);
    let out = split(&promoted, &[base, top, mate], "pf-frame", &o).expect("the group cuts");
    let root = out.node_map[&base];
    assert_eq!(offset_of(&out.part, root), Some(Placement::IDENTITY));
    assert_eq!(gauge_of(&out.part, root), None, "on the part's world");
    assert_eq!(gauge_of(&out.remainder, out.instance), Some(k));
    assert_eq!(offset_of(&out.remainder, out.instance), Some(Placement::IDENTITY));
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let (before, after) = (extent(&doc, &o), extent(&out.remainder, &with_resolver(store)));
    assert_eq!(before.2.to_bits(), after.2.to_bits());
    same_extent(before, after, "split-then-evaluate keeps the material");
    round_trip(&promoted, &[base, top, mate], &p, "pf-frame-r1");
}

/// **Cutting a gauge's content and folding the gauge left behind gives
/// the instance the gauge's placement**: K on g with two groups and a
/// gauge K2 under it. The cut of K's content anchors on K, so the
/// instance sits on K at the empty chain; folding K puts it on g at K's
/// placement, K's instances sit on the part's world at their offsets
/// and K2 hangs from the part's world, and nothing moves.
#[test]
fn a_cut_of_a_gauges_content_then_a_fold_gives_the_instance_the_gauges_placement() {
    let p = parts("pf-content");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("pf-content"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 8.0, 0.0])));
    let (doc, k) = insert(doc, Node::gauge(Some(g), literal([2.0, 0.0, 0.5])));
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, base, Some(k));
    let doc = set_offset(doc, base, Some(literal([0.0, 2.0, 0.0])));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, top, Some(k));
    let (doc, mate) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let (doc, lone) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, lone, Some(k));
    let doc = set_offset(doc, lone, Some(literal([16.0, 0.0, 0.0])));
    let (doc, k2) = insert(doc, Node::gauge(Some(k), literal([4.0, 0.0, 0.0])));
    let (doc, deep) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, deep, Some(k2));

    let out = split(&doc, &[base, top, mate, lone, k2, deep], "pf-content", &o).expect("cuts");
    assert_eq!(gauge_of(&out.remainder, out.instance), Some(k));
    assert_eq!(offset_of(&out.remainder, out.instance), Some(Placement::IDENTITY));
    let folded = fold(out.remainder.clone(), k);
    assert!(folded.node(k).is_none());
    assert_eq!(gauge_of(&folded, out.instance), Some(g));
    assert_eq!(
        offset_of(&folded, out.instance),
        Some(literal([2.0, 0.0, 0.5])),
        "the instance takes K's placement as its offset"
    );
    for (id, offset) in [
        (base, Some(literal([0.0, 2.0, 0.0]))),
        (lone, Some(literal([16.0, 0.0, 0.0]))),
        (top, None),
    ] {
        assert_eq!(gauge_of(&out.part, out.node_map[&id]), None, "{id:?}");
        assert_eq!(offset_of(&out.part, out.node_map[&id]), offset, "{id:?}");
    }
    assert_eq!(gauge_of(&out.part, out.node_map[&k2]), None, "K2 on the part's world");
    assert_eq!(
        gauge_of(&out.part, out.node_map[&deep]),
        Some(out.node_map[&k2])
    );
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let (before, after) = (extent(&doc, &o), extent(&folded, &with_resolver(store)));
    assert_eq!(before.2.to_bits(), after.2.to_bits());
    same_extent(before, after, "the fold moves no material");
}

/// **Each refusal names its subject and a recourse, and the recourse
/// clears it**: a promote of a node that is no instance, of an instance
/// with no offset, of a member whose offset is a check, and of a root
/// whose member carries an offset; a fold of a node that is no gauge,
/// and of a gauge whose fold would make a declaring mate start placing.
#[test]
fn promote_and_fold_refuse_typed_with_a_recourse_that_clears_them() {
    let (p, doc, [base, top, mate]) = placed_pair("pf-refusals");
    let o = p.opts();
    assert!(matches!(
        refused(&doc, DocEdit::Promote { instance: mate }),
        EditError::PromoteOnNonInstance { node } if node.id() == mate
    ));
    assert!(matches!(
        refused(&doc, DocEdit::Fold { gauge: base }),
        EditError::FoldOnNonGauge { node } if node.id() == base
    ));

    let unplaced = set_offset(doc.clone(), base, None);
    assert!(matches!(
        refused(&unplaced, DocEdit::Promote { instance: base }),
        EditError::PromoteWithoutOffset { node } if node.id() == base
    ));
    promote(set_offset(unplaced, base, Some(literal([1.0, 0.0, 0.0]))), base);

    let solved = Placement::literal(
        &solve(&doc, &o, Tol::witness())
            .placement(&doc, top)
            .expect("placed"),
    );
    let checked = set_offset(doc.clone(), top, Some(solved));
    assert!(matches!(
        refused(&checked, DocEdit::Promote { instance: top }),
        EditError::PromoteNonRoot { node, root } if node.id() == top && root.id() == base
    ));
    assert!(matches!(
        refused(&checked, DocEdit::Promote { instance: base }),
        EditError::PromoteMemberOffset { node, member } if node.id() == base && member.id() == top
    ));
    promote(set_offset(checked, top, None), base);

    // A block on K mated to one on K's parent declares; folding K would
    // put both on one gauge.
    let doc = ProfileDoc::empty(DocumentId::derive("pf-start"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 8.0, 0.0])));
    let (doc, k) = insert(doc, Node::gauge(Some(g), literal([0.0, 0.0, 4.0])));
    let (doc, on_g) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, on_g, Some(g));
    let (doc, on_k) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, on_k, Some(k));
    let (doc, declaring) = insert(doc, seat(head(p.top_cap(on_k)), head(p.base_cap(on_g))));
    assert!(matches!(
        refused(&doc, DocEdit::Fold { gauge: k }),
        EditError::FoldWouldStartPlacing { node, mate } if node.id() == k && mate.id() == declaring
    ));
    fold(step(doc, DocEdit::DeleteNode { id: declaring }).0, k);
}
