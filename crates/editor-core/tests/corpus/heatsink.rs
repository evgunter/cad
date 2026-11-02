//! Corpus document **heat_sink** — the parametric fin strip (#91 C5,
//! the tour's M4 showcase) as a self-contained recipe.
//!
//! The fins are ONE placement whose reads reach the index
//! `k = index(fins)` (D10 `[repetition]`): fin `k` slid
//! `scalar(k)·pitch` along the base's `x`, its foot `FOOT` up; the
//! `fins` Count document parameter is the index's count. A union of the
//! base and that family makes ONE solid, and the family stands in the
//! world beside it too, so the product is the union and every fin, each
//! fin overlapping the union.
//!
//! Vocabulary: Profile, Extrude, Place, Union, `InsertNode`,
//! `DeclareVar` (a Count and an index), `SetParam`.
//!
//! Geometry (dyadic): base `[0,3] × [0,1] × [0,0.25]`; fin footprint
//! `0.1875 × 0.75` sketched at `z = 0.1875` — 1/16 INSIDE the base,
//! because a flush fin base would be an undeclared coincidence — and
//! extruded `0.8125` (to `z = 1`); five fins at `x`-spacing `0.3125`.
//!
//! Exact oracle, derived here: base `3 · 1 · 0.25 = 0.75`; each fin
//! adds only the material above the base, `0.1875 · 0.75 · 0.75 =
//! 0.10546875`; five fins ⇒ `0.75 + 0.52734375` = **1.27734375**.
//!
//! D2 bump: the fin's `Distance` (mid-DAG — its cone is the fin, the
//! family and the union; the base half of the DAG is reused).

use super::PlacePart;
use editor_core::{Coord, Dimension, DocEdit, Formula, FreeVar, Node, SlotId, TargetAxis, VarName};

use crate::fixture::len;

use super::{CorpusDoc, MassPin, Recorder};

/// The fin count the document starts at.
const FINS: i64 = 5;
/// The fin pitch.
const PITCH: f64 = 0.3125;
/// The height of a fin's foot over the base's.
const FOOT: f64 = 0.1875;

/// The heat-sink corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static("fins"),
        def: editor_core::VarDecl::Free(FreeVar::Count { value: FINS }),
    });
    let base_p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (3.0, 0.0), (3.0, 1.0), (0.0, 1.0)]],
    );
    let base = r.insert(Node::Extrude {
        profile: base_p.into(),
        distance: len(0.25),
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
    // One fin per value of the index, its count the parameter.
    let k = r.index(
        "k",
        Formula::named(VarName::from_static("fins"), Dimension::Count),
    );
    let fins = r.copies_in(
        fin,
        base,
        vec![
            (
                Coord::Slide(TargetAxis::X),
                Formula::mul(crate::fixture::scalar_of(k), len(PITCH)).expect("a length"),
            ),
            (Coord::Slide(TargetAxis::Z), len(FOOT)),
        ],
    );
    // Every fin overlaps the base by 1/16, so no union has a
    // coincidence.
    let acc = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![base.into(), fins.into()]),
    });
    r.place_part(acc);
    r.place(fins);

    CorpusDoc {
        name: "heat_sink",
        about: "parametric fin strip: one placement over an index, unioned with its base and placed beside it",
        edits: r.edits,
        doc: r.doc,
        result: Some(acc),
        pin: Some(MassPin {
            volume: 1.27734375,
            area: None, // fin/base seams make the area laborious; volume is the oracle
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

/// **The heat sink's base and `fins` fins, each in the world, unfused**:
/// the base placed at the identity and the fins one placement into the
/// world over `k = index(fins)`, slid as [`document`] slides them, so the
/// product is the base and every fin, each its own solid, every fin
/// overlapping the base by `1/16`. The subject the registry split is
/// measured over (`m4_pr8_latency`), whose size
/// `docm5_subject::the_registry_split_is_measured_at_a_pinned_point`
/// pins.
pub fn placed_fins(fins: i64) -> editor_core::ProfileDoc {
    let mut r = Recorder::new();
    let base_p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (3.0, 0.0), (3.0, 1.0), (0.0, 1.0)]],
    );
    let base = r.insert(Node::Extrude {
        profile: base_p.into(),
        distance: len(0.25),
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
    r.place_part(base);
    let k = r.index("k", Formula::count(fins));
    let values = editor_core::Placed::of(
        &r.doc,
        Vec::new(),
        vec![
            (
                Coord::Slide(TargetAxis::X),
                Formula::mul(crate::fixture::scalar_of(k), len(PITCH)).expect("a length"),
            ),
            (Coord::Slide(TargetAxis::Z), len(FOOT)),
        ],
    )
    .values;
    r.in_world_by(fin, values);
    r.doc
}
