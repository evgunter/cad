//! **The selection's refusal ladder** (M6-5) — the N5 typed trio, at
//! the fillet node, executed one plant at a time.
//!
//! A selection is a COMMITMENT (the #217 freeze ruling), so a name
//! that stops resolving is a refusal, never a silent shrink. The
//! vocabulary is `resolve_declarations`' verbatim — the two sites
//! answer the same question, so they answer it the same way — plus two
//! refusals a single-operand selection needs and a Declare pair does
//! not: a mis-KINDED name and an EMPTY set.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use crate::fixture::len;
use editor_core::resolve::{Diagnosis, RecipeEditRef, ResolveError};
use editor_core::{
    CancelToken, EntityKind, EvalOptions, Node, NodeErrorKind, NodeResult, ProfileDoc,
    RecipeNodeId, RoleSeg, StableName, evaluate,
};
use geom_core::Tol;

fn eval(doc: &ProfileDoc) -> editor_core::Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// The plants build their own documents through the public authoring
/// door instead of mutating one: a cube, an extrude, and a fillet
/// whose selection is the plant.
/// This suite's documents all start the same way: the sketch frame,
/// the profile drawn on it, then the extrude whose body every rim name
/// below is minted by — the document's third node.
fn body(doc: &ProfileDoc) -> RecipeNodeId {
    doc.order()[2]
}

fn planted(selection: impl FnOnce(&ProfileDoc) -> Vec<StableName>) -> (ProfileDoc, RecipeNodeId) {
    use editor_core::{DocEdit, LoopProgram, ProfileProgram, apply};
    let square =
        LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]).expect("finite");
    let doc = ProfileDoc::empty_derived("m6_5_selection_refusals", Tol::witness());
    let (doc, plane) = fixture::insert(doc, fixture::xy_frame());
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![square],
            ids: Vec::new(),
        }),
    );
    let (doc, body) = fixture::insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let applied = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::fillet(body, len(0.125), selection(&doc))),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the fillet inserts");
    let id = applied.record.minted.expect("a minted id");
    (applied.doc, id)
}

/// The symmetric U cutter subtracted from a block — the corpus's
/// standard N2 TIE source (`m4_pr5_declare.rs`'s fixture, verbatim
/// geometry). Returns the document and the subtract node.
fn symmetric_u() -> (ProfileDoc, RecipeNodeId) {
    use editor_core::{BooleanOp, DocEdit, apply};
    use fixture::on_frame;
    let mut doc = ProfileDoc::empty_derived("m6_5_selection_refusals", Tol::witness());
    let insert = |doc: &ProfileDoc, node: Node<editor_core::ProfileProgram>| {
        let a = apply(
            doc,
            &DocEdit::InsertNode {
                node: Box::new(node),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("the fixture builds");
        let id = a.record.minted.expect("a minted id");
        (a.doc, id)
    };
    let square = vec![vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]];
    let (d, bp) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        square,
    );
    let (d, ua) = insert(
        &d,
        Node::Extrude {
            profile: bp,
            distance: len(4.0),
        },
    );
    let (d, up) = on_frame(
        d,
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (2.0, 1.0),
            (6.0, 1.0),
            (6.0, 3.0),
            (2.0, 3.0),
            (2.0, 2.5),
            (5.0, 2.5),
            (5.0, 1.5),
            (2.0, 1.5),
        ]],
    );
    let (d, ub) = insert(
        &d,
        Node::Extrude {
            profile: up,
            distance: len(2.0),
        },
    );
    let (d, us) = insert(
        &d,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: ua,
            b: ub,
            declare: None,
        },
    );
    doc = d;
    (doc, us)
}

fn rim(doc: &editor_core::ProfileDoc, node: RecipeNodeId, seg: u32) -> StableName {
    StableName {
        kind: EntityKind::Edge,
        node,
        path: vec![RoleSeg::RimEdge(
            editor_core::CapEnd::End,
            crate::fixture::piece(doc, node, 0, seg as usize),
        )],
    }
}

/// Runs the plant and hands its typed refusal to `check`.
fn refuses(
    doc: &editor_core::ProfileDoc,
    fillet: RecipeNodeId,
    check: impl FnOnce(&NodeErrorKind),
) {
    let ev = eval(doc);
    match ev.nodes.get(&fillet) {
        Some(NodeResult::Failed(e)) => check(&e.kind),
        other => panic!("the fillet must refuse, got {other:?}"),
    }
}

/// A never-existed id never reaches evaluation: the D3 carve-out
/// door refuses it at EDIT time, where the diagnostics are best. M6-5
/// puts a fillet's selection behind the same door as a Declare's
/// pairs.
#[test]
fn a_selection_naming_a_never_existed_node_refuses_at_edit_time() {
    use editor_core::{DocEdit, EditError, apply};
    let (doc, _) = planted(|doc| vec![rim(doc, body(doc), 0)]);
    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::fillet(
                body(&doc),
                len(0.125),
                vec![{
                    let mut elsewhere = rim(&doc, body(&doc), 0);
                    elsewhere.node = RecipeNodeId(99);
                    elsewhere
                }],
            )),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::DeclareNamesMissingNode { name }) => {
            assert_eq!(name.node, RecipeNodeId(99));
        }
        other => panic!("a typo id must refuse at the edit door, got {other:?}"),
    }
}

/// **NodeGone**: the selection names a node that EXISTED and was
/// deleted — the N5 dangling path, which the edit door deliberately
/// allows and evaluation refuses typed. Ids are never reused, so the
/// diagnosis says DELETED rather than foreign.
#[test]
fn a_selection_naming_a_deleted_node_is_node_gone() {
    use editor_core::{DocEdit, apply};
    // A second extrude off the same profile: nothing depends on it, so
    // it can be deleted out from under the selection.
    let (doc, _) = planted(|doc| vec![rim(doc, body(doc), 0)]);
    let spare = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Extrude {
                profile: doc.order()[1],
                distance: len(2.0),
            }),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the spare extrude inserts");
    let spare_id = spare.record.minted.expect("a minted id");
    let with_fillet = apply(
        &spare.doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::fillet(
                body(&doc),
                len(0.125),
                vec![rim(&spare.doc, spare_id, 0)],
            )),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the fillet inserts while the spare is live");
    let fillet = with_fillet.record.minted.expect("a minted id");
    let after = apply(
        &with_fillet.doc,
        &DocEdit::DeleteNode { id: spare_id },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("deleting a node a NAME references is allowed (N5)")
    .doc;
    refuses(&after, fillet, |kind| match kind {
        NodeErrorKind::BlendSelectionResolve { error, .. } => match error.as_ref() {
            ResolveError::NodeGone { name, edit } => {
                assert_eq!(name.node, spare_id);
                assert!(
                    matches!(edit, RecipeEditRef::NodeDeleted { node } if *node == spare_id),
                    "an id below the mint counter is DELETED, not foreign"
                );
            }
            other => panic!("expected NodeGone, got {other:?}"),
        },
        other => panic!("expected a selection-resolve refusal, got {other:?}"),
    });
}

/// **Vanished**: the node exists, but the name is not in its table —
/// the honest single-run fallback diagnosis names the minting node as
/// the disagreement site rather than claiming an edit happened.
#[test]
fn a_selection_naming_an_absent_entity_is_vanished() {
    // A piece the square never draws: a well-formed name for an edge
    // the extrude never minted.
    let (doc, fillet) = planted(|doc| {
        vec![StableName {
            kind: EntityKind::Edge,
            node: body(doc),
            path: vec![RoleSeg::RimEdge(
                editor_core::CapEnd::End,
                crate::fixture::no_piece_of(doc),
            )],
        }]
    });
    refuses(&doc, fillet, |kind| match kind {
        NodeErrorKind::BlendSelectionResolve { error, .. } => match error.as_ref() {
            ResolveError::Vanished {
                name,
                diagnosis,
                last_good,
            } => {
                assert_eq!(name.node, body(&doc));
                assert!(last_good.is_none(), "no prior run is consultable mid-eval");
                assert!(
                    matches!(
                        diagnosis,
                        Diagnosis::RecipeEdit {
                            edit: RecipeEditRef::NodeChanged { node }
                        } if *node == body(&doc)
                    ),
                    "the diagnosis names the minting node, got {diagnosis:?}"
                );
            }
            other => panic!("expected Vanished, got {other:?}"),
        },
        other => panic!("expected a selection-resolve refusal, got {other:?}"),
    });
}

/// **Ambiguous + TieWitness**: a tie-marked name has two or more
/// equally-admissible candidates, so a selection cannot pick one. The
/// tie is the symmetric U cutter's N2 tie — the same fixture
/// `m4_pr5_declare.rs` uses for the Declare side of this refusal, so
/// the two sites are shown answering the same question the same way.
#[test]
fn a_tied_selection_name_refuses_ambiguous_with_its_witness() {
    let (doc, us) = symmetric_u();
    let ev = eval(&doc);
    let tied: StableName = ev
        .nodes
        .get(&us)
        .and_then(|r| match r {
            NodeResult::Ok(v) => Some(v),
            _ => None,
        })
        .expect("the U subtract evaluates")
        .name_table
        .iter()
        // Any tied row will do: the resolver reaches the TIE before it
        // reaches the kind check, so ambiguity is refused first — which
        // is the right order (a name that names two things cannot be
        // checked for what it names).
        .find_map(|(n, e)| matches!(e, editor_core::Entry::Tied(_)).then(|| n.clone()))
        .expect("the U fixture ties");

    let applied = editor_core::apply(
        &doc,
        &editor_core::DocEdit::InsertNode {
            node: Box::new(Node::fillet(us, len(0.125), vec![tied.clone()])),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the fillet inserts");
    let fillet = applied.record.minted.expect("a minted id");
    refuses(&applied.doc, fillet, |kind| match kind {
        NodeErrorKind::BlendSelectionResolve { error, .. } => match error.as_ref() {
            ResolveError::Ambiguous {
                name,
                candidates,
                tie,
            } => {
                assert_eq!(*name, tied);
                assert_eq!(candidates.as_slice(), std::slice::from_ref(&tied));
                assert_eq!(tie.at, tied);
                assert!(tie.width >= 2, "a tie has at least two candidates");
                // Mid-evaluation the witness site IS the minting
                // node: there is no evaluation-wide index to name a
                // carrying node from.
                assert_eq!(tie.node, tied.node);
            }
            other => panic!("expected Ambiguous, got {other:?}"),
        },
        other => panic!("expected a selection-resolve refusal, got {other:?}"),
    });
}

/// **Mis-kinded**: a selection naming a FACE is a recipe bug, refused
/// rather than reinterpreted as "the face's edges".
#[test]
fn a_selection_naming_a_face_refuses_on_kind() {
    let face = |doc: &ProfileDoc| StableName {
        kind: EntityKind::Face,
        node: body(doc),
        path: vec![RoleSeg::Cap(editor_core::CapEnd::End)],
    };
    let (doc, fillet) = planted(|doc| vec![face(doc)]);
    let face = face(&doc);
    refuses(&doc, fillet, |kind| match kind {
        NodeErrorKind::BlendSelectionKind { name, found, .. } => {
            assert_eq!(**name, face);
            assert_eq!(found.kind(), EntityKind::Face);
        }
        other => panic!("expected a kind refusal, got {other:?}"),
    });
}

/// **Empty**: a fillet of nothing is not the identity — it is an
/// unfinished recipe. No op in this kernel silently returns its input.
#[test]
fn an_empty_selection_refuses() {
    let (doc, fillet) = planted(|_| Vec::new());
    refuses(&doc, fillet, |kind| {
        assert!(
            matches!(kind, NodeErrorKind::BlendSelectionEmpty { .. }),
            "{kind:?}"
        );
    });
}
