//! Corpus document **pin_in_bore** — a shouldered pin seated in a
//! revolved bore, twice: once pressed, once with clearance.
//!
//! Each housing is a rectangle `ρ ∈ [bore, 3/4]`, `y ∈ [0, 1]`
//! revolved a full turn about its own `y`; each pin a stepped meridian
//! (a shank under a head) revolved the same way. A pin is placed
//! against its housing's world copy by one `Axis`–`Axis` mate, the
//! shank wall's axis on the bore wall's, the spin that mate leaves
//! written `0` and its slide the housing's height. The world's product
//! is the `⊔` of the four copies.
//!
//! The press fit's bore and shank both read the document variable
//! `bore`, so its two walls are one cylinder by value and the census
//! meets them at rest; the clearance pin's shank is narrower, its walls
//! apart. Both heads rest on their housings' tops.
//!
//! No mass pin: every body is revolved, so its volume is `π`-valued.

use editor_core::{
    Coord, Dimension, DocEdit, Formula, FreeVar, Node, Operand, ProgramStep, ProgramTarget,
    RecipeNodeId, TargetAxis, VarDecl, VarName,
};

use crate::fixture::{axis, len};

use super::{CorpusDoc, PlacePart, Recorder, Revolved};

/// The document variable the press fit's bore and shank both read.
pub const BORE: &str = "bore";
/// The bore's radius, meters.
pub const BORE_R: f64 = 0.25;
/// The document variable the clearance pin's shank reads.
pub const LOOSE: &str = "loose";
/// The clearance pin's shank radius, meters.
pub const LOOSE_R: f64 = 0.1875;
/// The housing's outer radius, meters.
pub const HOUSING_R: f64 = 0.75;
/// The housing's height, meters.
pub const HOUSING_H: f64 = 1.0;
/// The pin head's radius, meters.
pub const HEAD_R: f64 = 0.5;
/// The pin head's height, meters.
pub const HEAD_H: f64 = 0.25;
/// The shank's length under the head, meters.
pub const SHANK_L: f64 = 0.75;
/// Where the clearance pair stands along the world's `x`, meters.
pub const LOOSE_AT: f64 = 2.0;

/// The bore's radius as the document variable.
fn bore() -> Formula {
    Formula::named(VarName::from_static(BORE), Dimension::Length)
}

/// A point of a meridian, its radius `rho` and height `y`.
fn at(rho: Formula, y: f64) -> [Formula; 2] {
    [rho, len(y)]
}

/// The meridian of straight legs through `points`, closed, revolved.
fn revolved(r: &mut Recorder, points: Vec<[Formula; 2]>) -> Revolved {
    let mut points = points.into_iter();
    let first = points.next().expect("a meridian has a start");
    let chain = std::iter::once(ProgramStep::At(first))
        .chain(points.map(|p| ProgramStep::LineTo(ProgramTarget::Point(p))))
        .chain([ProgramStep::LineTo(ProgramTarget::Start)])
        .collect();
    super::revolved(r, chain)
}

/// A housing whose bore is `radius`: its meridian's step 4 sweeps the
/// bore wall.
fn housing(r: &mut Recorder, radius: Formula) -> Revolved {
    revolved(
        r,
        vec![
            at(radius.clone(), 0.0),
            at(len(HOUSING_R), 0.0),
            at(len(HOUSING_R), HOUSING_H),
            at(radius, HOUSING_H),
        ],
    )
}

/// A pin whose shank is `radius`, its head's underside at its own
/// `y = 0`: its meridian's step 2 sweeps the shank wall.
fn pin(r: &mut Recorder, radius: Formula) -> Revolved {
    revolved(
        r,
        vec![
            at(Formula::zero(Dimension::Length), -SHANK_L),
            at(radius.clone(), -SHANK_L),
            at(radius, 0.0),
            at(len(HEAD_R), 0.0),
            at(len(HEAD_R), HEAD_H),
            at(Formula::zero(Dimension::Length), HEAD_H),
        ],
    )
}

/// **`pin` seated in the world copy `seat` of `housing`**: one
/// `Axis`–`Axis` mate, the shank wall's axis on the bore wall's, the
/// spin it leaves written `0` and its slide the housing's height, where
/// the shoulder meets the top. (A rest of the shoulder on the top beside
/// the axis mate states the axis's direction twice, which the mate door
/// refuses as overconstrained; the slide is measured from the housing's
/// origin, so `0` would stand the shoulder on the housing's base.)
fn seated(r: &mut Recorder, pin: Revolved, housing: Revolved, seat: RecipeNodeId) -> RecipeNodeId {
    let on = axis(Operand::select(pin.body(), vec![pin.wall(r, 2)]));
    let to = axis(Operand::select(
        Operand::output(seat, 0),
        vec![housing.wall(r, 4)],
    ));
    let placement = r.insert(crate::fixture::stood(editor_core::BodyRead::plain(
        crate::fixture::body_read(&r.doc, pin.body()),
    )));
    r.constrain(
        placement,
        crate::fixture::mate(on, to),
        vec![
            (Coord::Slide(TargetAxis::Y), len(HOUSING_H)),
            (Coord::SPIN, Formula::zero(Dimension::Angle)),
        ],
    );
    placement
}

/// **Each fit's two walls, bore then shank**, each as its revolve and
/// the step whose leg it sweeps, press fit first: what the census would
/// read as the fit's contact.
pub fn walls(doc: &editor_core::ProfileDoc) -> [[(RecipeNodeId, editor_core::StepId); 2]; 2] {
    let revolves: Vec<RecipeNodeId> = doc
        .ids()
        .into_iter()
        .filter(|&id| matches!(doc.node(id), Some(Node::Revolve { .. })))
        .collect();
    let wall = |revolve: RecipeNodeId, k: usize| {
        let profile = doc.upstream(revolve)[0];
        let Some(Node::Profile(program)) = doc.node(profile) else {
            unreachable!("a meridian is a profile")
        };
        (revolve, program.ids[0][k])
    };
    let [press_housing, press_pin, loose_housing, loose_pin] = revolves[..] else {
        unreachable!("two housings and two pins: {revolves:?}")
    };
    [
        [wall(press_housing, 4), wall(press_pin, 2)],
        [wall(loose_housing, 4), wall(loose_pin, 2)],
    ]
}

/// The pin-in-bore corpus document.
pub fn document() -> CorpusDoc {
    scene(
        (
            "pin_in_bore",
            "a shouldered pin seated in a bore by an axis mate, pressed and loose",
        ),
        BORE,
    )
}

/// **The scene with the press fit's shank its own variable**, `shank`,
/// written equal to `bore`: two variables equal by value where
/// [`document`] reads one. Beside the registry
/// ([`super::stage6_beside_the_registry`]), for the door census.
pub fn two_variables() -> CorpusDoc {
    scene(
        (
            "pin_in_bore_two_variables",
            "pin_in_bore with the press fit's shank a second variable typed equal to the bore",
        ),
        SHANK,
    )
}

/// The second variable [`two_variables`]' press-fit shank reads.
pub const SHANK: &str = "shank";

/// The two seated pins, the press fit's shank reading `shank`.
fn scene((name, about): (&'static str, &'static str), shank: &'static str) -> CorpusDoc {
    let mut r = Recorder::new();
    for var in std::iter::once(BORE).chain((shank != BORE).then_some(shank)) {
        r.push(DocEdit::DeclareVar {
            name: VarName::from_static(var),
            def: VarDecl::Free(FreeVar::continuous(Dimension::Length, BORE_R)),
        });
    }
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static(LOOSE),
        def: VarDecl::Free(FreeVar::continuous(Dimension::Length, LOOSE_R)),
    });

    let press_housing = housing(&mut r, bore());
    let press_pin = pin(
        &mut r,
        Formula::named(VarName::from_static(shank), Dimension::Length),
    );
    let press_seat = r.place_part(press_housing.body());
    seated(&mut r, press_pin, press_housing, press_seat);

    let loose_housing = housing(&mut r, len(BORE_R));
    let loose_pin = pin(
        &mut r,
        Formula::named(VarName::from_static(LOOSE), Dimension::Length),
    );
    let loose_seat = r.in_world(
        loose_housing.body(),
        &editor_core::Frame::translation([LOOSE_AT, 0.0, 0.0]),
    );
    seated(&mut r, loose_pin, loose_housing, loose_seat);

    CorpusDoc {
        name,
        about,
        edits: r.edits,
        doc: r.doc,
        result: None, // the head is the world's four copies
        pin: None,    // revolved: π-valued
        // The clearance pin's shank thins: the housings, the press pin
        // and their copies are reused.
        bump: DocEdit::SetVarValue {
            var: VarName::from_static(LOOSE).into(),
            value: editor_core::FreeValue::Continuous(0.125),
        },
        bump_root: loose_pin.profile,
    }
}
