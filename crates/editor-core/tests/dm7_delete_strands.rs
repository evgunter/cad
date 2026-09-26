//! **DM7 — a stranded payload name is REPORTED at the delete, never
//! refused.**
//!
//! A payload name — `Node::payload_names` is the list of which
//! payloads carry one — is not a DAG edge: the insert
//! door checks its minting node is live and no other door does, so a
//! later `DeleteNode` strands it. The delete stays legal — a full edge
//! would deadlock the declared union, whose `Declare` names the very
//! union that consumes it — and what the door owes instead is the
//! report: one `Maintenance::Strand` per surviving `(node, name)`
//! whose minting node the edit removed.
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
//! The door's third row is not a strand at all: a `Declare` whose
//! LAST consumer the delete removed is left inert rather than
//! dangling — the `declare` edge runs from the consumer to it, so no
//! name is stranded and the document stays legal — and rides the
//! same record as `Maintenance::OrphanedDeclare { declare }`. Its
//! rule is a TRANSITION (a fresh declaration is legally consumerless
//! until its consumer is authored), which is what the rows at the
//! end of this file are about.
//!
//! These rows are what goes red when a strand or an orphan goes
//! unreported.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::{block, declared_union, flush_pairs};
use crate::fixture;
use crate::fixture::resolver::PartStore;
use editor_core::{
    Alignment, Attr, AttrKind, AxisSense, BooleanOp, CapEnd, ContactClass, Datum, DocEdit,
    DocumentId, EditError, EntityKind, Maintenance, MateFrame, MatePrimitive, MeasureExpr,
    MeasurePrimitive, Node, ProfileDoc, RecipeNodeId, Rgba8, RoleSeg, SitedRef, StableName, apply,
    cascade_delete_order,
};
use fixture::{ang, fname, insert, len, wall};
use geom_core::Tol;

/// The strand rows an accepted edit reported, in the order it reported
/// them — the whole of what these rows assert about.
fn strands(applied: &[Maintenance]) -> Vec<(RecipeNodeId, StableName)> {
    applied
        .iter()
        .filter_map(|row| match row {
            Maintenance::Strand { node, name } => Some((*node, name.clone())),
            Maintenance::Cluster(_)
            | Maintenance::StrandedAppearance { .. }
            | Maintenance::OrphanedDeclare { .. } => None,
        })
        .collect()
}

/// The appearance keys an accepted edit reported stranded, in the
/// order it reported them — the store's half of the same report.
fn appearance_strands(applied: &[Maintenance]) -> Vec<StableName> {
    applied
        .iter()
        .filter_map(|row| match row {
            Maintenance::StrandedAppearance { name } => Some(name.clone()),
            Maintenance::Strand { .. }
            | Maintenance::Cluster(_)
            | Maintenance::OrphanedDeclare { .. } => None,
        })
        .collect()
}

/// Paint one face red, expecting the door to accept it: the name's
/// node is live at the edit, which is all `SetAppearance` asks.
fn paint(doc: &editor_core::ProfileDoc, name: &StableName) -> ProfileDoc {
    apply(
        doc,
        &DocEdit::SetAppearance {
            name: name.clone(),
            attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the painted name's node is live")
    .doc
}

/// Delete one node, expecting the door to accept it. The documents
/// these rows delete from hold no mated instance, so the reach is the
/// refusing one; a row whose delete moves a cluster's gauge goes
/// through [`delete_with`] and the store's reach.
fn delete(
    doc: &editor_core::ProfileDoc,
    id: RecipeNodeId,
) -> editor_core::Applied<editor_core::ProfileProgram> {
    delete_with(doc, id, &editor_core::RefusingReach)
}

/// [`delete`] through `reach` — the store's, where the delete moves a
/// mated cluster's gauge and the maintenance solves for its frame.
fn delete_with(
    doc: &ProfileDoc,
    id: RecipeNodeId,
    reach: &dyn editor_core::MateReach,
) -> editor_core::Applied<editor_core::ProfileProgram> {
    apply(doc, &DocEdit::DeleteNode { id }, Tol::witness(), reach)
        .expect("a payload name is not a DAG edge, so the delete is legal")
}

// ---------------------------------------------------------------------
// The declared union — DOCM-7's R1 probe, as a row.
// ---------------------------------------------------------------------

/// **A strand per NAME and none per site**, on a declared union.
///
/// The `Declare` is the union's INPUT, so deleting it dangles an edge
/// and is refused typed. What the pairs NAME is not an edge: a sited
/// pair names entities in the MEMBERS, so deleting the union strands
/// nothing at all, and deleting a member — once the union is gone and
/// nothing consumes it — strands the names minted there, one row per
/// name. The SITE is reported by neither delete: a site is a reading
/// edge, and a deleted one is N5's dangling case refused at the next
/// evaluation (`Node::payload_read_sites`, DM7's sentence).
#[test]
fn deleting_a_declared_member_names_its_pairs_and_its_site_reports_nothing() {
    let doc = ProfileDoc::empty_derived("dm7_declared_union", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let pairs = flush_pairs(&doc, (a, a), (b, b));
    let (doc, union, decl) = declared_union(doc, &[a, b], pairs);

    assert!(
        matches!(
            apply(
                &doc,
                &DocEdit::DeleteNode { id: decl },
                Tol::witness(),
                &editor_core::RefusingReach
            ),
            Err(EditError::DeleteWouldDangle { .. })
        ),
        "the declare edge IS a DAG edge and refuses"
    );

    let Some(Node::Declare { pairs }) = doc.node(decl) else {
        panic!("the declaration is a Declare")
    };
    assert_eq!(pairs.len(), 4, "four flush pairs");
    assert!(
        pairs
            .iter()
            .flat_map(|((x, y), _)| [x, y])
            .all(|r| r.name.node == r.at && [a, b].contains(&r.at)),
        "every side is sited at the member whose table holds it"
    );

    // The union is what the declaration's sites POINT AT, and a site
    // is not a name: deleting it strands nothing.
    let freed = delete(&doc, union);
    assert_eq!(
        strands(&freed.maintenance),
        Vec::new(),
        "a site is not a strand"
    );

    // The member, now consumed by nothing, IS a name's minting node.
    let applied = delete(&freed.doc, b);
    let expected: Vec<(RecipeNodeId, StableName)> = pairs
        .iter()
        .flat_map(|((x, y), _)| [x, y])
        .filter(|r| r.name.node == b)
        .map(|r| (decl, r.name.clone()))
        .collect();
    assert_eq!(
        expected.len(),
        4,
        "one name per pair is minted in the deleted member"
    );
    assert_eq!(
        strands(&applied.maintenance),
        expected,
        "the accepted delete names every stranded name, in the payload's own order"
    );
    // NOT "the Declare survives": a strand row names a SURVIVING
    // carrier by construction, so the rows above already say that.
    // What they do not say is that the payload is untouched — DM7
    // reports, it does not repair — so that is what is asserted.
    let Some(Node::Declare { pairs: after }) = applied.doc.node(decl) else {
        panic!("the Declare survives its member — the orphan the user cascades or deletes")
    };
    assert_eq!(
        after, pairs,
        "the report changes nothing: every pair still says exactly what it said"
    );
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
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);

    let f0 = fname(victim, wall(&doc, victim, 0));
    let f1 = fname(victim, wall(&doc, victim, 1));
    let f2 = fname(victim, wall(&doc, victim, 2));
    let f3 = fname(victim, RoleSeg::Cap(CapEnd::End));
    let f4 = fname(victim, RoleSeg::Cap(CapEnd::Start));

    let (doc, fillet) = insert(
        doc,
        Node::Fillet {
            target: body,
            radius: len(0.1),
            selection: vec![f0.clone()],
        },
    );
    let (doc, chamfer) = insert(
        doc,
        Node::Chamfer {
            target: body,
            distance: len(0.1),
            selection: vec![f1.clone()],
        },
    );
    let (doc, shell) = insert(
        doc,
        Node::Shell {
            target: body,
            thickness: len(0.1),
            open: vec![f2.clone()],
        },
    );
    let (doc, derived) = insert(
        doc,
        Node::Datum(Datum::FaceFrame {
            at: body,
            face: f3.clone(),
            spin: ang(0.0),
        }),
    );
    let (doc, measure) = insert(
        doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![
                SitedRef::new(body, f4.clone()),
                SitedRef::new(fillet, f0.clone()),
            ],
        )
        .expect("both indices address a reference"),
    );
    let (doc, decl) = insert(
        doc,
        Node::declare_rest(vec![(
            SitedRef::new(victim, f1.clone()),
            SitedRef::new(victim, f2.clone()),
        )]),
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
    for &id in doc.order() {
        let Some(node) = doc.node(id) else { continue };
        match node {
            Node::Fillet { .. } => expected.push((id, f0.clone())),
            Node::Chamfer { .. } => expected.push((id, f1.clone())),
            Node::Shell { .. } => expected.push((id, f2.clone())),
            Node::Datum(Datum::FaceFrame { .. }) => expected.push((id, f3.clone())),
            // Argument order, which is meaning for a measure.
            Node::Measure { .. } => expected.extend([(id, f4.clone()), (id, f0.clone())]),
            Node::Declare { .. } => expected.extend([(id, f1.clone()), (id, f2.clone())]),
            // A mate's heads are the seventh carrier and need an
            // instance to be minted by, which this document has none
            // of: its row is `a_mates_head_strands_and_its_read_site_does_not`.
            Node::Mate { .. } => panic!("this fixture builds no mate"),
            // An instance's crossing `outer`s are the eighth carrier
            // and need an interface record to ride, which only a
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
    assert_eq!(
        expected.iter().map(|(id, _)| *id).collect::<Vec<_>>(),
        vec![
            fillet, chamfer, shell, derived, measure, measure, decl, decl
        ],
        "six carriers, eight names, in document order"
    );

    let applied = delete(&doc, victim);
    assert_eq!(
        strands(&applied.maintenance),
        expected,
        "one row per carried name, in document order and then payload order"
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
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, spare) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let node = Node::Fillet {
        target: body,
        radius: len(0.1),
        selection: vec![fname(body, wall(&doc, body, 0))],
    };
    let (doc, _fillet) = insert(doc, node);

    let applied = delete(&doc, spare);
    assert!(
        applied.maintenance.is_empty(),
        "nothing named the deleted node: {:?}",
        applied.maintenance
    );
}

/// **A name that leaves with its own carrier strands nothing.**
/// Deleting the carrier and the minting node in one cascade must not
/// report the carrier's own names: there is no surviving node left
/// holding them, so there is nothing for a caller to rebind.
#[test]
fn a_carrier_deleted_with_the_node_it_names_reports_nothing() {
    let doc = ProfileDoc::empty_derived("dm7_self", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let node1 = Node::Fillet {
        target: body,
        radius: len(0.1),
        selection: vec![fname(body, wall(&doc, body, 0))],
    };
    let (doc, fillet) = insert(doc, node1);

    let order = cascade_delete_order(&doc, body);
    assert_eq!(order, vec![fillet, body], "the consumer goes first");
    let mut doc = doc;
    let mut reported = Vec::new();
    for id in order {
        let applied = delete(&doc, id);
        reported.extend(strands(&applied.maintenance));
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
/// The fillet is deleted because its target is, and a derived frame
/// outside the cascade names the fillet's space. Deleting the body
/// through `cascade_delete_order` therefore strands that frame — at
/// the step that removed the FILLET, not at the step that removed the
/// body, because the name says which node minted it and nothing else.
#[test]
fn a_cascade_reports_each_strand_at_the_step_that_made_it() {
    let doc = ProfileDoc::empty_derived("dm7_cascade", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, other) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let node2 = Node::Fillet {
        target: body,
        radius: len(0.1),
        selection: vec![fname(body, wall(&doc, body, 0))],
    };
    let (doc, fillet) = insert(doc, node2);
    let face = fname(fillet, wall(&doc, fillet, 2));
    let (doc, derived) = insert(
        doc,
        Node::Datum(Datum::FaceFrame {
            at: other,
            face: face.clone(),
            spin: ang(0.0),
        }),
    );

    let order = cascade_delete_order(&doc, body);
    assert_eq!(
        order,
        vec![fillet, body],
        "the derived frame is no consumer of the body: it survives the cascade"
    );
    let mut doc = doc;
    let mut per_step = Vec::new();
    for id in order {
        let applied = delete(&doc, id);
        per_step.push((id, strands(&applied.maintenance)));
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
    let Some(Node::Datum(Datum::FaceFrame { face: still, .. })) = doc.node(derived) else {
        panic!("the survivor is still a derived frame")
    };
    assert_eq!(
        still, &face,
        "the survivor holds the name unchanged, now resolving to nothing"
    );
}

// ---------------------------------------------------------------------
// A mate: a head is a name, an operand is not.
// ---------------------------------------------------------------------

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

/// **A mate's HEAD strands and its operand does not.**
///
/// The two references a mate carries are different kinds of thing. The
/// head is a `StableName` — resolved through the N5 ladder, repairable
/// by `Rebind` — so its minting node going is a strand and is reported.
/// The operand is a bare node id (`Node::payload_read_sites`, the A12
/// reading edge): nothing resolves it through a ladder and `Rebind`
/// cannot touch it, so a delete that takes it away is the solve's to
/// refuse, and this door says nothing about it. One row per head is
/// therefore the whole report, although the deleted instance was both.
#[test]
fn a_mates_head_strands_and_its_read_site_does_not() {
    let mut store = PartStore::new();
    let part = ProfileDoc::empty_derived("dm7_mate_part", Tol::witness());
    let (part, part_body) = block(part, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let doc_ref = store.insert(part, Tol::witness());
    // Deleting the gauge instance rewrites the pair's gauge, so the
    // delete levers the parts through the store's reach.
    let opts = fixture::resolver::with_resolver(store);
    let reach = editor_core::mate_reach::<f64>(&opts, Tol::witness());

    let doc = ProfileDoc::empty(DocumentId::derive("dm7_mate"), Tol::witness());
    let (doc, ia) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, ib) = insert(doc, Node::instantiate_part(doc_ref));
    let head_a = instance_face(ia, part_body);
    let (doc, mate) = insert(
        doc,
        Node::Mate {
            a: crate::fixture::head(head_a.clone()),
            b: crate::fixture::head(instance_face(ib, part_body)),
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

    let applied = delete_with(&doc, ia, &reach);
    assert_eq!(
        strands(&applied.maintenance),
        vec![(mate, head_a)],
        "the head is a name and is reported; the operand at the same id is not"
    );
    let first_cluster = applied
        .maintenance
        .iter()
        .position(|row| matches!(row, Maintenance::Cluster(_)));
    if let Some(at) = first_cluster {
        assert!(
            applied.maintenance[..at]
                .iter()
                .all(|row| matches!(row, Maintenance::Strand { .. })),
            "the strands are read at the door, before the registry reconciles: {:?}",
            applied.maintenance
        );
    }
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
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let pairs = flush_pairs(&doc, (a, a), (b, b));
    let (doc, union, _decl) = declared_union(doc, &[a, b], pairs);

    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("the document saves");
    let loaded = editor_core::persist::load(&text, Tol::witness()).expect("and loads");
    assert!(loaded.doc.bit_eq(&doc), "the snapshot round-trips");

    // The union goes first — a member is its DAG input while it lives —
    // and then the member, which is what a declared name is minted in.
    let direct = delete(&delete(&doc, union).doc, b);
    let after_load = delete(&delete(&loaded.doc, union).doc, b);
    assert!(
        !direct.maintenance.is_empty(),
        "the delete strands the declaration's names"
    );
    assert_eq!(
        direct.maintenance, after_load.maintenance,
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
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
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
    assert_eq!(
        appearance_strands(&applied.maintenance),
        vec![one, two],
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
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let live = fname(body, wall(&doc, body, 0));
    let doomed = fname(victim, wall(&doc, victim, 0));
    let doc = paint(&doc, &live);
    let doc = paint(&doc, &doomed);

    let applied = delete(&doc, victim);
    assert_eq!(
        appearance_strands(&applied.maintenance),
        vec![doomed],
        "the key the deleted node minted, and not the one the live node did: {:?}",
        applied.maintenance
    );
    assert!(
        applied.doc.appearance().contains_key(&live),
        "and the live node's paint is still in the store"
    );
}

/// **The payload strands come first, then the appearance strands.**
///
/// The order on `Applied::maintenance` is a contract, and this is the
/// edit that produces both kinds at once: one node mints the name a
/// surviving fillet carries AND the key the store holds. Written out
/// as one vector, so a walk that ran the store first goes red here
/// rather than somewhere a consumer finds it.
#[test]
fn an_appearance_strand_follows_the_payload_strands_of_the_same_delete() {
    let doc = ProfileDoc::empty_derived("dm7_appearance_order", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    // The fillet's own DAG input is `body`; what it NAMES is a face of
    // `victim`, which is a payload name and not an edge, so deleting
    // `victim` is accepted and strands it.
    let carried = fname(victim, wall(&doc, victim, 0));
    let (doc, fillet) = insert(
        doc,
        Node::Fillet {
            target: body,
            radius: len(0.1),
            selection: vec![carried.clone()],
        },
    );
    let painted = fname(victim, wall(&doc, victim, 2));
    let doc = paint(&doc, &painted);

    let applied = delete(&doc, victim);
    assert_eq!(
        applied.maintenance,
        vec![
            Maintenance::Strand {
                node: fillet,
                name: carried,
            },
            Maintenance::StrandedAppearance { name: painted },
        ],
        "the payload carriers are walked before the store"
    );
}

/// **The appearance strand is inside the strand segment, ahead of the
/// cluster acts** — the second boundary of `Applied::maintenance`'s
/// order contract, for the second carrier.
///
/// `a_mates_head_strands_and_its_read_site_does_not` holds that
/// boundary for a PAYLOAD strand only: its fixture paints nothing, so
/// a walk that appended the store's rows after `reconcile` passes it
/// unchanged. This is the edit that produces all three kinds at once
/// — a mate head stranded, a painted instance face stranded, and the
/// registry act the deleted instance forced — so it is the row that
/// reds when the store's pass moves behind the reconcile.
///
/// (Authored by the style review of this unit as
/// `rv_an_appearance_strand_precedes_the_cluster_acts_of_the_same_delete`
/// and adopted here, because what it pins is a documented boundary
/// rather than a mutant's residue.)
#[test]
fn an_appearance_strand_precedes_the_cluster_acts_of_the_same_delete() {
    let mut store = PartStore::new();
    let part = ProfileDoc::empty_derived("dm7_app_order_part", Tol::witness());
    let (part, part_body) = block(part, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let doc_ref = store.insert(part, Tol::witness());
    // Deleting the gauge instance rewrites the pair's gauge, so the
    // delete levers the parts through the store's reach.
    let opts = fixture::resolver::with_resolver(store);
    let reach = editor_core::mate_reach::<f64>(&opts, Tol::witness());

    let doc = ProfileDoc::empty(DocumentId::derive("dm7_app_order"), Tol::witness());
    let (doc, ia) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, ib) = insert(doc, Node::instantiate_part(doc_ref));
    let head_a = instance_face(ia, part_body);
    let (doc, mate) = insert(
        doc,
        Node::Mate {
            a: crate::fixture::head(head_a.clone()),
            b: crate::fixture::head(instance_face(ib, part_body)),
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
    let doc = paint(&doc, &painted);

    let applied = delete_with(&doc, ia, &reach);
    let cluster = applied
        .maintenance
        .iter()
        .position(|row| matches!(row, Maintenance::Cluster(_)))
        .expect("the deleted instance forces a registry act, so the row is not vacuous");
    assert_eq!(
        &applied.maintenance[..cluster],
        &[
            Maintenance::Strand {
                node: mate,
                name: head_a,
            },
            Maintenance::StrandedAppearance { name: painted },
        ],
        "both strand kinds are read at the door, before the registry reconciles: {:?}",
        applied.maintenance
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
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let node3 = Node::Fillet {
        target: body,
        radius: len(0.1),
        selection: vec![fname(body, wall(&doc, body, 0))],
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
        reported.push(appearance_strands(&applied.maintenance));
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
    let (doc, _body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let painted = fname(victim, wall(&doc, victim, 0));
    let doc = paint(&doc, &painted);

    let applied = delete(&doc, victim);
    assert_eq!(
        appearance_strands(&applied.maintenance),
        vec![painted.clone()],
        "the door named the key"
    );

    let cleared = apply(
        &applied.doc,
        &DocEdit::ClearAppearance {
            name: painted.clone(),
            kind: AttrKind::Color,
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("clearing does not require the name's node to be live")
    .doc;
    assert!(
        !cleared.appearance().contains_key(&painted),
        "the stranded attachment is gone once its last attribute is cleared"
    );
}

/// **The appearance half of the report survives the load boundary the
/// same way the payload half does**: by being recomputable. The store
/// round-trips with the document, so the same delete against the
/// loaded value reports the same keys — a stranded key's dead minting
/// node is below the mint counter and the save validator accepts it.
#[test]
fn a_round_tripped_document_reports_the_same_appearance_strands() {
    let doc = ProfileDoc::empty_derived("dm7_appearance_round_trip", Tol::witness());
    let (doc, _body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let doc = paint(&doc, &fname(victim, wall(&doc, victim, 0)));
    let doc = paint(&doc, &fname(victim, wall(&doc, victim, 2)));

    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("the document saves");
    let loaded = editor_core::persist::load(&text, Tol::witness()).expect("and loads");
    assert!(loaded.doc.bit_eq(&doc), "the snapshot round-trips");

    let direct = delete(&doc, victim);
    let after_load = delete(&loaded.doc, victim);
    assert_eq!(
        appearance_strands(&direct.maintenance).len(),
        2,
        "the delete strands both painted faces"
    );
    assert_eq!(
        direct.maintenance, after_load.maintenance,
        "the store's half is derived from the document and the edit too"
    );
}

// ---------------------------------------------------------------------
// The orphaned declaration: the other thing a delete can leave behind.
// ---------------------------------------------------------------------

/// **The delete that takes a declaration's LAST consumer reports it;
/// the one that leaves a consumer behind does not.**
///
/// Two unions over the same two members share one `Declare` — the
/// `declare` edge is an ordinary DAG input and nothing makes it
/// exclusive. Deleting the first union leaves the declaration read by
/// the second, so there is nothing to say; deleting the second says
/// it, once. A door that reported "this deleted node consumed a
/// `Declare`" rather than "and nothing else does" reds on the first
/// delete.
#[test]
fn an_orphan_is_reported_by_the_delete_that_takes_the_last_consumer() {
    let doc = ProfileDoc::empty_derived("dm7_orphan_two_consumers", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let pairs = flush_pairs(&doc, (a, a), (b, b));
    let (doc, first, decl) = declared_union(doc, &[a, b], pairs);
    let (doc, second) = insert(
        doc,
        Node::Union {
            members: vec![a, b],
            declare: Some(decl),
        },
    );

    let one_left = delete(&doc, first);
    assert_eq!(
        one_left.maintenance,
        Vec::new(),
        "the declaration is still read by the second union"
    );
    let none_left = delete(&one_left.doc, second);
    assert_eq!(
        none_left.maintenance,
        vec![Maintenance::OrphanedDeclare { declare: decl }],
        "the last consumer's delete is the transition, and reports it once"
    );
    let Some(Node::Declare { .. }) = none_left.doc.node(decl) else {
        panic!("the report is a report: the declaration is untouched and live")
    };

    // A `Boolean` and a `Union` sharing one declaration are the same
    // row, not a second: the door counts `Node::inputs`, so neither
    // kind is privileged and neither order is. Re-authored on the
    // document both unions have left, so the pair really is the
    // whole consumer set. Deleting either alone reports nothing; the
    // second reports the orphan once.
    let (mixed, the_union) = insert(
        none_left.doc,
        Node::Union {
            members: vec![a, b],
            declare: Some(decl),
        },
    );
    let (mixed, the_boolean) = insert(
        mixed,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(decl),
        },
    );
    for (leaves_one, takes_the_last) in [(the_union, the_boolean), (the_boolean, the_union)] {
        let one_left = delete(&mixed, leaves_one);
        assert_eq!(
            one_left.maintenance,
            Vec::new(),
            "the other kind still consumes the declaration"
        );
        let none_left = delete(&one_left.doc, takes_the_last);
        assert_eq!(
            none_left.maintenance,
            vec![Maintenance::OrphanedDeclare { declare: decl }],
            "the last consumer is the last consumer whatever kind it is"
        );
    }
}

/// **At most ONE orphan row is producible per accepted delete, under
/// the node vocabulary as it stands** — the guard that reds the day a
/// node kind holds two declarations.
///
/// `Node::declare_input` is an `Option` and no kind holds two, so the
/// `Vec` the door returns describes a set whose size is provably 0 or
/// 1 and the "in the deleted node's input order" clause of
/// `Applied::maintenance` carries no weight. The day a kind does hold
/// two, this row goes red and that clause needs a row of its own.
#[test]
fn no_delete_can_report_two_orphans_today() {
    let doc = ProfileDoc::empty_derived("dm7_orphan_at_most_one", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let pairs = flush_pairs(&doc, (a, a), (b, b));
    let (doc, union, decl) = declared_union(doc, &[a, b], pairs);
    // A second declaration, consumerless, in the same document.
    let pairs = flush_pairs(&doc, (a, a), (b, b));
    let (doc, spare) = insert(doc, Node::declare_rest(pairs));
    let applied = delete(&doc, union);
    assert_eq!(
        applied.maintenance,
        vec![Maintenance::OrphanedDeclare { declare: decl }],
        "one delete, one declare edge, one row"
    );
    assert!(
        applied.doc.node(spare).is_some(),
        "the other declaration is nobody's business here"
    );
    for id in applied.doc.order() {
        let node = applied.doc.node(*id).expect("live");
        let declares = node
            .inputs()
            .iter()
            .filter(|i| matches!(applied.doc.node(**i), Some(Node::Declare { .. })))
            .count();
        assert!(
            declares <= 1,
            "a node with two declare inputs would make the order clause load-bearing"
        );
    }
}

/// **No edit but `DeleteNode` can drop a `declare` edge**, which is
/// what makes the delete door the only place the report is owed.
/// `SetMembers` is the vocabulary's one rewire of a live node's
/// inputs, and it leaves `declare` exactly as it was.
#[test]
fn set_members_cannot_orphan_a_declaration() {
    let doc = ProfileDoc::empty_derived("dm7_orphan_set_members", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (1.0, 2.0), (0.0, 1.0), 0.0, 1.0);
    let pairs = flush_pairs(&doc, (a, a), (b, b));
    let (doc, union, decl) = declared_union(doc, &[a, b], pairs);

    let applied = apply(
        &doc,
        &DocEdit::SetMembers {
            node: union,
            members: vec![a, b, c],
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the member list is replaceable");
    assert_eq!(
        applied.maintenance,
        Vec::new(),
        "a rewire reports nothing, and there is nothing to report"
    );
    assert_eq!(
        applied.doc.node(union).expect("live").declare_input(),
        Some(decl),
        "SetMembers leaves the declare edge as it was, so it cannot orphan"
    );
}

/// **A delete elsewhere in the document never reports a declaration
/// that was already consumerless.**
///
/// The rule is a transition, not a state: a `Declare` is authored
/// FIRST and its consumer second (DM4), so a document is legally
/// carrying a consumerless declaration for the whole of that window,
/// and a delete of an unrelated node says nothing about it. This is
/// the row a walk over "every consumerless `Declare` in the document"
/// reds on, and the reason the candidates are the deleted node's own
/// inputs.
#[test]
fn a_consumerless_declare_is_not_reported_by_an_unrelated_delete() {
    let doc = ProfileDoc::empty_derived("dm7_orphan_transition", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, unrelated) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let pairs = flush_pairs(&doc, (a, a), (a, a));
    let (doc, decl) = insert(doc, Node::declare_rest(pairs));

    let applied = delete(&doc, unrelated);
    assert_eq!(
        applied.maintenance,
        Vec::new(),
        "the deleted node consumed nothing, so it orphaned nothing"
    );
    assert!(
        applied.doc.node(decl).is_some(),
        "and the declaration waiting for its consumer is untouched"
    );
}

/// **Deleting the declaration itself reports the orphan at the
/// consumer's step, and the next step removes its subject.**
///
/// The `Declare` is its consumer's DAG input, so the author who
/// deletes it deletes the union first (`cascade_delete_order`), and
/// THAT step is the same `(document, edit)` pair as deleting the
/// union for any other reason. Maintenance is a function of the
/// document and the edit, so the two cases cannot report differently:
/// the row is reported and then cancelled by the step that follows.
/// A caller who wants a cascade's net effect reads the document it
/// ended at, which is what the last assertion here does.
#[test]
fn cascading_a_declare_away_reports_the_orphan_and_then_removes_it() {
    let doc = ProfileDoc::empty_derived("dm7_orphan_cascade_decl", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let pairs = flush_pairs(&doc, (a, a), (b, b));
    let (doc, union, decl) = declared_union(doc, &[a, b], pairs);

    let order = cascade_delete_order(&doc, decl);
    assert_eq!(
        order,
        vec![union, decl],
        "the consumer goes first: the declare edge is a DAG input"
    );
    let mut doc = doc;
    let mut per_step = Vec::new();
    for id in order {
        let applied = delete(&doc, id);
        per_step.push((id, applied.maintenance));
        doc = applied.doc;
    }
    assert_eq!(
        per_step,
        vec![
            (union, vec![Maintenance::OrphanedDeclare { declare: decl }]),
            (decl, Vec::new()),
        ],
        "the union's step cannot tell this cascade from any other delete of the union"
    );
    assert!(
        doc.node(decl).is_none(),
        "the cascade ended with the declaration gone, which is the state the author asked for"
    );
}

/// **The transient is cancelled by the action's net, not by the
/// door.** `apply` is a function of `(document, edit)` and answers what
/// one delete did, so the cascade's first step reports the declaration
/// orphaned and its second deletes it. The NET over the action is
/// `MaintenanceNet`'s answer — the one spelling of which rows survive,
/// which every caller holding a sequence folds through (the viewer's
/// session; the pre-click count is
/// `work/offer/cascade-delete-shows-the-strand-count.md`) — and it is
/// empty here, so `apply` never needs cascade knowledge.
#[test]
fn the_orphan_transient_is_cancellable_at_the_cascade_door() {
    let doc = ProfileDoc::empty_derived("dm7_orphan_cancellable", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let pairs = flush_pairs(&doc, (a, a), (b, b));
    let (doc, _union, decl) = declared_union(doc, &[a, b], pairs);

    let doomed = cascade_delete_order(&doc, decl);
    let mut walked = doc;
    let mut rows: Vec<Maintenance> = Vec::new();
    let mut net = editor_core::MaintenanceNet::new();
    for id in &doomed {
        let applied = delete(&walked, *id);
        net.push(&applied);
        rows.extend(applied.maintenance);
        walked = applied.doc;
    }
    assert_eq!(rows, vec![Maintenance::OrphanedDeclare { declare: decl }]);
    assert_eq!(
        net.finish(&walked),
        Vec::new(),
        "the cascade's NET maintenance is empty"
    );
}

/// **What the orphaned declaration BECOMES**: the same delete that
/// reports it also registers it as a product ROOT.
///
/// `roots::on_delete` re-roots the deleted node's inputs that its
/// departure turned into sinks and does not ask what kind they are,
/// so `doc.roots()` gains the `Declare`. Pre-existing and not this
/// door's doing, but it is the fact the arm's `Display` sentence is
/// written against — "no node consumes the declaration", not
/// "nothing reads it", because the root set does. Whether a
/// `Declare` may be a root at all is
/// `work/edit/an-orphaned-declare-joins-the-product-root-set.md`.
#[test]
fn the_orphaned_declaration_is_re_rooted_by_the_same_delete() {
    let doc = ProfileDoc::empty_derived("dm7_orphan_roots", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let pairs = flush_pairs(&doc, (a, a), (b, b));
    let (doc, union, decl) = declared_union(doc, &[a, b], pairs);
    assert_eq!(doc.roots(), [union], "the union is the document's product");

    let applied = delete(&doc, union);
    assert_eq!(
        applied.maintenance,
        vec![Maintenance::OrphanedDeclare { declare: decl }]
    );
    assert!(
        applied.doc.roots().contains(&decl),
        "the declaration the delete reported as unconsumed is now a product root"
    );
}

/// **The orphan row follows the strands of the same delete** — the
/// third boundary of `Applied::maintenance`'s order contract.
///
/// One delete produces both kinds: the union mints the face name a
/// surviving fillet carries (a payload name, not an edge, so the
/// delete is legal and strands it) AND holds the declaration's last
/// `declare` edge. Written out as one vector, so a walk that put the
/// orphans ahead of the strands reds here rather than somewhere a
/// consumer finds it.
#[test]
fn an_orphaned_declare_follows_the_strands_of_the_same_delete() {
    let doc = ProfileDoc::empty_derived("dm7_orphan_order", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let pairs = flush_pairs(&doc, (a, a), (b, b));
    let (doc, union, decl) = declared_union(doc, &[a, b], pairs);
    let (doc, elsewhere) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    // The fillet's DAG input is `elsewhere`; what it NAMES is a face
    // of the union, which is a payload name and not an edge — so the
    // fillet survives the union's delete and carries a dead name.
    let carried = fname(union, wall(&doc, union, 0));
    let (doc, fillet) = insert(
        doc,
        Node::Fillet {
            target: elsewhere,
            radius: len(0.1),
            selection: vec![carried.clone()],
        },
    );

    let applied = delete(&doc, union);
    assert_eq!(
        applied.maintenance,
        vec![
            Maintenance::Strand {
                node: fillet,
                name: carried,
            },
            Maintenance::OrphanedDeclare { declare: decl },
        ],
        "the strands of a delete come before the declarations it left inert"
    );
}
