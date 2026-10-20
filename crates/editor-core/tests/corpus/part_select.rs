//! Corpus document **part_select** — DOCM-2's register payoff: ONE
//! body out of a multi-body value, spelled the way a user spells it.
//!
//! A box split by a plane at mid-height; BOTH halves read by port and
//! unioned back together (a rest contact across the two section faces,
//! which is what a union of two resting bodies is); a three-instance
//! linear pattern of the box with its MIDDLE instance selected by a
//! `Part` and placed in the world lifted. Every standard row — evaluation at
//! every CI ε row and under `interval`, persistence round-trip (D6.1),
//! the latency table — runs both end to end.
//!
//! The box's height is a document parameter (`h`) that the split's
//! target reads, so the Interval-lane rows (`docm2_part_interval`)
//! can widen it and read the Part's body against the instance read
//! off the pattern's own value at that lane.
//!
//! Dyadic mass pin: the two halves unioned are the 2 × 2 × 1 box
//! again — volume 4, area 2·4 + 4·2 = 16. The union of the halves
//! being the box is the document's own statement that a read of a port
//! IS the half: nothing was moved, re-stamped or lost on the way
//! through.
//!
//! D2 bump: the tool plane's height, the parameter `cut` (mid-DAG — its
//! cone is the split and the union; the pattern chain is reused).

use super::PlacePart;
use editor_core::{
    Dimension, DocEdit, EntityKind, Formula, FreeVar, Node, PartSelect, PatternKind, RecipeNodeId,
    RoleSeg, SplitHalf, StableName, UnitSym, VarName,
};

use crate::fixture::{desc, len, scl, start_cap_at};

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

/// The document parameter the tool plane's height reads.
pub const CUT: &str = "cut";

/// The split's section face, in the naming vocabulary — the name each
/// read of a half carries for its own copy of the section (NAMES N1: the
/// read scopes it, so the two halves' faces are one name).
pub fn section_face(split: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: split,
        path: vec![RoleSeg::SectionFace { section: 0 }],
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
    let pattern = r.insert(Node::Pattern {
        input: cube.into(),
        count: Formula::count(COUNT),
        kind: PatternKind::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: len(PITCH),
        },
    });
    let middle = r.insert(Node::Part {
        of: pattern.into(),
        select: PartSelect::Instance(Formula::count(1)),
    });
    r.place_part(whole);
    // The middle copy stands on its instance's start cap, `LIFT` up the
    // world's z, the three values zero. Its end cap, the face the
    // height moves, would carry that width into the placement at
    // `Interval` (`docm2_part_interval::a7`). `stand` cannot see that an
    // instance's start cap reads only the pattern's pitch, so it is
    // written here.
    let base = {
        let ev = r.evaluated();
        crate::fixture::table(ev, middle)
            .iter()
            .map(|(name, _)| name)
            .find(|name| {
                matches!(name.path.as_slice(), [RoleSeg::Instance { of, .. }]
                    if of.path == [RoleSeg::Cap(editor_core::CapEnd::Start)])
            })
            .cloned()
            .expect("the middle copy carries its instance's start cap")
    };
    let lifted = r.insert(Node::place(vec![editor_core::BodyRead::plain(
        editor_core::Operand::from(middle),
    )]));
    r.constrain(
        lifted,
        editor_core::Mate {
            on: editor_core::Operand::Pose(Box::new(editor_core::pose::PoseDef::Flip {
                pose: editor_core::pose::RoleRead::plain(crate::fixture::plane(
                    editor_core::Operand::select(middle, vec![base]),
                )),
            })),
            to: editor_core::Target::World(
                editor_core::WorldRead::through(editor_core::pose::PoseRole::Xy).slide(
                    editor_core::WorldRead::through(editor_core::pose::PoseRole::Z),
                    len(LIFT),
                ),
            ),
            class: editor_core::ContactClass::Rest,
        },
        vec![
            (
                editor_core::Coord::Slide(editor_core::TargetAxis::X),
                len(0.0),
            ),
            (
                editor_core::Coord::Slide(editor_core::TargetAxis::Y),
                len(0.0),
            ),
            (editor_core::Coord::Spin, crate::fixture::ang(0.0)),
        ],
    );

    CorpusDoc {
        name: "part_select",
        about: "DOCM-2: a split's two halves read by port, and a pattern's middle instance (Node::Part)",
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
