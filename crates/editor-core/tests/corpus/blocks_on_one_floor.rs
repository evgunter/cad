//! Corpus document **blocks_on_one_floor** — two blocks made coplanar
//! by resting each on one floor.
//!
//! A floor, the square `[−1, 3] × [−1, 2]` extruded `1/2`; two unit
//! cubes, each placed into the floor's space by one rest, its base on
//! the floor's top, and the slides and spin the rest leaves written: the
//! first at the origin, the second one unit along `x`, so the two abut
//! face to face. The two copies are unioned, and the floor and the union
//! are placed in the world, the union standing on the floor.
//!
//! Each mate equates a block with the floor, never the blocks with each
//! other, so the union's flush pair (the two bases, and the two tops)
//! is coplanar through the floor: the equivalence the two mates
//! generate. Whether the door proves it is what this document measures.
//!
//! Dyadic mass pin: the floor and the two cubes, which overlap nothing.

use editor_core::{
    CapEnd, Coord, Dimension, DocEdit, Formula, Node, Operand, RoleSeg, SlotId, TargetAxis,
};

use crate::fixture::{desc, fname, len, rest};

use super::{CorpusDoc, MassPin, PlacePart, Recorder};

/// The floor's footprint, meters.
pub const FLOOR: ((f64, f64), (f64, f64)) = ((-1.0, 3.0), (-1.0, 2.0));
/// The floor's thickness, meters.
pub const FLOOR_H: f64 = 0.5;

/// A unit cube on its own `xy`.
fn cube(r: &mut Recorder) -> editor_core::RecipeNodeId {
    let p = r.insert(Node::Profile(desc(vec![vec![
        (0.0, 0.0),
        (1.0, 0.0),
        (1.0, 1.0),
        (0.0, 1.0),
    ]])));
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
    })
}

/// The blocks-on-one-floor corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    let ((x0, x1), (y0, y1)) = FLOOR;
    let floor_p = r.insert(Node::Profile(desc(vec![vec![
        (x0, y0),
        (x1, y0),
        (x1, y1),
        (x0, y1),
    ]])));
    let floor = r.insert(Node::Extrude {
        profile: floor_p.into(),
        distance: len(FLOOR_H),
    });
    let top = || Operand::select(floor, vec![fname(floor, RoleSeg::Cap(CapEnd::End))]);

    let mut copies = Vec::new();
    for at in [0.0, 1.0] {
        let block = cube(&mut r);
        let base = Operand::select(block, vec![fname(block, RoleSeg::Cap(CapEnd::Start))]);
        let placed = r.insert(crate::fixture::stood(editor_core::BodyRead::plain(
            crate::fixture::body_read(&r.doc, block.into()),
        )));
        r.constrain(
            placed,
            rest(base, top()),
            vec![
                (Coord::Slide(TargetAxis::X), len(at)),
                (
                    Coord::Slide(TargetAxis::Y),
                    Formula::zero(Dimension::Length),
                ),
                (Coord::SPIN, Formula::zero(Dimension::Angle)),
            ],
        );
        copies.push(placed);
    }
    let both = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(copies.iter().map(|&c| c.into()).collect()),
    });
    r.place_part(floor);
    r.place_part(both);

    CorpusDoc {
        name: "blocks_on_one_floor",
        about: "two cubes each resting on one floor by a mate, abutting, unioned",
        edits: r.edits,
        doc: r.doc,
        result: Some(both),
        // The two cubes, side by side.
        pin: Some(MassPin {
            volume: 2.0,
            area: Some(10.0),
        }),
        // The floor thickens: both rests and the union recompute; the
        // cubes' profiles and extrudes are reused.
        bump: DocEdit::SetParam {
            node: floor,
            slot: SlotId::Distance,
            value: len(0.75).into(),
            fresh: Vec::new(),
        },
        bump_root: floor,
    }
}
