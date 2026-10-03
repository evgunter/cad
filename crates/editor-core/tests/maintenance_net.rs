//! **`MaintenanceNet` — an action's maintenance, net of what the
//! action itself made moot.**
//!
//! `Applied::maintenance` answers what ONE edit did. An action of
//! several edits can report a row an edit later in the same action
//! takes back, and the net is what is true of the document the action
//! ends at. These rows drive real edits through `Recording`, the one
//! recorder every caller that commits an action of several edits
//! applies them through.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::{block, declared_union, flush_pairs};
use crate::fixture::{ang, fname, insert, wall};
use editor_core::{
    Applied, Attr, AttrKind, DocEdit, EditError, Maintenance, Node, ProfileDoc, ProfileProgram,
    RecipeNodeId, Recorded, Recording, RefusingReach, Rgba8, StableName, apply,
};
use geom_core::Tol;

fn applied(
    doc: &editor_core::ProfileDoc,
    edit: DocEdit<ProfileProgram>,
) -> Applied<ProfileProgram> {
    apply(doc, &edit, Tol::witness(), &RefusingReach).expect("the edit lands")
}

/// The net of `edits` recorded in order from `doc`, and the document
/// they end at; each edit's own rows beside it, for the premises.
fn net_of(
    doc: &ProfileDoc,
    edits: Vec<DocEdit<ProfileProgram>>,
) -> (Vec<Maintenance>, Vec<Vec<Maintenance>>, ProfileDoc) {
    let mut action = Recording::start(doc, Tol::witness(), &RefusingReach);
    let mut each = Vec::new();
    for edit in edits {
        each.push(applied(action.doc(), edit.clone()).maintenance);
        action.apply(edit).expect("the edit lands");
    }
    let Recorded {
        doc, maintenance, ..
    } = action.finish();
    (maintenance, each, doc)
}

/// A derived frame on `at` carrying `face`.
fn frame_on(doc: ProfileDoc, at: RecipeNodeId, face: StableName) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Datum(editor_core::Datum::FaceFrame {
            at,
            face,
            spin: ang(0.0),
        }),
    )
}

/// **A strand survives only while its carrier still holds the stranded
/// name.** Deleting a block strands the frame's name on it; a `Rebind`
/// in the same action repairs it onto the kept block, so the action
/// strands nothing. The strand alone, without the repair, survives.
#[test]
fn a_strand_a_later_rebind_repairs_is_not_reported() {
    let doc = ProfileDoc::empty_derived("net-strand-rebind", Tol::witness());
    let (doc, kept) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let named = fname(victim, wall(&doc, victim, 0));
    let (doc, carrier) = frame_on(doc, kept, named.clone());

    let (alone, _, _) = net_of(&doc, vec![DocEdit::DeleteNode { id: victim }]);
    assert_eq!(
        alone,
        vec![Maintenance::Strand {
            node: doc.spoken(carrier),
            name: doc.spoken_name(&named),
        }]
    );
    let (repaired, each, _) = net_of(
        &doc,
        vec![
            DocEdit::DeleteNode { id: victim },
            DocEdit::Rebind {
                from: named,
                to: fname(kept, wall(&doc, kept, 0)),
            },
        ],
    );
    assert_eq!(each[0], alone, "the premise: the delete strands");
    assert_eq!(
        repaired,
        Vec::new(),
        "the carrier holds a live name at the end"
    );
}

/// **A strand whose carrier the action deleted is not reported.**
#[test]
fn a_strand_on_a_carrier_the_action_deleted_is_not_reported() {
    let doc = ProfileDoc::empty_derived("net-strand-carrier-gone", Tol::witness());
    let (doc, kept) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let named = fname(victim, wall(&doc, victim, 0));
    let (doc, carrier) = frame_on(doc, kept, named);
    let (net, each, _) = net_of(
        &doc,
        vec![
            DocEdit::DeleteNode { id: victim },
            DocEdit::DeleteNode { id: carrier },
        ],
    );
    assert_eq!(each[0].len(), 1, "the premise: the first delete strands");
    assert_eq!(net, Vec::new());
}

/// **An appearance strand survives only while the store holds its
/// key.** Deleting the painted block strands the paint; clearing it in
/// the same action leaves nothing stranded.
#[test]
fn an_appearance_strand_a_later_clear_removes_is_not_reported() {
    let doc = ProfileDoc::empty_derived("net-appearance-clear", Tol::witness());
    let (doc, _) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let painted = fname(victim, wall(&doc, victim, 0));
    let doc = applied(
        &doc,
        DocEdit::SetAppearance {
            name: painted.clone(),
            attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
        },
    )
    .doc;
    let (alone, _, _) = net_of(&doc, vec![DocEdit::DeleteNode { id: victim }]);
    assert_eq!(
        alone,
        vec![Maintenance::StrandedAppearance {
            name: doc.spoken_name(&painted)
        }]
    );
    let (cleared, _, _) = net_of(
        &doc,
        vec![
            DocEdit::DeleteNode { id: victim },
            DocEdit::ClearAppearance {
                name: painted,
                kind: AttrKind::Color,
            },
        ],
    );
    assert_eq!(cleared, Vec::new());
}

/// **An orphan survives only while nothing consumes the declaration.**
/// Deleting the union orphans its declaration; a new union over it in
/// the same action consumes it again, so nothing is orphaned. And the
/// cascade — the union, then the declaration itself — leaves no
/// declaration to be orphaned.
#[test]
fn an_orphan_a_later_edit_consumes_or_deletes_is_not_reported() {
    let doc = ProfileDoc::empty_derived("net-orphan", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let pairs = flush_pairs(&doc, (a, a), (b, b));
    let (doc, union, decl) = declared_union(doc, &[a, b], pairs);

    let (alone, _, _) = net_of(&doc, vec![DocEdit::DeleteNode { id: union }]);
    assert_eq!(
        alone,
        vec![Maintenance::OrphanedDeclare {
            declare: doc.spoken(decl)
        }]
    );
    let (reconsumed, _, _) = net_of(
        &doc,
        vec![
            DocEdit::DeleteNode { id: union },
            DocEdit::InsertNode {
                node: Box::new(Node::Union {
                    members: vec![a, b],
                    declare: Some(decl),
                }),
            },
        ],
    );
    assert_eq!(
        reconsumed,
        Vec::new(),
        "the declaration has a consumer again"
    );
    let (cascaded, _, _) = net_of(
        &doc,
        vec![
            DocEdit::DeleteNode { id: union },
            DocEdit::DeleteNode { id: decl },
        ],
    );
    assert_eq!(cascaded, Vec::new(), "the declaration went with the action");
}

/// **What a recorded action answers beside its net**: the edits in the
/// order they applied, what each minted — the typed insert's id among
/// them — and the document the last one produced, which is the one
/// `apply` threaded edit by edit produces. A refused edit records
/// nothing and the action goes on from where it stood; an action that
/// records nothing answers its start.
#[test]
fn a_recording_answers_its_edits_ids_and_document_in_order() {
    let doc = ProfileDoc::empty_derived("net-record", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let pairs = flush_pairs(&doc, (a, a), (b, b));
    let (doc, union, decl) = declared_union(doc, &[a, b], pairs);
    let reunion = Node::Union {
        members: vec![a, b],
        declare: Some(decl),
    };

    let mut action = Recording::start(&doc, Tol::witness(), &RefusingReach);
    let deleted = action.apply(DocEdit::DeleteNode { id: union });
    assert_eq!(deleted, Ok(None), "a delete mints nothing");
    let before_refusal = action.doc().clone();
    let refused = action.apply(DocEdit::DeleteNode { id: union });
    assert!(
        matches!(refused, Err(EditError::UnknownNode { .. })),
        "the node is gone: {refused:?}"
    );
    assert!(
        action.doc().bit_eq(&before_refusal),
        "a refused edit leaves the action where it stood"
    );
    let inserted = action.insert(reunion.clone()).expect("the union lands");
    assert_eq!(action.minted(), &[None, Some(inserted)]);
    let recorded = action.finish();

    let delete = DocEdit::DeleteNode { id: union };
    let insert = DocEdit::InsertNode {
        node: Box::new(reunion),
    };
    let first = applied(&doc, delete.clone());
    let second = applied(&first.doc, insert.clone());
    assert_eq!(
        second.record.minted,
        Some(inserted),
        "the typed insert answers the id the door mints"
    );
    assert_eq!(
        recorded,
        Recorded {
            doc: second.doc,
            edits: vec![delete, insert],
            maintenance: Vec::new(),
            minted: vec![None, Some(inserted)],
        },
        "the edits as applied, the orphan the re-union consumed netted out"
    );
    assert_eq!(
        first.maintenance,
        vec![Maintenance::OrphanedDeclare {
            declare: doc.spoken(decl)
        }],
        "the premise: the delete alone orphans the declaration"
    );

    let idle = Recording::start(&doc, Tol::witness(), &RefusingReach).finish();
    assert!(idle.doc.bit_eq(&doc), "no edit, the start");
    assert_eq!(
        (idle.edits, idle.maintenance, idle.minted),
        (Vec::new(), Vec::new(), Vec::new())
    );
}
