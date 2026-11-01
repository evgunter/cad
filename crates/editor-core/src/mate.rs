//! **Mates** — the A3 declaration node and its constructive solve
//! (ASSEMBLY-DESIGN A3/A11; ASM-R2a spec D-1…D-5).
//!
//! A mate equates two poses of one kind (D10, FORK-S3M): `on`, a pose
//! read off the copied shapes' geometry in their own coordinates, and
//! `to`, a pose read off a body in the target space. A mate reads only
//! geometry: no space has roles it can read. The kind is the primitive: `Frame`–`Frame`
//! coincides, `Axis`–`Axis` is coaxial, `Plane`–`Plane` rests,
//! `Point`–`Point` is a ball and `Direction`–`Direction` is parallel.
//! A mate holds no number: its sense is a `Flip` on one side, a
//! standoff a `Slide` on the target, and a freedom the bundle leaves is
//! a value's (spec §1). It places and never checks.
//!
//! A copy standing where its body stands is placed by the identity
//! bundle ([`stand`]): no mate, and the constant `0` on every freedom,
//! so its own coordinates are its target space's. Any other bundle is
//! the mates and moves its author wrote ([`Placed::of`]).
//!
//! # What a mate says, geometrically
//!
//! Each kind pins the copy's relative pose to a COSET of an SE(3)
//! subgroup, the target pose's own symmetry, carried onto the copy's
//! pose ([`mate_coset`]), and the mates of one placement fold by exact
//! coset intersection — [`coset`], whose table is the spec's binding
//! one. A mate declares no contact: where the copies it places touch
//! is the at-rest census's to record ([`mod@crate::checks::at_rest`]).

use crate::eval::NodeRefusal;
use crate::node::RecipeNodeId;

pub mod corner;
pub mod coset;
pub mod member;
pub mod orient;
pub mod pin;
pub mod reach;
pub mod solve;
pub mod turn;
pub mod unobservable;
pub(crate) mod value;

pub use corner::{PinRefusal, corner_frame, corner_frame_in};
pub use coset::{Coset, PoseSymmetry, Subgroup, SubgroupFamily};
pub use member::{Member, Placing, head_face, head_of, member_of, member_of_ref, member_reading};
pub use orient::{Orientation, SignedAxis};
pub use pin::{FrameMove, Placed, moved_frame, stand};
pub use reach::{ReachRefusal, SurfaceKind, body_reach, part_reach, part_reach_from};
pub use solve::{
    Atan2Branch, Composing, PoseRefusal, SolveScalar, SolvedPoses, in_space, mate_coset,
    relative_freedom_components, solve_document, solve_document_at,
};
pub use turn::{LeastTurn, slides_left_along};
pub use unobservable::{Unobservable, unobservable};
pub use value::flipped;

/// Which side of a mate a diagnostic is about.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum MateSide {
    /// The copy's pose.
    On,
    /// The pose it is equated with.
    To,
}

impl MateSide {
    /// The side's name, for messages.
    pub fn name(self) -> &'static str {
        match self {
            Self::On => "on",
            Self::To => "to",
        }
    }
}

/// **A mate** (D10, FORK-S3M, `[placement.mates]`): two poses of one
/// kind, `on` read off the copied shapes' geometry and `to` off a body
/// in the target space. A mate reads only geometry, holds no number,
/// declares no contact, and is a constraint of the placement that holds
/// it, never a node.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, bound = "")]
pub struct Mate<S: crate::Slot = crate::VarId> {
    /// The copy's pose.
    pub on: S::Read,
    /// The pose the copy's is equated with, read off a body in the
    /// target space.
    pub to: S::Read,
    /// **The orientation** (D10 `[placement.values]`): where the copy's
    /// axes lie along the target space's, so where the copy's mated
    /// direction should point. Read only beside a mate fixing a
    /// direction (a plane, an axis, a direction); a point or a frame
    /// mate carries the identity ([`MateFault::Unoriented`]).
    pub orient: Orientation,
}

impl<S: crate::Slot> Mate<S> {
    /// Bit-semantic equality (D7): a mate holds no scalar, so it is
    /// its reads' equality.
    #[must_use]
    pub fn bit_eq(&self, other: &Self) -> bool {
        self == other
    }

    /// **This mate in another slot form**: each pose read mapped by
    /// `read`, `on` first.
    ///
    /// # Errors
    ///
    /// The first refusal of `read`.
    pub fn try_map<S2: crate::Slot, E>(
        &self,
        mut read: impl FnMut(MateSide, &S::Read) -> Result<S2::Read, E>,
    ) -> Result<Mate<S2>, E> {
        Ok(Mate {
            on: read(MateSide::On, &self.on)?,
            to: read(MateSide::To, &self.to)?,
            orient: self.orient,
        })
    }
}

/// **The two faces a mate rests**, when it equates two faces' planes:
/// each side a face read as its plane, through any number of plain
/// flips (a rest's faces face each other) — the face–face contact a
/// `Rest` declares (A5), `on`'s face first. `None` for a mate of another
/// kind, and for a plane constructed off a face (a standoff): such a
/// mate places, and declares no contact.
pub(crate) fn rested_faces<P>(
    doc: &crate::doc::Doc<P>,
    mate: &Mate,
) -> Option<(crate::VarId, crate::VarId)> {
    let (on, to, _) = planes_equated(doc, mate)?;
    Some((on, to))
}

/// **The two faces whose planes a mate equates, and its sense**: each
/// side a face read as its plane through any number of plain flips
/// ([`poses_equated`]), `on`'s face first, and whether the flips on
/// both sides together are odd, so the two faces' outward normals are
/// opposite. `None` exactly where [`rested_faces`] answers `None`.
pub(crate) fn planes_equated<P>(
    doc: &crate::doc::Doc<P>,
    mate: &Mate,
) -> Option<(crate::VarId, crate::VarId, bool)> {
    let (on, to, flipped) = poses_equated(doc, mate);
    let face = |read: crate::pose::RoleRead<crate::VarId>| match read.role {
        None => match doc.var(read.read).map(crate::Var::def)? {
            crate::var::VarDef::Pose(crate::pose::PoseDef::Plane { face }) => Some(*face),
            _ => None,
        },
        Some(_) => None,
    };
    Some((face(on)?, face(to)?, flipped))
}

/// **The two poses a mate equates, and its sense**: each side unwound
/// through any number of flips of a pose read plainly, to the pose it
/// flips (read plainly or through a role), `on`'s first, and whether
/// the flips on both sides together are odd. A mate of every kind
/// equates the two (`[coincidence.structural]`: "a mate is that rung's
/// statement"), where its bundle is admitted
/// ([`solve::admits`]): this reads the mate alone, never whether it
/// stands.
pub(crate) fn poses_equated<P>(
    doc: &crate::doc::Doc<P>,
    mate: &Mate,
) -> (
    crate::pose::RoleRead<crate::VarId>,
    crate::pose::RoleRead<crate::VarId>,
    bool,
) {
    let unwound = |var: crate::VarId| {
        let mut read = crate::pose::RoleRead::plain(var);
        let mut flipped = false;
        while read.role.is_none()
            && let Some(crate::var::VarDef::Pose(crate::pose::PoseDef::Flip { pose })) =
                doc.var(read.read).map(crate::Var::def)
        {
            flipped = !flipped;
            read = *pose;
        }
        (read, flipped)
    };
    let (on, on_flipped) = unwound(mate.on);
    let (to, to_flipped) = unwound(mate.to);
    (on, to, on_flipped != to_flipped)
}

/// **How a placement copies the body a selection reads**
/// ([`copies_of`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Copying {
    /// The body is the placement's own output: the selection reads the
    /// copy itself.
    Own,
    /// A `Place` reads the body.
    Whole,
    /// A `Place` reads one member of the family `body` is
    /// (`xs[i]`, [`crate::BodyRead`]).
    Member,
}

/// **Every placement copying the body `body`**, in document order, and
/// how: the placement whose output it is, and each `Place` reading it,
/// whole or one member of it.
pub(crate) fn copies_of<P>(
    doc: &crate::doc::Doc<P>,
    body: crate::VarId,
) -> Vec<(RecipeNodeId, Copying)> {
    let own = doc.operation_of(body);
    let mut out = Vec::new();
    for p in doc.placements() {
        if own == Some(p) {
            out.push((p, Copying::Own));
            continue;
        }
        let Some(crate::Node::Place { source, .. }) = doc.node(p) else {
            continue;
        };
        if source.read == body {
            out.push((
                p,
                if source.is_indexed() {
                    Copying::Member
                } else {
                    Copying::Whole
                },
            ));
        }
    }
    out
}

/// **Whether a mate side's face, read in the copy `copy` of its body,
/// stands where the mate's statement puts it** (`[coincidence.structural]`):
/// the rule the coincidence door reads a mate side's copy by.
///
/// A side's pose is read off the body its selection reads, where that
/// body stands (`eval::wire::pose`). The mate's placement `placement`
/// moves its `on` side, so that side's statement holds in the
/// placement's copy of the body, or, where the selection already reads
/// the placement's own output, in that output. The `to` side is not
/// moved, so its statement holds where the body stands: in the body
/// itself where it is a placement's output, and in a copy a `Place`
/// stands by the identity bundle, decided off the recipe
/// ([`solve::identity_bundle`]), never off a value. A copy any other
/// bundle moves holds the face where the mate does not read it.
pub(crate) fn reads_in<P>(
    doc: &crate::doc::Doc<P>,
    placement: RecipeNodeId,
    side: MateSide,
    body: crate::VarId,
    (copy, how): (RecipeNodeId, Copying),
) -> bool {
    let moved_by = match side {
        MateSide::On if doc.operation_of(body) != Some(placement) => Some(placement),
        MateSide::On | MateSide::To => None,
    };
    match moved_by {
        Some(by) => copy == by && how != Copying::Own,
        None => {
            how == Copying::Own
                || doc
                    .node(copy)
                    .and_then(solve::bundle_of)
                    .is_some_and(|bundle| solve::identity_bundle(doc, bundle))
        }
    }
}

/// **A bundle's slots**: each value's number
/// ([`crate::SlotId::ValueSlide`], [`crate::SlotId::ValueSpin`]); a
/// mate holds none. A trait over the bundle so the node's slot table
/// reads it through the borrow it holds.
pub(crate) trait BundleRows<S> {
    /// The rows, shared.
    fn rows(&self) -> Vec<(crate::SlotId, &S)>;
    /// The same rows, exclusive, in the same order.
    fn rows_mut(&mut self) -> Vec<(crate::SlotId, &mut S)>;
}

impl<S: crate::Slot> BundleRows<S> for Vec<Constraint<S>> {
    fn rows(&self) -> Vec<(crate::SlotId, &S)> {
        self.iter()
            .filter_map(|c| c.value().map(|v| (value_slot(c.id, v.coord), &v.by)))
            .collect()
    }

    fn rows_mut(&mut self) -> Vec<(crate::SlotId, &mut S)> {
        let mut out = Vec::new();
        for c in self {
            let id = c.id;
            if let ConstraintKind::Value(v) = &mut c.kind {
                let coord = v.coord;
                out.push((value_slot(id, coord), &mut v.by));
            }
        }
        out
    }
}

/// The slot of a value's number.
fn value_slot(constraint: crate::node::ConstraintId, coord: Coord) -> crate::SlotId {
    match coord {
        Coord::Slide(_) => crate::SlotId::ValueSlide { constraint },
        Coord::Spin(_) => crate::SlotId::ValueSpin { constraint },
    }
}

/// **A constraint's address** (FORK-S3M): the placement that holds it
/// and the id minted with it. What a fault about one mate names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MateRef {
    /// The placement holding the constraint.
    pub placement: RecipeNodeId,
    /// The constraint's minted id.
    pub id: crate::node::ConstraintId,
}

impl MateRef {
    /// The constraint `id` of `placement`.
    #[must_use]
    pub const fn new(placement: RecipeNodeId, id: crate::node::ConstraintId) -> Self {
        Self { placement, id }
    }
}

/// **A constraint of a placement** (D10, FORK-S3M): addressed by its
/// placement and the id minted with it, never by its position. A
/// constraint is a mate or a value.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, bound = "")]
pub struct Constraint<S: crate::Slot = crate::VarId> {
    /// The id minted with it.
    pub id: crate::node::ConstraintId,
    /// What it states.
    pub kind: ConstraintKind<S>,
}

/// **What a constraint states**: two poses equated, or one freedom a
/// mate leaves set to a number.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", bound = "")]
pub enum ConstraintKind<S: crate::Slot = crate::VarId> {
    /// A mate.
    Mate(Mate<S>),
    /// A value.
    Value(Value<S>),
}

impl<S: crate::Slot> Constraint<S> {
    /// A mate, under `id`.
    #[must_use]
    pub fn of_mate(id: crate::node::ConstraintId, mate: Mate<S>) -> Self {
        Self {
            id,
            kind: ConstraintKind::Mate(mate),
        }
    }

    /// A value, under `id`.
    #[must_use]
    pub fn of_value(id: crate::node::ConstraintId, value: Value<S>) -> Self {
        Self {
            id,
            kind: ConstraintKind::Value(value),
        }
    }

    /// The mate it states, `None` for a value.
    #[must_use]
    pub fn mate(&self) -> Option<&Mate<S>> {
        match &self.kind {
            ConstraintKind::Mate(mate) => Some(mate),
            ConstraintKind::Value(_) => None,
        }
    }

    /// [`Self::mate`], writable.
    pub fn mate_mut(&mut self) -> Option<&mut Mate<S>> {
        match &mut self.kind {
            ConstraintKind::Mate(mate) => Some(mate),
            ConstraintKind::Value(_) => None,
        }
    }

    /// The value it states, `None` for a mate.
    #[must_use]
    pub fn value(&self) -> Option<&Value<S>> {
        match &self.kind {
            ConstraintKind::Value(value) => Some(value),
            ConstraintKind::Mate(_) => None,
        }
    }

    /// Bit-semantic equality (D7).
    #[must_use]
    pub fn bit_eq(&self, other: &Self) -> bool {
        self.id == other.id
            && match (&self.kind, &other.kind) {
                (ConstraintKind::Mate(a), ConstraintKind::Mate(b)) => a.bit_eq(b),
                (ConstraintKind::Value(a), ConstraintKind::Value(b)) => {
                    a.coord == b.coord && a.by.bit_eq(&b.by)
                }
                _ => false,
            }
    }

    /// **This constraint in another slot form**: a mate's pose reads
    /// mapped by `read` ([`Mate::try_map`]), a value's scalar by `slot`,
    /// its id kept.
    ///
    /// # Errors
    ///
    /// The first refusal of either.
    pub fn try_map<S2: crate::Slot, E>(
        &self,
        read: impl FnMut(MateSide, &S::Read) -> Result<S2::Read, E>,
        slot: &mut impl FnMut(&S) -> Result<S2, E>,
    ) -> Result<Constraint<S2>, E> {
        Ok(Constraint {
            id: self.id,
            kind: match &self.kind {
                ConstraintKind::Mate(mate) => ConstraintKind::Mate(mate.try_map(read)?),
                ConstraintKind::Value(value) => ConstraintKind::Value(Value {
                    coord: value.coord,
                    by: slot(&value.by)?,
                }),
            },
        })
    }
}

/// **A value** (FORK-S3M; D10 `[placement.values]`): one freedom of a
/// placement's motion that its mates leave — a slide or a spin — set
/// to a `Length` or an `Angle`. A value names no mate: its bundle holds
/// at most one mate fixing a direction, so its values have one chart.
///
/// Beside a mate fixing a direction `d`, the mate's orientation `O`
/// puts the copy's mated direction `d_c` at `n = O·d_c`, and the least
/// turn `S` carries `n` onto `d`; the copy's turn is `rot(d, θ)·S·O`.
/// The one [`Coord::Spin`] with no axis is `θ`, and a slide is the
/// copy's origin measured from the target space's along a target axis
/// as `S` carries it.
///
/// Where no direction is fixed (no mate, or a lone point mate), a
/// slide is the copy's origin measured from the target space's along
/// its own `x`, `y` or `z`, and a spin is one link of a chain of turns
/// about the target space's own axes (through the mated point, for a
/// point mate), taken in the order the bundle lists them.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, bound = "")]
pub struct Value<S: crate::Slot = crate::VarId> {
    /// Which freedom.
    pub coord: Coord,
    /// The number: a `Length` for a slide, an `Angle` for a spin.
    pub by: S,
}

/// **Which freedom a value sets.**
///
/// The freedoms each bundle's mates leave: a plane mate two slides and
/// the spin; an axis mate one slide and the spin; a direction mate the
/// three slides and the spin; a point mate its rotation, a chain of
/// spins about the target space's own axes; a frame mate nothing.
/// Beside a mate, which slides a mate takes is the source's geometry's
/// to say (the chart's span, [`NoChartWhy::SlidesDoNotSpan`]). With no
/// mate, a value slides along the target space's `x`, `y` or `z`, or
/// spins about one of them.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Coord {
    /// A slide along the target space's axis (carried by the least turn
    /// beside a mate).
    Slide(TargetAxis),
    /// A spin: with no axis, the turn about the target's own direction
    /// left after the least turn, beside a mate fixing a direction; about an
    /// axis, one link of a written chain of turns where none is fixed.
    Spin(Option<TargetAxis>),
}

/// **An axis of the target space**: its own where no mate fixes a
/// direction, carried by the least turn where one does.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum TargetAxis {
    /// The `x`.
    X,
    /// The `y`.
    Y,
    /// The `z`.
    Z,
}

impl TargetAxis {
    /// The three, in order.
    pub const ALL: [Self; 3] = [Self::X, Self::Y, Self::Z];

    /// The axis as a unit direction of its frame.
    #[must_use]
    pub fn unit<T: geom_core::Real>(self) -> geom_core::UnitVec3<T> {
        match self {
            Self::X => geom_core::UnitVec3::exact_x(),
            Self::Y => geom_core::UnitVec3::exact_y(),
            Self::Z => geom_core::UnitVec3::exact_z(),
        }
    }

    /// Which axis, `0` for `x`.
    #[must_use]
    pub fn index(self) -> usize {
        match self {
            Self::X => 0,
            Self::Y => 1,
            Self::Z => 2,
        }
    }

    /// The axis of `index`.
    #[must_use]
    pub fn of_index(index: usize) -> Self {
        Self::ALL[index % 3]
    }
}

/// **A set of slides, by the target axes they run along**: the slides
/// a mate leaves ([`turn::slides_left_along`]), and the ones a refusal
/// names to write ([`NoChartWhy::SlidesDoNotSpan`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SlideAxes([bool; 3]);

impl SlideAxes {
    /// The set of `axes`.
    #[must_use]
    pub fn of(axes: impl IntoIterator<Item = TargetAxis>) -> Self {
        let mut set = [false; 3];
        for axis in axes {
            set[axis.index()] = true;
        }
        Self(set)
    }

    /// Whether the set holds the slide along `axis`.
    #[must_use]
    pub fn contains(self, axis: TargetAxis) -> bool {
        self.0[axis.index()]
    }

    /// The axes, in order.
    pub fn axes(self) -> impl Iterator<Item = TargetAxis> {
        TargetAxis::ALL
            .into_iter()
            .filter(move |&a| self.contains(a))
    }

    /// How many.
    #[must_use]
    pub fn len(self) -> usize {
        self.axes().count()
    }

    /// Whether the set is empty.
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }
}

impl core::fmt::Display for SlideAxes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let names: Vec<&str> = self.axes().map(axis_name).collect();
        match names.as_slice() {
            [] => f.write_str("none"),
            [one] => f.write_str(one),
            [rest @ .., last] => write!(f, "{} and {last}", rest.join(", ")),
        }
    }
}

impl Coord {
    /// The spin about the target's own direction.
    pub const SPIN: Self = Self::Spin(None);

    /// The dimension of the number that sets it.
    #[must_use]
    pub fn dimension(self) -> crate::expr::Dimension {
        match self {
            Self::Slide(_) => crate::expr::Dimension::Length,
            Self::Spin(_) => crate::expr::Dimension::Angle,
        }
    }

    /// Its name, for messages.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Slide(TargetAxis::X) => "slide along x",
            Self::Slide(TargetAxis::Y) => "slide along y",
            Self::Slide(TargetAxis::Z) => "slide along z",
            Self::Spin(None) => "spin",
            Self::Spin(Some(TargetAxis::X)) => "spin about x",
            Self::Spin(Some(TargetAxis::Y)) => "spin about y",
            Self::Spin(Some(TargetAxis::Z)) => "spin about z",
        }
    }
}

/// **Why a value's freedom has no chart** ([`MateFault::NoChart`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoChartWhy {
    /// A slide beside a ball mate and a direction mate: the ball fixes
    /// every translation, so the slide could only fix the ball's turn
    /// through the copy's lever, a circle meeting a plane at two points
    /// or none, which the solve's structural settle does not choose
    /// between.
    SlideTurnsABall,
    /// The written slides do not span the plane (or line) the mate
    /// leaves: the source's mated direction, where the orientation puts
    /// it, lies across the one slide a plane mate leaves unwritten, or
    /// across the one an axis mate leaves written. Decided once from the
    /// source's geometry.
    SlidesDoNotSpan {
        /// The slide the least turn carries onto the mated direction,
        /// where the source's mated direction lies on a target axis
        /// exactly: the one a plane mate fixes and an axis mate leaves.
        along: Option<TargetAxis>,
        /// The slides to write, which span what the mate leaves
        /// ([`turn::slides_left_along`]).
        write: SlideAxes,
    },
}

/// **The recourse a [`MateFault::Overconstrained`] ends on**: what an
/// author does about a mate the rest of its bundle already fixes.
pub const OVERCONSTRAINED_RECOURSE: &str = "delete one of the named constraints, and write what \
     it held that the rest leave as a value (an axis mate with a spin is a prismatic joint); where \
     the two copies also meet, an assertion says so";

/// **The recourse a [`MateFault::Unpinned`] ends on**: what an author
/// writes for a bundle that leaves a freedom unwritten.
pub const UNPINNED_RECOURSE: &str = "write each freedom the mates leave as a value, the constant 0 \
     where the two spaces agree — with no mate, a slide along each of the target's x, y and z and \
     a chain of three spins about its axes, no two in a row about one axis — or mate the copy";

/// Why a lever could not be formed for a mate: one of its parts' reach
/// is not in hand, the member stands on no
/// instance, or the lever the two parts form is out of the format's
/// range.
#[derive(Debug, Clone, PartialEq)]
pub enum LeverRefusal {
    /// **The reach's own refusal**, named against the instance the
    /// solve was asking for and the part it stands on: the reach's
    /// vocabulary has its one home in [`ReachRefusal`], and the lever
    /// adds only its subject.
    Reach {
        /// The instance.
        instance: RecipeNodeId,
        /// Its part.
        part: crate::ident::DocRef,
        /// Why the part's reach is not in hand.
        refusal: ReachRefusal,
    },
    /// **A copied shape's reach**, refused: the body a placement
    /// copies has no finite extent to lever an angle over.
    Shape {
        /// The node whose body it is.
        shape: RecipeNodeId,
        /// Why its reach is not in hand.
        refusal: ReachRefusal,
    },
    /// Both parts' reach is in hand, but the lever they form with the
    /// datum's own terms is a length the format cannot decide a tilt
    /// over ([`coset::Arm::of`]): each half is finite where it is
    /// read, and their sum, or its square, overflows.
    OutOfRange {
        /// The two parts' reach, summed.
        parts: f64,
        /// The poses' own distances from their origins.
        datum: f64,
    },
    /// The lever is formed, but it is no longer than the band's zero
    /// threshold, so every tilt levered over it reads zero and no
    /// angle can be decided ([`coset::Arm::decides_over`]): the parts
    /// are smaller than the tolerance can see.
    BelowZeroBand {
        /// The lever, in metres.
        arm: f64,
        /// The band's zero threshold, in metres.
        zero: f64,
    },
}

// The part's own refusal is numbered in the part, so it is said by its
// own `Display`; the instance and the node are this document's.
impl crate::spoken::Say for LeverRefusal {
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::Reach {
                instance, refusal, ..
            } => write!(f, "{}'s part {refusal}", by.node_as(*instance, "instance")),
            Self::Shape { shape, refusal } => {
                write!(f, "the body {} copies {refusal}", by.node(*shape))
            }
            Self::OutOfRange { parts, datum } => {
                // A half that is not finite has no metre figure to print.
                let length = |f: &mut core::fmt::Formatter<'_>, x: f64, of: &str| {
                    if x.is_finite() {
                        write!(f, "{x} m of {of}")
                    } else {
                        write!(f, "a length of {of} that is not finite")
                    }
                };
                f.write_str("its lever, ")?;
                length(f, *parts, "its parts' reach")?;
                f.write_str(" plus ")?;
                length(f, *datum, "its datum")?;
                write!(
                    f,
                    ", is too long for a tilt to be decided over it. Recourse: {}",
                    geom_core::RANGE_RECOURSE
                )
            }
            Self::BelowZeroBand { arm, zero } => write!(
                f,
                "its lever, {arm} m, is no longer than the tolerance's zero threshold, {zero} m, \
                 so no tilt can be decided over it: the mated parts are smaller than the \
                 tolerance can see. Recourse: commit a tolerance finer than the parts"
            ),
        }
    }
}

/// The refusal where no document is at hand: each node by its tag.
impl core::fmt::Display for LeverRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

/// A typed mate refusal (D9: fail loud, never a guess). Every arm names
/// its subject — the mate, the pair, the predicate, the residual, or
/// the two documents a mispaired read named.
///
/// **Which mate an arm is about is read per arm by its two consumers,
/// `viewer::tree::blamed_mates` and `pncad_py::MateFaultPayload`.**
/// [`Self::PosesOfAnotherDocument`] names no mate and reaches no row,
/// being raised by [`SolvedPoses::placement`] and never inserted in a
/// fault map.
#[derive(Debug, Clone, PartialEq)]
pub enum MateFault {
    /// The solve is a solve of ANOTHER document (DI3). Names no mate
    /// and reaches no row.
    PosesOfAnotherDocument {
        /// The document whose placement was asked for.
        expected: crate::ident::DocumentId,
        /// The document the solve is of.
        found: crate::ident::DocumentId,
    },

    /// **A constraint any of whose equations the rest of its bundle
    /// already fixes** (D10; FORK-S3O): a mate places and never checks,
    /// so none is verified and kept. Decided by subgroup algebra over
    /// the families the bundle's definitions name, measuring nothing,
    /// pinned or not: the edit doors refuse the constraint being
    /// written, and the solve, for a state no door saw (a load, a
    /// rebind), names every constraint whose equations the others
    /// already fix. Constraints have no order, so none of them is "the
    /// later one".
    Overconstrained {
        /// The placement.
        placement: RecipeNodeId,
        /// Every constraint an equation of which the rest of the bundle
        /// already fixes.
        constraints: Vec<crate::node::ConstraintId>,
        /// How many of the copy's six freedoms the whole bundle leaves.
        free: u8,
    },
    /// **A bundle leaves a freedom unwritten** (D10
    /// `[placement.placement]`): its mates and values together set every
    /// freedom of the copy, so a bundle that leaves one unset, an empty
    /// one included, is refused at the door, and an unwritten freedom
    /// cannot be represented.
    Unpinned {
        /// The placement.
        placement: RecipeNodeId,
        /// How many of the copy's six freedoms the bundle leaves.
        free: u8,
    },
    /// **A mate's `on` does not read the copy** (D10): its pose reads
    /// geometry other than the placement's shapes (or, for an instance,
    /// its own output), so it pins nothing of that placement.
    NotOnTheCopy {
        /// The mate.
        mate: MateRef,
    },

    /// The mate's lever could not be formed: the copy's reach is not in
    /// hand, so no verdict over its extent can be levered.
    Unleverable {
        /// The mate.
        mate: MateRef,
        /// Why — boxed: the refusal carries the shape and the reach's
        /// own refusal,
        /// and the fault's every other arm stays the size it is.
        refusal: Box<LeverRefusal>,
    },

    /// **A mate equates poses of two kinds** (FORK-S3M): the kind is
    /// the primitive, so a plane mated to an axis is no mate.
    KindsDiffer {
        /// The mate.
        mate: MateRef,
        /// The kind `on` reads.
        on: crate::VarKind,
        /// The kind `to` reads.
        to: crate::VarKind,
    },
    /// **A value's freedom has no chart**: a slide beside a ball,
    /// which leaves it no translation to measure.
    NoChart {
        /// The value.
        value: MateRef,
        /// Why.
        why: NoChartWhy,
    },
    /// **A mate's orientation is written backwards** (D10
    /// `[placement.values]`): the mated direction lies opposite to where
    /// the orientation puts the copy's, within the band about that one
    /// pole, where the least turn has no answer. Decided at evaluation.
    Backwards {
        /// The mate.
        mate: MateRef,
        /// The orientation written.
        orient: Orientation,
        /// The orientation nearest it that puts the copy's mated
        /// direction nearest the mated one: the one to write.
        write: Orientation,
    },
    /// **An orientation on a mate that fixes no one direction** (D10
    /// `[placement.values]`): a point or a frame mate, beside which
    /// there is no orientation.
    Unoriented {
        /// The mate.
        mate: MateRef,
    },
    /// **A mate's pose did not evaluate**, in the evaluation layer's
    /// own words: the face it reads is gone or has no plane, a
    /// construction is degenerate, a scalar refused.
    SideUnevaluated {
        /// The mate.
        mate: MateRef,
        /// Which pose.
        side: MateSide,
        /// The evaluation layer's refusal.
        refusal: Box<NodeRefusal>,
    },
}

/// **The pairing predicate's finding, in this door's vocabulary.**
///
/// A2a's rule is one predicate (`ident::mispaired`) and one arm per
/// error type over it. The projection lives HERE, at the type that
/// owns the arm, so a door that runs the predicate writes `?` or
/// `m.into()` and no site re-spells which field goes where.
impl From<crate::ident::Mispaired> for MateFault {
    fn from(m: crate::ident::Mispaired) -> Self {
        Self::PosesOfAnotherDocument {
            expected: m.expected,
            found: m.found,
        }
    }
}

impl MateFault {
    /// **The refusal this fault carries, when it carries one**: the
    /// node that raised it and the refusal, which the sentence points
    /// at and never renders ([`crate::NodeErrorKind::carried_chain`]).
    ///
    /// Exhaustive, so an arm that comes to carry another node's
    /// refusal says so here rather than being drawn as though it did
    /// not.
    #[must_use]
    pub fn carried(&self) -> Option<(RecipeNodeId, &NodeRefusal)> {
        match self {
            Self::SideUnevaluated { refusal, mate, .. } => Some((mate.placement, refusal)),
            Self::PosesOfAnotherDocument { .. }
            | Self::Overconstrained { .. }
            | Self::Unpinned { .. }
            | Self::NotOnTheCopy { .. }
            | Self::Unleverable { .. }
            | Self::KindsDiffer { .. }
            | Self::NoChart { .. }
            | Self::Backwards { .. }
            | Self::Unoriented { .. } => None,
        }
    }

    /// **Every refusal this fault carries, outermost first**, each with
    /// the document its node is in — the mate's own at the first level
    /// ([`crate::NodeErrorKind::carried_chain`]).
    pub fn carried_chain(&self) -> crate::CarriedChain<'_> {
        crate::CarriedChain::from_first(self.carried(), crate::CarriedIn::ThisDocument)
    }
}

/// **What the mate solve's predicates decide, in words** (the lookup is
/// `decision::words`): each is decided where a placement's bundle is
/// solved (`coset`, and the chart its values are read on), so a flip
/// between two runs of a placement reads what turned.
pub(crate) const DECISION_WORDS: &[(&str, &str)] = &[
    (
        "mate_axes_parallel",
        "whether a mate's two axes are parallel",
    ),
    (
        "mate_axis_normal_perpendicular",
        "whether a mate's axis lies across its plane's normal",
    ),
    (
        "mate_axis_point_offset",
        "whether a mate's point lies on its axis",
    ),
    // The decided door's rigidity margins (`geom_core::RigidMotion::
    // check_rigid`), asked of a solved placement's map and of a
    // pattern's listed frames.
    (
        "transform_rigid_col0_unit",
        "whether the map keeps its first axis's length",
    ),
    (
        "transform_rigid_col1_unit",
        "whether the map keeps its second axis's length",
    ),
    (
        "transform_rigid_col2_unit",
        "whether the map keeps its third axis's length",
    ),
    (
        "transform_rigid_col01_orth",
        "whether the map keeps its first two axes square",
    ),
    (
        "transform_rigid_col12_orth",
        "whether the map keeps its last two axes square",
    ),
    (
        "transform_rigid_col02_orth",
        "whether the map keeps its first and third axes square",
    ),
    (
        "transform_rigid_det_plus_one",
        "whether the map keeps its handedness",
    ),
    (
        solve::SOLVE_MAP_REWITNESS,
        "whether a solved placement's map holds a frame of its own",
    ),
    (
        "mate_chart_pole",
        "whether the mated direction lies opposite to where the orientation puts the copy's",
    ),
    (
        "mate_chart_slides_span",
        "whether a placement's slides span what its mate leaves",
    ),
];

// Every node the fault names is in the mate's own document; a part's
// face and refusal are numbered in the part and said by their own
// `Display`. The mate is said by what it is, so a frame that has already
// named it ([`crate::spoken::Speaker::about`]) reads `this mate`.
impl crate::spoken::Say for MateFault {
    #[allow(clippy::too_many_lines)] // one arm per variant, each short
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        use crate::spoken::Said;
        let mate_ = |m: MateRef| by.mate(m);
        let ids = |placement: RecipeNodeId, ids: &[crate::node::ConstraintId]| {
            ids.iter()
                .map(|&id| by.constraint_numeral(MateRef { placement, id }))
                .collect::<Vec<_>>()
                .join(", ")
        };
        match self {
            // The two ids ride the payload; the sentence names the roles.
            Self::PosesOfAnotherDocument { .. } => f.write_str(
                "a placement was asked of a solve of another document than the one it was \
                 asked for, so no frame is read. Recourse: solve the document whose placement \
                 is asked for, and read it off that solve",
            ),
            Self::Overconstrained {
                placement,
                constraints,
                free,
            } => {
                let which = match constraints.as_slice() {
                    [_] => "constraint",
                    _ => "each of constraints",
                };
                let left = match free {
                    0 => "the bundle pins its copy".to_string(),
                    1 => "the bundle leaves its copy one freedom".to_string(),
                    n => format!("the bundle leaves its copy {n} freedoms"),
                };
                write!(
                    f,
                    "{}'s {which} {} fixes an equation the rest of its bundle already fixes \
                     ({left}). Recourse: {OVERCONSTRAINED_RECOURSE}",
                    by.node(*placement),
                    ids(*placement, constraints),
                )
            }
            Self::Unpinned { placement, free } => {
                let left = match free {
                    1 => "one freedom".to_string(),
                    n => format!("{n} freedoms"),
                };
                write!(
                    f,
                    "{}'s bundle leaves {left} of its copy unwritten, and every bundle pins its \
                     copy. Recourse: {UNPINNED_RECOURSE}",
                    by.node(*placement),
                )
            }
            Self::NotOnTheCopy { mate } => write!(
                f,
                "{}'s `on` pose reads geometry other than the copy its placement defines, so \
                 it pins nothing of it. Recourse: read `on` off the placement's shapes, or move \
                 the constraint to the placement it pins",
                mate_(*mate)
            ),
            Self::Unleverable { mate, refusal } => {
                write!(f, "{}: {}", mate_(*mate), Said(&**refusal, by))
            }
            Self::KindsDiffer { mate, on, to } => write!(
                f,
                "{} equates an `on` pose of kind {} with a `to` pose of kind {}; a mate's two \
                 poses are of one kind, which is its primitive. Recourse: read both poses as one kind, through a role where one \
                 is finer",
                mate_(*mate),
                on,
                to
            ),
            Self::NoChart {
                value,
                why: NoChartWhy::SlideTurnsABall,
            } => write!(
                f,
                "{} sets a slide beside a ball mate, which fixes every translation, so the \
                 slide could only turn the copy about the ball. Recourse: set the turn as a \
                 spin value, or pin it by a frame mate and its values",
                mate_(*value)
            ),
            Self::NoChart {
                value,
                why: NoChartWhy::SlidesDoNotSpan { along, write },
            } => {
                let along = match along {
                    Some(axis) => format!(
                        ": the mate's direction lands on the slide along {}, which a plane \
                         mate fixes and an axis mate leaves",
                        axis_name(*axis)
                    ),
                    None => String::new(),
                };
                write!(
                    f,
                    "{}'s slides do not span what its mate leaves{along}. Recourse: write the \
                     slides along {write} instead, or an orientation that puts the copy's mated \
                     direction where the slides written meet it",
                    mate_(*value)
                )
            }
            Self::Backwards {
                mate,
                orient,
                write,
            } => write!(
                f,
                "{}'s orientation ({orient}) puts the copy's mated direction opposite to the \
                 mated one, where no least turn is decided. Recourse: write the orientation \
                 {write}",
                mate_(*mate)
            ),
            Self::Unoriented { mate } => write!(
                f,
                "{} fixes no one direction, so it carries no orientation. Recourse: write its \
                 orientation as the identity, and its rotation as spin values",
                mate_(*mate)
            ),
            // The refusal is the pose's own, drawn on the carried line.
            Self::SideUnevaluated { mate, side, .. } => write!(
                f,
                "{}'s `{}` pose did not evaluate, so the bundle is not folded. Recourse: \
                 repair the pose's definition, or what it reads",
                mate_(*mate),
                side.name()
            ),
        }
    }
}

/// An axis's letter.
fn axis_name(axis: TargetAxis) -> &'static str {
    match axis {
        TargetAxis::X => "x",
        TargetAxis::Y => "y",
        TargetAxis::Z => "z",
    }
}

/// The fault where no document is at hand: each node by its tag.
impl core::fmt::Display for MateFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

impl MateFault {
    /// **The fault as the frame holding the mate's document says it**:
    /// each node as `doc` holds it now ([`crate::Doc::spoken`]). A fault
    /// is memoized with the solve and the evaluation, so it holds ids,
    /// never a label.
    #[must_use]
    pub fn spoken<P: crate::ProfilePayload>(&self, doc: &crate::doc::Doc<P>) -> String {
        crate::spoken::spoken_by(self, doc)
    }
}

impl core::error::Error for MateFault {}
