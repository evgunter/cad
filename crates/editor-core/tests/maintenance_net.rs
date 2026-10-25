//! **`MaintenanceNet` — an action's maintenance, net of what the
//! action itself made moot.**
//!
//! `Applied::maintenance` answers what ONE edit did. An action of
//! several edits can report a row an edit later in the same action
//! takes back, and the net is what is true of the document the action
//! ends at. These rows drive real edits through `Recording`, the
//! kernel's recorder for an action of several edits.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::block;
use crate::fixture::{fname, insert, len, wall};
use editor_core::{
    Applied, Attr, AttrKind, DocEdit, EditError, Maintenance, Node, ProfileDoc, ProfileProgram,
    RecipeNodeId, Recorded, Recording, Rgba8, StableName, apply,
};
use geom_core::Tol;

fn applied(
    doc: &editor_core::ProfileDoc,
    edit: DocEdit<ProfileProgram>,
) -> Applied<ProfileProgram> {
    apply(doc, &edit, Tol::witness()).expect("the edit lands")
}

/// The net of `edits` recorded in order from `doc`, and the document
/// they end at; each edit's own rows beside it, for the premises.
fn net_of(
    doc: &ProfileDoc,
    edits: Vec<DocEdit<ProfileProgram>>,
) -> (Vec<Maintenance>, Vec<Vec<Maintenance>>, ProfileDoc) {
    let mut action = Recording::start(doc, Tol::witness());
    let mut each = Vec::new();
    for edit in edits {
        each.push(crate::fixture::without_anonymous(
            &applied(action.doc(), edit.clone()).maintenance,
        ));
        action.apply(edit).expect("the edit lands");
    }
    let Recorded {
        doc, maintenance, ..
    } = action.finish().expect("every edit landed");
    (crate::fixture::without_anonymous(&maintenance), each, doc)
}

/// A shell of `at` opening `face`: a node carrying a selection of the
/// name.
fn shell_on(doc: ProfileDoc, at: RecipeNodeId, face: StableName) -> (ProfileDoc, RecipeNodeId) {
    insert(doc, Node::shell(at, len(0.0625), vec![face]))
}

/// The selection a shell reads.
fn face_of(doc: &ProfileDoc, shell: RecipeNodeId) -> editor_core::VarId {
    match doc.node(shell) {
        Some(Node::Shell { open, .. }) => *open,
        other => panic!("a shell, got {other:?}"),
    }
}

/// **A strand survives only while its carrier still holds the stranded
/// name.** Deleting a block strands the shell's name on it; a `Rebind`
/// in the same action repairs it onto the kept block, so the action
/// strands nothing. The strand alone, without the repair, survives.
#[test]
fn a_strand_a_later_rebind_repairs_is_not_reported() {
    let doc = ProfileDoc::empty_derived("net-strand-rebind", Tol::witness());
    let (doc, kept) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 1.0);
    let named = fname(victim, wall(&doc, victim, 0));
    let (doc, carrier) = shell_on(doc, kept, named.clone());

    let (alone, _, _) = net_of(&doc, vec![DocEdit::DeleteNode { id: victim }]);
    assert_eq!(
        alone,
        vec![Maintenance::StrandedSelection {
            var: doc.spoken_var(face_of(&doc, carrier)),
            readers: vec![doc.spoken(carrier)],
            name: doc.spoken_name(&named),
            took: editor_core::Took::Node
        }]
    );
    let (repaired, each, _) = net_of(
        &doc,
        vec![
            DocEdit::DeleteNode { id: victim },
            DocEdit::Rebind {
                body: doc.output(kept, 0),
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
    let (doc, kept) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 1.0);
    let named = fname(victim, wall(&doc, victim, 0));
    let (doc, carrier) = shell_on(doc, kept, named);
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
    let (doc, _) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, victim) = block(doc, (4.0, 5.0), (0.0, 1.0), 1.0);
    let painted = fname(victim, wall(&doc, victim, 0));
    let doc = applied(
        &doc,
        DocEdit::SetAppearance {
            body: crate::fixture::painted_in(&doc, &painted),
            name: painted.clone(),
            attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
        },
    )
    .doc;
    let (alone, _, _) = net_of(&doc, vec![DocEdit::DeleteNode { id: victim }]);
    assert_eq!(
        alone,
        vec![Maintenance::StrandedAppearance {
            body: doc.spoken_var(crate::fixture::painted_in(&doc, &painted)),
            name: doc.spoken_name(&painted),
            took: editor_core::Took::Node
        }]
    );
    let (cleared, _, _) = net_of(
        &doc,
        vec![
            DocEdit::DeleteNode { id: victim },
            DocEdit::ClearAppearance {
                body: crate::fixture::painted_in(&doc, &painted),
                name: painted,
                kind: AttrKind::Color,
            },
        ],
    );
    assert_eq!(cleared, Vec::new());
}

/// **What a recorded action answers beside its net**: the edits in the
/// order they applied, what each minted — the typed insert's id among
/// them — and the document the last one produced, which is the one
/// `apply` threaded edit by edit produces. A strand an early delete
/// made and a later `Rebind` of the same action repaired is netted
/// out; an action that records nothing answers its start.
#[test]
fn a_recording_answers_its_edits_ids_and_document_in_order() {
    let doc = ProfileDoc::empty_derived("net-record", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 1.0);
    let (doc, c) = block(doc, (4.0, 5.0), (0.0, 1.0), 1.0);
    let named = fname(b, wall(&doc, b, 0));
    let (doc, _) = shell_on(doc, a, named.clone());
    // `c` joins `a`'s space, as an operator's operands share one.
    let (doc, c) = crate::fixture::stood_in(doc, c, a);
    let other = Node::Union {
        members: editor_core::Bodies::Spelled(vec![a.into(), c.into()]),
    };

    let mut action = Recording::start(&doc, Tol::witness());
    assert!(action.is_empty(), "a started action has recorded nothing");
    let deleted = action.apply(DocEdit::DeleteNode { id: b });
    assert_eq!(deleted, Ok(None), "a delete mints nothing");
    assert!(!action.is_empty(), "the delete is recorded");
    let inserted = action.insert(other.clone()).expect("the union lands");
    let repair = DocEdit::Rebind {
        body: doc.output(a, 0),
        from: named,
        to: fname(a, wall(&doc, a, 0)),
    };
    let repaired = action.apply(repair.clone());
    assert_eq!(repaired, Ok(None), "a rebind mints nothing");
    assert_eq!(action.minted(), &[None, Some(inserted), None]);
    let recorded = action.finish().expect("every edit landed");

    let delete = DocEdit::DeleteNode { id: b };
    let insert = DocEdit::InsertNode {
        node: Box::new(other),
        fresh: Vec::new(),
    };
    let first = applied(&doc, delete.clone());
    let second = applied(&first.doc, insert.clone());
    let third = applied(&second.doc, repair.clone());
    assert_eq!(
        second.record.minted,
        Some(inserted),
        "the typed insert answers the id the door mints"
    );
    assert_eq!(
        recorded,
        Recorded {
            doc: third.doc,
            edits: vec![delete, insert, repair],
            // The delete's retired anonymous variables stand: nothing
            // later takes them back.
            maintenance: first
                .maintenance
                .iter()
                .filter(|m| matches!(m, Maintenance::AnonymousVarRemoved { .. }))
                .cloned()
                .collect(),
            minted: vec![None, Some(inserted), None],
        },
        "the edits as applied, the strand the rebind repaired netted out"
    );
    assert!(
        first
            .maintenance
            .iter()
            .any(|row| matches!(row, Maintenance::StrandedSelection { .. })),
        "the premise: the delete alone strands the shell's name: {:?}",
        crate::fixture::without_anonymous(&first.maintenance)
    );

    let idle = Recording::start(&doc, Tol::witness())
        .finish()
        .expect("nothing was refused");
    assert!(idle.doc.bit_eq(&doc), "no edit, the start");
    assert_eq!(
        (
            idle.edits,
            crate::fixture::without_anonymous(&idle.maintenance),
            idle.minted
        ),
        (Vec::new(), Vec::new(), Vec::new())
    );
}

/// **A refusal ends the action**, whether or not its caller stops at
/// it: the refused edit records nothing, an edit that would land on its
/// own after it is refused with the same refusal and applies nothing,
/// and `finish` answers that refusal instead of the edits around it. A
/// caller that swallows the refusal therefore cannot finish with a
/// partial action: this row reds if a recording goes on past an
/// `apply`'s refusal, or if `finish` answers edits around one. An
/// `insert` that drops its own refusal is held by `lib_placedunion` and
/// the viewer's `an_action_that_swallows_a_refusal_commits_nothing`.
#[test]
fn a_refusal_ends_the_action_even_when_the_caller_goes_on() {
    let doc = ProfileDoc::empty_derived("net-refused", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block(doc, (4.0, 5.0), (0.0, 1.0), 1.0);
    let (doc, c) = block(doc, (8.0, 9.0), (0.0, 1.0), 1.0);
    // `c` joins `b`'s space, as an operator's operands share one.
    let (doc, c) = crate::fixture::stood_in(doc, c, b);
    let delete_b = DocEdit::DeleteNode { id: b };
    let union = Node::Union {
        members: editor_core::Bodies::Spelled(vec![b.into(), c.into()]),
    };

    let mut action = Recording::start(&doc, Tol::witness());
    action
        .apply(DocEdit::DeleteNode { id: a })
        .expect("the first delete lands");
    let after_first = action.doc().clone();
    for edit in [
        delete_b.clone(),
        DocEdit::InsertNode {
            node: Box::new(union.clone()),
            fresh: Vec::new(),
        },
    ] {
        assert!(
            apply(&after_first, &edit, Tol::witness()).is_ok(),
            "the premise: {edit:?} lands on its own where the refusal left the action"
        );
    }
    let refused = action
        .apply(DocEdit::DeleteNode { id: a })
        .expect_err("the node is gone");
    assert!(
        matches!(refused, EditError::UnknownNode { .. }),
        "the door's own refusal: {refused:?}"
    );
    // The caller swallows the refusal and goes on.
    assert_eq!(
        action.apply(delete_b),
        Err(refused.clone()),
        "an apply after the refusal answers it"
    );
    assert_eq!(
        action.insert(union),
        Err(refused.clone()),
        "an insert after the refusal answers it"
    );
    assert!(
        action.doc().bit_eq(&after_first),
        "nothing after the refusal applied"
    );
    assert_eq!(
        action.minted(),
        &[None],
        "only the edit before the refusal is recorded"
    );
    assert_eq!(
        action.finish(),
        Err(refused),
        "the action finishes as its refusal, not as the edits around it"
    );
}
