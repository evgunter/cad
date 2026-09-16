//! **The name-level edit door binds its document to its evaluation**
//! (DI3, A2a).
//!
//! Why the door checks is `apply_with_names`'s own docs. What is
//! pinned here is that BOTH wrong answers a mispairing produces are
//! reachable, one row each over the [`Twins`] fixture: a name the
//! handed table happens to carry, admitted into a document whose own
//! tables do not carry it; and a name the edited document's own tables
//! DO carry, refused `NameUnresolvedInEvaluation` because the twin's
//! do not. A third row holds the premise — each document against its
//! own evaluation — and a fourth pins that the check is the DOOR's
//! rather than the name loop's, an edit carrying no name being refused
//! just the same.
//!
//! Two rows are review lanes' probes, adopted: the foreign-evaluation
//! shape is R2's DOCM-4 probe
//! `red_apply_with_names_admits_a_foreign_evaluation`
//! (`docm/4-review-r2`), widened here to both directions; the
//! version-survives-pairing row and the `NodePick` measurement row are
//! lane `pair-rv`'s (`review/pair-rv`) and keep their own headers.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, CapEnd, DocEdit, DocumentId, EditError, EvalOptions, Evaluation, Node, ProfileDoc,
    ProfileEdgeRef, RecipeNodeId, RoleSeg, apply_with_names, evaluate,
};
use fixture::{ename, insert, len, on_frame};
use geom_core::Tol;

/// A regular `n`-gon of circumradius 1, centred on the plane origin.
fn ngon(n: u32) -> Vec<(f64, f64)> {
    (0..n)
        .map(|k| {
            let t = f64::from(k) * core::f64::consts::TAU / f64::from(n);
            (t.cos(), t.sin())
        })
        .collect()
}

/// One recipe under one identity: an `n`-gon extruded to a prism.
/// Returns the document and its extrude node.
fn prism(id: &str, n: u32) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(id), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![ngon(n)],
    );
    insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    )
}

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// **The twins, and the one name that tells them apart.**
///
/// Two documents of one recipe shape whose tables differ in exactly
/// one thing: an `n`-gon prism's rim edges are one per outer segment,
/// so `edit` selects the FOURTH, which the square carries and the
/// triangle cannot. Everything else — the recipe's shape, its node
/// ids, the carve-out's `Ok` values — is identical by construction,
/// which is the condition `Evaluation::document` exists for, and the
/// constructor asserts each half of it rather than assuming it.
struct Twins {
    square: ProfileDoc,
    triangle: ProfileDoc,
    ev_square: Evaluation<f64>,
    ev_triangle: Evaluation<f64>,
    /// The name only the square's tables carry.
    fourth: editor_core::StableName,
    /// An `InsertNode` whose payload is that name.
    edit: DocEdit<editor_core::ProfileProgram>,
}

impl Twins {
    fn build() -> Self {
        let (square, sq) = prism("edit-pair-apply-square", 4);
        let (triangle, tri) = prism("edit-pair-apply-triangle", 3);
        assert_ne!(square.id(), triangle.id(), "two documents");
        assert_eq!(sq, tri, "the premise: the two recipes mint the same ids");

        let ev_square = run(&square);
        let ev_triangle = run(&triangle);
        assert!(
            ev_square.value(sq).is_some() && ev_triangle.value(tri).is_some(),
            "the premise: both extrudes evaluate Ok, so the carve-out is \
             satisfied on either evaluation"
        );

        let fourth = ename(
            sq,
            RoleSeg::RimEdge(
                CapEnd::End,
                ProfileEdgeRef {
                    loop_index: 0,
                    segment: 3,
                },
            ),
        );
        let edit = DocEdit::InsertNode {
            node: Node::fillet(sq, len(0.1), vec![fourth.clone()]),
        };
        Self {
            square,
            triangle,
            ev_square,
            ev_triangle,
            fourth,
            edit,
        }
    }
}

/// The pairing refusal, with both ids read off it.
#[track_caller]
fn expect_pairing(
    got: Result<editor_core::Applied<editor_core::ProfileProgram>, EditError>,
    expected: DocumentId,
    found: DocumentId,
    what: &str,
) {
    match got {
        Err(EditError::EvaluationOfAnotherDocument {
            expected: e,
            found: f,
        }) => {
            assert_eq!(e, expected, "{what}: names the document being edited");
            assert_eq!(f, found, "{what}: and the one the evaluation is of");
        }
        Ok(_) => panic!("{what}: the door admitted an evaluation of another document"),
        Err(other) => panic!("{what}: expected the pairing refusal, got {other}"),
    }
}

/// **The premise, stated as a row**: each document against its OWN
/// evaluation, which is what the pairing check must leave untouched.
/// The square's tables carry the fourth rim edge and the triangle's do
/// not, and both answers are the door's behaviour before and after.
#[test]
fn a_document_against_its_own_evaluation_answers_as_it_always_did() {
    let t = Twins::build();
    let tol = Tol::witness();
    assert!(
        apply_with_names(&t.square, &t.edit, &t.ev_square, tol).is_ok(),
        "the square's own tables carry the fourth rim edge"
    );
    assert_eq!(
        apply_with_names(&t.triangle, &t.edit, &t.ev_triangle, tol)
            .expect_err("the triangle has no fourth outer segment"),
        EditError::NameUnresolvedInEvaluation { name: t.fourth },
        "the triangle's own tables do not"
    );
}

/// **A false admission.** The name is absent from the document being
/// edited and present in the twin's tables, so the carve-out fires on
/// the twin's `Ok` value and the lookup hits the twin's row: without
/// the pairing check the edit is recorded, stranding the name in the
/// document it was recorded against.
#[test]
fn a_name_only_the_twin_carries_is_not_admitted() {
    let t = Twins::build();
    expect_pairing(
        apply_with_names(&t.triangle, &t.edit, &t.ev_square, Tol::witness()),
        t.triangle.id(),
        t.square.id(),
        "a name the twin carries and this document does not",
    );
}

/// **A spurious `NameUnresolvedInEvaluation`.** The mirror of the row
/// above: the name IS in the edited document's tables and absent from
/// the twin's, so without the pairing check a legal edit is refused —
/// and refused by a sentence about the name rather than about the
/// evaluation, which is the wrong thing to go and look at.
#[test]
fn a_name_this_document_carries_is_not_refused_for_the_twins_tables() {
    let t = Twins::build();
    expect_pairing(
        apply_with_names(&t.square, &t.edit, &t.ev_triangle, Tol::witness()),
        t.square.id(),
        t.triangle.id(),
        "a name this document carries and the twin does not",
    );
}

/// **The check is the DOOR's, not the name loop's.** An edit carrying
/// no `StableName` at all reaches no lookup, so only a check sited
/// before the payload is read refuses it — while the same edit against
/// its own evaluation still applies.
#[test]
fn an_edit_carrying_no_name_is_refused_on_the_pairing_too() {
    let t = Twins::build();
    let tol = Tol::witness();
    let nameless = DocEdit::SetTolerance { eps: 1e-7 };
    assert!(
        apply_with_names(&t.triangle, &nameless, &t.ev_triangle, tol).is_ok(),
        "the premise: the edit itself is legal"
    );
    expect_pairing(
        apply_with_names(&t.triangle, &nameless, &t.ev_square, tol),
        t.triangle.id(),
        t.square.id(),
        "an edit carrying no name",
    );
}

/// REVIEW PROBE (lane `pair-rv`, claim 2): the pairing is IDENTITY and
/// never a version. `ident::mispaired` is `!=` on `DocumentId` alone,
/// so a document that has moved on — a new version under the same id,
/// the shape a re-save produces — still pairs with an evaluation taken
/// before the move, and the door answers out of its own tables. A pin
/// in the stamp would turn this row red, which is the cost DI3
/// declines to pay.
#[test]
fn the_pairing_is_identity_and_survives_a_new_version_of_the_document() {
    let tol = Tol::witness();
    let (square, sq) = prism("edit-pair-apply-square", 4);
    let ev_square = run(&square);

    // A new version under the SAME identity.
    let moved = editor_core::apply(&square, &DocEdit::SetTolerance { eps: 1e-7 }, tol)
        .expect("the edit is legal")
        .doc;
    assert_eq!(moved.id(), square.id(), "identity survives every edit");

    let fourth = ename(
        sq,
        RoleSeg::RimEdge(
            CapEnd::End,
            ProfileEdgeRef {
                loop_index: 0,
                segment: 3,
            },
        ),
    );
    let edit = DocEdit::InsertNode {
        node: Node::fillet(sq, len(0.1), vec![fourth]),
    };
    assert!(
        apply_with_names(&moved, &edit, &ev_square, tol).is_ok(),
        "a stale-but-own evaluation still pairs: the stamp is the id"
    );
}

/// REVIEW PROBE (lane `pair-rv`, claim 5): a door the PR's sweep
/// pattern could not match and did not go looking for by hand.
/// `NodePick` is built FROM one evaluation and then handed ANOTHER at
/// [`NodePick::patch_names`] / `boundary_names`; the index carries no
/// `DocumentId`, so nothing can run `ident::mispaired` there. Against
/// a twin's evaluation the door answers with the TWIN's names — no
/// refusal, no `Unnamed`, just other geometry's names in patch order.
/// Kept as a documentation row: it asserts what happens today.
#[test]
fn nodepick_patch_names_answers_out_of_a_twins_tables() {
    let tol = Tol::witness();
    let (square, sq) = prism("edit-pair-pick-square", 4);
    let (triangle, _tri) = prism("edit-pair-pick-triangle", 3);
    let ev_square = run(&square);
    let ev_triangle = run(&triangle);

    let pick = editor_core::NodePick::build(&ev_square, sq, 0, 0.1, tol)
        .expect("the square prism tessellates");
    let own = pick.patch_names(&ev_square);
    let foreign = pick.patch_names(&ev_triangle);

    assert_eq!(own.len(), foreign.len(), "same index, same patch order");
    assert!(
        own.iter().all(Result::is_ok),
        "the premise: every patch of the square is named by its own run"
    );
    let named_by_the_twin = foreign.iter().filter(|r| r.is_ok()).count();
    let differs = own
        .iter()
        .zip(foreign.iter())
        .filter(|(a, b)| a != b)
        .count();
    assert_ne!(
        differs, 0,
        "the finding: a foreign evaluation changes the answer, silently"
    );
    println!(
        "PROBE NodePick::patch_names: {} patches, {} named by the twin, {} answers differ",
        own.len(),
        named_by_the_twin,
        differs
    );
}
