//! **The operand door's two halves, pinned together.**
//!
//! Every typed operand mismatch in `eval::wire` is built by one door
//! (`operand`, or `node_operand` for the road that holds no value) out
//! of two independent parts: the `expected:` phrase the door was asked
//! for, and the `found:` family the input actually carries, which the
//! door reads for itself.
//!
//! The rows below drive one document through six miswirings — four
//! distinct `expected:` phrases, three distinct `found:` families — and
//! assert both halves BYTE-EXACT. Two independent things follow, and a
//! door that lost either goes red here:
//!
//! - **`expected:` varies with the caller.** A door that hard-coded any
//!   one phrase fails every row that asks for a different one.
//! - **`found:` varies with the value, under a FIXED `expected:`.** The
//!   two `"datum frame"` rows differ only in what was wired in, and the
//!   two `"profile"` rows do the same on the node road — so a door that
//!   answered a constant, or the negation of its own `expected:`
//!   (*"not a datum frame"*), fails while the phrase beside it stays
//!   right.
//!
//! These refusals are DOCUMENT-REACHABLE: the strings here are what an
//! author reads.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Datum, EvalOptions, Expr, Node, NodeErrorKind, ProfileDoc, RecipeNodeId, TubeWindow,
};
use fixture::{ang, desc, insert, len, on_frame_keeping, scl, square};
use geom_core::Tol;

/// A miswired node and the refusal it owes.
struct Row {
    what: &'static str,
    node: RecipeNodeId,
    expected: &'static str,
    found: &'static str,
    /// The operand the refusal must name — the node wired IN, not the
    /// node that refused.
    input: RecipeNodeId,
}

/// One document holding every miswiring, plus the well-formed profile
/// and body the miswirings borrow.
///
/// One document and one evaluation rather than six: nextest is
/// process-per-test, so six tests would pay six document builds for one
/// claim (`memories/test-suite-cost.md`), and every row labels itself
/// well enough to read a failure off the message alone.
fn wired() -> (ProfileDoc, RecipeNodeId, RecipeNodeId, Vec<Row>) {
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
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.0)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (mut doc, axis3) = insert(
        doc,
        Node::Datum(Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );

    let mut rows = Vec::new();

    // ---- the value road: `found:` is the payload's family ----

    // A profile drawn on a PLANE datum. The frame door refuses at the
    // profile's own evaluation, naming the plane it was pointed at.
    let (d, on_a_plane) = insert(doc, Node::Profile(desc(plane, vec![square(0.0, 0.0, 1.0)])));
    doc = d;
    rows.push(Row {
        what: "a profile drawn on a plane datum",
        node: on_a_plane,
        expected: "datum frame",
        found: "datum",
        input: plane,
    });

    // An in-plane axis whose frame is a BODY: the same phrase, a
    // different family, through the frame value's second reader.
    let (d, axis_on_a_body) = insert(doc, fixture::axis_in_plane(body, (0.0, 0.0), (1.0, 0.0)));
    doc = d;
    rows.push(Row {
        what: "an in-plane axis written against a body",
        node: axis_on_a_body,
        expected: "datum frame",
        found: "body",
        input: body,
    });

    // A tube spined on a PROFILE: a phrase narrower than the family,
    // over a third `found:` word.
    let (d, tube) = insert(
        doc,
        Node::Tube {
            spine: profile,
            u_ref: [scl(1.0), scl(0.0), scl(0.0)],
            major_radius: len(2.0),
            window: TubeWindow::Full,
            minor_radius: len(0.5),
        },
    );
    doc = d;
    rows.push(Row {
        what: "a tube spined on a profile",
        node: tube,
        expected: "datum axis",
        found: "profile",
        input: profile,
    });

    // A revolve about a 3-D axis: the whole-sentence phrase.
    let (d, revolve) = insert(
        doc,
        Node::Revolve {
            profile,
            axis: axis3,
            angle: ang(1.0),
        },
    );
    doc = d;
    rows.push(Row {
        what: "a revolve about a 3-D axis datum",
        node: revolve,
        expected: "an axis in a sketch frame (Datum::AxisInPlane)",
        found: "datum",
        input: axis3,
    });

    // ---- the node road: `found:` is the NODE's family ----

    // A loft whose second section is a body, and one whose second
    // section is a frame datum: one phrase, two families, read off the
    // recipe rather than off a value.
    for (what, section, found) in [
        ("a loft over a body", body, "body"),
        ("a loft over a frame datum", sketch, "datum"),
    ] {
        let (d, loft) = insert(
            doc,
            Node::Loft {
                profiles: vec![profile, section],
                v_degree: Expr::count(1),
            },
        );
        doc = d;
        rows.push(Row {
            what,
            node: loft,
            expected: "profile",
            found,
            input: section,
        });
    }

    (doc, profile, body, rows)
}

/// Both halves of every refusal, byte-exact, over one evaluation.
#[test]
fn every_operand_refusal_names_the_phrase_asked_for_and_the_family_found() {
    let (doc, profile, body, rows) = wired();
    let ev = fixture::run(&doc, &EvalOptions::default());
    // The rows are evidence about the door only if the well-formed
    // nodes they borrow actually evaluated: a poisoned body or profile
    // would refuse for a reason that is not this test's.
    for (name, id) in [("profile", profile), ("body", body)] {
        assert!(
            ev.node_error(id).is_none(),
            "the {name} fixture must evaluate: {:?}",
            ev.node_error(id)
        );
    }
    for row in &rows {
        let got = match ev.node_error(row.node).map(|e| &e.kind) {
            Some(NodeErrorKind::WrongOperand {
                input,
                expected,
                found,
            }) => (*expected, *found, *input),
            other => panic!("{}: not a WrongOperand: {other:?}", row.what),
        };
        assert_eq!(got, (row.expected, row.found, row.input), "{}", row.what);
    }
}
