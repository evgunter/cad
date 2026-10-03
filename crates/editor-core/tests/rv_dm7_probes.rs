//! REVIEW probes for DM7 — cases the unit suites do not distinguish.
//! Not a substitute for their rows; a probe lives here when a mutant
//! showed the case unpinned and the claim it pins is a residue rather
//! than a documented contract. A probe that turns out to hold a
//! documented contract moves into the unit suite instead (that is
//! where `an_appearance_strand_precedes_the_cluster_acts_of_the_same_delete`
//! went, to `dm7_delete_strands.rs`).
//!
//! Two carriers are covered. The payload walk (`review/strands-rv`): a
//! carrier that names its own space, a mate operand that is a read site
//! and not a name, and a cascade whose strands are about carriers it
//! then deletes. The appearance store (`review/appstrand-rv`): the
//! reported key's durability across save/load after the delete, and
//! `Rebind` as the repair that needs a live node to move to.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::block;
use crate::fixture;
use crate::fixture::resolver::PartStore;
use editor_core::{
    Alignment, AxisSense, ContactClass, DocEdit, DocumentId, EntityKind, Maintenance, MateFrame,
    MatePrimitive, Node, ProfileDoc, RecipeNodeId, RoleSeg, StableName, apply, solve_document,
};
use fixture::{ang, fname, insert, len, scl, wall};
use geom_core::Tol;

fn delete(
    doc: &editor_core::ProfileDoc,
    id: RecipeNodeId,
) -> editor_core::Applied<editor_core::ProfileProgram> {
    apply(
        doc,
        &DocEdit::DeleteNode { id },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the delete is legal")
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
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let node = Node::Fillet {
        target: body,
        radius: len(0.1),
        selection: vec![fname(body, wall(&doc, body, 0))],
    };
    let (doc, fillet) = insert(doc, node);
    // The repair door moves the fillet's selection into the fillet's
    // OWN space: `to` must name a live node and nothing forbids that
    // node being the carrier itself.
    let applied = apply(
        &doc,
        &DocEdit::Rebind {
            from: fname(body, wall(&doc, body, 0)),
            to: fname(fillet, wall(&doc, fillet, 2)),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the rebind target is live");
    let doc = applied.doc;
    let Some(Node::Fillet { selection, .. }) = doc.node(fillet) else {
        panic!("the fillet is a Fillet")
    };
    assert_eq!(
        selection,
        &vec![fname(fillet, wall(&doc, fillet, 2))],
        "the carrier now names its own space"
    );

    // Deleting the fillet: the name goes with the node that carries it
    // AND with the node that minted it, so there is no survivor to
    // rebind and nothing to report.
    let applied = delete(&doc, fillet);
    assert!(
        applied.maintenance.is_empty(),
        "a name whose carrier and mint are the same deleted node strands nothing: {:?}",
        applied.maintenance
    );
}

fn mate_frame() -> MateFrame {
    MateFrame::authored(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        geom_core::Tol::witness(),
    )
    .expect("a definite frame")
}

fn instance_face(instance: RecipeNodeId, part_body: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: StableName {
                kind: EntityKind::Face,
                node: part_body,
                path: vec![RoleSeg::Cap(CapEnd::Start)],
            }
            .into(),
        }],
    }
}

use editor_core::CapEnd;

/// **Deleting a mate's OPERAND alone reports no strand, and the solve
/// refuses it typed.** DM7's carve-out for `payload_read_sites` says
/// the loss is the solve's to refuse; this is that claim end to end,
/// for an operand that is not also the head's minting node.
#[test]
fn rv_a_deleted_mate_operand_is_silent_here_and_typed_at_the_solve() {
    let mut store = PartStore::new();
    let part = ProfileDoc::empty_derived("rv_operand_part", Tol::witness());
    let (part, part_body) = block(part, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let doc_ref = store.insert(part, Tol::witness());
    // The delete moves the pair's root and the solve levers the
    // parts, so both go through the store's reach.
    let opts = fixture::resolver::with_resolver(store);
    let reach = editor_core::mate_reach::<f64>(&opts, Tol::witness());

    let doc = ProfileDoc::empty(DocumentId::derive("rv_operand"), Tol::witness());
    let (doc, ia) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, ib) = insert(doc, Node::instantiate_part(doc_ref));
    // A placed copy of `ib`: the mate reads `ib`'s face AT the
    // transform, so the operand is a node the name does not name.
    let (doc, placed) = insert(
        doc,
        Node::transform(
            ib,
            editor_core::Step::Rigid {
                translation: [len(2.0), len(0.0), len(0.0)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.0),
            },
        ),
    );
    let head_b = instance_face(ib, part_body);
    let (doc, mate) = insert(
        doc,
        Node::Mate {
            a: crate::fixture::head(instance_face(ia, part_body)),
            b: crate::fixture::head_at(placed, head_b),
            class: ContactClass::Rest,
            alignment: Alignment {
                a: mate_frame(),
                b: mate_frame(),
                primitive: MatePrimitive::Coaxial,
                sense: AxisSense::Aligned,
                clocking: Some(0.0),
            },
        },
    );

    let applied = apply(
        &doc,
        &DocEdit::DeleteNode { id: placed },
        Tol::witness(),
        &reach,
    )
    .expect("the delete is legal");
    assert!(
        !applied
            .maintenance
            .iter()
            .any(|row| matches!(row, Maintenance::Strand { .. })),
        "the operand is a read site, not a name: {:?}",
        applied.maintenance
    );
    let fault = solve_document(&applied.doc, &reach, Tol::witness())
        .fault(mate)
        .expect("the solve refuses the mate whose operand left")
        .clone();
    assert!(
        matches!(&fault, editor_core::MateFault::DanglingHead { head, .. } if *head == placed),
        "typed, naming the node the walk stopped at, not silence: {fault:?}"
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
    let (doc, _body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let painted = fname(victim, wall(&doc, victim, 0));
    let doc = apply(
        &doc,
        &DocEdit::SetAppearance {
            name: painted.clone(),
            attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the name's node is live")
    .doc;

    let after = delete(&doc, victim).doc;
    let text = editor_core::persist::save(&after, &[], Tol::witness())
        .expect("the post-delete document saves with its stranded key");
    let loaded = editor_core::persist::load(&text, Tol::witness()).expect("and loads");
    assert!(
        loaded.doc.appearance().contains_key(&painted),
        "the stranded attachment is document state, not a residue"
    );
    assert!(
        loaded.doc.node(victim).is_none(),
        "and its minting node is still gone"
    );
}

/// **`Rebind` is the other repair the arm's doc names, and it moves a
/// reported key.** `ClearAppearance` has a row in the unit's suite;
/// this is the half that needs a live node to move TO, which is what
/// makes it the repair `Strand` and `StrandedAppearance` share.
#[test]
fn rv_a_reported_appearance_strand_is_rebindable() {
    use editor_core::{Attr, Rgba8};

    let doc = ProfileDoc::empty_derived("rv_app_rebind", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let painted = fname(victim, wall(&doc, victim, 0));
    let doc = apply(
        &doc,
        &DocEdit::SetAppearance {
            name: painted.clone(),
            attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the name's node is live")
    .doc;

    let applied = delete(&doc, victim);
    assert_eq!(
        applied.maintenance,
        vec![Maintenance::StrandedAppearance {
            name: doc.spoken_name(&painted)
        }],
        "the door named the key"
    );
    let live = fname(body, wall(&doc, body, 0));
    let repaired = apply(
        &applied.doc,
        &DocEdit::Rebind {
            from: painted.clone(),
            to: live.clone(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the reported key is rebindable onto a live name")
    .doc;
    assert!(
        !repaired.appearance().contains_key(&painted),
        "the stranded key is gone"
    );
    assert!(
        repaired.appearance().contains_key(&live),
        "and the attachment moved to the live name"
    );
}
