//! Corpus document **shaft_and_bearing** — two copies made coaxial by
//! reading one `Axis` twice.
//!
//! The housing is a ring `ρ ∈ [1/2, 1]`, `y ∈ [0, 1]` revolved a full turn
//! about its own `y`. A shaft (a rectangle on the axis, revolved) and a
//! collar (a ring `ρ ∈ [1/2, 3/4]` of height `1/2`, revolved) are each
//! placed into the housing's space by one mate, its own revolve's `axis`
//! output on the housing's and its slide and spin about that axis written;
//! then all three are unioned. Every full revolve starts its seam at its
//! own `+x`, so the shaft is spun a quarter turn, which keeps its seams off
//! the housing's and the collar's. The shaft's radius is typed equal to the
//! bore's and to the collar's, so the union glues the shaft's wall to the
//! housing's bore and to the collar's, which stands on the shaft below the
//! housing.
//!
//! The scene stands beside the corpus registry
//! ([`super::stage6_beside_the_registry`]): its union is green at `f64`
//! and escalates at `Interval`, which `coincidence_door.rs` pins beside
//! its rows. So do two more, each the same three parts with one changed,
//! whose unions refuse at `f64`; `coincidence_door.rs` pins each
//! refusal, the place a coaxial classification needs the boolean to
//! reach:
//!
//! - [`rounded_mouth`]: the mouth of the bore rounded by a profile
//!   fillet, so the bore opens through a torus, and the shaft wider than
//!   the bore and narrower than its mouth, its wall crossing the torus
//!   on a circle (the site `CoaxialCylinderTorus`);
//! - [`with_a_ball`]: the collar a ball, its centre on the shaft's axis
//!   (the site `CoaxialSphere`), turned so no edge of the ball
//!   crosses the shaft's wall.
//!
//! No mass pin: every body is revolved, so its volume is `π`-valued.

use editor_core::{
    Coord, Dimension, DocEdit, Formula, Node, ProgramStep, ProgramTarget, RecipeNodeId, SlotId,
    TargetAxis,
};

use crate::fixture::{len, len2, scl};

use super::{CorpusDoc, PlacePart, Recorder, Revolved, placed_in, revolved};

/// The bore's radius, meters, and the shaft's and the collar's bore.
pub const BORE_R: f64 = 0.5;
/// The housing's outer radius, meters.
pub const HOUSING_R: f64 = 1.0;
/// The housing's height, meters.
pub const HOUSING_H: f64 = 1.0;
/// The shaft's ends along the axis, meters.
pub const SHAFT_Y: (f64, f64) = (-2.0, 1.5);
/// The collar's outer radius, meters.
pub const COLLAR_R: f64 = 0.75;
/// The collar's height, meters.
pub const COLLAR_H: f64 = 0.5;
/// The collar's (or the ball's) slide along the housing's axis, meters.
pub const BELOW: f64 = -1.0;
/// The rounded mouth's fillet radius, meters.
pub const MOUTH_R: f64 = 0.125;
/// The rounded-mouth shaft's radius, meters: between the bore's and
/// the mouth's.
pub const WIDE_R: f64 = 0.5625;
/// The ball's radius, meters.
pub const BALL_R: f64 = 0.75;

/// What stands on the shaft below the housing.
#[derive(Clone, Copy)]
enum Below {
    /// A collar pressed on the shaft, mated to the housing's axis.
    Collar,
    /// A ball, its centre on the axis, placed by values.
    Ball,
}

/// A rectangle `ρ ∈ [inner, outer]`, `y ∈ [lo, hi]`, as a meridian.
fn rectangle(inner: f64, outer: f64, (lo, hi): (f64, f64)) -> Vec<ProgramStep<Formula>> {
    let to = |x, y| ProgramTarget::Point(len2([x, y]));
    vec![
        ProgramStep::At(len2([inner, lo])),
        ProgramStep::LineTo(to(outer, lo)),
        ProgramStep::LineTo(to(outer, hi)),
        ProgramStep::LineTo(to(inner, hi)),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]
}

/// **A copy of `body` on `housing`'s axis**: its own revolve's axis
/// output equated with the housing's, slid `by` along it and spun
/// `spin` about it.
fn on_the_axis(
    r: &mut Recorder,
    housing: Revolved,
    body: Revolved,
    (by, spin): (Formula, Formula),
) -> RecipeNodeId {
    let placed = editor_core::Placed {
        mates: vec![crate::fixture::mate(body.axis(), housing.axis())],
        values: vec![(Coord::Slide(TargetAxis::Y), by), (Coord::SPIN, spin)],
    };
    placed_in(r, &housing.body(), body.body(), placed)
}

/// The shaft-and-bearing corpus document.
pub fn document() -> CorpusDoc {
    scene(
        (
            "shaft_and_bearing",
            "a shaft and a collar each mated to a housing revolve's axis output, unioned",
        ),
        rectangle(BORE_R, HOUSING_R, (0.0, HOUSING_H)),
        BORE_R,
        Below::Collar,
    )
}

/// **The scene with the bore's mouth rounded** (module docs): beside
/// the registry, its union refused.
pub fn rounded_mouth() -> CorpusDoc {
    scene(
        (
            "shaft_and_bearing_rounded",
            "shaft_and_bearing with the bore's mouth a torus the shaft crosses",
        ),
        vec![
            ProgramStep::At(len2([HOUSING_R, 0.0])),
            ProgramStep::LineTo(ProgramTarget::Point(len2([HOUSING_R, HOUSING_H]))),
            ProgramStep::Toward {
                dx: scl(-1.0),
                dy: scl(0.0),
            },
            ProgramStep::Fillet(len(MOUTH_R)),
            ProgramStep::Toward {
                dx: scl(0.0),
                dy: scl(-1.0),
            },
            ProgramStep::FarEndTo(len2([BORE_R, 0.0])),
            ProgramStep::LineTo(ProgramTarget::Start),
        ],
        WIDE_R,
        Below::Collar,
    )
}

/// **The scene with a ball for the collar** (module docs): beside the
/// registry, its union refused.
pub fn with_a_ball() -> CorpusDoc {
    scene(
        (
            "shaft_and_bearing_ball",
            "shaft_and_bearing with a ball on the shaft, its centre on the axis",
        ),
        rectangle(BORE_R, HOUSING_R, (0.0, HOUSING_H)),
        BORE_R,
        Below::Ball,
    )
}

/// The housing whose meridian is `housing`, a shaft of radius `shaft_r`
/// and what stands on it `below`, unioned.
fn scene(
    (name, about): (&'static str, &'static str),
    housing: Vec<ProgramStep<Formula>>,
    shaft_r: f64,
    below: Below,
) -> CorpusDoc {
    let mut r = Recorder::new();
    let housing = revolved(&mut r, housing);
    let shaft = revolved(&mut r, rectangle(0.0, shaft_r, SHAFT_Y));
    let quarter = Formula::turn_over(4).expect("a quarter turn");
    let shaft_copy = on_the_axis(
        &mut r,
        housing,
        shaft,
        (Formula::zero(Dimension::Length), quarter),
    );
    let (third, third_copy) = match below {
        Below::Collar => {
            let collar = revolved(&mut r, rectangle(shaft_r, COLLAR_R, (0.0, COLLAR_H)));
            let below = (len(BELOW), Formula::zero(Dimension::Angle));
            (collar, on_the_axis(&mut r, housing, collar, below))
        }
        Below::Ball => {
            let ball = revolved(
                &mut r,
                vec![
                    ProgramStep::At(len2([0.0, -BALL_R])),
                    ProgramStep::ArcTo(editor_core::ProgramArcData::Bulge {
                        target: ProgramTarget::Point(len2([0.0, BALL_R])),
                        b: scl(1.0),
                    }),
                    ProgramStep::LineTo(ProgramTarget::Start),
                ],
            );
            // Its poles across the axis and its seam circle square to
            // it: its own `x`, `y`, `z` along the housing's `z`, `x`,
            // `y`, a quarter turn back about `x`, then about `y`.
            let back =
                || Formula::neg(Formula::turn_over(4).expect("a quarter turn")).expect("negates");
            let zero = || Formula::zero(Dimension::Length);
            let across = editor_core::Placed {
                mates: Vec::new(),
                values: vec![
                    (Coord::Slide(TargetAxis::X), zero()),
                    (Coord::Slide(TargetAxis::Y), len(BELOW)),
                    (Coord::Slide(TargetAxis::Z), zero()),
                    (Coord::Spin(Some(TargetAxis::X)), back()),
                    (Coord::Spin(Some(TargetAxis::Y)), back()),
                    (
                        Coord::Spin(Some(TargetAxis::Z)),
                        Formula::zero(Dimension::Angle),
                    ),
                ],
            };
            (
                ball,
                placed_in(&mut r, &housing.body(), ball.body(), across),
            )
        }
    };
    let whole = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![
            editor_core::BodyRead::plain(housing.body()),
            shaft_copy.into(),
            third_copy.into(),
        ]),
    });
    r.place_part(whole);

    CorpusDoc {
        name,
        about,
        edits: r.edits,
        doc: r.doc,
        result: Some(whole),
        pin: None, // revolved: π-valued
        // The third part's revolve turns its full turn again: the
        // housing and the shaft are reused.
        bump: DocEdit::SetParam {
            node: third.revolve,
            slot: SlotId::RevolveAngle,
            value: Formula::turn_over(1).expect("a full turn").into(),
            fresh: Vec::new(),
        },
        bump_root: third.revolve,
    }
}
