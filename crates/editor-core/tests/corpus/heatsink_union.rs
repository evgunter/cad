//! Corpus document **heat_sink_fins** — the register payoff: F4's
//! out-of-document fin union moves INTO the document.
//!
//! `heat_sink` carries the gap in its own body: a `Pattern` node whose
//! `Instances` payload no `Boolean` can consume, and BESIDE it the
//! explicit Transform + Union chain that assembles the fins. The demo
//! tour says the same thing in demo code, "honestly outside the
//! document", because `body_operand` refuses `Instances`.
//!
//! `PlacedUnion(fin, Linear{..})` is the fin group said once: ONE node,
//! ONE body out, the count still riding the `fins` document parameter
//! through `SetStructuralParam`. The value is an ordinary `Body`, so
//! every downstream door consumes it with no new arms — `die_tool`
//! feeds its group straight into a `Boolean` in this same corpus.
//!
//! # What this document does NOT carry, and why
//!
//! The heat sink's BASE is not unioned here — this fixture's subject is
//! the GROUP node, and it stops there.
//!
//! It is not stopping at a wall. This finding used to read that fusing
//! the group into a base "needs a kernel door that does not exist",
//! because `combine` takes two SINGLE-SOLID operands; that described
//! the pre-#571 lowering, whose output was N solids. The adjudicated
//! lowering grafts onto existing solids, so the value is ONE solid of N
//! shells — the representation the pairwise chain it replaces produced,
//! and the one the seamed boolean path accepts (`topo::instance`'s door
//! docs). `die_tool` in this same corpus feeds its group straight into
//! a `Boolean`, and the demo tour's heat sink now unions one into a
//! base.
//!
//! Vocabulary: Profile, Extrude, PlacedUnion (Linear), `InsertNode`,
//! `SetDocParam`, `SetStructuralParam`, `SetParam`.
//!
//! Geometry is `heat_sink`'s fin, constant for constant: footprint
//! `0.1875 × 0.75` at `z = 0.1875`, extruded `0.8125`, five of them at
//! pitch `0.3125` — which leaves `0.3125 − 0.1875 = 0.125` of clear air
//! between neighbours, the clearance the disjointness certificate needs.
//!
//! Exact oracles, derived here (both dyadic): each fin is
//! `0.1875 · 0.75 · 0.8125 = 0.1142578125`, so five are
//! **0.5712890625**; each fin's area is `2 · 0.140625` of cap plus
//! `2 · (0.1875 + 0.75) · 0.8125 = 1.5234375` of wall = `1.8046875`,
//! so five are **9.0234375**.
//!
//! D2 bump: the fin master's `Distance` — its cone is the master
//! extrude and the group, which is the memoized-recompute claim
//! `lib_placedunion.rs` pins.

use editor_core::{Dimension, DocEdit, DocParam, Expr, Node, ParamName, PatternKind, SlotId};

use super::super::fixture::{desc, len, scl};
use super::{CorpusDoc, MassPin, Recorder};

/// The fin count the document starts at (`heat_sink`'s).
const FINS: i64 = 5;
/// The fin pitch (`heat_sink`'s).
const PITCH: f64 = 0.3125;

/// The grouped fin corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: ParamName::new("fins"),
        value: DocParam::Count { value: FINS },
    });
    let fin_p = r.insert(Node::Profile(desc(
        [0.0, 0.0, 0.1875],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.25, 0.125),
            (0.4375, 0.125),
            (0.4375, 0.875),
            (0.25, 0.875),
        ]],
    )));
    let fin = r.insert(Node::Extrude {
        profile: fin_p,
        distance: len(0.8125),
    });
    // The whole fin group, in ONE node.
    let fins = r.insert(
        Node::placed_union(
            fin,
            Expr::count(FINS),
            PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(PITCH),
            },
        )
        .expect("a stepped rule takes a count"),
    );
    // …driven by the document's Count parameter, exactly as the
    // ungrouped twin's pattern is.
    r.push(DocEdit::SetStructuralParam {
        node: fins,
        slot: SlotId::Count,
        expr: Expr::param(ParamName::new("fins"), Dimension::Count),
    });
    CorpusDoc {
        name: "heat_sink_fins",
        about: "the parametric fin group as ONE node: PlacedUnion(fin, Linear) driven by the fins param",
        edits: r.edits,
        doc: r.doc,
        result: Some(fins),
        pin: Some(MassPin {
            volume: 0.5712890625,
            area: Some(9.0234375),
        }),
        bump: DocEdit::SetParam {
            node: fin,
            slot: SlotId::Distance,
            expr: Expr::literal(0.6875, Dimension::Length).expect("dyadic length literal"),
        },
        bump_root: fin,
    }
}
