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
//! **Beside them, the pick index's own pairing** (A2a's `NodePick`
//! doors): an index built from one evaluation refuses a twin's at
//! `patch_names`, `boundary_names` and `pick_face`, and admits a
//! later evaluation of its own document, which is where DI3's line
//! between identity and version falls.
//!
//! Three rows are review lanes' probes, adopted: the foreign-evaluation
//! shape is R2's DOCM-4 probe
//! `red_apply_with_names_admits_a_foreign_evaluation`
//! (`docm/4-review-r2`), widened here to both directions; the
//! version-survives-pairing row and the `NodePick` row are lane
//! `pair-rv`'s (`review/pair-rv`) and keep their own headers — the
//! second measured the wrong answers the refusal now replaces.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, CapEnd, DocEdit, DocumentId, EditError, EvalOptions, Evaluation, HitTestError,
    Node, ProfileDoc, ProfileEdgeRef, RecipeNodeId, RoleSeg, SlotId, apply_with_names, evaluate,
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

/// **The index's name doors refuse a twin's evaluation** — adopted
/// from lane `pair-rv`'s probe (claim 5), which measured this same
/// pairing when nothing checked it: a square prism's index, handed a
/// triangular prism's evaluation, answered five of its six patches out
/// of the twin's tables and differed from the truth on three, with no
/// `Unnamed` and no refusal.
///
/// A `NodePick` is BUILT from one evaluation and then handed ANOTHER
/// at [`NodePick::patch_names`] / `boundary_names`. The index now
/// carries the building evaluation's `DocumentId`, so those doors run
/// the pairing predicate before a table is read, and the measurement
/// above is what the refusal replaces.
///
/// The refusal is of the CALL: one fact about the arguments, outside
/// the vector, not `n` copies of it inside one. The row asserts that
/// shape as well as the arm — a per-slot spelling would still be
/// `Err` in every slot and would pass a test that only asked "is
/// anything wrong".
#[test]
fn the_name_doors_refuse_a_twins_evaluation() {
    let tol = Tol::witness();
    let (square, sq) = prism("edit-pair-pick-square", 4);
    let (triangle, _tri) = prism("edit-pair-pick-triangle", 3);
    let ev_square = run(&square);
    let ev_triangle = run(&triangle);

    let pick = editor_core::NodePick::build(&ev_square, sq, 0, 0.1, tol)
        .expect("the square prism tessellates");

    // The premise the probe measured: the twin's tables DO answer
    // these lookups, so the refusal is the only thing standing
    // between a consumer and other geometry's names.
    let own = pick
        .patch_names(&ev_square)
        .expect("the index's own evaluation pairs");
    assert!(
        own.iter().all(Result::is_ok),
        "the premise: every patch of the square is named by its own run"
    );
    assert!(
        own.len() > 1,
        "the premise: more than one patch, so a per-slot refusal would \
         be distinguishable from a refusal of the call"
    );

    let expected = HitTestError::EvaluationOfAnotherDocument {
        expected: square.id(),
        found: triangle.id(),
    };
    assert_eq!(
        pick.patch_names(&ev_triangle),
        Err(expected),
        "the finding: a foreign evaluation used to answer out of the \
         twin's tables, in patch order"
    );
    assert_eq!(
        pick.boundary_names(&ev_triangle),
        Err(expected),
        "the edge twin refuses the same way"
    );

    // The pick's OWN evaluation still answers, so the refusal is the
    // pairing and not a door that stopped working.
    assert!(
        pick.boundary_names(&ev_square).is_ok(),
        "the index's own evaluation still pairs at the edge door"
    );
}

/// **`pick_face` refuses a target of another document**, before it
/// reads a triangle or a node's standing.
///
/// The door's own targets carry the stamp ([`NodePick::target`]), and
/// the standing loop would not have caught this: the twin mints the
/// same node ids, so every target's node has an `Ok` value in the
/// twin's evaluation and the ray would have resolved to a name out of
/// the twin's table.
#[test]
fn pick_face_refuses_a_target_of_another_document() {
    let tol = Tol::witness();
    let (square, sq) = prism("edit-pair-face-square", 4);
    let (triangle, tri) = prism("edit-pair-face-triangle", 3);
    let ev_square = run(&square);
    let ev_triangle = run(&triangle);
    assert!(
        ev_triangle.value(tri).is_some(),
        "the premise: the twin has an Ok value for the SAME node id, so \
         the standing loop admits the target"
    );

    let pick = editor_core::NodePick::build(&ev_square, sq, 0, 0.1, tol)
        .expect("the square prism tessellates");
    let targets = [pick.target()];
    // Straight down the prism's axis, through the end cap.
    let ray = editor_core::Ray {
        origin: geom_core::Point3::new(0.0, 0.0, 5.0),
        dir: geom_core::Vec3::new(0.0, 0.0, -1.0),
    };
    assert!(
        editor_core::pick_face(&ev_square, &targets, &ray)
            .expect("the index's own evaluation pairs")
            .is_some(),
        "the premise: the ray hits, so a mispairing would have answered \
         a name rather than a miss"
    );
    assert_eq!(
        editor_core::pick_face(&ev_triangle, &targets, &ray)
            .expect_err("a target of another document is refused, not resolved"),
        HitTestError::EvaluationOfAnotherDocument {
            expected: square.id(),
            found: triangle.id(),
        },
        "the refusal is the pairing arm, in the ray door's own vocabulary"
    );
}

/// **A LATER evaluation of the SAME document is admitted** — the
/// boundary DI3 draws, pinned rather than implied.
///
/// A pairing is about IDENTITY. The prism's own extrusion distance is
/// EDITED between the two runs, so the later run recomputes and
/// re-tessellates the very node the index was built for and the
/// index's mesh is a picture behind — and the doors still answer.
/// What may be reused across such a run is the content keys' business
/// (`PickMemo`), not the pairing's, and a door that refused here would
/// be stamping a version half DI3 deliberately does not stamp.
#[test]
fn a_later_evaluation_of_the_same_document_is_admitted() {
    let tol = Tol::witness();
    let (doc, ext) = prism("edit-pair-pick-later", 4);
    let before = run(&doc);
    let pick = editor_core::NodePick::build(&before, ext, 0, 0.1, tol)
        .expect("the prism tessellates");
    let patches_before = pick
        .patch_names(&before)
        .expect("the building evaluation pairs")
        .len();

    // The node ITSELF is edited, so the later run recomputes and
    // re-tessellates it: the index's mesh is a picture behind the
    // evaluation it is about to be handed.
    let edited = doc
        .apply(
            &DocEdit::SetParam {
                node: ext,
                slot: SlotId::Distance,
                expr: len(2.0),
            },
            tol,
        )
        .expect("a length goes into the extrusion distance")
        .doc;
    assert_eq!(edited.id(), doc.id(), "an edit never forks identity (DI4)");
    let after = run(&edited);
    assert_ne!(
        before.value(ext).expect("the prism evaluated").content_key,
        after.value(ext).expect("the prism evaluated").content_key,
        "the premise: the node's own value moved, so this is not a \
         re-run that happens to be identical"
    );

    let names = pick
        .patch_names(&after)
        .expect("a later evaluation of the same document is admitted");
    assert_eq!(
        names.len(),
        patches_before,
        "the index is the one that was built, so its patch order is \
         unchanged — what the later run may have re-tessellated is the \
         content keys' business, not the pairing's"
    );
    assert!(
        pick.boundary_names(&after).is_ok(),
        "the edge door draws the same line"
    );
    assert!(
        editor_core::pick_face(&after, &[pick.target()], &editor_core::Ray {
            origin: geom_core::Point3::new(0.0, 0.0, 5.0),
            dir: geom_core::Vec3::new(0.0, 0.0, -1.0),
        })
        .is_ok(),
        "and so does the ray door"
    );
}
