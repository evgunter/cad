//! **Split carries a space by the variable rule** (`refactor.rs`'s
//! module docs, "Variables go where their readers go"): a space read is
//! an edge to the node defining the space. A `new_space()` whose readers
//! are all cut joins the cut, named or not, in either mint order; one
//! read on both sides refuses; a construction's own space goes with its
//! construction, so a placement into it across the cut is severed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::fixture;
use crate::fixture::resolver::PartStore;
use crate::fixture::round_trip::{composed, flat, same_up_to_ids};
use editor_core::{
    DocEdit, DocumentId, Maintenance, Node, Operand, OperandSlot, PartResolver, ProfileDoc,
    RecipeNodeId, SplitError, SplitOutcome, VarId,
};
use fixture::{insert, len, on_frame, square, step};
use geom_core::Tol;

/// A unit block centred at `(x, 0)`: its profile and its extrude, a
/// construction in a space of its own.
fn block(doc: ProfileDoc, x: f64) -> (ProfileDoc, [RecipeNodeId; 2]) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(x, 0.0, 0.5)],
    );
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    (doc, [profile, extrude])
}

/// `body` placed into `target` at the identity.
fn place_into(doc: ProfileDoc, body: RecipeNodeId, target: VarId) -> (ProfileDoc, RecipeNodeId) {
    let (doc, placement) = step(
        doc,
        DocEdit::place(
            vec![body.into()],
            Operand::Var(target),
            editor_core::stand().bundle(),
        ),
    );
    (doc, placement.expect("the placement mints"))
}

/// A new space.
fn new_space(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, VarId) {
    let (doc, node) = insert(doc, Node::Space);
    let var = doc.output(node, 0).expect("a new space defines its space");
    (doc, node, var)
}

fn split(doc: &ProfileDoc, cut: &[RecipeNodeId], seed: &str) -> Result<SplitOutcome, SplitError> {
    editor_core::split(
        doc,
        &cut.iter().copied().collect::<BTreeSet<_>>(),
        DocumentId::derive(seed),
        Tol::witness(),
        None,
    )
}

/// A kept block, then a block `a` placed into a `new_space()` and that
/// copy placed into the world — the space minted before its reader when
/// `space_first`, after it (a placement re-targeted) otherwise. Answers
/// the document, the space's node, and the cut without the space: `a`,
/// its placement into the space, and the world placement of that copy.
fn spaced(seed: &str, space_first: bool) -> (ProfileDoc, RecipeNodeId, Vec<RecipeNodeId>) {
    let doc = ProfileDoc::empty(DocumentId::derive(seed), Tol::witness());
    let (doc, [_, kept]) = block(doc, 10.0);
    let (doc, _) = fixture::place(doc, kept);
    let (doc, [profile, a]) = block(doc, 0.0);
    let (doc, space, inner) = if space_first {
        let (doc, space, var) = new_space(doc);
        let (doc, inner) = place_into(doc, a, var);
        (doc, space, inner)
    } else {
        let (doc, inner) = place_into(doc, a, VarId::WORLD);
        let (doc, var) = fixture::into_new_space(doc, inner);
        let space = doc.operation_of(var).expect("a space's node");
        (doc, space, inner)
    };
    let (doc, outer) = fixture::place(doc, inner);
    if !space_first {
        assert!(
            doc.ids().iter().position(|&id| id == space)
                > doc.ids().iter().position(|&id| id == inner),
            "{seed}: the space is minted after its reader"
        );
    }
    (doc, space, vec![profile, a, inner, outer])
}

/// The split of `cut`, inlined back at the identity and flattened, is
/// `doc` up to minted ids and order (A4).
fn round_trips(doc: &ProfileDoc, out: &SplitOutcome, seed: &str) {
    let mut store = PartStore::default();
    store.insert(out.part.clone(), Tol::witness());
    let store: Arc<dyn PartResolver> = Arc::new(store);
    let back = editor_core::inline(&out.remainder, out.instance, &store, Tol::witness())
        .unwrap_or_else(|e| panic!("{seed}: the part inlines back: {e}"));
    let (nodes, steps) = composed(doc, out, &back);
    same_up_to_ids(doc, &flat(&back), &nodes, &steps)
        .unwrap_or_else(|e| panic!("{seed}: inline(split(d)) is d up to ids:\n{e}"));
}

/// **A `new_space()` whose readers are all cut goes with them**, named
/// in the cut or not, minted before its reader or after it: the part
/// holds the space and the placement into it, the remainder holds
/// neither, nothing strands, and the round trip is the document.
///
/// *Breaks if* an unnamed space read only by the cut is left behind
/// (the carry cannot map the placement's target), a space minted after
/// its reader is carried after it, or a named one is carried twice.
#[test]
fn a_space_read_only_by_the_cut_goes_with_it() {
    for (named, space_first) in [(false, true), (false, false), (true, true), (true, false)] {
        let seed = format!("split-space-named-{named}-first-{space_first}");
        let (doc, space, mut cut) = spaced(&seed, space_first);
        let inner = cut[2];
        if named {
            cut.push(space);
        }
        let out = split(&doc, &cut, &format!("{seed}-part"))
            .unwrap_or_else(|e| panic!("{seed}: the split admits the cut: {e}"));
        let carried = *out
            .node_map
            .get(&space)
            .unwrap_or_else(|| panic!("{seed}: the space is carried"));
        assert!(
            matches!(out.part.node(carried), Some(Node::Space)),
            "{seed}: the part holds a new space"
        );
        assert_eq!(
            out.part.node_space(out.node_map[&inner]),
            out.part.output(carried, 0),
            "{seed}: the carried placement reads the carried space"
        );
        assert!(
            out.remainder.node(space).is_none(),
            "{seed}: the remainder keeps no space"
        );
        assert!(
            !out.remainder_maintenance
                .iter()
                .any(|m| matches!(m, Maintenance::StrandedRead { .. })),
            "{seed}: nothing strands: {:?}",
            out.remainder_maintenance
        );
        round_trips(&doc, &out, &seed);
    }
}

/// **A space named in the cut whose reader stays refuses**: the kept
/// placement would read a space the remainder no longer holds, so the
/// edge is severed, rather than the delete stranding the read.
///
/// *Breaks if* the split admits the cut and strands the kept placement's
/// target.
#[test]
fn a_space_cut_from_under_its_kept_reader_refuses() {
    let (doc, space, cut) = spaced("split-space-strand", true);
    let inner = cut[2];
    let err = split(&doc, &[space], "split-space-strand-part")
        .expect_err("the kept placement reads the cut space");
    assert!(
        matches!(
            &err,
            SplitError::SeveredEdge { consumer, slot: Some(OperandSlot::Space), input, consumer_is_cut: false }
                if consumer.id() == inner && input.id() == space
        ),
        "{err:?}"
    );
}

/// **A space read on both sides refuses**, naming one reader on each,
/// whether the cut names the space or not.
///
/// *Breaks if* a shared space is carried, left, or refused naming the
/// wrong readers.
#[test]
fn a_space_read_on_both_sides_refuses() {
    for named in [false, true] {
        let seed = format!("split-space-shared-{named}");
        let (doc, space, cut) = spaced(&seed, true);
        let var = doc.output(space, 0).expect("the space");
        let (doc, [_, b]) = block(doc, 5.0);
        let (doc, kept) = place_into(doc, b, var);
        let mut cut = cut;
        if named {
            cut.push(space);
        }
        let err = split(&doc, &cut, &format!("{seed}-part")).expect_err("the space is shared");
        assert!(
            matches!(
                &err,
                SplitError::UncutVarReference { var: v, cut_node, kept_node }
                    if v.id() == var && cut_node.id() == cut[2] && kept_node.id() == kept
            ),
            "{seed}: {err:?}"
        );
    }
}

/// **A placement into a construction's own space is an edge to the
/// construction**: a kept placement into a cut construction's space, and
/// a cut placement into a kept one's, are each severed, naming the
/// construction.
///
/// *Breaks if* either crosses the cut with its target left on the other
/// side.
#[test]
fn a_placement_into_a_constructions_space_across_the_cut_is_severed() {
    // Kept into cut.
    let doc = ProfileDoc::empty(DocumentId::derive("split-own-space-kept"), Tol::witness());
    let (doc, [pa, a]) = block(doc, 0.0);
    let (doc, world_a) = fixture::place(doc, a);
    let (doc, [_, b]) = block(doc, 5.0);
    let own = fixture::space(&doc, a);
    let (doc, into_a) = place_into(doc, b, own);
    let err = split(&doc, &[pa, a, world_a], "split-own-space-kept-part")
        .expect_err("the kept placement's target is cut");
    assert!(
        matches!(
            &err,
            SplitError::SeveredEdge { consumer, slot: Some(OperandSlot::Space), input, consumer_is_cut: false }
                if consumer.id() == into_a && input.id() == a
        ),
        "kept into cut: {err:?}"
    );

    // Cut into kept.
    let doc = ProfileDoc::empty(DocumentId::derive("split-own-space-cut"), Tol::witness());
    let (doc, [_, a]) = block(doc, 0.0);
    let (doc, _) = fixture::place(doc, a);
    let (doc, [pb, b]) = block(doc, 5.0);
    let own = fixture::space(&doc, a);
    let (doc, into_a) = place_into(doc, b, own);
    let (doc, world) = fixture::place(doc, into_a);
    let err = split(&doc, &[pb, b, into_a, world], "split-own-space-cut-part")
        .expect_err("the cut placement's target stays");
    assert!(
        matches!(
            &err,
            SplitError::SeveredEdge { consumer, slot: Some(OperandSlot::Space), input, consumer_is_cut: true }
                if consumer.id() == into_a && input.id() == a
        ),
        "cut into kept: {err:?}"
    );
}
