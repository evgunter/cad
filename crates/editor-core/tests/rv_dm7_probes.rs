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

fn delete(doc: &ProfileDoc, id: RecipeNodeId) -> editor_core::Applied<editor_core::ProfileProgram> {
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
        &editor_core::RefusingReach,
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
    MateFrame::authored([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0])
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
    // The delete moves the pair's gauge and the solve levers the
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

/// **A sited declaration strands nothing inside a cascade**, so the
/// door's count and a pre-click count built from the survivors agree.
///
/// The class this row measured has MOVED rather than vanished, and
/// where it moved to is worth stating. Noise needs a DOOMED carrier
/// that names a node the same cascade deletes BEFORE it. A cascade
/// deletes dependents first, and a payload name points at a producer
/// upstream of its carrier, so a doomed carrier is always deleted
/// before the node it names — with one exception, which was this
/// row's: a `Declare` naming entities in its own CONSUMER's space.
/// The consumer is a dependent, so it went first, and the door
/// reported eight strands about a `Declare` it deleted next. A sited
/// pair names entities that exist BEFORE the consumer, so no payload
/// points downstream any more and the exception is closed by type.
///
/// No carrier in the vocabulary reopens it: a blend's selection names
/// its own target, a mate head's name is read at an operand, an
/// appearance key names an upstream row — every one of them upstream
/// of the carrier. The door still reports a REAL strand, on a carrier
/// that survives its subject; that half is
/// `dm7_delete_strands::deleting_a_declared_member_names_its_pairs_and_its_site_reports_nothing`.
#[test]
fn rv_a_sited_declaration_strands_nothing_inside_a_cascade() {
    use crate::docm7_union_declare::{declared_union, flush_pairs};
    use editor_core::cascade_delete_order;

    let doc = ProfileDoc::empty_derived("rv_cascade_noise", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, decl) = declared_union(doc, &[a, b], flush_pairs((a, a), (b, b)));

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
    // RE-BASELINED with the sited payload: the count used to be eight.
    // A sited pair names entities in the MEMBERS, which this cascade
    // does not touch, so the doomed set strands nothing and the
    // door's count agrees with the survivors'.
    assert_eq!(
        reported, survivor_carried,
        "a sited declaration names entities outside the doomed set, so the door's \
         count and a pre-click count built from the survivors agree"
    );
    assert_eq!(reported, 0);
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
    let painted = fname(victim, wall(0));
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
