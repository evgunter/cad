//! **LIB-U5 — the name→geometry doors and their refusal ladder.**
//!
//! The happy path is exercised at the façade (`pncad::select`'s worked
//! example). What is pinned HERE is the part a doctest cannot reach
//! comfortably: the rungs of [`InterrogateError`] the name→geometry
//! doors produce, and `edge_frame` against a body whose edges are
//! lines.
//!
//! **"The name→geometry doors" is the narrow reading, and it is the
//! one every claim below takes**: `denotation`, `face_frame`,
//! `face_carrier_kind`, `edge_frame`, `edge_carrier_kind` and
//! `vertex_position` — the doors `names::interrogate` exports, whose
//! own return type is this enum. Two other public doors surface an
//! `InterrogateError` WHOLE inside a refusal of their own and are not
//! this suite's subject: `select_where` and `find_flush_candidates`
//! wrap one in `SelectRefusal::Unreadable` (`names::select`,
//! `names::flush`), and a measurement reference wraps one in
//! `NodeErrorKind::MeasureRefUnreadable` (`eval::wire`). They index
//! the payload with the name table's own `ent.body`, exactly as
//! `interrogate::entity_of` does, so the two exclusions the ladder row
//! measures hold at those sites too — but nothing here drives them.
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
//! measurement that excludes it, and that measurement is RE-TAKEN on
//! every run rather than remembered. The set is deliberately not
//! restated here, nor counted: the row reds when it and the enum
//! disagree, and a number written out beside it would answer to
//! nothing.
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
    RecipeNodeId, RoleSeg, StableName, all_edges, all_faces, all_vertices, denotation,
    edge_carrier_kind, edge_frame, evaluate, face_carrier_kind, face_frame, vertex_position,
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

/// The profile every document in this file extrudes: one unit square
/// on the frame's own axes, wound counter-clockwise.
fn unit_square() -> Vec<Vec<(f64, f64)>> {
    vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]]
}

/// A well-formed FACE name nothing in `node` answers to: `OutputBody`
/// is a BODY's role segment, so no face ever carries it.
fn a_name_no_face_answers_to(node: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::OutputBody],
    }
}

/// A unit box as an extruded square, and its extrude node.
fn box_doc() -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("lib_u5_interrogate", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        unit_square(),
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
    let stranger = a_name_no_face_answers_to(node);
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

/// The symmetric U cutter's N2 tie, evaluated once: the subtract node,
/// the tied FACE name with its candidate count, and a unique FACE name
/// of the same node to compare it against.
struct UCutterTie {
    ev: editor_core::Evaluation<f64>,
    sub: RecipeNodeId,
    tied: StableName,
    candidates: usize,
    unique: StableName,
}

/// Build and evaluate that fixture. `label` is the derived document's
/// id, so two rows of this file do not share one.
fn u_cutter_tied_face(label: &str) -> UCutterTie {
    use editor_core::Entry;

    let (doc, sub) = fixture::u_cutter_tie(ProfileDoc::empty_derived(label, Tol::witness()));
    let ev = eval(&doc);
    let (tied, candidates, unique) = {
        let table = &ev.value(sub).expect("the U subtract evaluates").name_table;
        let (tied, candidates) = table
            .iter()
            .find_map(|(n, e)| match e {
                Entry::Tied(c) if n.kind == EntityKind::Face => Some((n.clone(), c.len())),
                _ => None,
            })
            .expect("the U fixture ties a face");
        let unique = table
            .iter()
            .find_map(|(n, e)| {
                (n.kind == EntityKind::Face && matches!(e, Entry::Unique(_))).then(|| n.clone())
            })
            .expect("the U subtract names a unique face");
        (tied, candidates, unique)
    };
    UCutterTie {
        ev,
        sub,
        tied,
        candidates,
        unique,
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
    let UCutterTie {
        ev,
        sub,
        tied,
        unique,
        ..
    } = u_cutter_tied_face("lib_u5_interrogate_tie");

    // The tie is REAL and the door's own kind is the one the name
    // does NOT denote: without both, the row below passes vacuously.
    // Asked through `denotation` rather than read off the table, so
    // the count below is the DOOR's.
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
        unit_square(),
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

/// An entity of `ev` of the given kind whose carrier is a NURBS one,
/// with the node it lives on — in `loft_prism`, the skinned wall the
/// non-affine middle section forces and the section boundary that
/// walks it. Found by asking the carrier-kind door rather than by
/// naming a node and an entity: those are the loft's to change.
///
/// One function over both kinds, not a face copy and an edge copy: the
/// two differ only in which `all_*` enumerates and which carrier-kind
/// door answers.
fn a_frameless_carrier(
    ev: &editor_core::Evaluation<f64>,
    kind: EntityKind,
) -> (RecipeNodeId, StableName) {
    let nodes: Vec<RecipeNodeId> = ev.nodes.keys().copied().collect();
    nodes
        .into_iter()
        .flat_map(|node| {
            let names = match kind {
                EntityKind::Face => all_faces(ev, node),
                EntityKind::Edge => all_edges(ev, node),
                other => panic!("{other:?} has no carrier-kind door"),
            };
            names.into_iter().map(move |name| (node, name))
        })
        .find(|(node, name)| match kind {
            EntityKind::Face => {
                face_carrier_kind(ev, *node, name) == Ok(geom_brep::SurfaceKind::Nurbs)
            }
            _ => edge_carrier_kind(ev, *node, name) == Ok(editor_core::CurveKind::Nurbs),
        })
        .expect("the loft's skinned walls are NURBS, and so are the curves that bound them")
}

type ReadDoor =
    fn(&editor_core::Evaluation<f64>, RecipeNodeId, &StableName) -> Option<InterrogateError>;

/// Each door paired with its own name, spelled by `stringify!` off the
/// path rather than typed beside it: a door renamed in `src/` moves
/// the call and the label together, and no string here can be left
/// saying the old name.
macro_rules! read_doors {
    ($($door:path),+ $(,)?) => {
        &[$((
            stringify!($door),
            (|ev, node, name| $door(ev, node, name).err()) as ReadDoor,
        )),+]
    };
}

/// The READ doors — the ones sharing `interrogate::read`, so every one
/// of them raises the same [`InterrogateError`] ladder. Named, so a
/// sweep that finds something can say which door asked.
///
/// **This list is hand-kept and nothing welds it.** rustc checks that
/// each entry is a real door and that they share one signature; it
/// cannot check that they are EVERY door sharing `read`. A sixth added
/// to `names::interrogate` tomorrow would simply not be swept, and the
/// zero below would narrow without anything going red. No census is
/// available for it either: a scan of the source text for `pub fn`
/// would be a tripwire on ordinary Rust, not a weld.
const READ_DOORS: &[(&str, ReadDoor)] = read_doors![
    face_frame,
    face_carrier_kind,
    edge_frame,
    edge_carrier_kind,
    vertex_position,
];

/// What a corpus-wide sweep for the two body-index rungs found.
struct BodyIndexSweep {
    /// One line per (document, node, name, door) that answered
    /// `NoSuchBody` or `NoBodies`. **Empty is the claim.**
    reached: Vec<String>,
    /// Name-table rows driven, over the whole corpus.
    rows: usize,
    /// Rows whose entity sits on a body index other than the first —
    /// the rows for which `NoSuchBody` is a question `output_body`
    /// actually asks. Zero here would make the zero above vacuous.
    off_first_body: usize,
}

/// **Re-takes the measurement that excludes `NoSuchBody` and
/// `NoBodies`**: every name of every corpus node's name table, through
/// every door in [`READ_DOORS`].
///
/// The exclusion in the row below is a ZERO over a population that
/// grows. A corpus document added tomorrow whose node carries a payload
/// with no bodies and a name table that is not empty produces
/// `NoBodies`; a table row whose body index outruns its payload
/// produces `NoSuchBody`. Neither would move anything an exclusion LIST
/// asserts — that list only says the two identifiers are absent from
/// what was driven, which stays true while the exclusion rots. So the
/// measurement is taken again on every run instead of remembered.
///
/// **The counts are not the claim and are not pinned.** A corpus is
/// meant to grow, and a number here would answer to nothing; what is
/// pinned is that the two rungs stay at zero and that the sweep is not
/// vacuous.
fn sweep_the_corpus_for_body_index_rungs() -> BodyIndexSweep {
    use editor_core::{EntityRef, Entry};

    let mut out = BodyIndexSweep {
        reached: Vec::new(),
        rows: 0,
        off_first_body: 0,
    };
    for doc in crate::corpus::documents() {
        let ev = eval(&doc.doc);
        let nodes: Vec<RecipeNodeId> = ev.nodes.keys().copied().collect();
        for node in nodes {
            let Some(value) = ev.value(node) else {
                continue;
            };
            for (name, entry) in value.name_table.iter() {
                out.rows += 1;
                let refs: &[EntityRef] = match entry {
                    Entry::Unique(e) => core::slice::from_ref(e),
                    Entry::Tied(v) => v,
                };
                if refs.iter().any(|e| e.body != 0) {
                    out.off_first_body += 1;
                }
                for (door, ask) in READ_DOORS {
                    let Some(err) = ask(&ev, node, name) else {
                        continue;
                    };
                    if matches!(
                        err,
                        InterrogateError::NoSuchBody { .. } | InterrogateError::NoBodies { .. }
                    ) {
                        out.reached.push(format!(
                            "{}: {door} on {name:?} at {node:?} answered {err:?}",
                            doc.name
                        ));
                    }
                }
            }
        }
    }
    out
}

/// **Every rung a name→geometry door produces, driven through one, in
/// one process, welded to [`InterrogateError`]'s own roster.**
///
/// The header above states a scope; this row is what makes that scope
/// falsifiable. Each rung below is the value a DOOR handed back — never
/// one this test constructed — and its identifier is read off that
/// value's own `Debug` (`test_utils::f6::variant_identifier`), so a
/// variant renamed in `src/` moves the pattern and the witness together
/// and no string here can be left saying the old name.
///
/// **`face_frame` for the ladder; `edge_frame` for the one rung that
/// carries a second vocabulary.** The five read doors share
/// `interrogate::read`, so `face_frame` walks every rung `value_of`,
/// `entity_of` and `output_body` can raise, and a second door says
/// nothing more about `InterrogateError` itself. It says something
/// about `Readback`: that rung's payload is the DOOR's own readback
/// refusal rather than the ladder's, and the face door and the edge
/// door hand back different ones off the same loft. Both are driven
/// below. `denotation` is the one name→geometry door that does NOT
/// share `read`, and the rows above pin it against rungs of this same
/// ladder.
///
/// **`ReadbackError`'s own vocabulary is out of scope here and
/// unpinned.** ONE of its arms is driven below — `NoCanonicalFrame`,
/// twice, carrying a different carrier word each time, because two
/// values that render differently are what this file exists to keep
/// apart. Its other arms are `topo::readback`'s to account for; nothing
/// here welds them, and nothing here counts them. The weld below covers
/// [`InterrogateError`]'s roster and stops at the `Readback` rung: the
/// rung is welded, the enum inside it is not.
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
/// corpus node's table through every door in [`READ_DOORS`] produces
/// neither rung — and that is not a number remembered from the day it was
/// taken: [`sweep_the_corpus_for_body_index_rungs`] re-takes the
/// measurement on every run, and this row reds the day either rung
/// turns up.
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
    use editor_core::all_bodies;
    use test_utils::f6::variant_identifier;
    use topo::readback::ReadbackError;

    let (doc, good, failed, poisoned) = box_with_a_failed_and_a_poisoned_node();
    let ev = eval(&doc);
    let face = all_faces(&ev, good)[0].clone();
    let edge = all_edges(&ev, good)[0].clone();
    let body = all_bodies(&ev, good)[0].clone();
    let foreign = RecipeNodeId(4242);
    let stranger = a_name_no_face_answers_to(good);

    // The N2 tie, from the fixture that mints one.
    let UCutterTie {
        ev: tie_ev,
        sub,
        tied,
        candidates,
        ..
    } = u_cutter_tied_face("lib_u5_interrogate_ladder");

    let loft = crate::corpus::loft_prism::document();
    let loft_ev = eval(&loft.doc);
    let (face_frameless_at, face_frameless) = a_frameless_carrier(&loft_ev, EntityKind::Face);
    let (edge_frameless_at, edge_frameless) = a_frameless_carrier(&loft_ev, EntityKind::Edge);

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
            face_frame(&loft_ev, face_frameless_at, &face_frameless),
        ),
    ];
    // The same rung through the edge door, carrying the OTHER readback
    // payload: `Readback` wraps a second vocabulary, and a row that
    // pinned one payload would let those two collapse into each other
    // exactly as this file's header says untested rungs do.
    let second_readback_payload = (
        "an edge whose carrier is a NURBS curve",
        InterrogateError::Readback(ReadbackError::NoCanonicalFrame {
            carrier: "nurbs curve",
        }),
        edge_frame(&loft_ev, edge_frameless_at, &edge_frameless),
    );

    let mut witnessed: Vec<String> = Vec::new();
    for (asked, want, got) in driven.iter().chain([&second_readback_payload]) {
        let got = got.as_ref().err().unwrap_or_else(|| {
            panic!("{asked}: the door answered instead of refusing, so this rung was not driven")
        });
        assert_eq!(got, want, "{asked}");
        witnessed.push(variant_identifier(got));
    }

    // The exclusions below rest on a measurement, so the measurement is
    // TAKEN HERE rather than remembered: a corpus that grows a node
    // reaching either rung reds this row instead of leaving it green
    // over a stale exclusion.
    let sweep = sweep_the_corpus_for_body_index_rungs();
    assert!(
        sweep.rows > 0 && sweep.off_first_body > 0,
        "the corpus sweep drove {} rows, {} of them off the first body — a zero in either \
         makes the exclusion below vacuous rather than measured",
        sweep.rows,
        sweep.off_first_body
    );
    assert!(
        sweep.reached.is_empty(),
        "a corpus name now reaches a rung this row declares undriven — drive it here and \
         drop it from the undriven list:\n{}",
        sweep.reached.join("\n")
    );

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
