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
    assert_eq!(
        label(&fold(named.clone(), k), lone).as_deref(),
        Some("bench")
    );
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
/// promoted document up to node ids. A kept declaring mate reading the
/// root refuses the frame rule while the root sits off the empty chain,
/// and crosses once the root is promoted.
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
    assert_eq!(
        offset_of(&out.remainder, out.instance),
        Some(Placement::IDENTITY)
    );
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let (before, after) = (
        extent(&doc, &o),
        extent(&out.remainder, &with_resolver(store)),
    );
    assert_eq!(before.2.to_bits(), after.2.to_bits());
    same_extent(before, after, "split-then-evaluate keeps the material");
    round_trip(&promoted, &[base, top, mate], &p, "pf-frame-r1");

    let (doc, own) = insert(doc, Node::gauge(None, literal([0.0, 0.0, 9.0])));
    let (doc, kept) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, kept, Some(own));
    let (doc, crossing) = insert(doc, seat(head(p.top_cap(kept)), head(p.base_cap(base))));
    let err = split(&doc, &[base, top, mate], "pf-frame-crossing", &o)
        .expect_err("the root sits off the empty chain");
    assert!(
        matches!(&err, editor_core::SplitError::MateFrameCrosses { mate: m, promote: Some(r), .. }
            if m.id() == crossing && r.id() == base),
        "{err:?}"
    );
    assert!(
        err.to_string().contains(&format!(
            "Recourse: promote {} (Promote), so it sits at the empty chain, or delete {}, then \
             split",
            doc.spoken(base),
            doc.spoken(crossing)
        )),
        "{err}"
    );
    let (promoted, _) = promote(doc, base);
    split(&promoted, &[base, top, mate], "pf-frame-crossing", &o)
        .expect("promoted, the root sits at the empty chain and the mate crosses");
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
    assert_eq!(
        offset_of(&out.remainder, out.instance),
        Some(Placement::IDENTITY)
    );
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
    assert_eq!(
        gauge_of(&out.part, out.node_map[&k2]),
        None,
        "K2 on the part's world"
    );
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
/// moves forward**: a promote of a node that is no instance, of a member
/// that is not its group's root (with or without a checked offset, and
/// in a group on a dead chain), of an instance whose group carries no
/// offset, and of a root whose member carries an offset; a fold of a
/// node that is no gauge, and of a gauge whose fold would make a
/// declaring mate start placing. Root-ness is asked first, so no
/// recourse leads to a refusal that sends the caller back.
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

    // A mate-placed member with no offset: the root is named, and
    // promoting it clears the refusal.
    let err = refused(&doc, DocEdit::Promote { instance: top });
    assert!(
        matches!(&err, EditError::PromoteNonRoot { node, root } if node.id() == top && root.id() == base),
        "{err:?}"
    );
    assert!(
        err.to_string()
            .contains(&format!("Recourse: promote {}", doc.spoken(base))),
        "{err}"
    );
    promote(doc.clone(), base);

    // A group with no offset at all: either member refuses for want of
    // one, and setting that member's makes it the root.
    let unplaced = set_offset(doc.clone(), base, None);
    for instance in [base, top] {
        assert!(matches!(
            refused(&unplaced, DocEdit::Promote { instance }),
            EditError::PromoteWithoutOffset { node } if node.id() == instance
        ));
        promote(
            set_offset(unplaced.clone(), instance, Some(literal([1.0, 0.0, 0.0]))),
            instance,
        );
    }

    // A group on a dead chain: the member named is the one carrying the
    // offset, and promoting it refuses on the dead gauge, not for want
    // of an offset.
    let (dead, g) = insert(doc.clone(), Node::gauge(None, literal([0.0, 1.0, 0.0])));
    let dead = set_gauge(set_gauge(dead, base, Some(g)), top, Some(g));
    let dead = set_offset(
        set_offset(dead, base, None),
        top,
        Some(literal([0.0, 0.0, 2.0])),
    );
    let dead = step(dead, DocEdit::DeleteNode { id: g }).0;
    assert!(matches!(
        refused(&dead, DocEdit::Promote { instance: base }),
        EditError::PromoteNonRoot { node, root } if node.id() == base && root.id() == top
    ));
    assert!(matches!(
        refused(&dead, DocEdit::Promote { instance: top }),
        EditError::GaugeNotLive { gauge, .. } if gauge.id() == g
    ));

    let solved = Placement::literal(
        &solve(&doc, &o, Tol::witness())
            .placement(&doc, top)
            .expect("placed"),
    );
    // A member with a checked offset: the root is named; promoting it
    // names the member's offset, and clearing that clears it.
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

// ---- poses under rotations ----

/// A literal step turning by `angle` about x or z, then moving by `t`:
/// non-dyadic coordinates, so a regrouped composition would round
/// differently.
fn turn(angle: f64, about_x: bool, t: [f64; 3]) -> Placement {
    let (s, c) = angle.sin_cos();
    let columns = if about_x {
        [[1.0, 0.0, 0.0], [0.0, c, s], [0.0, -s, c]]
    } else {
        [[c, s, 0.0], [-s, c, 0.0], [0.0, 0.0, 1.0]]
    };
    Placement::literal(&editor_core::Frame {
        columns,
        translation: t,
    })
}

/// Two turns in one chain.
fn two_turns(a: f64, b: f64, t: [f64; 3]) -> Placement {
    turn(a, true, t).compose(&turn(b, false, [0.25, -0.5, 0.0]))
}

/// Every instance's world pose in `before` and in `after` agree bit
/// for bit.
fn same_poses(
    before: &ProfileDoc,
    after: &ProfileDoc,
    o: &editor_core::EvalOptions,
    ids: &[RecipeNodeId],
    what: &str,
) {
    let (a, b) = (
        solve(before, o, Tol::witness()),
        solve(after, o, Tol::witness()),
    );
    for &id in ids {
        let x = a.placement(before, id).expect("placed before");
        let y = b.placement(after, id).expect("placed after");
        assert!(x.bit_eq(&y), "{what}: {id:?} moved: {x:?} vs {y:?}");
    }
}

/// **P1 under rotations**: a placed pair on a turned gauge, its base
/// at an offset of two turns. Promote and its fold move no pose bit,
/// and the fold returns the document.
#[test]
fn p1_promote_and_fold_move_no_pose_bit_under_rotations() {
    let (p, doc, [base, top, _]) = placed_pair("pf-turned-p1");
    let o = p.opts();
    let (doc, g) = insert(doc, Node::gauge(None, turn(0.3, false, [0.0, 8.0, 0.0])));
    let doc = set_gauge(set_gauge(doc, base, Some(g)), top, Some(g));
    let doc = set_offset(doc, base, Some(two_turns(0.7, 0.2, [4.0, 0.1, 0.0])));
    let (promoted, k) = promote(doc.clone(), base);
    same_poses(&doc, &promoted, &o, &[base, top], "promote");
    let folded = fold(promoted.clone(), k);
    same_poses(&promoted, &folded, &o, &[base, top], "fold ∘ promote");
    let (map, steps) = identity(&doc);
    same_up_to_ids(&doc, &folded, &map, &steps)
        .unwrap_or_else(|e| panic!("fold ∘ promote is the identity:\n{e}"));
}

/// **P2 under rotations**: K, two turns, on a turned g; a lone instance
/// on K at the empty chain, and K2, a turn, on K holding an instance at
/// a turned offset. Folding K moves no pose bit, though each of K's
/// dependents now holds K's steps in its own chain, and promoting the
/// lone instance back moves none either.
#[test]
fn p2_fold_and_promote_move_no_pose_bit_under_rotations() {
    let p = parts("pf-turned-p2");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("pf-turned-p2"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, turn(0.3, false, [0.0, 8.0, 0.0])));
    let (doc, k) = insert(
        doc,
        Node::gauge(Some(g), two_turns(0.7, 0.2, [2.0, 0.0, 0.5])),
    );
    let (doc, lone) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, lone, Some(k));
    let (doc, k2) = insert(doc, Node::gauge(Some(k), turn(1.1, true, [0.0, 0.0, 1.0])));
    let (doc, deep) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, deep, Some(k2));
    let doc = set_offset(doc, deep, Some(turn(0.4, false, [1.0, 2.0, 0.0])));
    let folded = fold(doc.clone(), k);
    same_poses(&doc, &folded, &o, &[lone, deep], "fold");
    let (back, _) = promote(fold(folded.clone(), k2), lone);
    same_poses(&doc, &back, &o, &[lone, deep], "promote after two folds");
}

/// **P4 under rotations**: K, two turns, on a turned g, holding a group
/// that is cut and an instance at a turned offset that stays. Folding K
/// after the cut moves the instance left behind and the one that stayed
/// by no bit.
#[test]
fn p4_a_cut_then_a_fold_moves_no_pose_bit_under_rotations() {
    let p = parts("pf-turned-p4");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("pf-turned-p4"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, turn(0.3, false, [0.0, 8.0, 0.0])));
    let (doc, k) = insert(
        doc,
        Node::gauge(Some(g), two_turns(0.7, 0.2, [2.0, 0.0, 0.5])),
    );
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, base, Some(k));
    let doc = set_offset(doc, base, Some(literal([0.0, 2.0, 0.0])));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, top, Some(k));
    let (doc, mate) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let (doc, stay) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, stay, Some(k));
    let doc = set_offset(doc, stay, Some(turn(0.9, true, [16.0, 0.0, 0.0])));

    let out = split(&doc, &[base, top, mate], "pf-turned-p4", &o).expect("cuts");
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let with_part = with_resolver(store);
    let folded = fold(out.remainder.clone(), k);
    same_poses(
        &out.remainder,
        &folded,
        &with_part,
        &[out.instance, stay],
        "fold after the cut",
    );
    same_poses(
        &doc,
        &folded,
        &with_part,
        &[stay],
        "the instance that stayed",
    );
}

// ---- what a fold takes out ----

/// **A fold goes as a delete does** (item: one cleanup): a gauge an
/// in-plane axis reads as its plane refuses the delete and the fold
/// alike, naming the reader with a recourse; taking the recourse, the
/// fold leaves a document the save door accepts.
#[test]
fn a_fold_refuses_a_gauge_another_node_reads_as_an_input() {
    let p = parts("pf-dangle");
    let doc = ProfileDoc::empty(DocumentId::derive("pf-dangle"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 8.0, 0.0])));
    let (doc, on_g) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, on_g, Some(g));
    let (doc, axis) = insert(
        doc,
        Node::Datum(editor_core::Datum::AxisInPlane {
            plane: g,
            origin: [fixture::len(0.0), fixture::len(0.0)],
            direction: [fixture::scl(1.0), fixture::scl(0.0)],
        }),
    );
    assert!(matches!(
        refused(&doc, DocEdit::DeleteNode { id: g }),
        EditError::DeleteWouldDangle { id, referenced_by } if id.id() == g && referenced_by.id() == axis
    ));
    let err = refused(&doc, DocEdit::Fold { gauge: g });
    assert!(
        matches!(&err, EditError::FoldWouldDangle { node, referenced_by } if node.id() == g && referenced_by.id() == axis),
        "{err:?}"
    );
    assert!(
        err.to_string().contains(&format!(
            "Recourse: delete {} (and what reads it), then fold {}",
            doc.spoken(axis),
            doc.spoken(g)
        )),
        "{err}"
    );
    let cleared = step(doc, DocEdit::DeleteNode { id: axis }).0;
    let folded = fold(cleared, g);
    assert_eq!(gauge_of(&folded, on_g), None);
    editor_core::persist::save(&folded, &[], Tol::witness()).expect("the folded document saves");
}

/// **A label a fold cannot hand on is reported**: K labelled, with two
/// dependents, or with one already labelled, drops its label and says
/// so; with one unlabelled dependent it hands the label on and reports
/// nothing.
#[test]
fn a_label_a_fold_cannot_hand_on_is_reported() {
    let p = parts("pf-label");
    let doc = ProfileDoc::empty(DocumentId::derive("pf-label"), Tol::witness());
    let (doc, k) = insert(doc, Node::gauge(None, literal([2.0, 0.0, 0.0])));
    let (doc, one) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, one, Some(k));
    let doc = labelled(doc, k, "bench");
    let maintenance = |d: &ProfileDoc| {
        d.apply(&DocEdit::Fold { gauge: k }, Tol::witness(), &RefusingReach)
            .expect("folds")
            .maintenance
    };
    assert!(
        maintenance(&doc).is_empty(),
        "a lone unlabelled dependent takes it"
    );
    let (two, other) = insert(doc.clone(), Node::instantiate_part(p.top));
    let two = set_gauge(two, other, Some(k));
    let named = labelled(doc, one, "post");
    for (what, d) in [("two dependents", two), ("a labelled dependent", named)] {
        let rows = maintenance(&d);
        assert!(
            matches!(
                rows.as_slice(),
                [editor_core::Maintenance::LabelDropped { gauge, label }]
                    if gauge.id() == k && label.as_str() == "bench"
            ),
            "{what}: {rows:?}"
        );
    }
}
