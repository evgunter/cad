//! **LIB-U5 — the name→geometry doors and their refusal ladder.**
//!
//! The happy path is exercised at the façade (`pncad::select`'s worked
//! example). What is pinned HERE is the part a doctest cannot reach
//! comfortably: the rungs of [`InterrogateError`] a public door
//! produces, and `edge_frame` against a body whose edges are lines.
//!
//! Why the ladder deserves a test of its own: these doors are the
//! only route from a stored selection to a coordinate, and a stale
//! selection is the NORMAL case after an upstream edit. Each refusal
//! is a different fact about the model — the name is gone, the name
//! is tied, the node never evaluated, the node failed, the entity is
//! a whole body — and a caller that cannot tell them apart cannot
//! recover from any of them. An untested ladder is one where two
//! rungs silently collapse into each other.
//!
//! # Which rungs those are, and what decides it
//!
//! `the_reachable_ladder_is_driven_through_its_doors` — it drives each
//! one through a door and compares what the doors HANDED BACK against
//! [`InterrogateError`]'s own identifier roster, so every rung of the
//! enum is either driven there or excluded there by name with the
//! measurement that excludes it. The set is deliberately not restated
//! here, nor counted: the row reds when it and the enum disagree, and a
//! number written out beside it would answer to nothing.
//!
//! **This paragraph is prose, and nothing checks it.** The row welds its
//! driven set to the enum; no line anywhere compares these words to that
//! row, so a sentence here that goes stale goes stale silently. That is
//! the defect this header carried one scope wider — the claim was "every
//! rung" over evidence for five — and narrowing a claim installs no guard
//! against carrying it again.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, Dimension, EntityKind, EvalOptions, Expr, InterrogateError, Node, ProfileDoc,
    RecipeNodeId, RoleSeg, StableName, all_edges, all_faces, all_vertices, denotation, edge_frame,
    evaluate, face_frame, vertex_position,
};
use geom_core::Tol;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("a length literal")
}

fn eval(doc: &ProfileDoc) -> editor_core::Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// A unit box as an extruded square, and its extrude node.
fn box_doc() -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("lib_u5_interrogate", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
        },
    )
}

/// **`edge_frame` answers, and a straight edge answers honestly.**
///
/// Every edge of a box is a line: a direction and NO distinguished
/// perpendicular. The door reports `u_ref: None` rather than
/// inventing one, and `v_ref()` follows it.
#[test]
fn edge_frame_reads_every_line_carrier_and_declines_to_invent_a_perpendicular() {
    let (doc, node) = box_doc();
    let ev = eval(&doc);
    let edges = all_edges(&ev, node);
    assert_eq!(edges.len(), 12);
    for name in &edges {
        let pose = edge_frame(&ev, node, name).expect("a certified line carrier");
        assert!(
            pose.u_ref.is_none() && pose.v_ref().is_none(),
            "a line fixes no reference perpendicular"
        );
        // The direction is a unit vector — the carrier's own, copied.
        let n = pose.axis;
        assert!((n.x.abs() + n.y.abs() + n.z.abs() - 1.0).abs() < 1e-12);
    }
}

/// **The doors are kind-checked, and a whole body is its own fact.**
#[test]
fn the_doors_refuse_the_wrong_kind_and_name_a_whole_body_separately() {
    let (doc, node) = box_doc();
    let ev = eval(&doc);
    let face = all_faces(&ev, node)[0].clone();
    let edge = all_edges(&ev, node)[0].clone();
    let vertex = all_vertices(&ev, node)[0].clone();

    assert!(matches!(
        face_frame(&ev, node, &edge),
        Err(InterrogateError::WrongKind {
            wanted: EntityKind::Face,
            found: EntityKind::Edge
        })
    ));
    assert!(matches!(
        edge_frame(&ev, node, &vertex),
        Err(InterrogateError::WrongKind {
            wanted: EntityKind::Edge,
            found: EntityKind::Vertex
        })
    ));
    assert!(matches!(
        vertex_position(&ev, node, &face),
        Err(InterrogateError::WrongKind {
            wanted: EntityKind::Vertex,
            found: EntityKind::Face
        })
    ));

    // A whole body has no single frame — a DIFFERENT fact from "you
    // asked for the wrong kind of entity", so it gets its own rung.
    let body = editor_core::all_bodies(&ev, node)[0].clone();
    assert!(matches!(
        face_frame(&ev, node, &body),
        Err(InterrogateError::WholeBody)
    ));
}

/// **A name nothing answers to is `NoSuchName`, not a panic and not
/// a zero value** — the stale-selection case, which is what happens
/// normally after an upstream edit.
#[test]
fn an_unknown_name_refuses_typed() {
    let (doc, node) = box_doc();
    let ev = eval(&doc);
    // Well-formed, and not in this node's table: `OutputBody` is a
    // BODY's role segment, so no FACE ever answers to this name.
    let stranger = StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::OutputBody],
    };
    assert_eq!(
        face_frame(&ev, node, &stranger).unwrap_err(),
        InterrogateError::NoSuchName
    );
    assert_eq!(
        denotation(&ev, node, &stranger).unwrap_err(),
        InterrogateError::NoSuchName
    );
}

/// **A node with no result in this evaluation is `NodeNotEvaluated`**
/// — distinguishable from "the node evaluated and has no such name",
/// which is the distinction a caller recovers differently from.
#[test]
fn a_foreign_node_id_refuses_typed_and_differs_from_an_unknown_name() {
    let (doc, node) = box_doc();
    let ev = eval(&doc);
    let name = all_faces(&ev, node)[0].clone();
    let foreign = RecipeNodeId(4242);

    assert_eq!(
        face_frame(&ev, foreign, &name).unwrap_err(),
        InterrogateError::NodeNotEvaluated { node: foreign }
    );
    assert_eq!(
        denotation(&ev, foreign, &name).unwrap_err(),
        InterrogateError::NodeNotEvaluated { node: foreign }
    );
    // The two failures are NOT the same value: the ladder's rungs stay
    // apart.
    assert_ne!(
        face_frame(&ev, foreign, &name).unwrap_err(),
        InterrogateError::NoSuchName
    );
}

/// **`denotation` agrees with the doors**: every name the
/// materializers hand back resolves uniquely, and the geometry doors
/// answer for exactly those. (A tie would refuse `Ambiguous`; this
/// corpus mints none, which is itself worth pinning — the tie path is
/// N2's, not a routine outcome.)
#[test]
fn every_materialized_name_denotes_uniquely_and_answers() {
    let (doc, node) = box_doc();
    let ev = eval(&doc);
    for name in all_faces(&ev, node) {
        assert_eq!(
            denotation(&ev, node, &name),
            Ok(editor_core::Denotation::Unique)
        );
        assert!(face_frame(&ev, node, &name).is_ok());
    }
    for name in all_vertices(&ev, node) {
        assert!(vertex_position(&ev, node, &name).is_ok());
    }
}

/// **A read door asks what a name denotes before it asks how many
/// entities answer to it** — PORT-DOORS-1's rule
/// (`assembly::resolve_face`) at the body the five read doors share.
///
/// A face name handed to `edge_frame` is unreadable there however few
/// entities answer to it, so narrowing is no recourse and `WrongKind`
/// is the whole fault. Before the order changed, the answer depended
/// on whether the name happened to be tied: a unique face name got
/// `WrongKind`, a tied one got `Ambiguous` and an instruction to
/// narrow. The rows below pin that the two now agree, and that the
/// tie still refuses at the door that DOES read faces.
#[test]
fn a_read_door_refuses_a_tied_name_of_another_kind_by_its_kind() {
    use editor_core::Entry;

    // The symmetric U cutter's N2 tie.
    let (doc, sub) = fixture::u_cutter_tie(ProfileDoc::empty_derived(
        "lib_u5_interrogate_tie",
        Tol::witness(),
    ));
    let ev = eval(&doc);
    let table = &ev.value(sub).expect("the U subtract evaluates").name_table;
    let tied: StableName = table
        .iter()
        .find_map(|(n, e)| {
            (n.kind == EntityKind::Face && matches!(e, Entry::Tied(_))).then(|| n.clone())
        })
        .expect("the U fixture ties a face");
    let unique: StableName = table
        .iter()
        .find_map(|(n, e)| {
            (n.kind == EntityKind::Face && matches!(e, Entry::Unique(_))).then(|| n.clone())
        })
        .expect("the U subtract names a unique face");

    // The tie is REAL and the door's own kind is the one the name
    // does NOT denote: without both, the row below passes vacuously.
    let candidates = match denotation(&ev, sub, &tied) {
        Ok(editor_core::Denotation::Tied { candidates }) => candidates,
        other => panic!("the declared name must really be tied, got {other:?}"),
    };
    assert!(candidates >= 2, "a tie is two or more candidates");

    let wrong_kind = InterrogateError::WrongKind {
        wanted: EntityKind::Edge,
        found: EntityKind::Face,
    };
    assert_eq!(
        edge_frame(&ev, sub, &tied).err(),
        Some(wrong_kind),
        "an edge door handed a FACE name is not a door that must pick one — the tie is \
         not the fault and narrowing is no recourse"
    );
    assert_eq!(
        edge_frame(&ev, sub, &unique).err(),
        Some(wrong_kind),
        "and the unique name of the same kind gets the same word"
    );
    // The tie still refuses where the kind DOES match: this changes
    // the order of two questions, not whether a tie is referenceable.
    assert_eq!(
        face_frame(&ev, sub, &tied).err(),
        Some(InterrogateError::Ambiguous { candidates }),
        "a tied FACE name at the face door is still the N2 tie"
    );
}

/// The box, plus a node that FAILED and a node POISONED by that
/// failure — the two node-ladder rungs no evaluation of a good
/// document can produce.
///
/// A zero-distance extrude is degenerate, so its node fails; anything
/// downstream of a failed node is poisoned THROUGH it.
fn box_with_a_failed_and_a_poisoned_node() -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId)
{
    let (doc, good) = box_doc();
    let (doc, square) = fixture::on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, failed) = fixture::insert(
        doc,
        Node::Extrude {
            profile: square,
            distance: len(0.0),
        },
    );
    let (doc, poisoned) = fixture::insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: failed,
            b: good,
            declare: None,
        },
    );
    (doc, good, failed, poisoned)
}

/// A face of `loft_prism` whose carrier is a NURBS patch, with the node
/// it lives on — the skinned wall the loft's non-affine middle section
/// forces. Found by asking the carrier-kind door rather than by naming
/// a node and a face: those are the loft's to change.
fn a_frameless_carrier(ev: &editor_core::Evaluation<f64>) -> (RecipeNodeId, StableName) {
    let nodes: Vec<RecipeNodeId> = ev.nodes.keys().copied().collect();
    nodes
        .into_iter()
        .flat_map(|node| {
            all_faces(ev, node)
                .into_iter()
                .map(move |name| (node, name))
        })
        .find(|(node, name)| {
            editor_core::face_carrier_kind(ev, *node, name) == Ok(geom_brep::SurfaceKind::Nurbs)
        })
        .expect("the loft's skinned walls are NURBS patches")
}

/// **Every rung a public door produces, driven through a door, in one
/// process, welded to [`InterrogateError`]'s own roster.**
///
/// The header above states a scope; this row is what makes that scope
/// falsifiable. Each rung below is the value a DOOR handed back — never
/// one this test constructed — and its identifier is read off that
/// value's own `Debug` (`test_utils::f6::variant_identifier`), so a
/// variant renamed in `src/` moves the pattern and the witness together
/// and no string here can be left saying the old name.
///
/// **One door, because the ladder is one ladder.** The five read doors
/// share `interrogate::read`, so `face_frame` walks every rung
/// `value_of`, `entity_of` and `output_body` can raise; a second door
/// here would re-drive the same sites and say nothing more about the
/// enum. `denotation` is the one public door that does NOT share that
/// body, and the rows above pin it against rungs of this same ladder.
///
/// **The two rungs this does not drive, measured rather than argued**
/// (`memories/refusal-text-is-not-cause.md`: "the arm looks
/// unreachable" is a claim about a call graph, so the doors were run
/// and their payloads read rather than their callers grepped).
///
/// Both are raised by `interrogate::output_body`, and a read door
/// reaches it with the NAME TABLE's own body index — emission's, never
/// a caller's. So through these doors `NoSuchBody` means the emission
/// and the value disagree, which is the kernel bug its own doc comment
/// names, and `NoBodies` needs a node whose value carries no bodies and
/// whose table nonetheless holds a row. Driving every name of every
/// corpus node's table produced neither rung.
///
/// The one door whose body index IS the caller's is `clearance`, and it
/// answers `ClearanceRefusal::Selection(SelectionRefusal::NoSuchBody
/// { .. })` for a bad index — a `map_err(|_| ..)` one frame up destroys
/// the `InterrogateError` before a caller can see it, which is
/// `work/shell/clearance-reports-a-no-bodies-payload-as-a-bad-body-index`
/// on SHELL's slate. A row for either rung today would pin that defect.
/// If SHELL's repair lands, that door becomes the place to drive them.
///
/// They are CONSTRUCTED here rather than named in a string for the
/// reason the driven rungs are not: rustc checks a constructor's
/// variant and its fields, and a rename that left one of these behind
/// would not compile.
#[test]
fn the_reachable_ladder_is_driven_through_its_doors() {
    use editor_core::{Entry, all_bodies};
    use test_utils::f6::variant_identifier;
    use topo::readback::ReadbackError;

    let (doc, good, failed, poisoned) = box_with_a_failed_and_a_poisoned_node();
    let ev = eval(&doc);
    let face = all_faces(&ev, good)[0].clone();
    let edge = all_edges(&ev, good)[0].clone();
    let body = all_bodies(&ev, good)[0].clone();
    let foreign = RecipeNodeId(4242);
    // Well-formed and answered by no FACE: `OutputBody` is a body's
    // role segment.
    let stranger = StableName {
        kind: EntityKind::Face,
        node: good,
        path: vec![RoleSeg::OutputBody],
    };

    // The N2 tie, from the fixture that mints one.
    let (tie_doc, sub) = fixture::u_cutter_tie(ProfileDoc::empty_derived(
        "lib_u5_interrogate_ladder",
        Tol::witness(),
    ));
    let tie_ev = eval(&tie_doc);
    let table = &tie_ev
        .value(sub)
        .expect("the U subtract evaluates")
        .name_table;
    let (tied, candidates) = table
        .iter()
        .find_map(|(n, e)| match e {
            Entry::Tied(c) if n.kind == EntityKind::Face => Some((n.clone(), c.len())),
            _ => None,
        })
        .expect("the U fixture ties a face");

    let loft = crate::corpus::loft_prism::document();
    let loft_ev = eval(&loft.doc);
    let (frameless_at, frameless) = a_frameless_carrier(&loft_ev);

    // Each entry: what the door was asked, what it must answer, and
    // what it actually answered.
    let driven = [
        (
            "a node id this run did not produce",
            InterrogateError::NodeNotEvaluated { node: foreign },
            face_frame(&ev, foreign, &face),
        ),
        (
            "a node whose own evaluation failed",
            InterrogateError::NodeFailed { node: failed },
            face_frame(&ev, failed, &face),
        ),
        (
            "a node poisoned by that failure",
            InterrogateError::NodePoisoned {
                node: poisoned,
                through: failed,
            },
            face_frame(&ev, poisoned, &face),
        ),
        (
            "a well-formed name nothing in the node answers to",
            InterrogateError::NoSuchName,
            face_frame(&ev, good, &stranger),
        ),
        (
            "a tied face name at the door that reads faces",
            InterrogateError::Ambiguous { candidates },
            face_frame(&tie_ev, sub, &tied),
        ),
        (
            "an edge name at the door that reads faces",
            InterrogateError::WrongKind {
                wanted: EntityKind::Face,
                found: EntityKind::Edge,
            },
            face_frame(&ev, good, &edge),
        ),
        (
            "a whole-body name, which has no single frame",
            InterrogateError::WholeBody,
            face_frame(&ev, good, &body),
        ),
        (
            "a face whose carrier is a NURBS patch",
            InterrogateError::Readback(ReadbackError::NoCanonicalFrame {
                carrier: "nurbs surface",
            }),
            face_frame(&loft_ev, frameless_at, &frameless),
        ),
    ];

    let mut witnessed: Vec<String> = Vec::new();
    for (asked, want, got) in &driven {
        let got = got.as_ref().err().unwrap_or_else(|| {
            panic!("{asked}: the door answered instead of refusing, so this rung was not driven")
        });
        assert_eq!(got, want, "{asked}");
        witnessed.push(variant_identifier(got));
    }

    // The undriven rungs, with the reason in the doc comment above.
    let undriven = [
        InterrogateError::NoBodies { payload: "datum" },
        InterrogateError::NoSuchBody { index: 7 },
    ];
    for err in &undriven {
        let ident = variant_identifier(err);
        assert!(
            !witnessed.contains(&ident),
            "{ident} is both driven and declared undriven — drop it from the undriven list"
        );
        witnessed.push(ident);
    }

    let accounted: Vec<&str> = witnessed.iter().map(String::as_str).collect();
    if let Some(report) = test_utils::census::set_difference(
        crate::display_contract::INTERROGATE_ERROR.identifiers(),
        &accounted,
        "InterrogateError's rungs and what this suite accounts for disagree",
        "accounted for here and not a variant of the enum — fix its spelling",
        "a rung of the enum that no door here drives and no line here excludes — drive it, \
         or name it undriven with the measurement that says why",
    ) {
        panic!("{report}");
    }
}
