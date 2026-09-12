//! Corpus document **part_select** — DOCM-2's register payoff: ONE
//! body out of a multi-body value, spelled the way a user spells it.
//!
//! A box split by a plane at mid-height; BOTH halves selected by two
//! `Node::Part`s and unioned back together (a declared rest contact
//! across the two section faces, which is what a union of two
//! resting bodies is); a three-instance linear pattern of the box with
//! its MIDDLE instance selected by a third `Part` and placed by a
//! transform. Each selector is consumed downstream, so the census
//! counts both sub-kinds and every standard row — evaluation at every
//! CI ε row and under `interval`, persistence round-trip (D6.1), the
//! latency table — runs the projection end to end.
//!
//! The box's height is a document parameter (`h`) that the split's
//! target reads, so the Interval-lane rows (`docm2_part_interval`)
//! can widen it and read each Part's body against the half read off
//! the split's own value at that lane.
//!
//! Dyadic mass pin: the two halves unioned are the 2 × 2 × 1 box
//! again — volume 4, area 2·4 + 4·2 = 16. The union of the halves
//! being the box is the document's own statement that a Part IS the
//! half: nothing was moved, re-stamped or lost on the way through.
//!
//! D2 bump: the tool plane's height (mid-DAG — its cone is the plane,
//! the split, both halves and the union; the pattern chain is reused).

use editor_core::{
    BooleanOp, Dimension, DocEdit, DocParam, EntityKind, Expr, Node, ParamName, PartSelect,
    PatternKind, RecipeNodeId, RoleSeg, SlotId, SplitHalf, StableName, UnitSym,
};

use crate::fixture::{ang, desc, len, scl, xy_frame};

use super::{CorpusDoc, MassPin, Recorder};

/// The box's footprint half-extent, meters.
const BOX_HALF: f64 = 1.0;
/// The box's height, meters — the parameter `h`'s nominal.
const BOX_H: f64 = 1.0;
/// The tool plane's height, meters.
const CUT_Z: f64 = 0.5;
/// The pattern's instance count.
const COUNT: i64 = 3;
/// The pattern's pitch along x, meters — clear of the box's 2 m width.
const PITCH: f64 = 3.0;
/// The middle instance's lift, meters.
const LIFT: f64 = 2.0;

/// The document parameter the box's height reads.
pub const H: &str = "h";

/// The split's section face bounding `side`, in the naming
/// vocabulary — the name a Part hands through verbatim.
pub fn section_face(split: RecipeNodeId, side: SplitHalf) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: split,
        path: vec![RoleSeg::SectionFace { side, section: 0 }],
    }
}

/// The part-select corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: ParamName::new(H),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: BOX_H,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: None,
        },
    });

    // ---- the box, [-1, 1]² × [0, h] ----
    let box_plane = r.insert(xy_frame());
    let box_p = r.insert(Node::Profile(desc(
        box_plane,
        vec![vec![
            (-BOX_HALF, -BOX_HALF),
            (BOX_HALF, -BOX_HALF),
            (BOX_HALF, BOX_HALF),
            (-BOX_HALF, BOX_HALF),
        ]],
    )));
    let cube = r.insert(Node::Extrude {
        profile: box_p,
        distance: Expr::param(ParamName::new(H), Dimension::Length),
    });

    // ---- the cut at mid-height, and its two halves as bodies ----
    let tool = r.insert(Node::Datum(editor_core::Datum::Plane {
        origin: [len(0.0), len(0.0), len(CUT_Z)],
        normal: [scl(0.0), scl(0.0), scl(1.0)],
    }));
    let split = r.insert(Node::Split { target: cube, tool });
    let above = r.insert(Node::Part {
        of: split,
        select: PartSelect::SplitHalf(SplitHalf::Above),
    });
    let below = r.insert(Node::Part {
        of: split,
        select: PartSelect::SplitHalf(SplitHalf::Below),
    });
    // The two halves rest on each other across the section — a
    // declared contact, named through the split's own vocabulary
    // because each Part carries the split's names verbatim.
    let rest = r.insert(Node::declare_rest(vec![(
        section_face(split, SplitHalf::Above),
        section_face(split, SplitHalf::Below),
    )]));
    let whole = r.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: above,
        b: below,
        declare: Some(rest),
    });

    // ---- three boxes along x, and the middle one lifted ----
    let pattern = r.insert(Node::Pattern {
        input: cube,
        count: Expr::count(COUNT),
        kind: PatternKind::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: len(PITCH),
        },
    });
    let middle = r.insert(Node::Part {
        of: pattern,
        select: PartSelect::Instance(Expr::count(1)),
    });
    let lifted = r.insert(Node::Transform {
        input: middle,
        translation: [len(0.0), len(0.0), len(LIFT)],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: ang(0.0),
    });
    let _ = lifted;

    CorpusDoc {
        name: "part_select",
        about: "DOCM-2: a split's two halves and a pattern's middle instance as bodies (Node::Part)",
        edits: r.edits,
        doc: r.doc,
        result: Some(whole),
        pin: Some(MassPin {
            volume: (2.0 * BOX_HALF) * (2.0 * BOX_HALF) * BOX_H,
            area: Some(2.0 * (2.0 * BOX_HALF) * (2.0 * BOX_HALF) + 4.0 * (2.0 * BOX_HALF) * BOX_H),
        }),
        bump: DocEdit::SetParam {
            node: tool,
            slot: SlotId::Origin(editor_core::Axis3::Z),
            expr: len(0.25),
        },
        bump_root: tool,
    }
}
