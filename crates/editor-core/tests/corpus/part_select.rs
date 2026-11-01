//! Corpus document **part_select** — DOCM-2's register payoff: ONE
//! body out of a multi-body value, spelled the way a user spells it.
//!
//! A box split by a plane at mid-height; BOTH halves read by port and
//! unioned back together (a rest contact across the two section faces,
//! which is what a union of two resting bodies is); a three-member row
//! of the box, one placement over `k = index(3)` (D10 `[repetition]`),
//! with its MIDDLE member read as `row[1]` and placed in the world
//! lifted. Every standard row — evaluation at
//! every CI ε row and under `interval`, persistence round-trip (D6.1),
//! the latency table — runs both end to end.
//!
//! The box's height is a document parameter (`h`) that the split's
//! target reads, so the Interval-lane rows (`docm2_part_interval`)
//! can widen it and read the member's body against the one read off
//! the row's own value at that lane.
//!
//! Dyadic mass pin: the two halves unioned are the 2 × 2 × 1 box
//! again — volume 4, area 2·4 + 4·2 = 16. The union of the halves
//! being the box is the document's own statement that a read of a port
//! IS the half: nothing was moved, re-stamped or lost on the way
//! through.
//!
//! D2 bump: the tool plane's height, the parameter `cut` (mid-DAG — its
//! cone is the split and the union; the row is reused).

use super::PlacePart;
use editor_core::{
    Coord, Dimension, DocEdit, EntityKind, Formula, FreeVar, Node, RecipeNodeId, RoleSeg,
    SplitHalf, StableName, TargetAxis, UnitSym, VarName,
};

use crate::fixture::{desc, len, start_cap_at};

use super::{CorpusDoc, MassPin, Recorder};

/// The box's footprint half-extent, meters.
const BOX_HALF: f64 = 1.0;
/// The box's height, meters — the parameter `h`'s nominal.
const BOX_H: f64 = 1.0;
/// The tool plane's height, meters.
const CUT_Z: f64 = 0.5;
/// The row's member count.
const COUNT: i64 = 3;
/// The row's pitch along x, meters — clear of the box's 2 m width.
const PITCH: f64 = 3.0;
/// The middle member's lift, meters.
const LIFT: f64 = 2.0;

/// The document parameter the box's height reads.
pub const H: &str = "h";

/// The document parameter the tool plane's height reads.
pub const CUT: &str = "cut";

/// The split's section face, in the naming vocabulary — the name each
/// read of a half carries for its own copy of the section (NAMES N1: the
/// read scopes it, so the two halves' faces are one name).
pub fn section_face(split: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: split,
        path: vec![RoleSeg::SectionFace],
    }
}

/// The part-select corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static(H),
        def: editor_core::VarDecl::Free(FreeVar::Continuous {
            dim: Dimension::Length,
            value: BOX_H,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: None,
        }),
    });
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static(CUT),
        def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, CUT_Z)),
    });

    // ---- the box, [-1, 1]² × [0, h] ----
    let box_p = r.insert(Node::Profile(desc(vec![vec![
        (-BOX_HALF, -BOX_HALF),
        (BOX_HALF, -BOX_HALF),
        (BOX_HALF, BOX_HALF),
        (-BOX_HALF, BOX_HALF),
    ]])));
    let cube = r.insert(Node::Extrude {
        profile: box_p.into(),
        distance: Formula::named(VarName::from_static(H), Dimension::Length),
    });

    // ---- the cut at mid-height, and its two halves as bodies ----
    let split = r.insert(Node::Split {
        target: cube.into(),
        tool: start_cap_at(
            cube,
            Formula::named(VarName::from_static(CUT), Dimension::Length),
        ),
    });
    let half = |r: &Recorder, side: SplitHalf| {
        r.doc
            .output(split, side.port())
            .expect("a split defines a port per half")
    };
    // The two halves rest on each other across the section.
    let whole = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![
            half(&r, SplitHalf::Above).into(),
            half(&r, SplitHalf::Below).into(),
        ]),
    });

    // ---- three boxes along x, and the middle one lifted ----
    let k = r.index("k", Formula::count(COUNT));
    let row = r.copies(
        cube,
        vec![(
            Coord::Slide(TargetAxis::X),
            Formula::mul(crate::fixture::scalar_of(k), len(PITCH)).expect("a length"),
        )],
    );
    r.place_part(whole);
    // The middle member stands `LIFT` up the world's z, its other
    // values the constant zero: a placement with no mate states its
    // motion on the world's axes.
    let lifted = r.insert(crate::fixture::stood(editor_core::BodyRead::indexed(
        row,
        vec![Formula::count(1)],
    )));
    let placed = editor_core::Placed::of(
        &r.doc,
        Vec::new(),
        vec![(
            editor_core::Coord::Slide(editor_core::TargetAxis::Z),
            len(LIFT),
        )],
    );
    r.write(lifted, placed);

    CorpusDoc {
        name: "part_select",
        about: "DOCM-2: a split's two halves read by port, and a row's middle member read as row[1]",
        edits: r.edits,
        doc: r.doc,
        result: Some(whole),
        pin: Some(MassPin {
            volume: (2.0 * BOX_HALF) * (2.0 * BOX_HALF) * BOX_H,
            area: Some(2.0 * (2.0 * BOX_HALF) * (2.0 * BOX_HALF) + 4.0 * (2.0 * BOX_HALF) * BOX_H),
        }),
        bump: DocEdit::SetVarValue {
            var: VarName::from_static(CUT).into(),
            value: editor_core::FreeValue::Continuous(0.25),
        },
        bump_root: split,
    }
}
