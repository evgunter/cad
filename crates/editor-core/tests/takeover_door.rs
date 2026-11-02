//! **A takeover's rebinds on adversarial results** (`editor_core::take_over`):
//! a world copy of a block `B`, a shell holding `B`'s top read in that
//! copy, and the copy taken over by a result that splits, merges or
//! keeps the top. The door rebinds the held top only where the result
//! carries it as one whole face; a top the result splits keeps no
//! rebind, so the shell reads stale rather than one piece of it.
//!
//! The last row takes over an instance's copy: the held face is spelled
//! in the part's ids, and the result's name for it carries that name
//! inside a name of the host's. The edit doors hold each level of such
//! a name to its own document's log (`edit::check_name_steps`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on};
use crate::fixture::{insert, len, out, stood};

use editor_core::{
    Bodies, CapEnd, DocEdit, EntityKind, EvalOptions, Node, Operand, ProfileDoc, ProfileProgram,
    RecipeNodeId, RoleSeg, StableName, take_over,
};
use geom_core::Tol;

fn cap(node: RecipeNodeId, end: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Cap(end)],
    }
}

/// `b`'s world copy, with a shell opening its top: a selection of the
/// copy's body that holds `b`'s top.
fn copy_with_held_top(doc: ProfileDoc, b: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    let (doc, copy) = insert(doc, stood(b));
    let copy_out = out(&doc, copy);
    let (doc, _) = insert(
        doc,
        Node::Shell {
            thickness: len(0.01),
            open: Operand::select(Operand::Var(copy_out), vec![cap(b, CapEnd::End)]),
        },
    );
    (doc, copy)
}

/// Takes `copy` over with `u` and answers the rebinds written.
fn rebinds(doc: &ProfileDoc, copy: RecipeNodeId, u: RecipeNodeId) -> Vec<DocEdit<ProfileProgram>> {
    take_over(
        doc,
        &[copy],
        &Operand::Node(u),
        &EvalOptions::default(),
        Tol::witness(),
    )
    .expect("the takeover applies")
    .edits
    .into_iter()
    .filter(|e| matches!(e, DocEdit::Rebind { .. }))
    .collect()
}

fn union(doc: ProfileDoc, members: Vec<RecipeNodeId>) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(members.into_iter().map(Into::into).collect()),
        },
    )
}

/// **A face the result splits, one piece merged and one not**: `B`'s top
/// is crossed by a tall rib of `D`, and `D`'s flange, flush with the top,
/// overlaps it on one side of the rib only. In `B ∪ D` the top's far
/// piece is a fragment and its near piece a member of the merged face:
/// the top is split, and a rebind to the merged face would re-seat the
/// held top on one piece of it.
#[test]
fn a_split_top_with_one_piece_merged_is_not_rebound() {
    let doc = ProfileDoc::empty_derived("takeover-split-merged", Tol::witness());
    let (doc, b) = block(doc, (0.0, 4.0), (0.0, 2.0), 1.0);
    let (doc, rib) = block_on(doc, b, (1.8, 2.2), (-1.0, 3.0), 0.0, 2.0);
    let (doc, flange) = block_on(doc, b, (2.2, 6.0), (0.0, 2.0), 0.5, 0.5);
    let (doc, d) = union(doc, vec![rib, flange]);
    let (doc, u) = union(doc, vec![b, d]);
    let (doc, copy) = copy_with_held_top(doc, b);
    let written = rebinds(&doc, copy, u);
    assert!(
        written.is_empty(),
        "no piece of the split top is it: {written:?}"
    );
}

/// The same with the three members spelled in one union (the fold).
#[test]
fn a_split_top_in_a_three_member_union_is_not_rebound() {
    let doc = ProfileDoc::empty_derived("takeover-split-merged-3", Tol::witness());
    let (doc, b) = block(doc, (0.0, 4.0), (0.0, 2.0), 1.0);
    let (doc, rib) = block_on(doc, b, (1.8, 2.2), (-1.0, 3.0), 0.0, 2.0);
    let (doc, flange) = block_on(doc, b, (2.2, 6.0), (0.0, 2.0), 0.5, 0.5);
    let (doc, u) = union(doc, vec![b, rib, flange]);
    let (doc, copy) = copy_with_held_top(doc, b);
    let written = rebinds(&doc, copy, u);
    assert!(written.is_empty(), "{written:?}");
}

/// **A rib alone splits the top into two fragments**: no carrier.
#[test]
fn a_top_split_into_two_fragments_is_not_rebound() {
    let doc = ProfileDoc::empty_derived("takeover-split", Tol::witness());
    let (doc, b) = block(doc, (0.0, 4.0), (0.0, 2.0), 1.0);
    let (doc, rib) = block_on(doc, b, (1.8, 2.2), (-1.0, 3.0), 0.0, 2.0);
    let (doc, u) = union(doc, vec![b, rib]);
    let (doc, copy) = copy_with_held_top(doc, b);
    let written = rebinds(&doc, copy, u);
    assert!(written.is_empty(), "{written:?}");
}

/// **A flush block overlapping a corner**: the top stays one face, its
/// region grown and merged with the block's top, and the held top is
/// rebound to the merged face.
#[test]
fn a_flush_overlap_merges_the_top_and_rebinds_it() {
    let doc = ProfileDoc::empty_derived("takeover-flush", Tol::witness());
    let (doc, b) = block(doc, (0.0, 4.0), (0.0, 2.0), 1.0);
    let (doc, f) = block_on(doc, b, (3.0, 6.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, u) = union(doc, vec![b, f]);
    let (doc, copy) = copy_with_held_top(doc, b);
    let written = rebinds(&doc, copy, u);
    let [DocEdit::Rebind { from, to, .. }] = written.as_slice() else {
        panic!("one rebind: {written:?}");
    };
    assert_eq!(*from, cap(b, CapEnd::End));
    assert!(
        matches!(to.path.as_slice(), [RoleSeg::Merged(set)] if set.len() == 2),
        "onto the merged top: {to:?}"
    );
}

/// **A body unioned with itself** (A ∪ A): the union carries each face
/// of the one read once, and the held top is rebound onto that row.
#[test]
fn a_body_unioned_with_itself_rebinds_the_top_once() {
    let doc = ProfileDoc::empty_derived("takeover-self", Tol::witness());
    let (doc, b) = block(doc, (0.0, 4.0), (0.0, 2.0), 1.0);
    let (doc, u) = union(doc, vec![b, b]);
    let (doc, copy) = copy_with_held_top(doc, b);
    let written = rebinds(&doc, copy, u);
    assert_eq!(written.len(), 1, "{written:?}");
}

/// **An instance's copy taken over**: a world copy of an instance, a
/// shell holding a lateral face of it (a name spelled in the part's ids,
/// its `Lateral` step the part's), and a fillet of the instance as the
/// result. The result's name for the face is the fillet's, a node of
/// the host, carrying the part's name inside it: the rebind is written,
/// the shell's selection holds a name the copy answers as one face, and
/// the document saves and loads.
#[test]
fn an_instances_copy_taken_over_with_a_lateral_face_held() {
    use crate::fixture::resolver::{PartStore, with_resolver};
    use editor_core::{
        DocumentId, VarId,
        names::{all_edges, all_faces},
    };
    let mut store = PartStore::new();
    let part = ProfileDoc::empty(DocumentId::derive("takeover-part"), Tol::witness());
    let (r, _) = store.insert_part(block(part, (0.0, 1.0), (0.0, 1.0), 1.0), Tol::witness());
    let o = with_resolver(store);
    let host = ProfileDoc::empty_derived("takeover-host", Tol::witness());
    let (host, inst) = insert(
        host,
        Node::instantiate_part(r, Operand::Var(VarId::WORLD), editor_core::stand().bundle()),
    );
    let (host, copy) = insert(host, stood(inst));
    let ev = crate::fixture::run(&host, &o);
    let lateral_face = all_faces(&ev, copy)
        .into_iter()
        .find(|n| n.path.iter().any(|s| matches!(s, RoleSeg::Lateral(_))))
        .expect("the copy holds a lateral face");
    let lateral_edge = all_edges(&ev, inst)
        .into_iter()
        .find(|n| n.path.iter().any(|s| matches!(s, RoleSeg::LateralEdge(_))))
        .expect("the instance holds a lateral edge");
    let copy_out = out(&host, copy);
    let (host, _) = insert(
        host,
        Node::Shell {
            thickness: len(0.01),
            open: Operand::select(Operand::Var(copy_out), vec![lateral_face.clone()]),
        },
    );
    let inst_out = out(&host, inst);
    let (host, f) = insert(
        host,
        Node::Fillet {
            radius: len(0.1),
            selection: Operand::select(Operand::Var(inst_out), vec![lateral_edge]),
        },
    );
    let done = take_over(&host, &[copy], &Operand::Node(f), &o, Tol::witness())
        .expect("a part-spelled face is rebound, not refused");
    let written: Vec<_> = done
        .edits
        .iter()
        .filter(|e| matches!(e, DocEdit::Rebind { .. }))
        .collect();
    let [DocEdit::Rebind { from, to, .. }] = written.as_slice() else {
        panic!("one rebind: {written:?}");
    };
    assert_eq!(*from, lateral_face, "the part's name, moved");
    assert_eq!(to.node, f, "onto the fillet's own name for it");
    // The shell's selection holds the fillet's name, which the copy's
    // table now answers as one face. (The shell itself then refuses the
    // filleted block's corner, which is geometry's, not the name's.)
    let held = done
        .doc
        .vars()
        .values()
        .filter_map(|v| v.def().select())
        .find(|s| s.body == copy_out)
        .expect("the shell's selection");
    assert_eq!(
        held.names,
        vec![to.clone()],
        "the selection holds the new name"
    );
    let ev = crate::fixture::run(&done.doc, &o);
    assert!(
        matches!(
            ev.table(copy_out).and_then(|t| t.lookup(to)),
            Some(editor_core::names::Entry::Unique(_))
        ),
        "the copy holds it as one face"
    );
    // The appearance door holds the same name to the same rule: the
    // fillet's level is the host's, the part's name inside it the
    // part's.
    let doc = editor_core::apply(
        &done.doc,
        &DocEdit::SetAppearance {
            body: copy_out,
            name: to.clone(),
            attr: editor_core::Attr::Color(editor_core::Rgba8 {
                r: 1,
                g: 2,
                b: 3,
                a: 255,
            }),
        },
        Tol::witness(),
    )
    .expect("a colour on the new name is admitted")
    .doc;
    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("it saves");
    let loaded = editor_core::persist::load(&text, Tol::witness()).expect("and loads");
    assert!(loaded.doc.bit_eq(&doc), "the whole document round-trips");
}

/// **The door evaluates only for a reader**: a copy nothing reads is
/// re-pointed with no evaluation and answers no moved names; a copy a
/// selection reads, or one a reader outside the document holds a name
/// on (`held`), is evaluated, its names answered moved, and its
/// evaluation handed out for the run of the finished document to start
/// from. Goes red if the door evaluates for an unread copy, misses a
/// holder outside the document, or stops handing its run out.
#[test]
fn the_door_evaluates_only_where_something_reads_a_copy() {
    let doc = ProfileDoc::empty_derived("takeover-unread", Tol::witness());
    let (doc, b) = block(doc, (0.0, 4.0), (0.0, 2.0), 1.0);
    let (doc, f) = block_on(doc, b, (3.0, 6.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, u) = union(doc, vec![b, f]);
    let take = |doc: &ProfileDoc, copy, held: &dyn Fn(RecipeNodeId) -> bool| {
        let mut run = editor_core::Recording::start(doc, Tol::witness());
        run.take_over(
            &[copy],
            &Operand::Node(u),
            &EvalOptions::default(),
            None,
            held,
        )
        .expect("the takeover applies")
    };
    let (unread, copy) = insert(doc.clone(), stood(b));
    let none = take(&unread, copy, &|_| false);
    assert!(none.evaluation.is_none(), "nothing reads the copy");
    assert!(none.moved.is_empty(), "and nothing is answered moved");
    let outside = take(&unread, copy, &|node| node == copy);
    assert!(
        outside.evaluation.is_some(),
        "a pick outside holds the copy's top"
    );
    assert!(
        outside.moved.of(copy, &cap(b, CapEnd::End)).is_some(),
        "its top moved, answered for the holder"
    );
    let (read, copy) = copy_with_held_top(doc, b);
    assert!(
        take(&read, copy, &|_| false).evaluation.is_some(),
        "a shell reads the copy's top"
    );
}

/// **A copy of the copy carries its names as they are**, so a reader of
/// the second copy reads the first's names: the door follows the
/// placement chain, rebinds the reader, and answers the moved names for
/// the second copy too, for a pick held on it. Goes red if the door
/// reads only the copies it re-points.
#[test]
fn a_reader_of_a_copy_of_the_copy_is_rebound() {
    let doc = ProfileDoc::empty_derived("takeover-copy-of-copy", Tol::witness());
    let (doc, b) = block(doc, (0.0, 4.0), (0.0, 2.0), 1.0);
    let (doc, f) = block_on(doc, b, (3.0, 6.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, u) = union(doc, vec![b, f]);
    let (doc, copy) = insert(doc, stood(b));
    let (doc, copy2) = insert(doc, stood(copy));
    let copy2_out = out(&doc, copy2);
    let (doc, _) = insert(
        doc,
        Node::Shell {
            thickness: len(0.01),
            open: Operand::select(Operand::Var(copy2_out), vec![cap(b, CapEnd::End)]),
        },
    );
    let o = EvalOptions::default();
    let done = take_over(&doc, &[copy], &Operand::Node(u), &o, Tol::witness()).expect("applies");
    assert_eq!(
        rebinds_in(&done.edits),
        1,
        "the second copy's reader is rebound"
    );
    let ev = crate::fixture::run(&done.doc, &o);
    let held = done
        .doc
        .vars()
        .values()
        .filter_map(|v| v.def().select())
        .find(|s| s.body == copy2_out)
        .expect("the shell's selection");
    let table = ev.table(copy2_out).expect("the second copy evaluates");
    assert!(
        held.names.iter().all(|n| table.lookup(n).is_some()),
        "the reader of the copy's copy resolves: {:?}",
        held.names
    );
    assert_eq!(
        done.taken.moved.of(copy2, &cap(b, CapEnd::End)),
        done.taken.moved.of(copy, &cap(b, CapEnd::End)),
        "the second copy's top moved as the first's did"
    );
    assert!(done.taken.moved.of(copy2, &cap(b, CapEnd::End)).is_some());
}

/// **A copy given twice is refused**: re-pointed once, it no longer
/// copies the body the second would read its names from. Goes red if
/// the door takes the repeat.
#[test]
fn a_copy_given_twice_is_refused() {
    let doc = ProfileDoc::empty_derived("takeover-twice", Tol::witness());
    let (doc, b) = block(doc, (0.0, 4.0), (0.0, 2.0), 1.0);
    let (doc, f) = block_on(doc, b, (3.0, 6.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, u) = union(doc, vec![b, f]);
    let (doc, copy) = insert(doc, stood(b));
    let refused = take_over(
        &doc,
        &[copy, copy],
        &Operand::Node(u),
        &EvalOptions::default(),
        Tol::witness(),
    )
    .expect_err("a repeat refuses");
    assert!(
        matches!(refused, editor_core::EditError::TakeOverRepeatsCopy { .. }),
        "{refused:?}"
    );
}

fn rebinds_in(edits: &[DocEdit<ProfileProgram>]) -> usize {
    edits
        .iter()
        .filter(|e| matches!(e, DocEdit::Rebind { .. }))
        .count()
}

/// A copy of `b` taken over by `b ∪ f` (a flush block: `b`'s top
/// merges), `link` built on the copy, and a shell holding `b`'s top
/// read in the link's body. Answers the takeover and the shell's
/// selection after it, resolved in the link's table.
fn a_reader_down_a_link(
    name: &str,
    link: impl FnOnce(&ProfileDoc, RecipeNodeId) -> editor_core::AuthoredNode,
) -> (editor_core::TakeOverOutcome, RecipeNodeId, bool) {
    let doc = ProfileDoc::empty_derived(name, Tol::witness());
    let (doc, b) = block(doc, (0.0, 4.0), (0.0, 2.0), 1.0);
    let (doc, f) = block_on(doc, b, (3.0, 6.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, u) = union(doc, vec![b, f]);
    let (doc, copy) = insert(doc, stood(b));
    let node = link(&doc, copy);
    let (doc, l) = insert(doc, node);
    let l_out = out(&doc, l);
    let (doc, _) = insert(
        doc,
        Node::Shell {
            thickness: len(0.01),
            open: Operand::select(Operand::Var(l_out), vec![cap(b, CapEnd::End)]),
        },
    );
    let o = EvalOptions::default();
    let done = take_over(&doc, &[copy], &Operand::Node(u), &o, Tol::witness()).expect("applies");
    let ev = crate::fixture::run(&done.doc, &o);
    let held = done
        .doc
        .vars()
        .values()
        .filter_map(|v| v.def().select())
        .find(|s| s.body == l_out)
        .expect("the shell's selection");
    let resolves = held
        .names
        .iter()
        .all(|n| ev.table(l_out).is_some_and(|t| t.lookup(n).is_some()));
    (done, l, resolves)
}

/// **A mirror of the copy carries its names as they are**
/// (`verbatim_edge`'s `Reflected`), so a reader of the mirror reads the
/// copy's names: the door follows the chain, and — since the mirror's
/// own plane is a selection on the copy, so the mirror evaluates only
/// once that is rebound — rebinds in rounds until the mirror's reader
/// is rebound too. `Moved` answers for the mirror. Goes red if the
/// chain walk follows placements alone, or the door rebinds in one
/// round.
#[test]
fn a_reader_of_a_mirror_of_the_copy_is_rebound() {
    let (done, mirror, resolves) =
        a_reader_down_a_link("takeover-mirror-of-copy", |doc, copy| Node::Mirror {
            body: copy.into(),
            plane: crate::fixture::plane_on(doc, copy, [0.0, 1.0, 0.5], [1.0, 0.0, 0.0]),
        });
    assert!(resolves, "the mirror's reader resolves after the takeover");
    assert!(
        done.taken
            .moved
            .0
            .get(&mirror)
            .is_some_and(|moved| !moved.is_empty()),
        "and the mirror's names are answered moved"
    );
}

/// **A split of the copy carries the faces it leaves intact as they
/// are** (`verbatim_edge`'s `Intact`): its plane below the top, the top
/// is intact before the takeover and after it, so a reader of the
/// split's top is rebound onto the merged top. Goes red if the chain
/// walk does not follow a split.
#[test]
fn a_reader_of_a_split_of_the_copy_is_rebound_on_an_intact_face() {
    let (done, split, resolves) =
        a_reader_down_a_link("takeover-split-of-copy", |doc, copy| Node::Split {
            target: copy.into(),
            tool: crate::fixture::plane_on(doc, copy, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]),
        });
    assert!(resolves, "the split's reader resolves after the takeover");
    assert!(
        done.taken
            .moved
            .0
            .get(&split)
            .is_some_and(|moved| !moved.is_empty()),
        "and the split's intact names are answered moved"
    );
}
