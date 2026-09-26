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
//! between identity and version falls. Four rows say what the
//! headline three leave implicit — which refusal wins when standing
//! would also refuse, what the memo's own document half refuses, what
//! the admitted later run answers and costs, and what a RAW
//! `PickTarget` still claims rather than proves.
//!
//! Rows carried from review lanes, adopted with their headers: the
//! foreign-evaluation shape is R2's DOCM-4 probe
//! `red_apply_with_names_admits_a_foreign_evaluation`
//! (`docm/4-review-r2`), widened here to both directions; the
//! version-survives-pairing row and the `NodePick` row are lane
//! `pair-rv`'s (`review/pair-rv`) — the second measured the wrong
//! answers the refusal now replaces; the four rows above are lane
//! `nodepick-rv`'s (`review/nodepick-rv`).

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::fixture;

use std::collections::BTreeSet;

use editor_core::{
    CancelToken, CapEnd, DocEdit, DocumentId, EditError, EvalOptions, Evaluation, HitTestError,
    Node, ProfileDoc, RecipeNodeId, RoleSeg, SlotId, apply_with_names, evaluate,
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
    /// The extrude node, ONE id in both documents (asserted below).
    node: RecipeNodeId,
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
            RoleSeg::RimEdge(CapEnd::End, crate::fixture::piece(&square, sq, 0, 3)),
        );
        let edit = DocEdit::InsertNode {
            node: Node::fillet(sq, len(0.1), vec![fourth.clone()]),
        };
        Self {
            square,
            triangle,
            node: sq,
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
        apply_with_names(
            &t.square,
            &t.edit,
            &t.ev_square,
            tol,
            &editor_core::RefusingReach
        )
        .is_ok(),
        "the square's own tables carry the fourth rim edge"
    );
    assert_eq!(
        apply_with_names(
            &t.triangle,
            &t.edit,
            &t.ev_triangle,
            tol,
            &editor_core::RefusingReach
        )
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
        apply_with_names(
            &t.triangle,
            &t.edit,
            &t.ev_square,
            Tol::witness(),
            &editor_core::RefusingReach,
        ),
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
        apply_with_names(
            &t.square,
            &t.edit,
            &t.ev_triangle,
            Tol::witness(),
            &editor_core::RefusingReach,
        ),
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
        apply_with_names(
            &t.triangle,
            &nameless,
            &t.ev_triangle,
            tol,
            &editor_core::RefusingReach
        )
        .is_ok(),
        "the premise: the edit itself is legal"
    );
    expect_pairing(
        apply_with_names(
            &t.triangle,
            &nameless,
            &t.ev_square,
            tol,
            &editor_core::RefusingReach,
        ),
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
    let moved = editor_core::apply(
        &square,
        &DocEdit::SetTolerance { eps: 1e-7 },
        tol,
        &editor_core::RefusingReach,
    )
    .expect("the edit is legal")
    .doc;
    assert_eq!(moved.id(), square.id(), "identity survives every edit");

    let fourth = ename(
        sq,
        RoleSeg::RimEdge(CapEnd::End, crate::fixture::piece(&square, sq, 0, 3)),
    );
    let edit = DocEdit::InsertNode {
        node: Node::fillet(sq, len(0.1), vec![fourth]),
    };
    assert!(
        apply_with_names(&moved, &edit, &ev_square, tol, &editor_core::RefusingReach).is_ok(),
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
/// The premise is asserted where it belongs — on the TWIN. A refusal
/// is only interesting when the wrong answer was available, so the row
/// hands the twin's evaluation an index OF ITS OWN and shows it
/// answers every patch, out of a name set that is not the square's:
/// the tables the refused call would have read are there, populated,
/// and about other geometry.
///
/// The refusal is of the CALL: one fact about the arguments, outside
/// the vector, not `n` copies of it inside one. The row asserts that
/// shape as well as the arm — a per-slot spelling would still be
/// `Err` in every slot and would pass a test that only asked "is
/// anything wrong".
#[test]
fn the_name_doors_refuse_a_twins_evaluation() {
    let t = Twins::build();
    let tol = Tol::witness();
    let pick = editor_core::NodePick::build(&t.ev_square, t.node, 0, 0.1, tol)
        .expect("the square prism tessellates");

    let own = pick
        .patch_names(&t.ev_square)
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

    // The premise on the TWIN: its tables answer these lookups. The
    // same node id, the same body, an index of its own — every patch
    // named, and the set is NOT the square's, so an answer out of it
    // would have been wrong rather than harmlessly identical.
    let twin_pick = editor_core::NodePick::build(&t.ev_triangle, t.node, 0, 0.1, tol)
        .expect("the triangular prism tessellates under the SAME node id");
    let twin_own = twin_pick
        .patch_names(&t.ev_triangle)
        .expect("the twin's index pairs with the twin's run");
    let twin_names: BTreeSet<_> = twin_own
        .iter()
        .map(|n| {
            n.as_ref()
                .expect("the twin names every patch of its own")
                .clone()
        })
        .collect();
    assert_eq!(
        twin_names.len(),
        twin_own.len(),
        "the premise: the twin's tables answer a full set, one name per patch"
    );
    let square_names: BTreeSet<_> = own
        .iter()
        .map(|n| n.as_ref().expect("named").clone())
        .collect();
    assert_ne!(
        twin_names, square_names,
        "the premise: the twin's answers are OTHER geometry's names — a \
         refusal here buys something"
    );

    let expected = HitTestError::EvaluationOfAnotherDocument {
        expected: t.square.id(),
        found: t.triangle.id(),
    };
    assert_eq!(
        pick.patch_names(&t.ev_triangle),
        Err(expected.clone()),
        "the finding: a foreign evaluation used to answer out of the \
         twin's tables, in patch order"
    );
    assert_eq!(
        pick.boundary_names(&t.ev_triangle),
        Err(expected),
        "the edge twin refuses the same way"
    );

    // The pick's OWN evaluation still answers, so the refusal is the
    // pairing and not a door that stopped working.
    assert!(
        pick.boundary_names(&t.ev_square).is_ok(),
        "the index's own evaluation still pairs at the edge door"
    );
}

/// Straight down the prism's axis, through the end cap.
fn down() -> editor_core::Ray {
    editor_core::Ray {
        origin: geom_core::Point3::new(0.0, 0.0, 5.0),
        dir: geom_core::Vec3::new(0.0, 0.0, -1.0),
    }
}

/// **`pick_face` refuses a target of another document**, before it
/// reads a triangle or a node's standing.
///
/// The door's own targets carry the stamp ([`NodePick::target`]), and
/// the standing loop would not have caught this: [`Twins`] asserts the
/// two documents mint the SAME node ids and that both extrudes have an
/// `Ok` value, so every target's node stands in the twin's evaluation
/// and the ray would have resolved to a name out of the twin's table.
#[test]
fn pick_face_refuses_a_target_of_another_document() {
    let t = Twins::build();
    let tol = Tol::witness();
    let pick = editor_core::NodePick::build(&t.ev_square, t.node, 0, 0.1, tol)
        .expect("the square prism tessellates");
    let targets = [pick.target()];
    assert!(
        editor_core::pick_face(&t.ev_square, &targets, &down())
            .expect("the index's own evaluation pairs")
            .is_some(),
        "the premise: the ray hits, so a mispairing would have answered \
         a name rather than a miss"
    );
    assert_eq!(
        editor_core::pick_face(&t.ev_triangle, &targets, &down())
            .expect_err("a target of another document is refused, not resolved"),
        HitTestError::EvaluationOfAnotherDocument {
            expected: t.square.id(),
            found: t.triangle.id(),
        },
        "the refusal is the pairing arm, in the ray door's own vocabulary"
    );
}

/// **A LATER evaluation of the SAME document is admitted** — the
/// boundary DI3 draws, pinned rather than implied — **and what it
/// answers is asserted, not just that it answers.**
///
/// A pairing is about IDENTITY. The prism's own extrusion distance is
/// EDITED between the two runs, so the later run recomputes and
/// re-tessellates the very node the index was built for and the
/// index's mesh is a picture behind — and the doors still answer.
///
/// **Whether a stale index can answer a WRONG name here, measured.**
/// Not for a parameter edit: a name in this kernel is anchored to the
/// recipe's program (a rim edge is its profile segment, a cap is its
/// end), so the extrude's face names are a function of the recipe's
/// STRUCTURE and not of its parameter values — move the distance and
/// every face keeps its name. So the first half asserts the stronger
/// thing the row left open: the later run answers the SAME names, slot
/// for slot, and "admitted" is not covering a difference. The second
/// half takes the same parameter to a degenerate value, so the node
/// FAILS in the later run, and shows the split the signature exists
/// for: the CALL is still admitted (identity is unchanged) and every
/// SLOT refuses, so a stale index announces itself per patch rather
/// than answering a plausible name.
///
/// What may be reused across such a run is the content keys' business
/// (`PickMemo`), not the pairing's — and
/// [`what_the_admitted_later_evaluation_answers`] shows that a caller
/// going through the memo never holds a stale index at all.
#[test]
fn a_later_evaluation_of_the_same_document_is_admitted() {
    let tol = Tol::witness();
    let (doc, ext) = prism("edit-pair-pick-later", 4);
    let before = run(&doc);
    let pick =
        editor_core::NodePick::build(&before, ext, 0, 0.1, tol).expect("the prism tessellates");
    let names_before = pick
        .patch_names(&before)
        .expect("the building evaluation pairs");

    // The node ITSELF is edited, so the later run recomputes and
    // re-tessellates it: the index's mesh is a picture behind the
    // evaluation it is about to be handed.
    let later = |distance: f64| {
        let edited = doc
            .apply(
                &DocEdit::SetParam {
                    node: ext,
                    slot: SlotId::Distance,
                    expr: len(distance),
                },
                tol,
                &editor_core::RefusingReach,
            )
            .expect("a length goes into the extrusion distance")
            .doc;
        assert_eq!(edited.id(), doc.id(), "an edit never forks identity (DI4)");
        run(&edited)
    };

    let after = later(2.0);
    assert_ne!(
        before.value(ext).expect("the prism evaluated").content_key,
        after.value(ext).expect("the prism evaluated").content_key,
        "the premise: the node's own value moved, so this is not a \
         re-run that happens to be identical"
    );
    let names_after = pick
        .patch_names(&after)
        .expect("a later evaluation of the same document is admitted");
    assert_eq!(
        names_after, names_before,
        "the index is the one that was built, and a canonical name — \
         counted from the loop's authored start — does not move when a \
         parameter does: the later run answers the same name in every slot"
    );
    assert!(
        pick.boundary_names(&after).is_ok(),
        "the edge door draws the same line"
    );
    assert!(
        editor_core::pick_face(&after, &[pick.target()], &down()).is_ok(),
        "and so does the ray door"
    );

    // The loud end of the same admission: the node FAILS in the later
    // run. Identity is unchanged, so the call is admitted; the stale
    // index's patches have no table to invert, so every slot refuses.
    let broken = later(0.0);
    let names_broken = pick
        .patch_names(&broken)
        .expect("identity is unchanged, so the CALL is still admitted");
    assert_eq!(
        names_broken.len(),
        names_before.len(),
        "the index is still the one that was built"
    );
    assert!(
        names_broken
            .iter()
            .all(|n| matches!(n, Err(HitTestError::NodeFailed { node }) if *node == ext)),
        "a stale index over a node that has since failed announces \
         itself in every slot rather than answering a plausible name"
    );
}

// ---------------------------------------------------------------
// Adopted from lane `nodepick-rv`'s review probes (PR #2773). Each
// keeps the header that says what it was asked to settle.
// ---------------------------------------------------------------

/// **The MEMO's document half of its key** (lane `nodepick-rv`,
/// claim 2), which is `entry.pick.document == eval.document` and had
/// no row.
///
/// Two documents of ONE recipe mint the same node ids AND the same
/// content and naming keys, so every other half of the memo's key
/// matches: only the document comparison stands between a second
/// document's build and the first document's `NodePick`. The seam owns
/// one memo across builds (`viewer::evalseam::build_index`), so this is
/// not a hypothetical shape. Mutating the comparison to `true` leaves
/// this row as the one that reds.
#[test]
fn the_memo_refuses_a_prior_of_another_document() {
    let tol = Tol::witness();
    let (a, na) = prism("edit-pair-memo-a", 4);
    let (b, nb) = prism("edit-pair-memo-b", 4);
    assert_ne!(a.id(), b.id(), "two documents");
    assert_eq!(na, nb, "the premise: one recipe, so the same node ids");
    let ev_a = run(&a);
    let ev_b = run(&b);
    assert_eq!(
        ev_a.value(na).expect("a evaluated").content_key,
        ev_b.value(nb).expect("b evaluated").content_key,
        "the premise: identical recipes, so the CONTENT key half of the \
         memo's key matches and only the document half can refuse"
    );
    assert_eq!(
        ev_a.value(na).expect("a evaluated").naming_key,
        ev_b.value(nb).expect("b evaluated").naming_key,
        "the premise: the naming key half matches too"
    );

    let mut memo = editor_core::PickMemo::new();
    let _first = editor_core::NodePick::build_with(&ev_a, na, 0, 0.1, tol, &mut memo)
        .expect("a's prism tessellates");
    memo.end_picture();
    let second = editor_core::NodePick::build_with(&ev_b, nb, 0, 0.1, tol, &mut memo)
        .expect("b's prism tessellates");
    memo.end_picture();

    // The observable: the pick b got is stamped with b, so b's own
    // evaluation pairs with it and a's does not. Served a's entry, it
    // would be the other way round.
    assert!(
        second.patch_names(&ev_b).is_ok(),
        "the pick b got is b's: it pairs with the evaluation it was \
         built for"
    );
    assert_eq!(
        second
            .patch_names(&ev_a)
            .expect_err("a is the other document"),
        HitTestError::EvaluationOfAnotherDocument {
            expected: b.id(),
            found: a.id(),
        },
        "and it is not a's"
    );
}

/// **Which refusal wins in `pick_face` when both would fire** (lane
/// `nodepick-rv`, claim 3).
///
/// A2a says the pairing refuses "before reading anything of the
/// value". The rows above use a twin whose node stands `Ok`, so they
/// cannot tell the two loops apart by their ANSWER — both orders give
/// the pairing arm there only because standing does not refuse. This
/// row hands a foreign evaluation in which the target's node does not
/// exist at all: the pairing arm is the answer iff the pairing loop
/// runs first.
#[test]
fn the_pairing_refusal_wins_over_standing() {
    let tol = Tol::witness();
    let (square, sq) = prism("edit-pair-order-square", 4);
    // A second document that is only a profile: the square's extrude
    // node id has no value in it at all.
    let bare = ProfileDoc::empty(DocumentId::derive("edit-pair-order-bare"), tol);
    let (bare, _p) = on_frame(
        bare,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![ngon(4)],
    );
    let ev_square = run(&square);
    let ev_bare = run(&bare);
    assert!(
        ev_bare.value(sq).is_none(),
        "the premise: the other document has NO value for this node, so \
         the standing loop would refuse it too"
    );

    let pick = editor_core::NodePick::build(&ev_square, sq, 0, 0.1, tol)
        .expect("the square prism tessellates");
    assert_eq!(
        editor_core::pick_face(&ev_bare, &[pick.target()], &down()).expect_err("refused"),
        HitTestError::EvaluationOfAnotherDocument {
            expected: square.id(),
            found: bare.id(),
        },
        "the pairing is read before the node's standing (A2a: before \
         reading anything of the value)"
    );
}

/// **A raw `PickTarget` is a CLAIM in every half** (lane
/// `nodepick-rv`, claim 5), and a minted one cannot be re-stamped.
///
/// The probe this row is adopted from forged the checked half out of
/// an honest target — `PickTarget { document: twin, ..pick.target() }`
/// — and got a confidently wrong name. That move is now a compile
/// error: `PickTarget`'s fields are private (the `compile_fail` row is
/// on the type), and neither it nor `NodePick` hands its `MeshPick`
/// out, so a target minted by `NodePick::target` cannot be taken
/// apart.
///
/// What is left is the raw path, `PickTarget::new`, where the caller
/// supplies a mesh index of its own and DECLARES what it is of. This
/// row measures that the declaration is not checked — in the document
/// half exactly as in the node half.
///
/// **Which is why this row is now the door's whole reachable
/// statement.** `PickTarget::new` and `MeshPick::build` live behind
/// `editor-core`'s `test-support` feature, on a dev-dependency edge no
/// consumer's build graph carries, so the two calls below exist in
/// this binary and in no shipped build: what the row measures is the
/// cost of the TEST-SUPPORT door, not a lane a consumer can take.
/// `NodePick` is the only mint a consumer has, and the ignored witness
/// `gui1_pick_r2::a_mesh_paired_with_the_wrong_node_does_not_answer_a_name`
/// holds the node half of the same class (#1098) under the same
/// feature.
#[test]
fn a_raw_target_is_a_claim_in_every_half() {
    let t = Twins::build();
    let tol = Tol::witness();
    let pick = editor_core::NodePick::build(&t.ev_square, t.node, 0, 0.1, tol)
        .expect("the square prism tessellates");
    assert_eq!(
        editor_core::pick_face(&t.ev_triangle, &[pick.target()], &down())
            .expect_err("the minted target carries the index's document"),
        HitTestError::EvaluationOfAnotherDocument {
            expected: t.square.id(),
            found: t.triangle.id(),
        },
        "the minted target refuses the twin's evaluation"
    );

    // The raw path: the SQUARE's mesh, indexed by hand, declared to be
    // of the triangle's document under the square's node id.
    let forged_index =
        editor_core::MeshPick::build(pick.mesh()).expect("the square's mesh indexes");
    let forged = editor_core::PickTarget::new(&t.ev_triangle, t.node, 0, &forged_index);
    let hit = editor_core::pick_face(&t.ev_triangle, &[forged], &down())
        .expect("a raw target's declaration is taken at its word");
    assert!(
        hit.is_some(),
        "the raw path answers a name for the square's mesh out of the \
         TRIANGLE's tables: the document half of a hand-assembled \
         target is contracted, not proved, exactly as the node half is"
    );
}

/// **The pairing arm's Display** (lane `nodepick-rv`, Q3 on the Display
/// roster): `m4_pr4_hit.rs`'s row asserts the pair `["document",
/// "not"]`, both of which survive a message that names NEITHER
/// document. This row asserts what that one cannot: both ids render,
/// by their `hex()`, as the `product` twin
/// (`docm4_evaluation_identity::the_product_doors_refuse_an_evaluation_of_another_document`)
/// asserts for `ProductError`'s arm.
#[test]
fn the_pairing_arm_renders_both_documents() {
    let expected = DocumentId::derive("probe-render-expected");
    let found = DocumentId::derive("probe-render-found");
    let shown = HitTestError::EvaluationOfAnotherDocument { expected, found }.to_string();
    assert!(shown.contains(&expected.hex()), "{shown}");
    assert!(shown.contains(&found.hex()), "{shown}");
}

/// **What the ADMITTED case costs, measured** (lane `nodepick-rv`,
/// claim 6).
///
/// [`a_later_evaluation_of_the_same_document_is_admitted`] says what
/// the doors answer. This row says what a LIVE caller does, which is
/// never to hold a stale index against a later run at all: the memo's
/// key carries the node's content key, the edit moved it, so the memo
/// MISSES and rebuilds. The admitted case is reachable only by keeping
/// an index across pictures by hand.
#[test]
fn what_the_admitted_later_evaluation_answers() {
    let tol = Tol::witness();
    let (doc, ext) = prism("edit-pair-probe-later", 4);
    let before = run(&doc);
    let mut memo = editor_core::PickMemo::new();
    let pick = editor_core::NodePick::build_with(&before, ext, 0, 0.1, tol, &mut memo)
        .expect("the prism tessellates");
    memo.end_picture();
    assert_eq!(memo.node_misses(), 1, "the first picture builds");

    let edited = doc
        .apply(
            &DocEdit::SetParam {
                node: ext,
                slot: SlotId::Distance,
                expr: len(2.0),
            },
            tol,
            &editor_core::RefusingReach,
        )
        .expect("a length goes into the extrusion distance")
        .doc;
    let after = run(&edited);
    assert_eq!(
        pick.patch_names(&after).expect("admitted").len(),
        pick.patch_names(&before).expect("own run").len(),
        "the premise: the index is the same one, so the two answers are \
         comparable slot for slot"
    );

    // What a live caller does with the later run: the memo's content
    // key half moved, so the stale index is not served — it is rebuilt.
    let _rebuilt = editor_core::NodePick::build_with(&after, ext, 0, 0.1, tol, &mut memo)
        .expect("the prism tessellates again");
    memo.end_picture();
    assert_eq!(
        memo.node_hits(),
        0,
        "the content key moved, so the memo does NOT serve the stale \
         index: a live caller never pairs one"
    );
    assert_eq!(memo.node_misses(), 1, "it rebuilt instead");
}
