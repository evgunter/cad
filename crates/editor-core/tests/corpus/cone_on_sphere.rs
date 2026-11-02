//! Corpus document **cone_on_sphere** — Ev's cone on a sphere, as
//! numbers.
//!
//! A sphere of radius `5/4`, a half disc revolved a full turn about its
//! own `y`; a cone, the triangle with its apex on the axis at
//! `y = −25/16` and its base the chord `y = −1`, `ρ ≤ 3/4`, revolved
//! the same way. The point `(3/4, −1)` is on the sphere, and the
//! generator from the apex through it is perpendicular to its radius
//! there (`3/4 · 3/4 + (−1)(−1 + 25/16) = 0`), so the cone's wall is
//! tangent to the sphere along its base rim: but only because those
//! four numbers were typed so. The cone's base disc is a chord of the
//! sphere and lies inside it. The cone, placed where it stands in the
//! sphere's space, is unioned with the sphere.
//!
//! Every number is dyadic. No mass pin: both volumes are `π`-valued.

use editor_core::{DocEdit, Formula, Node, ProgramArcData, ProgramStep, ProgramTarget, SlotId};

use crate::fixture::{len2, scl};

use super::{CorpusDoc, PlacePart, Recorder, placed_in, revolved};

/// The sphere's radius, meters.
pub const SPHERE_R: f64 = 1.25;
/// The cone's apex on the axis, meters.
pub const APEX: f64 = -1.5625;
/// The tangent rim: its radius and height, meters.
pub const RIM: (f64, f64) = (0.75, -1.0);

/// The cone-on-sphere corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    let to = |x, y| ProgramTarget::Point(len2([x, y]));
    let sphere = revolved(
        &mut r,
        vec![
            ProgramStep::At(len2([0.0, -SPHERE_R])),
            ProgramStep::ArcTo(ProgramArcData::Bulge {
                target: to(0.0, SPHERE_R),
                b: scl(1.0),
            }),
            ProgramStep::LineTo(ProgramTarget::Start),
        ],
    );
    let cone = revolved(
        &mut r,
        vec![
            ProgramStep::At(len2([0.0, APEX])),
            ProgramStep::LineTo(to(RIM.0, RIM.1)),
            ProgramStep::LineTo(to(0.0, RIM.1)),
            ProgramStep::LineTo(ProgramTarget::Start),
        ],
    );
    // The cone stands where it is in the sphere's space: its numbers
    // are the sphere's coordinates.
    let placed = placed_in(&mut r, &sphere.body(), cone.body(), editor_core::stand());
    let whole = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![
            editor_core::BodyRead::plain(sphere.body()),
            placed.into(),
        ]),
    });
    r.place_part(whole);

    CorpusDoc {
        name: "cone_on_sphere",
        about: "a revolved cone whose generator is typed tangent to a sphere, unioned with it",
        edits: r.edits,
        doc: r.doc,
        result: Some(whole),
        pin: None, // revolved: π-valued
        // The cone turns its full turn again: the sphere is reused.
        bump: DocEdit::SetParam {
            node: cone.revolve,
            slot: SlotId::RevolveAngle,
            value: Formula::turn_over(1).expect("a full turn").into(),
            fresh: Vec::new(),
        },
        bump_root: cone.revolve,
    }
}
