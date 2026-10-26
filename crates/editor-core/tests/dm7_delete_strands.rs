//! **DM7 — a stranded payload name is REPORTED at the delete, never
//! refused.**
//!
//! A payload name — `Node::payload_names` is the list of which
//! payloads carry one — is not a DAG edge: the insert
//! door checks its minting node is live and no other door does, so a
//! later `DeleteNode` strands it. The delete stays legal, and what the
//! door owes instead is the report: one `Maintenance::Strand` per
//! surviving `(node, name)` whose minting node the edit removed.
//!
//! The document holds a `StableName` in one other place — the
//! appearance store, whose keys `DocEdit::SetAppearance` gives the
//! same N5 semantics — so the report has a second carrier and a
//! second arm, `Maintenance::StrandedAppearance { name }`, with no
//! carrying node because the store is what carries it.
//!
//! `m4_pr7_appearance::deleting_the_minting_node_strands_the_attribute_loudly`
//! is the other end of that key's life: it asserts the typed
//! `AppearanceLoss` the next EVALUATION answers, where the appearance
//! rows here assert the report the DELETE DOOR made — the door is what
//! DM7 added, the loss is what it was limping along on.
//!
//! These rows are what goes red when a strand goes unreported.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::block;
use crate::fixture;
use crate::fixture::resolver::PartStore;
use editor_core::Formula;
use editor_core::{
    Attr, AttrKind, CapEnd, DocEdit, DocumentId, EntityKind, Maintenance, MeasurePrimitive, Node,
    ProfileDoc, RecipeNodeId, Rgba8, RoleSeg, SitedRef, StableName, apply, cascade_delete_order,
};
use fixture::{fname, insert, len, wall};
use geom_core::Tol;

/// The strand rows an accepted edit reported, in the order it reported
/// them — the whole of what these rows assert about. A selection's row
/// is said by the node that reads the selection.
fn strands(applied: &[Maintenance]) -> Vec<(RecipeNodeId, StableName)> {
    applied
        .iter()
        .filter_map(|row| match row {
            Maintenance::Strand { node, name, .. } => Some((node.id(), name.name().clone())),
            Maintenance::StrandedSelection { readers, name, .. } => {
                let [reader] = readers.as_slice() else {
                    panic!("an unnamed selection has one reader: {readers:?}");
                };
                Some((reader.id(), name.name().clone()))
            }
            Maintenance::StrandedAppearance { .. }
            | Maintenance::StrandedRead { .. }
            | Maintenance::StrandedSpace { .. }
            | Maintenance::ProductOutsideWorld { .. }
            | Maintenance::ValueDropped { .. }
            | Maintenance::AnonymousVarRemoved { .. } => None,
        })
        .collect()
}

/// The selection a blend reads.
fn selection_of(doc: &ProfileDoc, blend: RecipeNodeId) -> editor_core::VarId {
    match doc.node(blend) {
        Some(Node::Fillet { selection, .. } | Node::Chamfer { selection, .. }) => *selection,
        other => panic!("a blend, got {other:?}"),
    }
}

/// The appearance keys an accepted edit reported stranded, in the
/// order it reported them — the store's half of the same report.
fn appearance_strands(applied: &[Maintenance]) -> Vec<StableName> {
    applied
        .iter()
        .filter_map(|row| match row {
            Maintenance::StrandedAppearance { name, .. } => Some(name.name().clone()),
            Maintenance::Strand { .. }
            | Maintenance::StrandedSelection { .. }
            | Maintenance::StrandedRead { .. }
            | Maintenance::StrandedSpace { .. }
            | Maintenance::ProductOutsideWorld { .. }
            | Maintenance::ValueDropped { .. }
            | Maintenance::AnonymousVarRemoved { .. } => None,
        })
        .collect()
}

/// Paint one face red, expecting the door to accept it: the name's
/// node is live at the edit, which is all `SetAppearance` asks. The
/// face is painted where it is read ([`crate::fixture::Sited`]).
fn paint(doc: &editor_core::ProfileDoc, name: &(impl crate::fixture::Sited + Clone)) -> ProfileDoc {
    apply(
        doc,
        &DocEdit::SetAppearance {
            body: doc
                .read_of_node(name.site())
                .expect("the face's body is read"),
            name: name.clone().name(),
            attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
        },
        Tol::witness(),
    )
    .expect("the painted name's node is live")
    .doc
}

/// Delete one node, expecting the door to accept it: a delete asks no
/// reach, so the reach is the refusing one.
fn delete(
    doc: &editor_core::ProfileDoc,
    id: RecipeNodeId,
) -> editor_core::Applied<editor_core::ProfileProgram> {
    apply(doc, &DocEdit::DeleteNode { id }, Tol::witness())
        .expect("a payload name is not a DAG edge, so the delete is legal")
}

// ---------------------------------------------------------------------
// Every payload kind that carries a name.
// ---------------------------------------------------------------------

/// **Every payload that carries a name reports its strand** — the row
/// a walk that skips one payload kind goes red on.
///
/// One document, one deleted node, and one carrier of each
/// name-bearing kind whose own DAG input is somewhere else. Which
/// kinds those are is not restated here: the match below carries one
/// arm per carrier, and the two this document cannot mint — a mate's
/// heads and an instance's crossing `outer`s, which need a part —
/// have arms there naming the rows that cover them instead. The
/// expectation is written out carrier by carrier, name by name, so
/// dropping a kind from the walk cannot pass by reporting a shorter
/// list.
#[test]
fn every_payload_kind_that_carries_a_name_reports_its_strand() {
    let doc = ProfileDoc::empty_derived("dm7_carriers", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 1.0);

    let f0 = fname(victim, wall(&doc, victim, 0));
    let f2 = fname(victim, wall(&doc, victim, 2));
    let f4 = fname(victim, RoleSeg::Cap(CapEnd::Start));
    let e0 = crate::fixture::end_rim(&doc, victim, 0);
    let e1 = crate::fixture::end_rim(&doc, victim, 1);

    let (doc, fillet) = insert(
        doc,
        Node::Fillet {
            radius: len(0.1),
            selection: editor_core::Operand::select(body, vec![e0.clone()]),
        },
    );
    let (doc, chamfer) = insert(
        doc,
        Node::Chamfer {
            distance: len(0.1),
            selection: editor_core::Operand::select(body, vec![e1.clone()]),
        },
    );
    let (doc, shell) = insert(
        doc,
        Node::Shell {
            thickness: len(0.1),
            open: editor_core::Operand::select(body, vec![f2.clone()]),
        },
    );
    let (doc, measure) = crate::fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            SitedRef::new(body, f4.clone()),
            SitedRef::new(fillet, f0.clone()),
        ],
    );
    // **The expectation is DERIVED by a match over `Node`, not
    // written out as a list.** A list is only ever as complete as
    // whoever last edited it. Each name-carrying kind gets an arm
    // saying which name this fixture's node of that kind strands;
    // every other kind falls to the last arm, which does not CLAIM
    // the kind is name-free but asks the crate. A `_ => {}` is the
    // hole that replaces — a kind this suite thinks carries nothing
    // while `Node::payload_names` reads a name out of it.
    let mut expected: Vec<(RecipeNodeId, StableName)> = Vec::new();
    for id in doc.ids() {
        let Some(node) = doc.node(id) else { continue };
        match node {
            Node::Fillet { .. } => expected.push((id, e0.clone())),
            Node::Chamfer { .. } => expected.push((id, e1.clone())),
            Node::Shell { .. } => expected.push((id, f2.clone())),
            // Argument order, which is meaning for a measure.
            Node::Measure { .. } => expected.extend([(id, f4.clone()), (id, f0.clone())]),
            // An instance's crossing `outer`s need an interface
            // record to ride, which only a
            // split mints: their rows are
            // `edit_instance_crossing_names`.
            Node::InstantiatePart { .. } => panic!("this fixture builds no instance"),
            // Every remaining kind, by the crate's own answer rather
            // than a second list of name-free variants — that list
            // was stale once already, and a stale one reads exactly
            // like a true one. A kind that starts carrying a name
            // reds here for want of an arm instead of being quietly
            // contradicted.
            other => assert!(
                other.payload_names().is_empty(),
                "node {id:?} carries payload names {:?}, so its kind needs an arm in this \
                 fixture's expectation",
                other.payload_names()
            ),
        }
    }
    // The match is total over `Node`; this says it ran over the
    // document this fixture actually built, so an arm cannot go
    // unreached and look satisfied.
    // The report walks the carriers in the order they were minted.
    assert_eq!(
        expected.iter().map(|(id, _)| *id).collect::<Vec<_>>(),
        vec![fillet, chamfer, shell, measure, measure],
        "four carriers, five names, as minted"
    );

    let applied = delete(&doc, victim);
    assert_eq!(
        strands(&crate::fixture::without_anonymous(&applied.maintenance)),
        expected,
        "one row per carried name, each in its own order"
    );
}

// ---------------------------------------------------------------------
// The negative rows.
// ---------------------------------------------------------------------

/// **A delete that strands nothing reports nothing.** The record is a
/// report of what happened, not a slot that is always filled: a
/// document with a live name-carrier is silent about it when the
/// deleted node is not the one the name was minted by.
#[test]
fn a_delete_that_strands_nothing_reports_nothing() {
    let doc = ProfileDoc::empty_derived("dm7_silent", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, spare) = block(doc, (4.0, 5.0), (0.0, 1.0), 1.0);
    let node = Node::Fillet {
        radius: len(0.1),
        selection: editor_core::Operand::select(body, vec![crate::fixture::end_rim(&doc, body, 0)]),
    };
    let (doc, _fillet) = insert(doc, node);

    let applied = delete(&doc, spare);
    assert!(
        crate::fixture::without_anonymous(&applied.maintenance).is_empty(),
        "nothing named the deleted node: {:?}",
        crate::fixture::without_anonymous(&applied.maintenance)
    );
}

/// **A name that leaves with its own carrier strands nothing.**
/// Deleting the carrier and the minting node in one cascade must not
/// report the carrier's own names: there is no surviving node left
/// holding them, so there is nothing for a caller to rebind.
#[test]
fn a_carrier_deleted_with_the_node_it_names_reports_nothing() {
    let doc = ProfileDoc::empty_derived("dm7_self", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let node1 = Node::Fillet {
        radius: len(0.1),
        selection: editor_core::Operand::select(body, vec![crate::fixture::end_rim(&doc, body, 0)]),
    };
    let (doc, fillet) = insert(doc, node1);

    let order = cascade_delete_order(&doc, body);
    assert_eq!(order, vec![fillet, body], "the consumer goes first");
    let mut doc = doc;
    let mut reported = Vec::new();
    for id in order {
        let applied = delete(&doc, id);
        reported.extend(strands(&crate::fixture::without_anonymous(
            &applied.maintenance,
        )));
        doc = applied.doc;
    }
    assert!(
        reported.is_empty(),
        "the fillet's own selection left with the fillet: {reported:?}"
    );
}

// ---------------------------------------------------------------------
// The cascade, and the survivor outside it.
// ---------------------------------------------------------------------

/// **A cascade reports the strand at the step that made it.**
///
/// The fillet is deleted because its target is, and a shell of another
/// body outside the cascade names the fillet's space. Deleting the body
/// through `cascade_delete_order` therefore strands that shell — at
/// the step that removed the FILLET, not at the step that removed the
/// body, because the name says which node minted it and nothing else.
#[test]
fn a_cascade_reports_each_strand_at_the_step_that_made_it() {
    let doc = ProfileDoc::empty_derived("dm7_cascade", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, other) = block(doc, (4.0, 5.0), (0.0, 1.0), 1.0);
    let node2 = Node::Fillet {
        radius: len(0.1),
        selection: editor_core::Operand::select(body, vec![crate::fixture::end_rim(&doc, body, 0)]),
    };
    let (doc, fillet) = insert(doc, node2);
    let face = fname(fillet, wall(&doc, fillet, 2));
    let (doc, derived) = insert(
        doc,
        Node::Shell {
            thickness: len(0.1),
            open: editor_core::Operand::select(other, vec![face.clone()]),
        },
    );

    let order = cascade_delete_order(&doc, body);
    assert_eq!(
        order,
        vec![fillet, body],
        "the shell is no consumer of the body: it survives the cascade"
    );
    let mut doc = doc;
    let mut per_step = Vec::new();
    for id in order {
        let applied = delete(&doc, id);
        per_step.push((
            id,
            strands(&crate::fixture::without_anonymous(&applied.maintenance)),
        ));
        doc = applied.doc;
    }
    assert_eq!(
        per_step,
        vec![(fillet, vec![(derived, face.clone())]), (body, Vec::new())],
        "the strand is reported by the step that deleted the minting node"
    );
    // Again not "it survives" — the order assertion above already
    // says the cascade does not reach it. What is asserted is that it
    // still CARRIES the dead name: the report is a report, and the
    // repair is the user's `Rebind`.
    let Some(Node::Shell { open: still, .. }) = doc.node(derived) else {
        panic!("the survivor is still a shell")
    };
    assert_eq!(
        doc.selection(*still).map(|select| select.names.clone()),
        Some(vec![face]),
        "the survivor's selection holds the name unchanged, now resolving to nothing"
    );
}

// ---------------------------------------------------------------------
// A mate: a side is a read.
// ---------------------------------------------------------------------

/// A rest of `on`'s face against `to`'s, `on` the face of the copy it
/// places.
fn rest(
    to: impl crate::fixture::Sited,
    on: impl crate::fixture::Sited,
) -> editor_core::Mate<Formula> {
    fixture::rest(fixture::head(on), fixture::head(to))
}

fn instance_face(
    instance: RecipeNodeId,
    part_body: RecipeNodeId,
) -> crate::fixture::resolver::PartName {
    crate::fixture::resolver::PartName {
        at: instance,
        name: StableName {
            kind: EntityKind::Face,
            node: part_body,
            path: vec![RoleSeg::Cap(CapEnd::Start)],
        },
    }
}

/// **A mate's side strands as a read.**
///
/// A mate side is a read (D10): deleting the instance it is read in
/// strands the read, reported once, typed by the side's slot. The
/// side's name rides its selection and is not a second row.
#[test]
fn a_mates_side_strands_as_a_read() {
    let mut store = PartStore::new();
    let part = ProfileDoc::empty_derived("dm7_mate_part", Tol::witness());
    let (part, part_body) = block(part, (0.0, 1.0), (0.0, 1.0), 1.0);
    let part = crate::fixture::assemble_world_copies(part);
    let doc_ref = store.insert(part, Tol::witness());

    let doc = ProfileDoc::empty(DocumentId::derive("dm7_mate"), Tol::witness());
    let (doc, ia) = insert(doc, crate::fixture::mated_instance(doc_ref));
    let (doc, ib) = insert(doc, fixture::mated_instance(doc_ref));
    let head_a = instance_face(ia, part_body);
    let (doc, mate) = fixture::constrain(doc, rest(head_a.clone(), instance_face(ib, part_body)));

    let applied = delete(&doc, ia);
    let rows = crate::fixture::without_anonymous(&applied.maintenance);
    assert_eq!(
        rows.first(),
        Some(&Maintenance::StrandedRead {
            node: doc.spoken(mate.placement),
            slot: editor_core::OperandSlot::Side {
                constraint: mate.id,
                side: editor_core::MateSide::To,
            },
            var: doc.spoken_var(doc.output(ia, 0).expect("the instance's body")),
        }),
        "the side's read is reported"
    );
    // The product's `⊔` read the copy too, and strands as a read.
    assert!(
        matches!(
            &rows[1..],
            [
                Maintenance::StrandedRead { slot: editor_core::OperandSlot::Member(0), .. },
                Maintenance::StrandedSelection { readers, name, .. },
            ] if readers == &vec![doc.spoken(mate.placement)] && *name.name() == *head_a
        ),
        "and its selection, once, naming the side's face: {rows:?}"
    );
}

// ---------------------------------------------------------------------
// Determinism, and the load boundary.
// ---------------------------------------------------------------------

/// **The report is a function of the document and the edit.**
///
/// `Doc::replay` and `persist::load` hand back the document WITHOUT the
/// maintenance its edits performed — the documented load boundary, and
/// the asymmetry `replay-and-load-keep-the-document-without-its-
/// maintenance` records rather than this row. What makes that discard
/// lossless is asserted here instead: the same delete against the
/// round-tripped document reports exactly the rows it reported against
/// the document it was authored on, so a caller who wants them asks the
/// door again rather than needing them carried across the boundary.
#[test]
fn a_round_tripped_document_reports_the_same_strands() {
    let doc = ProfileDoc::empty_derived("dm7_round_trip", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block(doc, (4.0, 5.0), (0.0, 1.0), 1.0);
    let rim_of_b = crate::fixture::end_rim(&doc, b, 0);
    let (doc, _) = insert(
        doc,
        Node::Fillet {
            radius: len(0.1),
            selection: editor_core::Operand::select(body, vec![rim_of_b]),
        },
    );

    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("the document saves");
    let loaded = editor_core::persist::load(&text, Tol::witness()).expect("and loads");
    assert!(loaded.doc.bit_eq(&doc), "the snapshot round-trips");

    let direct = delete(&doc, b);
    let after_load = delete(&loaded.doc, b);
    assert!(
        !crate::fixture::without_anonymous(&direct.maintenance).is_empty(),
        "the delete strands the fillet's selected name"
    );
    assert_eq!(
        crate::fixture::without_anonymous(&direct.maintenance),
        crate::fixture::without_anonymous(&after_load.maintenance),
        "the report is derived from the document and the edit, so it survives the boundary \
         by being recomputable rather than by being carried"
    );
}

// ---------------------------------------------------------------------
// The second carrier: the appearance store.
// ---------------------------------------------------------------------

/// **A delete reports the appearance keys it stranded, and leaves the
/// attachments alone.**
///
/// The store is not a `Node::payload_names` carrier, so the payload
/// walk cannot see a key: this row is what the store's own pass
/// answers for. The attachment survives the delete — that is what
/// makes the key stranded rather than gone — and DM7 reports it
/// rather than repairing it, so the store is asserted unchanged
/// beside the report.
#[test]
fn a_delete_reports_the_appearance_keys_it_stranded() {
    let doc = ProfileDoc::empty_derived("dm7_appearance", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 1.0);
    let one = fname(victim, wall(&doc, victim, 0));
    let two = fname(victim, wall(&doc, victim, 2));
    let live = fname(body, wall(&doc, body, 0));
    let doc = paint(&doc, &one);
    let doc = paint(&doc, &two);
    let doc = paint(&doc, &live);
    // The WHOLE store, not the three keys: "it never repairs" is a
    // claim about every key and every record, and a walk that emptied
    // a stranded record's attrs would survive `contains_key`.
    let before = doc.appearance().clone();

    let applied = delete(&doc, victim);
    // The report runs in the store's order, which is name order.
    let mut stranded = vec![one, two];
    stranded.sort();
    assert_eq!(
        appearance_strands(&crate::fixture::without_anonymous(&applied.maintenance)),
        stranded,
        "both keys the deleted node minted, and only those"
    );
    assert_eq!(
        applied.doc.appearance(),
        &before,
        "the store is untouched: DM7 reports, it never repairs"
    );
}

/// **A key whose minting node is live is never reported, in the same
/// document as one whose minting node just went.** The report is a
/// report of what this edit did, not a dump of the store — and the
/// row carries tension both ways: a walk that reported every key
/// fails on the live one, a walk that reported none fails on the
/// stranded one. `a_delete_that_strands_nothing_reports_nothing` is
/// the sibling that holds the all-silent case, where there is no
/// stranded key to find; this one is the discrimination.
#[test]
fn an_appearance_key_minted_by_a_live_node_is_never_reported() {
    let doc = ProfileDoc::empty_derived("dm7_appearance_live", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 1.0);
    let live = fname(body, wall(&doc, body, 0));
    let doomed = fname(victim, wall(&doc, victim, 0));
    let doc = paint(&doc, &live);
    let doc = paint(&doc, &doomed);

    let applied = delete(&doc, victim);
    assert_eq!(
        appearance_strands(&crate::fixture::without_anonymous(&applied.maintenance)),
        vec![doomed],
        "the key the deleted node minted, and not the one the live node did: {:?}",
        crate::fixture::without_anonymous(&applied.maintenance)
    );
    assert!(
        crate::fixture::painted(&applied.doc, &live).is_some(),
        "and the live node's paint is still in the store"
    );
}

/// **The selection strands come first, then the appearance strands.**
///
/// The order on `Applied::maintenance` is a contract, and this is the
/// edit that produces both kinds at once: one node mints the name a
/// surviving fillet's selection holds AND the key the store holds. Written out
/// as one vector, so a walk that ran the store first goes red here
/// rather than somewhere a consumer finds it.
#[test]
fn an_appearance_strand_follows_the_payload_strands_of_the_same_delete() {
    let doc = ProfileDoc::empty_derived("dm7_appearance_order", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 1.0);
    // The fillet's selection reads `body`; what it NAMES is an edge of
    // `victim`, which is a name and not a read, so deleting `victim` is
    // accepted and strands it.
    let carried = crate::fixture::end_rim(&doc, victim, 0);
    let (doc, fillet) = insert(
        doc,
        Node::Fillet {
            radius: len(0.1),
            selection: editor_core::Operand::select(body, vec![carried.clone()]),
        },
    );
    let painted = fname(victim, wall(&doc, victim, 2));
    let doc = paint(&doc, &painted);

    let applied = delete(&doc, victim);
    assert_eq!(
        crate::fixture::without_anonymous(&applied.maintenance),
        vec![
            Maintenance::StrandedSelection {
                var: doc.spoken_var(selection_of(&doc, fillet)),
                readers: vec![doc.spoken(fillet)],
                name: doc.spoken_name(&carried),
                took: editor_core::Took::Node
            },
            Maintenance::StrandedAppearance {
                body: doc.spoken_var(crate::fixture::painted_in(&doc, &painted)),
                name: doc.spoken_name(&painted),
                took: editor_core::Took::Node
            },
        ],
        "the payload carriers are walked before the store"
    );
}

/// **A delete's report is its strands alone, payload then store, and
/// adding a constraint reports nothing.**
///
/// Deleting an instance a mate reads leaves the bundle that holds the
/// mate where it is (ASSEMBLY.md A11 (2)), and an added constraint
/// strands nothing. This row pins both halves: the constraint's add
/// reports no row, and the delete of the painted instance the mate
/// reads reports the read strand, then the appearance strand, and
/// nothing after them.
#[test]
fn a_delete_reports_its_strands_alone_and_a_constraint_add_reports_nothing() {
    let mut store = PartStore::new();
    let part = ProfileDoc::empty_derived("dm7_app_order_part", Tol::witness());
    let (part, part_body) = block(part, (0.0, 1.0), (0.0, 1.0), 1.0);
    let part = crate::fixture::assemble_world_copies(part);
    let doc_ref = store.insert(part, Tol::witness());

    let doc = ProfileDoc::empty(DocumentId::derive("dm7_app_order"), Tol::witness());
    let (doc, ia) = insert(doc, crate::fixture::mated_instance(doc_ref));
    let (doc, ib) = insert(doc, fixture::mated_instance(doc_ref));
    let head_a = instance_face(ia, part_body);
    let mated = apply(
        &doc,
        // The mate written with a zero on each freedom it leaves, whole:
        // every bundle pins its copy.
        &crate::fixture::squared(
            &doc,
            rest(head_a.clone(), instance_face(ib, part_body)),
            &[],
        )
        .write(ib),
        Tol::witness(),
    )
    .expect("the mate is added");
    assert!(
        crate::fixture::without_anonymous(&mated.maintenance).is_empty(),
        "the constraint's add strands nothing: {:?}",
        crate::fixture::without_anonymous(&mated.maintenance)
    );
    let mate = ib;
    let doc = mated.doc;
    let painted = instance_face(ia, part_body);
    let doc = paint(&doc, &painted);

    let applied = delete(&doc, ia);
    let rows = crate::fixture::without_anonymous(&applied.maintenance);
    assert!(
        matches!(
            rows.as_slice(),
            [
                Maintenance::StrandedRead { node, .. },
                Maintenance::StrandedRead { slot: editor_core::OperandSlot::Member(0), .. },
                Maintenance::StrandedSelection { .. },
                Maintenance::StrandedAppearance { name, .. },
            ] if *node == doc.spoken(mate) && *name == doc.spoken_name(&painted.name)
        ),
        "both strand kinds are read at the door, and the delete reports no placement row: \
         {rows:?}"
    );
}

/// **A cascade reports each appearance strand at the step that made
/// it**, and the store keeps every attachment the cascade orphaned.
///
/// The payload half has a case where a strand is never reported at
/// all — a carrier deleted alongside the node it names
/// (`a_carrier_deleted_with_the_node_it_names_reports_nothing`). The
/// store has no such case: it is not a node and no cascade removes
/// it, so a key whose minting node goes is reported at that node's
/// step and outlives the whole cascade. That asymmetry is the shape a
/// caller counting strands over a doomed set has to know about.
#[test]
fn a_cascade_reports_each_appearance_strand_at_the_step_that_made_it() {
    let doc = ProfileDoc::empty_derived("dm7_appearance_cascade", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let node3 = Node::Fillet {
        radius: len(0.1),
        selection: editor_core::Operand::select(body, vec![crate::fixture::end_rim(&doc, body, 0)]),
    };
    let (doc, fillet) = insert(doc, node3);
    let on_fillet = fname(fillet, wall(&doc, fillet, 2));
    let on_body = fname(body, wall(&doc, body, 2));
    let doc = paint(&doc, &on_fillet);
    let doc = paint(&doc, &on_body);
    let before = doc.appearance().clone();

    let order = cascade_delete_order(&doc, body);
    assert_eq!(order, vec![fillet, body], "the consumer goes first");
    let mut doc = doc;
    let mut reported = Vec::new();
    for id in order {
        let applied = delete(&doc, id);
        reported.push(appearance_strands(&crate::fixture::without_anonymous(
            &applied.maintenance,
        )));
        doc = applied.doc;
    }
    assert_eq!(
        reported,
        vec![vec![on_fillet.clone()], vec![on_body.clone()]],
        "each key at the step that removed the node that minted it"
    );
    assert_eq!(
        doc.appearance(),
        &before,
        "every attachment outlives the whole cascade, unchanged"
    );
}

/// **The repair path still works on a reported key.**
/// `ClearAppearance` deliberately does not require a live node — it is
/// how a stranded attachment is retired — so the row the door just
/// reported is actionable, and clearing it is what takes it out of the
/// store. `Rebind` is the other repair; this is the one that needs the
/// dead node to be acceptable.
#[test]
fn a_reported_appearance_strand_is_still_clearable() {
    let doc = ProfileDoc::empty_derived("dm7_appearance_clear", Tol::witness());
    let (doc, _body) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 1.0);
    let painted = fname(victim, wall(&doc, victim, 0));
    let doc = paint(&doc, &painted);

    let applied = delete(&doc, victim);
    assert_eq!(
        appearance_strands(&crate::fixture::without_anonymous(&applied.maintenance)),
        vec![painted.clone()],
        "the door named the key"
    );

    let cleared = apply(
        &applied.doc,
        &DocEdit::ClearAppearance {
            body: crate::fixture::painted_in(&doc, &painted),
            name: painted.clone(),
            kind: AttrKind::Color,
        },
        Tol::witness(),
    )
    .expect("clearing does not require the name's node to be live")
    .doc;
    assert!(
        !crate::fixture::painted(&cleared, &painted).is_some(),
        "the stranded attachment is gone once its last attribute is cleared"
    );
}

/// **The appearance half of the report survives the load boundary the
/// same way the payload half does**: by being recomputable. The store
/// round-trips with the document, so the same delete against the
/// loaded value reports the same keys — a stranded key's dead minting
/// node is in the mint log and the save validator accepts it.
#[test]
fn a_round_tripped_document_reports_the_same_appearance_strands() {
    let doc = ProfileDoc::empty_derived("dm7_appearance_round_trip", Tol::witness());
    let (doc, _body) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 1.0);
    let doc = paint(&doc, &fname(victim, wall(&doc, victim, 0)));
    let doc = paint(&doc, &fname(victim, wall(&doc, victim, 2)));

    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("the document saves");
    let loaded = editor_core::persist::load(&text, Tol::witness()).expect("and loads");
    assert!(loaded.doc.bit_eq(&doc), "the snapshot round-trips");

    let direct = delete(&doc, victim);
    let after_load = delete(&loaded.doc, victim);
    assert_eq!(
        appearance_strands(&crate::fixture::without_anonymous(&direct.maintenance)).len(),
        2,
        "the delete strands both painted faces"
    );
    assert_eq!(
        crate::fixture::without_anonymous(&direct.maintenance),
        crate::fixture::without_anonymous(&after_load.maintenance),
        "the store's half is derived from the document and the edit too"
    );
}
