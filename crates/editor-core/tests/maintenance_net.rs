//! **`MaintenanceNet` — an action's maintenance, net of what the
//! action itself made moot.**
//!
//! `Applied::maintenance` answers what ONE edit did. An action of
//! several edits can report a row an edit later in the same action
//! takes back, and the net is what is true of the document the action
//! ends at. These rows drive real edits through `Recording`, the
//! kernel's recorder for an action of several edits.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::block;
use crate::fixture::{ang, flush_pairs, fname, insert, union_over, wall};
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

/// **A strand on a declared pair survives only while the declaring
/// node still holds the pair.** With `b` dropped from the union's
/// members, deleting it strands the names the union's declared pairs
/// carry; a `SetDeclare` clearing the list in the same action leaves
/// nothing stranded.
#[test]
fn a_declared_strand_a_later_set_declare_clears_is_not_reported() {
    let doc = ProfileDoc::empty_derived("net-declare-cleared", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let pairs = editor_core::declare_continuation(flush_pairs(&doc, (a, a), (b, b)));
    let (doc, union) = union_over(doc, &[a, b, c], pairs);
    let doc = applied(
        &doc,
        DocEdit::SetMembers {
            node: union,
            members: vec![a, c],
        },
    )
    .doc;

    let (alone, _, _) = net_of(&doc, vec![DocEdit::DeleteNode { id: b }]);
    assert_eq!(
        alone.len(),
        4,
        "the premise: one strand per declared name minted in `b`: {alone:?}"
    );
    let (cleared, _, _) = net_of(
        &doc,
        vec![
            DocEdit::DeleteNode { id: b },
            DocEdit::SetDeclare {
                node: union,
                pairs: Vec::new(),
            },
        ],
    );
    assert_eq!(
        cleared,
        Vec::new(),
        "the union holds no stranded pair at the end"
    );
}

/// **What a recorded action answers beside its net**: the edits in the
/// order they applied, what each minted — the typed insert's id among
/// them — and the document the last one produced, which is the one
/// `apply` threaded edit by edit produces. A refused edit records
/// nothing and the action goes on from where it stood; a strand an
/// early delete made and a later `SetDeclare` of the same action
/// cleared is netted out; an action that records nothing answers its
/// start.
#[test]
fn a_recording_answers_its_edits_ids_and_document_in_order() {
    let doc = ProfileDoc::empty_derived("net-record", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let pairs = editor_core::declare_continuation(flush_pairs(&doc, (a, a), (b, b)));
    let (doc, union) = union_over(doc, &[a, b, c], pairs);
    let doc = applied(
        &doc,
        DocEdit::SetMembers {
            node: union,
            members: vec![a, c],
        },
    )
    .doc;
    let other = Node::Union {
        members: vec![a, c],
        declare: Vec::new(),
    };

    let mut action = Recording::start(&doc, Tol::witness(), &RefusingReach);
    assert!(action.is_empty(), "a started action has recorded nothing");
    let deleted = action.apply(DocEdit::DeleteNode { id: b });
    assert_eq!(deleted, Ok(None), "a delete mints nothing");
    assert!(!action.is_empty(), "the delete is recorded");
    let before_refusal = action.doc().clone();
    let refused = action.apply(DocEdit::DeleteNode { id: b });
    assert!(
        matches!(refused, Err(EditError::UnknownNode { .. })),
        "the node is gone: {refused:?}"
    );
    assert!(
        action.doc().bit_eq(&before_refusal),
        "a refused edit leaves the action where it stood"
    );
    let inserted = action.insert(other.clone()).expect("the union lands");
    let cleared = action.apply(DocEdit::SetDeclare {
        node: union,
        pairs: Vec::new(),
    });
    assert_eq!(cleared, Ok(None), "a set-declare mints nothing");
    assert_eq!(action.minted(), &[None, Some(inserted), None]);
    let recorded = action.finish();

    let delete = DocEdit::DeleteNode { id: b };
    let insert = DocEdit::InsertNode {
        node: Box::new(other),
    };
    let clear = DocEdit::SetDeclare {
        node: union,
        pairs: Vec::new(),
    };
    let first = applied(&doc, delete.clone());
    let second = applied(&first.doc, insert.clone());
    let third = applied(&second.doc, clear.clone());
    assert_eq!(
        second.record.minted,
        Some(inserted),
        "the typed insert answers the id the door mints"
    );
    assert_eq!(
        recorded,
        Recorded {
            doc: third.doc,
            edits: vec![delete, insert, clear],
            maintenance: Vec::new(),
            minted: vec![None, Some(inserted), None],
        },
        "the edits as applied, the strands the cleared declaration held netted out"
    );
    assert!(
        first
            .maintenance
            .iter()
            .any(|row| matches!(row, Maintenance::Strand { .. })),
        "the premise: the delete alone strands the declared names: {:?}",
        first.maintenance
    );

    let idle = Recording::start(&doc, Tol::witness(), &RefusingReach).finish();
    assert!(idle.doc.bit_eq(&doc), "no edit, the start");
    assert_eq!(
        (idle.edits, idle.maintenance, idle.minted),
        (Vec::new(), Vec::new(), Vec::new())
    );
}
