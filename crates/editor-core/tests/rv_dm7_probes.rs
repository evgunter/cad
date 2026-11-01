//! REVIEW probes for DM7 — cases the unit suites do not distinguish.
//! Not a substitute for their rows; a probe lives here when a mutant
//! showed the case unpinned and the claim it pins is a residue rather
//! than a documented contract. A probe that turns out to hold a
//! documented contract moves into the unit suite instead, as the
//! delete's strand order did
//! (`dm7_delete_strands::a_delete_reports_its_strands_alone_and_only_a_mate_insert_clears_an_offset`).
//!
//! Two carriers are covered. The payload walk (`review/strands-rv`): a
//! carrier that names its own space, a mate operand that is a read site
//! and not a name, and a cascade whose strands are about carriers it
//! then deletes. The appearance store (`review/appstrand-rv`): the
//! reported key's durability across save/load after the delete, and
//! `Rebind` as the repair that needs a live node to move to.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::block;
use crate::fixture;
use crate::fixture::resolver::PartStore;
use editor_core::{
    DocEdit, DocumentId, Maintenance, MateSide, Node, ProfileDoc, RecipeNodeId, apply,
};
use fixture::{fname, insert, len, wall};
use geom_core::Tol;

fn delete(
    doc: &editor_core::ProfileDoc,
    id: RecipeNodeId,
) -> editor_core::Applied<editor_core::ProfileProgram> {
    apply(doc, &DocEdit::DeleteNode { id }, Tol::witness()).expect("the delete is legal")
}

/// **A carrier that names ITS OWN space reports nothing when it is
/// deleted** — the case that distinguishes the walk over the document
/// AFTER the removal from the same walk over the one before it.
///
/// `stranded_names`' doc-comment justifies taking `&new`; the suite's
/// `a_carrier_deleted_with_the_node_it_names_reports_nothing` does not
/// reach that choice (its carrier's names are minted elsewhere, so
/// both walks agree). A `Rebind` onto a name in the carrier's own
/// space is a reachable way to build one that disagrees.
#[test]
fn rv_a_self_naming_carrier_reports_nothing_when_it_is_deleted() {
    let doc = ProfileDoc::empty_derived("rv_self_naming", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let node = Node::Fillet {
        radius: len(0.1),
        selection: editor_core::Operand::select(body, vec![crate::fixture::end_rim(&doc, body, 0)]),
    };
    let (doc, fillet) = insert(doc, node);
    // The repair door moves the fillet's selection into the fillet's
    // OWN space: `to` must name a live node and nothing forbids that
    // node being the carrier itself.
    let applied = apply(
        &doc,
        &DocEdit::Rebind {
            body: doc.output(body, 0),
            from: crate::fixture::end_rim(&doc, body, 0),
            to: crate::fixture::end_rim(&doc, fillet, 2),
        },
        Tol::witness(),
    )
    .expect("the rebind target is live");
    let doc = applied.doc;
    let Some(Node::Fillet { selection, .. }) = doc.node(fillet) else {
        panic!("the fillet is a Fillet")
    };
    assert_eq!(
        crate::fixture::selected(&doc, *selection),
        vec![crate::fixture::end_rim(&doc, fillet, 2)],
        "the carrier now names its own space"
    );

    // Deleting the fillet: the name goes with the node that carries it
    // AND with the node that minted it, so there is no survivor to
    // rebind and nothing to report.
    let applied = delete(&doc, fillet);
    assert!(
        crate::fixture::without_anonymous(&applied.maintenance).is_empty(),
        "a name whose carrier and mint are the same deleted node strands nothing: {:?}",
        crate::fixture::without_anonymous(&applied.maintenance)
    );
}

/// The start cap of `part_body`'s world copy in its part, as
/// `instance`'s copies carry it.
fn instance_face(
    instance: RecipeNodeId,
    part_body: RecipeNodeId,
) -> crate::fixture::resolver::PartName {
    crate::fixture::resolver::in_part(instance, part_body, CapEnd::Start)
}

use editor_core::CapEnd;

/// **Deleting a mate's OPERAND alone reports no strand, and the
/// placement refuses it typed where it evaluates**: its target's read
/// is unresolved at the mate's `to` seat. DM7's carve-out for
/// `payload_read_sites` says the loss is the solve's to refuse; this is
/// that claim end to end, for an operand that is not also the head's
/// minting node.
#[test]
fn rv_a_deleted_mate_operand_is_silent_here_and_typed_at_the_solve() {
    let mut store = PartStore::new();
    let part = ProfileDoc::empty_derived("rv_operand_part", Tol::witness());
    let (doc_ref, part_body) =
        store.insert_part(block(part, (0.0, 1.0), (0.0, 1.0), 1.0), Tol::witness());
    // The delete moves the pair's root and the evaluation reads the
    // parts, so both go through the store.
    let opts = fixture::resolver::with_resolver(store);

    let doc = ProfileDoc::empty(DocumentId::derive("rv_operand"), Tol::witness());
    // `ia`'s bundle is empty: the mate places it.
    let (doc, ia) = insert(doc, crate::fixture::mated_instance(doc_ref));
    let (doc, ib) = fixture::insert_pinned(doc, crate::fixture::mated_instance(doc_ref), &opts);
    // A copy of `ib`'s copy: the mate reads `ib`'s face AT the copy,
    // so the operand is a node the name does not name.
    let ib_copy =
        editor_core::Operand::Var(doc.output(ib, 0).expect("an instance defines its copies"));
    let (doc, placed) = insert(
        doc,
        crate::fixture::stood(editor_core::BodyRead::plain(ib_copy)),
    );
    let head_b = instance_face(ib, part_body);
    let (doc, mate) = crate::fixture::constrain_at(
        doc,
        ia,
        crate::fixture::rest(
            crate::fixture::head(instance_face(ia, part_body)),
            crate::fixture::head_at(placed, head_b),
        ),
    );

    let applied = apply(&doc, &DocEdit::DeleteNode { id: placed }, Tol::witness())
        .expect("the delete is legal");
    assert!(
        !applied
            .maintenance
            .iter()
            .any(|row| matches!(row, Maintenance::Strand { .. })),
        "the operand is a read site, not a name: {:?}",
        crate::fixture::without_anonymous(&applied.maintenance)
    );
    let ev = fixture::run(&applied.doc, &opts);
    let failure = ev
        .node_error(mate.placement)
        .expect("the placement refuses the mate whose operand left");
    assert!(
        matches!(
            failure.kind,
            editor_core::NodeErrorKind::UnresolvedRead {
                slot: editor_core::OperandSlot::Side { constraint, side: MateSide::To },
                ..
            } if constraint == mate.id
        ),
        "typed, naming the stranded side, not silence: {failure:?}"
    );
}

/// **A reported key survives the save/load boundary as a stranded
/// key**, so the report is about durable document state and not about
/// an in-memory residue the next load would tidy away. The unit's
/// round-trip row saves the document BEFORE the delete; this one
/// saves the one the delete produced.
#[test]
fn rv_a_stranded_appearance_key_round_trips_after_the_delete() {
    use editor_core::{Attr, Rgba8};

    let doc = ProfileDoc::empty_derived("rv_app_persist", Tol::witness());
    let (doc, _body) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 1.0);
    let painted = fname(victim, wall(&doc, victim, 0));
    let doc = apply(
        &doc,
        &DocEdit::SetAppearance {
            body: crate::fixture::painted_in(&doc, &painted),
            name: painted.clone(),
            attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
        },
        Tol::witness(),
    )
    .expect("the name's node is live")
    .doc;

    let after = delete(&doc, victim).doc;
    let text = editor_core::persist::save(&after, &[], Tol::witness())
        .expect("the post-delete document saves with its stranded key");
    let loaded = editor_core::persist::load(&text, Tol::witness()).expect("and loads");
    assert!(
        loaded
            .doc
            .appearance_of(crate::fixture::painted_in(&loaded.doc, &painted), &painted)
            .is_some(),
        "the stranded attachment is document state, not a residue"
    );
    assert!(
        loaded.doc.node(victim).is_none(),
        "and its minting node is still gone"
    );
}

/// **A reported key is repaired by re-attaching it.** `ClearAppearance`
/// has a row in the unit's suite; this is the half that needs a live
/// body to move TO. The delete took the key's read with its node, and a
/// `Rebind` never moves a key's read (NAMES N1: the key is the read's),
/// so the repair is the re-attachment the arm's doc names: the record
/// set in a live read, the stranded key cleared.
#[test]
fn rv_a_reported_appearance_strand_is_reattachable() {
    use editor_core::{Attr, Rgba8};

    let doc = ProfileDoc::empty_derived("rv_app_rebind", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 1.0);
    let painted = fname(victim, wall(&doc, victim, 0));
    let doc = apply(
        &doc,
        &DocEdit::SetAppearance {
            body: crate::fixture::painted_in(&doc, &painted),
            name: painted.clone(),
            attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
        },
        Tol::witness(),
    )
    .expect("the name's node is live")
    .doc;

    let applied = delete(&doc, victim);
    assert_eq!(
        crate::fixture::without_anonymous(&applied.maintenance),
        vec![Maintenance::StrandedAppearance {
            body: doc.spoken_var(crate::fixture::painted_in(&doc, &painted)),
            name: doc.spoken_name(&painted),
            took: editor_core::Took::Node
        }],
        "the door named the key"
    );
    let live = fname(body, wall(&doc, body, 0));
    let gone = crate::fixture::painted_in(&applied.doc, &painted);
    let reattached = apply(
        &applied.doc,
        &DocEdit::SetAppearance {
            body: crate::fixture::out(&applied.doc, body),
            name: live.clone(),
            attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
        },
        Tol::witness(),
    )
    .expect("a live read takes the record")
    .doc;
    let repaired = apply(
        &reattached,
        &DocEdit::ClearAppearance {
            body: gone,
            name: painted.clone(),
            kind: editor_core::AttrKind::Color,
        },
        Tol::witness(),
    )
    .expect("the stranded key clears")
    .doc;
    assert!(
        repaired.appearance_of(gone, &painted).is_none(),
        "the stranded key is gone"
    );
    assert!(
        crate::fixture::painted(&repaired, &live).is_some(),
        "and the attachment moved to the live name"
    );
}
