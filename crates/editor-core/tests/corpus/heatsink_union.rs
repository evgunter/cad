//! Corpus document **heat_sink_fins** — the fin group fused in the
//! document: one placement over the index `k = index(fins)` and a
//! union reading its family whole (D10 `[repetition]`). Its members
//! are rigid images of one body, which the reads show, so the union's
//! separation certificate answers for every pair at once.
//!
//! The heat sink's BASE is not unioned here — this fixture's subject is
//! the group, and it stops there; `heat_sink` carries the base.
//!
//! Vocabulary: Profile, Extrude, Place, Union, `InsertNode`,
//! `DeclareVar` (a Count and an index), `SetParam`.
//!
//! Geometry is `heat_sink`'s fin, constant for constant: footprint
//! `0.1875 × 0.75` at `z = 0.1875`, extruded `0.8125`, five of them at
//! pitch `0.3125` — which leaves `0.3125 − 0.1875 = 0.125` of clear air
//! between neighbours.
//!
//! Exact oracles, derived here (both dyadic): each fin is
//! `0.1875 · 0.75 · 0.8125 = 0.1142578125`, so five are
//! **0.5712890625**; each fin's area is `2 · 0.140625` of cap plus
//! `2 · (0.1875 + 0.75) · 0.8125 = 1.5234375` of wall = `1.8046875`,
//! so five are **9.0234375**.
//!
//! D2 bump: the fin's `Distance` — its cone is the fin, the family and
//! the union.

use editor_core::{Coord, Dimension, DocEdit, Formula, FreeVar, Node, SlotId, TargetAxis, VarName};

use crate::fixture::len;

use super::{CorpusDoc, MassPin, Recorder};

/// The fin count the document starts at (`heat_sink`'s).
const FINS: i64 = 5;
/// The fin pitch (`heat_sink`'s).
const PITCH: f64 = 0.3125;

/// The grouped fin corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static("fins"),
        def: editor_core::VarDecl::Free(FreeVar::Count { value: FINS }),
    });
    let fin_p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.25, 0.125),
            (0.4375, 0.125),
            (0.4375, 0.875),
            (0.25, 0.875),
        ]],
    );
    let fin = r.insert(Node::Extrude {
        profile: fin_p.into(),
        distance: len(0.8125),
    });
    // One fin per value of the index, its count the parameter, and
    // their union.
    let k = r.index(
        "k",
        Formula::named(VarName::from_static("fins"), Dimension::Count),
    );
    let row = r.copies(
        fin,
        vec![(
            Coord::Slide(TargetAxis::X),
            Formula::mul(crate::fixture::scalar_of(k), len(PITCH)).expect("a length"),
        )],
    );
    let fins = r.insert(Node::Union {
        members: editor_core::Bodies::Family(row.into()),
    });
    // In the world with the fins' feet 0.1875 up, where `heat_sink`'s
    // stand.
    r.in_world(fins, &editor_core::Frame::translation([0.0, 0.0, 0.1875]));

    CorpusDoc {
        name: "heat_sink_fins",
        about: "the parametric fin group fused: a union of one family over the fins index",
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
            value: len(0.6875).into(),
            fresh: Vec::new(),
        },
        bump_root: fin,
    }
}
