//! REVIEW probes for DM7 (`review/strands-rv`) — two cases the unit's
//! own suite does not distinguish. Not a substitute for its rows; these
//! exist to pin what a mutant showed to be unpinned.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::block;
use crate::fixture;
use crate::fixture::resolver::PartStore;
use editor_core::{
    Alignment, AxisSense, ContactClass, DocEdit, DocumentId, EntityKind, Maintenance, MateFrame,
    MatePrimitive, Node, ProfileDoc, RecipeNodeId, RoleSeg, SitedRef, StableName, apply,
    solve_document,
};
use fixture::{ang, fname, insert, len, scl, wall};
use geom_core::Tol;

fn delete(doc: &ProfileDoc, id: RecipeNodeId) -> editor_core::Applied<editor_core::ProfileProgram> {
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
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, fillet) = insert(
        doc,
        Node::Fillet {
            target: body,
            radius: len(0.1),
            selection: vec![fname(body, wall(0))],
        },
    );
    // The repair door moves the fillet's selection into the fillet's
    // OWN space: `to` must name a live node and nothing forbids that
    // node being the carrier itself.
    let applied = apply(
        &doc,
        &DocEdit::Rebind {
            from: fname(body, wall(0)),
            to: fname(fillet, wall(2)),
        },
        Tol::witness(),
    )
    .expect("the rebind target is live");
    let doc = applied.doc;
    let Some(Node::Fillet { selection, .. }) = doc.node(fillet) else {
        panic!("the fillet is a Fillet")
    };
    assert_eq!(
        selection,
        &vec![fname(fillet, wall(2))],
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
    MateFrame {
        origin: [0.0, 0.0, 0.0],
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    }
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

    let doc = ProfileDoc::empty(DocumentId::derive("rv_operand"), Tol::witness());
    let (doc, ia) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, ib) = insert(doc, Node::instantiate_part(doc_ref));
    // A placed copy of `ib`: the mate reads `ib`'s face AT the
    // transform, so the operand is a node the name does not name.
    let (doc, placed) = insert(
        doc,
        Node::Transform {
            input: ib,
            translation: [len(2.0), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    let head_b = instance_face(ib, part_body);
    let (doc, mate) = insert(
        doc,
        Node::Mate {
            a: SitedRef::at_mint(instance_face(ia, part_body)),
            b: SitedRef::new(placed, head_b),
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

    let applied = delete(&doc, placed);
    assert!(
        !applied
            .maintenance
            .iter()
            .any(|row| matches!(row, Maintenance::Strand { .. })),
        "the operand is a read site, not a name: {:?}",
        applied.maintenance
    );
    let fault = solve_document(&applied.doc, Tol::witness())
        .fault(mate)
        .expect("the solve refuses the mate whose operand left")
        .clone();
    assert!(
        matches!(&fault, editor_core::MateFault::DanglingHead { head, .. } if *head == placed),
        "typed, naming the node the walk stopped at, not silence: {fault:?}"
    );
}

/// **A cascade reports strands about carriers the same cascade is
/// about to delete.** Cascading the `Declare` of a declared union
/// deletes the union first (it consumes the `Declare`), which strands
/// every pair of the `Declare` — and then deletes the `Declare`. The
/// rows are true of the document between the two steps and are about
/// nothing that survives the cascade, which is the count the CHROME
/// row's "strand count beside the dependent count" has to define.
#[test]
fn rv_a_cascade_reports_strands_on_carriers_it_then_deletes() {
    use crate::docm7_union_declare::{declared_union, flush_pairs};
    use editor_core::cascade_delete_order;

    let doc = ProfileDoc::empty_derived("rv_cascade_noise", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, decl) = declared_union(doc, &[a, b], |u| flush_pairs(u, (a, a), (b, b)));

    let order = cascade_delete_order(&doc, decl);
    assert_eq!(order, vec![union, decl], "the union consumes the declare");

    // What a pre-click count built from the SURVIVORS would say: the
    // doomed set mints nothing any survivor carries.
    let doomed: Vec<RecipeNodeId> = order.clone();
    let mut survivor_carried = 0usize;
    for &id in doc.order() {
        if doomed.contains(&id) {
            continue;
        }
        let Some(node) = doc.node(id) else { continue };
        survivor_carried += node
            .payload_names()
            .iter()
            .filter(|n| doomed.contains(&n.node))
            .count();
    }
    assert_eq!(survivor_carried, 0, "no survivor carries a doomed name");

    let mut doc = doc;
    let mut reported = 0usize;
    for id in order {
        let applied = delete(&doc, id);
        reported += applied
            .maintenance
            .iter()
            .filter(|row| matches!(row, Maintenance::Strand { .. }))
            .count();
        doc = applied.doc;
    }
    assert_eq!(
        reported, 8,
        "the door reports eight strands about a Declare the cascade then deletes"
    );
}

/// **An appearance attachment the same delete strands IS reported,
/// and is left where it is.** DM7's second carrier, end to end at the
/// door: paint a face of `victim`, delete `victim`, and the report
/// names the key while the store still holds the attachment — the
/// clause reports, it never repairs.
///
/// The key is not a `Node::payload_names` carrier, so the payload
/// walk cannot see it: a row here is evidence of the store's own
/// pass, not of the other one reaching further.
#[test]
fn rv_a_stranded_appearance_key_is_in_the_report() {
    use editor_core::{Attr, Rgba8};

    let doc = ProfileDoc::empty_derived("rv_appearance", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let painted = fname(victim, wall(0));
    let doc = apply(
        &doc,
        &DocEdit::SetAppearance {
            name: painted.clone(),
            attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
        },
        Tol::witness(),
    )
    .expect("the name's node is live")
    .doc;
    assert!(doc.appearance().contains_key(&painted), "the paint landed");
    let _ = body;

    let applied = delete(&doc, victim);
    assert_eq!(
        applied.maintenance,
        vec![Maintenance::StrandedAppearance {
            name: painted.clone()
        }],
        "the door names the key the delete stranded"
    );
    assert!(
        applied.doc.appearance().contains_key(&painted),
        "and leaves the attachment where it is: DM7 reports, never repairs"
    );
}

/// **The appearance strand is inside the strand segment, ahead of the
/// cluster acts** — the second boundary of `Applied::maintenance`'s
/// order contract, for the second carrier.
///
/// `a_mates_head_strands_and_its_read_site_does_not` pins that
/// boundary for a PAYLOAD strand only: its fixture paints nothing, so
/// a walk that appended the appearance rows after `reconcile` passes
/// it. This is the edit that produces all three kinds at once — a
/// mate head stranded, a painted instance face stranded, and the
/// registry act the deleted instance forced.
#[test]
fn rv_an_appearance_strand_precedes_the_cluster_acts_of_the_same_delete() {
    use editor_core::{Attr, Rgba8};

    let mut store = PartStore::new();
    let part = ProfileDoc::empty_derived("rv_app_order_part", Tol::witness());
    let (part, part_body) = block(part, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let doc_ref = store.insert(part, Tol::witness());

    let doc = ProfileDoc::empty(DocumentId::derive("rv_app_order"), Tol::witness());
    let (doc, ia) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, ib) = insert(doc, Node::instantiate_part(doc_ref));
    let head_a = instance_face(ia, part_body);
    let (doc, _mate) = insert(
        doc,
        Node::Mate {
            a: SitedRef::at_mint(head_a.clone()),
            b: SitedRef::at_mint(instance_face(ib, part_body)),
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
    let painted = instance_face(ia, part_body);
    let doc = apply(
        &doc,
        &DocEdit::SetAppearance {
            name: painted.clone(),
            attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
        },
        Tol::witness(),
    )
    .expect("the painted instance is live")
    .doc;

    let applied = delete(&doc, ia);
    let cluster = applied
        .maintenance
        .iter()
        .position(|row| matches!(row, Maintenance::Cluster(_)))
        .expect("the deleted instance forces a registry act, so the row is not vacuous");
    let strand = applied
        .maintenance
        .iter()
        .position(|row| matches!(row, Maintenance::StrandedAppearance { .. }))
        .expect("the delete stranded the painted key");
    assert!(
        strand < cluster,
        "the strands are read at the door, before the registry reconciles: {:?}",
        applied.maintenance
    );
}

/// **The delete leaves the appearance store BIT-IDENTICAL**, not
/// merely still holding the reported key. The unit's rows assert
/// `contains_key` on the names they painted, which a walk that
/// emptied a record's attrs, or dropped an unrelated key, would
/// survive. "DM7 reports, it never repairs" is a statement about the
/// whole store, so the whole store is what is compared.
#[test]
fn rv_the_delete_leaves_the_whole_appearance_store_alone() {
    use editor_core::{Attr, Rgba8};

    let doc = ProfileDoc::empty_derived("rv_app_untouched", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let mut doc = doc;
    for name in [
        fname(victim, wall(0)),
        fname(victim, wall(2)),
        fname(body, wall(0)),
    ] {
        doc = apply(
            &doc,
            &DocEdit::SetAppearance {
                name,
                attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
            },
            Tol::witness(),
        )
        .expect("every painted node is live")
        .doc;
    }
    let before = doc.appearance().clone();

    let applied = delete(&doc, victim);
    assert_eq!(
        applied.doc.appearance(),
        &before,
        "every key and every record survives the delete unchanged"
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
    let painted = fname(victim, wall(0));
    let doc = apply(
        &doc,
        &DocEdit::SetAppearance {
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
    let painted = fname(victim, wall(0));
    let doc = apply(
        &doc,
        &DocEdit::SetAppearance {
            name: painted.clone(),
            attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
        },
        Tol::witness(),
    )
    .expect("the name's node is live")
    .doc;

    let applied = delete(&doc, victim);
    assert_eq!(
        applied.maintenance,
        vec![Maintenance::StrandedAppearance {
            name: painted.clone()
        }],
        "the door named the key"
    );
    let live = fname(body, wall(0));
    let repaired = apply(
        &applied.doc,
        &DocEdit::Rebind {
            from: painted.clone(),
            to: live.clone(),
        },
        Tol::witness(),
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
