//! **Every document-reachable operand refusal, both halves, byte-exact.**
//!
//! Every typed operand mismatch in `eval::wire` is built by one door
//! (`operand`, or `node_operand` for the road that holds no value) out
//! of two independent parts: the `expected:` phrase the door was asked
//! for, and the `found:` family the input actually carries, which the
//! door reads for itself.
//!
//! The document below is the reviewer of PR 2480's — a 20-miswiring
//! instrument written to RENDER `(expected, found, input)` for every
//! such refusal, so the same file could be compiled unchanged on two
//! trees and diffed. Adopted here with its rows intact and its verdict
//! turned into assertions, because a test that prints is evidence for
//! whoever is reading that day and a gate for nobody
//! (`memories/test-suite-cost.md`).
//!
//! Eight distinct `expected:` phrases over four distinct `found:`
//! families, so two independent things are pinned and a door that lost
//! either goes red:
//!
//! - **`expected:` varies with the caller.** A door that hard-coded any
//!   one phrase fails every row that asks for a different one.
//! - **`found:` varies with the value, under a FIXED `expected:`.** The
//!   `"datum frame"` rows differ only in what was wired in, the
//!   `"datum plane"` rows likewise, and the `"profile"` rows do the
//!   same on the node road — so a door that answered a constant, or the
//!   negation of its own `expected:` (*"not a datum frame"*), fails
//!   while the phrase beside it stays right.
//!
//! **Three rows never reach evaluation**, and that is the finding they
//! carry: the edit door refuses an assertion over a non-measure and a
//! declare reference that is not a `Declare`, so `wire_assertion`'s and
//! `declared_pairs`' kind refusals are defences behind a door rather
//! than sentences a document author can read. They are asserted as
//! edit-door refusals, so a door that stopped refusing them — and
//! started shipping those refusals to users — reds here.
//!
//! These refusals are DOCUMENT-REACHABLE: the strings here are what an
//! author reads. The SOURCE rules behind them (one construction site,
//! no phrase literal at a call site) are guarded separately, by
//! `eval::mod`'s `operand_vocabulary_census`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    AssertionDir, Datum, DocEdit, EvalOptions, Expr, Node, NodeErrorKind, PartSelect, PatternKind,
    ProfileDoc, ProfileProgram, RecipeNodeId, SplitHalf, TubeWindow,
};
use fixture::{ang, desc, insert, len, on_frame_keeping, scl, square};
use geom_core::Tol;

/// What a miswired node owes.
enum Owes {
    /// `WrongOperand`, with these two halves and the wired node as its
    /// `input`.
    Refusal(&'static str, &'static str),
    /// The edit door refuses the node, so no evaluation happens and the
    /// operand door behind it is unreachable from a document.
    EditDoor,
}

/// A miswired node: what it is, what it owes, and the operand the
/// refusal must name.
struct Row {
    what: &'static str,
    owes: Owes,
    input: RecipeNodeId,
    /// `None` when the edit door refused the insert.
    node: Option<RecipeNodeId>,
}

/// One document holding every miswiring, plus the well-formed profile,
/// body and pattern the miswirings borrow.
///
/// One document and one evaluation rather than twenty: nextest is
/// process-per-test, so a test per row would pay twenty document builds
/// for one claim, and every row labels itself well enough to read a
/// failure off the message alone.
fn wired() -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    Vec<Row>,
) {
    let doc = ProfileDoc::empty_derived("wire_operand_door", Tol::witness());
    let (doc, sketch, profile) = on_frame_keeping(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 1.0)],
    );
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let (doc, body2) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(2.0),
        },
    );
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.0)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, axis3) = insert(
        doc,
        Node::Datum(Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: body,
            count: Expr::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(3.0),
            },
        },
    );

    let mut rows: Vec<Row> = Vec::new();
    // Inserts the miswired node, recording whether the EDIT door took
    // it — which is itself a row's answer, not a reason to stop.
    let add = |d: ProfileDoc,
               rows: &mut Vec<Row>,
               what: &'static str,
               owes: Owes,
               node: Node<ProfileProgram>,
               input: RecipeNodeId|
     -> ProfileDoc {
        match d.apply(&DocEdit::InsertNode { node }, Tol::witness()) {
            Ok(applied) => {
                rows.push(Row {
                    what,
                    owes,
                    input,
                    node: applied.record.minted,
                });
                applied.doc
            }
            Err(_) => {
                rows.push(Row {
                    what,
                    owes,
                    input,
                    node: None,
                });
                d
            }
        }
    };

    // ---- the value road: `found:` is the payload's family ----

    let mut doc = add(
        doc,
        &mut rows,
        "body_operand over a plane datum (Shell)",
        Owes::Refusal("body", "datum"),
        Node::Shell {
            target: plane,
            thickness: len(0.1),
            open: vec![],
        },
        plane,
    );
    doc = add(
        doc,
        &mut rows,
        "body_operand over instances (Shell of a pattern)",
        Owes::Refusal("body", "instances"),
        Node::Shell {
            target: pattern,
            thickness: len(0.1),
            open: vec![],
        },
        pattern,
    );
    doc = add(
        doc,
        &mut rows,
        "placeable_operand over a profile (Pattern)",
        Owes::Refusal("body or instances", "profile"),
        Node::Pattern {
            input: profile,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(3.0),
            },
        },
        profile,
    );
    doc = add(
        doc,
        &mut rows,
        "profile_plane_f64 (a profile drawn on a plane datum)",
        Owes::Refusal("datum frame", "datum"),
        Node::Profile(desc(plane, vec![square(0.0, 0.0, 1.0)])),
        plane,
    );
    doc = add(
        doc,
        &mut rows,
        "frame_value (an in-plane axis written against a body)",
        Owes::Refusal("datum frame", "body"),
        fixture::axis_in_plane(body, (0.0, 0.0), (1.0, 0.0)),
        body,
    );
    doc = add(
        doc,
        &mut rows,
        "wire_swept (an Extrude of a plane datum)",
        Owes::Refusal("profile", "datum"),
        Node::Extrude {
            profile: plane,
            distance: len(1.0),
        },
        plane,
    );
    doc = add(
        doc,
        &mut rows,
        "wire_revolve's profile pre-check (a Revolve of a plane datum)",
        Owes::Refusal("profile", "datum"),
        Node::Revolve {
            profile: plane,
            axis: axis3,
            angle: ang(1.0),
        },
        plane,
    );
    doc = add(
        doc,
        &mut rows,
        "wire_revolve's axis (a Revolve about a 3-D axis)",
        Owes::Refusal("an axis in a sketch frame (Datum::AxisInPlane)", "datum"),
        Node::Revolve {
            profile,
            axis: axis3,
            angle: ang(1.0),
        },
        axis3,
    );
    doc = add(
        doc,
        &mut rows,
        "tube_args (a Tube spined on a profile)",
        Owes::Refusal("datum axis", "profile"),
        Node::Tube {
            spine: profile,
            u_ref: [scl(1.0), scl(0.0), scl(0.0)],
            major_radius: len(2.0),
            window: TubeWindow::Full,
            minor_radius: len(0.5),
        },
        profile,
    );
    doc = add(
        doc,
        &mut rows,
        "wire_assertion's measure operand — behind the edit door",
        Owes::EditDoor,
        Node::Assertion {
            measure: plane,
            bound: len(1.0),
            dir: AssertionDir::AtMost,
        },
        plane,
    );
    doc = add(
        doc,
        &mut rows,
        "wire_split's tool (a Split tooled by a profile)",
        Owes::Refusal("datum plane", "profile"),
        Node::Split {
            target: body,
            tool: profile,
        },
        profile,
    );
    doc = add(
        doc,
        &mut rows,
        "wire_split's tool (a Split tooled by a 3-D axis datum)",
        Owes::Refusal("datum plane", "datum"),
        Node::Split {
            target: body,
            tool: axis3,
        },
        axis3,
    );
    doc = add(
        doc,
        &mut rows,
        "wire_part's SplitHalf arm (a half of a plain body)",
        Owes::Refusal("split", "body"),
        Node::Part {
            of: body,
            select: PartSelect::SplitHalf(SplitHalf::Above),
        },
        body,
    );
    doc = add(
        doc,
        &mut rows,
        "wire_part's Instance arm (an instance of a plain body)",
        Owes::Refusal("instances", "body"),
        Node::Part {
            of: body,
            select: PartSelect::Instance(Expr::count(0)),
        },
        body,
    );
    doc = add(
        doc,
        &mut rows,
        "declared_pairs on the union road — behind the edit door",
        Owes::EditDoor,
        Node::Union {
            members: vec![body, body2],
            declare: Some(plane),
        },
        plane,
    );
    doc = add(
        doc,
        &mut rows,
        "declared_pairs on the boolean road — behind the edit door",
        Owes::EditDoor,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: body,
            b: body2,
            declare: Some(plane),
        },
        plane,
    );
    doc = add(
        doc,
        &mut rows,
        "stepped_map's circular axis (a Pattern about a plane datum)",
        Owes::Refusal("datum axis", "datum"),
        Node::Pattern {
            input: body,
            count: Expr::count(3),
            kind: PatternKind::Circular {
                axis: plane,
                step: ang(0.5),
            },
        },
        plane,
    );

    // ---- the node road: `found:` is the NODE's family ----

    doc = add(
        doc,
        &mut rows,
        "section_of (a Loft over a body)",
        Owes::Refusal("profile", "body"),
        Node::Loft {
            profiles: vec![profile, body],
            v_degree: Expr::count(1),
        },
        body,
    );
    doc = add(
        doc,
        &mut rows,
        "section_of (a Loft over a frame datum)",
        Owes::Refusal("profile", "datum"),
        Node::Loft {
            profiles: vec![profile, sketch],
            v_degree: Expr::count(1),
        },
        sketch,
    );
    doc = add(
        doc,
        &mut rows,
        "section_of on the sweep road (a Sweep whose profile is a body)",
        Owes::Refusal("profile", "body"),
        Node::Sweep {
            profile: body,
            path: profile,
            stations: Expr::count(3),
            v_degree: Expr::count(1),
        },
        body,
    );

    (doc, profile, body, pattern, rows)
}

/// Both halves of every refusal, byte-exact, over one evaluation.
#[test]
fn every_operand_refusal_names_the_phrase_asked_for_and_the_family_found() {
    let (doc, profile, body, pattern, rows) = wired();
    let ev = fixture::run(&doc, &EvalOptions::default());
    // The rows are evidence about the door only if the well-formed
    // nodes they borrow actually evaluated: a poisoned body, profile or
    // pattern would refuse for a reason that is not this test's.
    for (name, id) in [("profile", profile), ("body", body), ("pattern", pattern)] {
        assert!(
            ev.node_error(id).is_none(),
            "the {name} fixture must evaluate: {:?}",
            ev.node_error(id)
        );
    }
    let mut phrases: Vec<&'static str> = Vec::new();
    let mut families: Vec<&'static str> = Vec::new();
    for row in &rows {
        match (&row.owes, row.node) {
            (Owes::EditDoor, node) => assert!(
                node.is_none(),
                "{}: the edit door took a node it used to refuse — the operand door behind it \
                 is now document-reachable and owes its refusal a row here",
                row.what
            ),
            (Owes::Refusal(expected, found), None) => panic!(
                "{}: the edit door refused the insert, so nothing reaches the door that owes \
                 ({expected:?}, {found:?})",
                row.what
            ),
            (Owes::Refusal(expected, found), Some(node)) => {
                let got = match ev.node_error(node).map(|e| &e.kind) {
                    Some(NodeErrorKind::WrongOperand {
                        input,
                        expected,
                        found,
                    }) => (*expected, *found, *input),
                    other => panic!("{}: not a WrongOperand: {other:?}", row.what),
                };
                assert_eq!(got, (*expected, *found, row.input), "{}", row.what);
                phrases.push(expected);
                families.push(found);
            }
        }
    }
    // A census that read nothing would pass vacuously, and one that
    // reached a single phrase or a single family would pin neither half
    // against a door that answers a constant.
    assert_eq!(phrases.len(), 17, "the document-reachable rows");
    phrases.sort_unstable();
    phrases.dedup();
    families.sort_unstable();
    families.dedup();
    assert!(
        phrases.len() >= 8 && families.len() >= 4,
        "the rows must vary both halves: {} phrases over {} families",
        phrases.len(),
        families.len()
    );
}
