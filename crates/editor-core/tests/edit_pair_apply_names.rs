//! **The name-level edit door binds its document to its evaluation**
//! (DI3, A2a).
//!
//! `apply_with_names` validates an edit's recorded names against the
//! tables of the evaluation it is handed, under a carve-out keyed on
//! `eval.value(name.node)`. Node ids are minted by a per-document
//! counter, so two documents built from one recipe carry the same ids
//! for the same nodes: a foreign evaluation satisfies the carve-out on
//! every name and the door then answers out of the wrong table. Both
//! wrong answers are reachable and both are pinned here — a name the
//! handed table happens to carry is admitted into a document whose own
//! tables do not carry it, and a name the document's own tables DO
//! carry is refused `NameUnresolvedInEvaluation` because the twin's
//! tables do not.
//!
//! The twins differ in ONE way that the tables can see: an `n`-gon's
//! rim edges are one per outer segment, so the square prism has a
//! fourth and the triangular prism does not. Everything else — the
//! recipe's shape, its node ids, the carve-out's `Ok` values — is
//! identical by construction, which is the condition the field exists
//! for.
//!
//! The door's check therefore runs before any name is read, and a
//! name-free edit is refused too: the pairing is a property of the
//! two arguments, not of what the edit happens to carry.
//!
//! (The foreign-evaluation row is review lane R2's DOCM-4 probe
//! `red_apply_with_names_admits_a_foreign_evaluation`, on branch
//! `docm/4-review-r2`, adopted and widened to both directions.)

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

#[test]
fn apply_with_names_refuses_an_evaluation_of_another_document() {
    let tol = Tol::witness();
    let (square, sq) = prism("edit-pair-apply-square", 4);
    let (triangle, tri) = prism("edit-pair-apply-triangle", 3);

    // The construction: different documents, identical node ids.
    assert_ne!(square.id(), triangle.id());
    assert_eq!(sq, tri, "the premise: the two recipes mint the same ids");

    let ev_square = run(&square);
    let ev_triangle = run(&triangle);
    assert!(
        ev_square.value(sq).is_some() && ev_triangle.value(tri).is_some(),
        "the premise: both extrudes evaluate Ok, so the carve-out is \
         satisfied on either evaluation"
    );

    // A fillet selecting the rim edge of the FOURTH outer segment: the
    // square prism's tables carry it, the triangular prism's cannot.
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

    // The premise, both halves, each door against its OWN evaluation —
    // and the answers the pairing check must leave untouched.
    assert!(
        apply_with_names(&square, &edit, &ev_square, tol).is_ok(),
        "the premise: the square's own tables carry the fourth rim edge"
    );
    assert_eq!(
        apply_with_names(&triangle, &edit, &ev_triangle, tol).unwrap_err(),
        EditError::NameUnresolvedInEvaluation {
            name: fourth.clone()
        },
        "the premise: the triangle's own tables do not"
    );

    // A false admission: the name is absent from the document being
    // edited and present in the twin's tables.
    expect_pairing(
        apply_with_names(&triangle, &edit, &ev_square, tol),
        triangle.id(),
        square.id(),
        "a name the twin carries and this document does not",
    );

    // A spurious `NameUnresolvedInEvaluation`: the name is present in
    // the document being edited and absent from the twin's tables.
    expect_pairing(
        apply_with_names(&square, &edit, &ev_triangle, tol),
        square.id(),
        triangle.id(),
        "a name this document carries and the twin does not",
    );

    // The check is the DOOR's, not the name loop's: an edit carrying
    // no `StableName` at all is refused on the pairing just the same,
    // while the same edit against its own evaluation still applies.
    let nameless = DocEdit::SetTolerance { eps: 1e-7 };
    assert!(
        apply_with_names(&triangle, &nameless, &ev_triangle, tol).is_ok(),
        "the premise: the edit itself is legal"
    );
    expect_pairing(
        apply_with_names(&triangle, &nameless, &ev_square, tol),
        triangle.id(),
        square.id(),
        "an edit carrying no name",
    );
}
