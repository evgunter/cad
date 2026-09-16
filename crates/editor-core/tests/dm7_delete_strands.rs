//! **DM7 — a stranded payload name is REPORTED at the delete, never
//! refused.**
//!
//! A payload name (`Node::payload_names` — a `Declare`'s pairs, a
//! blend's selection, a shell's rim list, a derived frame's face, a
//! measure's references, a mate's heads) is not a DAG edge: the insert
//! door checks its minting node is live and no other door does, so a
//! later `DeleteNode` strands it. The delete stays legal — a full edge
//! would deadlock the declared union, whose `Declare` names the very
//! union that consumes it — and what the door owes instead is the
//! report: one `Maintenance::Strand` per surviving `(node, name)`
//! whose minting node the edit removed.
//!
//! These rows are what goes red when a strand goes unreported.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::{block, declared_union, flush_pairs};
use crate::fixture;
use crate::fixture::resolver::PartStore;
use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, Datum, DocEdit, DocumentId, EditError, EntityKind,
    Maintenance, MateFrame, MatePrimitive, MeasureExpr, MeasurePrimitive, Node, ProfileDoc,
    RecipeNodeId, RoleSeg, SitedRef, StableName, apply, cascade_delete_order,
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
            Maintenance::Cluster(_) => None,
        })
        .collect()
}

/// Delete one node, expecting the door to accept it.
fn delete(doc: &ProfileDoc, id: RecipeNodeId) -> editor_core::Applied<editor_core::ProfileProgram> {
    apply(doc, &DocEdit::DeleteNode { id }, Tol::witness())
        .expect("a payload name is not a DAG edge, so the delete is legal")
}

// ---------------------------------------------------------------------
// The declared union — DOCM-7's R1 probe, as a row.
// ---------------------------------------------------------------------

/// **Deleting a declared union reports every pair of its `Declare`,
/// and deleting the `Declare` still refuses.**
///
/// The two halves are the whole ruling in one document. The `Declare`
/// is the union's INPUT, so deleting it dangles an edge and is refused
/// typed. The union is what the `Declare`'s pairs NAME, which is not an
/// edge, so that delete is accepted — and every pair it stranded is in
/// the record, at the door, rather than waiting for the next
/// evaluation to answer `NodeGone`.
#[test]
fn deleting_a_declared_union_names_every_pair_of_its_declare() {
    let doc = ProfileDoc::empty_derived("dm7_declared_union", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, decl) = declared_union(doc, &[a, b], |u| flush_pairs(u, (a, a), (b, b)));

    assert!(
        matches!(
            apply(&doc, &DocEdit::DeleteNode { id: decl }, Tol::witness()),
            Err(EditError::DeleteWouldDangle { .. })
        ),
        "the declare edge IS a DAG edge and refuses"
    );

    let Some(Node::Declare { pairs }) = doc.node(decl) else {
        panic!("the declaration is a Declare")
    };
    let expected: Vec<(RecipeNodeId, StableName)> = pairs
        .iter()
        .flat_map(|((x, y), _)| [x.clone(), y.clone()])
        .map(|name| (decl, name))
        .collect();
    assert_eq!(
        expected.len(),
        8,
        "four flush pairs, two names each, all in the union's own space"
    );
    assert!(
        expected.iter().all(|(_, name)| name.node == union),
        "a member-space name carries the union's id, which is what the delete strands"
    );

    let applied = delete(&doc, union);
    assert_eq!(
        strands(&applied.maintenance),
        expected,
        "the accepted delete names every stranded pair, in the payload's own order"
    );
    assert!(
        applied.doc.node(decl).is_some(),
        "the Declare survives its union — the orphan the user cascades or deletes"
    );
}

// ---------------------------------------------------------------------
// Every payload kind that carries a name.
// ---------------------------------------------------------------------

/// **Every payload that carries a name reports its strand** — the row
/// a walk that skips one payload kind goes red on.
///
/// One document, one deleted node, and a carrier of each name-bearing
/// kind whose own DAG input is somewhere else: a fillet and a chamfer
/// selection, a shell's rim list, a derived frame's face, a measure's
/// two references and a `Declare`'s pair. The expectation is written
/// out in full — carrier by carrier, name by name — so dropping a kind
/// from the walk cannot pass by reporting a shorter list.
#[test]
fn every_payload_kind_that_carries_a_name_reports_its_strand() {
    let doc = ProfileDoc::empty_derived("dm7_carriers", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);

    let f0 = fname(victim, wall(0));
    let f1 = fname(victim, wall(1));
    let f2 = fname(victim, wall(2));
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
    let (doc, decl) = insert(doc, Node::declare_rest(vec![(f1.clone(), f2.clone())]));

    let applied = delete(&doc, victim);
    assert_eq!(
        strands(&applied.maintenance),
        vec![
            (fillet, f0.clone()),
            (chamfer, f1.clone()),
            (shell, f2.clone()),
            (derived, f3),
            (measure, f4),
            (measure, f0),
            (decl, f1),
            (decl, f2),
        ],
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
    let (doc, _fillet) = insert(
        doc,
        Node::Fillet {
            target: body,
            radius: len(0.1),
            selection: vec![fname(body, wall(0))],
        },
    );

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
    let (doc, fillet) = insert(
        doc,
        Node::Fillet {
            target: body,
            radius: len(0.1),
            selection: vec![fname(body, wall(0))],
        },
    );

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
    let (doc, fillet) = insert(
        doc,
        Node::Fillet {
            target: body,
            radius: len(0.1),
            selection: vec![fname(body, wall(0))],
        },
    );
    let face = fname(fillet, wall(2));
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
        vec![(fillet, vec![(derived, face)]), (body, Vec::new()),],
        "the strand is reported by the step that deleted the minting node"
    );
    assert!(
        doc.node(derived).is_some(),
        "the survivor is still in the document, carrying a name that now resolves to nothing"
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

    let doc = ProfileDoc::empty(DocumentId::derive("dm7_mate"), Tol::witness());
    let (doc, ia) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, ib) = insert(doc, Node::instantiate_part(doc_ref));
    let head_a = instance_face(ia, part_body);
    let (doc, mate) = insert(
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

    let applied = delete(&doc, ia);
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
    let (doc, union, _decl) = declared_union(doc, &[a, b], |u| flush_pairs(u, (a, a), (b, b)));

    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("the document saves");
    let loaded = editor_core::persist::load(&text, Tol::witness()).expect("and loads");
    assert!(loaded.doc.bit_eq(&doc), "the snapshot round-trips");

    let direct = delete(&doc, union);
    let after_load = delete(&loaded.doc, union);
    assert!(
        !direct.maintenance.is_empty(),
        "the delete strands the declaration's pairs"
    );
    assert_eq!(
        direct.maintenance, after_load.maintenance,
        "the report is derived from the document and the edit, so it survives the boundary \
         by being recomputable rather than by being carried"
    );
}
