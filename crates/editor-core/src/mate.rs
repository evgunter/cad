//! **Mates** — the A3 declaration node and its constructive solve
//! (ASSEMBLY-DESIGN A3/A11; ASM-R2a spec D-1…D-5).
//!
//! A mate equates two poses of one kind (D10, FORK-S3M): `on`, a pose
//! read off the copied shapes' geometry in their own coordinates, and
//! `to`, a pose read off geometry of the space the copy joins, or the
//! world read through the pose reads the world offers
//! ([`WorldRead`]). The kind is the primitive: `Frame`–`Frame`
//! coincides, `Axis`–`Axis` is coaxial, `Plane`–`Plane` rests,
//! `Point`–`Point` is a ball and `Direction`–`Direction` is parallel.
//! A mate holds no number: its sense is a `Flip` on one side, a
//! standoff a `Slide` on the target, and a freedom the bundle leaves is
//! a value's (spec §1). It places and never checks.
//!
//! # What a mate says, geometrically
//!
//! Each kind pins the copy's relative pose to a COSET of an SE(3)
//! subgroup, the target pose's own symmetry, carried onto the copy's
//! pose ([`mate_coset`]), and the mates of one placement fold by exact
//! coset intersection — [`coset`], whose table is the spec's binding
//! one.
//!
//! # v1's admitted classes
//!
//! `class` is the KERNEL [`ContactClass`] (M9-1), re-exported rather
//! than re-minted, so a mate's declaration is already the currency the
//! boolean wrapper's records speak.
//!
//! **Which classes v1 admits is [`class_admission`], not a sentence
//! here.** A class passes TWO doors — the solve, which folds cosets,
//! and the assembly gate's mint, which needs a kernel record type to
//! carry the declaration at rest — and the two do not admit the same
//! set. The table is one function both doors read, so a class can
//! never be admitted at one and refused at the other without saying
//! so: `Rest` clears both, `Tangent` solves and then refuses typed at
//! the mint door (the at-rest census decides no tangent locus for its
//! face–face record to cite — [`crate::MintRefusal::NoAtRestRecord`]), and
//! every later class — `Fit { gap }` when it lands — refuses at the
//! solve door, because a declared clearance changes what "coincide"
//! means and this unit solves coincidence only.

use crate::eval::NodeRefusal;
use crate::node::RecipeNodeId;
use geom_core::predicate::{Indeterminate, Margin};

pub mod coset;
pub mod member;
pub mod pin;
pub mod reach;
pub mod solve;
pub mod unobservable;
pub(crate) mod value;

pub use coset::{Coset, PoseSymmetry, Subgroup, SubgroupFamily};
pub use member::{Member, Placing, head_face, head_of, member_of, member_reading};
pub use pin::{PinRefusal, Placed, PlacedBy, pin_at, place_at};
pub use reach::{MateReach, ReachRefusal, RefusingReach, SurfaceKind, body_reach, part_reach};
pub use solve::{
    PoseRefusal, SolveScalar, SolvedPoses, Space, mate_coset, relative_freedom_components,
    solve_document, solve_document_at,
};
pub use unobservable::{Unobservable, unobservable};

/// The kernel's contact vocabulary, re-exported (M9-1 PR-1: one enum,
/// defined lowest). A mate's class IS a contact declaration.
pub use topo::ContactClass;

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

/// **A pose of the world, read inside a mate** (spec Q7). The world
/// defines no pose variable, so a mate's target reads it here: its
/// frame, a role of it, and the target constructions — a slide along a
/// direction or an axis, a spin about an axis, a flip. The read is part
/// of the mate and nothing else can read it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields, bound = "")]
pub enum WorldRead<S: crate::Slot = crate::VarId> {
    /// The world's own frame.
    Frame,
    /// A pose of the world read through a role its kind offers.
    Role {
        /// The pose read.
        pose: Box<WorldRead<S>>,
        /// The role.
        role: crate::pose::PoseRole,
    },
    /// The pose slid along a direction by a length.
    Slide {
        /// The pose moved.
        pose: Box<WorldRead<S>>,
        /// The direction or axis it moves along.
        along: Box<WorldRead<S>>,
        /// How far.
        by: S,
    },
    /// The pose spun about an axis by an angle, right-handed.
    Spin {
        /// The pose moved.
        pose: Box<WorldRead<S>>,
        /// The axis it turns about.
        about: Box<WorldRead<S>>,
        /// How far.
        by: S,
    },
    /// The pose's opposite sense.
    Flip {
        /// The pose flipped.
        pose: Box<WorldRead<S>>,
    },
}

impl<S: crate::Slot> WorldRead<S> {
    /// The world's frame read through `role`.
    #[must_use]
    pub fn through(role: crate::pose::PoseRole) -> Self {
        Self::Role {
            pose: Box::new(Self::Frame),
            role,
        }
    }

    /// `self` slid along `along` by `by`.
    #[must_use]
    pub fn slide(self, along: Self, by: S) -> Self {
        Self::Slide {
            pose: Box::new(self),
            along: Box::new(along),
            by,
        }
    }

    /// `self` spun about `about` by `by`.
    #[must_use]
    pub fn spin(self, about: Self, by: S) -> Self {
        Self::Spin {
            pose: Box::new(self),
            about: Box::new(about),
            by,
        }
    }

    /// `self`'s opposite sense.
    #[must_use]
    pub fn flip(self) -> Self {
        Self::Flip {
            pose: Box::new(self),
        }
    }

    /// **How many reads deep this read nests**: the world's frame is
    /// one, and each construction is one more than the deepest read it
    /// moves or reads along or about.
    pub fn depth(&self) -> usize {
        1 + match self {
            Self::Frame => 0,
            Self::Role { pose, .. } | Self::Flip { pose } => pose.depth(),
            Self::Slide {
                pose, along: line, ..
            }
            | Self::Spin {
                pose, about: line, ..
            } => pose.depth().max(line.depth()),
        }
    }

    /// **The kind this read is**, or the first construction its kinds
    /// refuse: a slide's along a direction or an axis and of no
    /// direction, a spin's about an axis, a flip of no point, a role
    /// its pose's kind offers. A read nested deeper than
    /// [`MAX_WORLD_READ_DEPTH`] refuses before its kinds are read.
    ///
    /// # Errors
    ///
    /// [`WorldReadFault`] naming the construction, or
    /// [`WorldReadFault::TooDeep`].
    pub fn kind(&self) -> Result<crate::VarKind, WorldReadFault> {
        let depth = self.depth();
        if depth > MAX_WORLD_READ_DEPTH {
            return Err(WorldReadFault::TooDeep { depth });
        }
        self.kind_within()
    }

    fn kind_within(&self) -> Result<crate::VarKind, WorldReadFault> {
        use crate::VarKind as K;
        Ok(match self {
            Self::Frame => K::Frame,
            Self::Role { pose, role } => {
                let found = pose.kind_within()?;
                if role.of() != found {
                    return Err(WorldReadFault::Role { role: *role, found });
                }
                role.kind()
            }
            Self::Slide { pose, along, .. } => {
                if !matches!(along.kind_within()?, K::Direction | K::Axis) {
                    return Err(WorldReadFault::Along);
                }
                match pose.kind_within()? {
                    K::Direction => return Err(WorldReadFault::SlidDirection),
                    kind => kind,
                }
            }
            Self::Spin { pose, about, .. } => {
                if about.kind_within()? != K::Axis {
                    return Err(WorldReadFault::About);
                }
                pose.kind_within()?
            }
            Self::Flip { pose } => match pose.kind_within()? {
                K::Point => return Err(WorldReadFault::FlippedPoint),
                kind => kind,
            },
        })
    }

    /// Each scalar of the read in pre-order, with whether it is a
    /// spin's angle (`true`) or a slide's length: the read's slots
    /// ([`crate::SlotId::WorldSlide`], [`crate::SlotId::WorldSpin`]).
    pub fn scalars(&self) -> Vec<(bool, &S)> {
        let mut out = Vec::new();
        self.walk(&mut out);
        out
    }

    fn walk<'a>(&'a self, out: &mut Vec<(bool, &'a S)>) {
        match self {
            Self::Frame => {}
            Self::Role { pose, .. } | Self::Flip { pose } => pose.walk(out),
            Self::Slide { pose, along, by } => {
                pose.walk(out);
                along.walk(out);
                out.push((false, by));
            }
            Self::Spin { pose, about, by } => {
                pose.walk(out);
                about.walk(out);
                out.push((true, by));
            }
        }
    }

    /// [`Self::scalars`], writable, in the same order.
    pub fn scalars_mut(&mut self) -> Vec<(bool, &mut S)> {
        let mut out = Vec::new();
        self.walk_mut(&mut out);
        out
    }

    fn walk_mut<'a>(&'a mut self, out: &mut Vec<(bool, &'a mut S)>) {
        match self {
            Self::Frame => {}
            Self::Role { pose, .. } | Self::Flip { pose } => pose.walk_mut(out),
            Self::Slide { pose, along, by } => {
                pose.walk_mut(out);
                along.walk_mut(out);
                out.push((false, by));
            }
            Self::Spin { pose, about, by } => {
                pose.walk_mut(out);
                about.walk_mut(out);
                out.push((true, by));
            }
        }
    }

    /// This read in another slot form, each scalar mapped by `f` in
    /// pre-order.
    ///
    /// # Errors
    ///
    /// `f`'s first.
    pub fn try_map<S2: crate::Slot, E>(
        &self,
        f: &mut impl FnMut(&S) -> Result<S2, E>,
    ) -> Result<WorldRead<S2>, E> {
        Ok(match self {
            Self::Frame => WorldRead::Frame,
            Self::Role { pose, role } => WorldRead::Role {
                pose: Box::new(pose.try_map(f)?),
                role: *role,
            },
            Self::Slide { pose, along, by } => WorldRead::Slide {
                pose: Box::new(pose.try_map(f)?),
                along: Box::new(along.try_map(f)?),
                by: f(by)?,
            },
            Self::Spin { pose, about, by } => WorldRead::Spin {
                pose: Box::new(pose.try_map(f)?),
                about: Box::new(about.try_map(f)?),
                by: f(by)?,
            },
            Self::Flip { pose } => WorldRead::Flip {
                pose: Box::new(pose.try_map(f)?),
            },
        })
    }
}

/// **The deepest world read a mate holds** ([`WorldRead::depth`]):
/// fifteen. The `place_at` gesture writes at most nine — the world's
/// xy plane read through its role after three spins and three slides
/// (`mate::pin`), each about or along an axis read two deep — and
/// inline at a landing off the identity appends at most six more, the
/// landing's own spins and slides. A world read is written inside its
/// mate, never as a variable (spec Q7), so this bound is what holds a
/// saved mate within the load door's nesting limit
/// (`persist::nesting::ENVELOPE`).
pub const MAX_WORLD_READ_DEPTH: usize = 15;

/// **Why a world read is no pose** ([`WorldRead::kind`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorldReadFault {
    /// A role the read pose's kind does not offer.
    Role {
        /// The role asked.
        role: crate::pose::PoseRole,
        /// The kind read.
        found: crate::VarKind,
    },
    /// A slide along what is no direction and no axis.
    Along,
    /// A slide of a direction, which has no position.
    SlidDirection,
    /// A spin about what is no axis.
    About,
    /// A flip of a point, which has no sense.
    FlippedPoint,
    /// A read nested deeper than [`MAX_WORLD_READ_DEPTH`]: the world
    /// read is written inside the mate, so the depth a save writes is
    /// bounded where the read is admitted, as an expression's is.
    TooDeep {
        /// The read's depth ([`WorldRead::depth`]).
        depth: usize,
    },
}

impl core::fmt::Display for WorldReadFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Role { role, found } => write!(
                f,
                "a world read through its {} role reads a {}, which offers none",
                role.label(),
                found
            ),
            Self::Along => f.write_str("a world read slides along what is no direction or axis"),
            Self::SlidDirection => {
                f.write_str("a world read slides a direction, which has no position")
            }
            Self::About => f.write_str("a world read spins about what is no axis"),
            Self::FlippedPoint => f.write_str("a world read flips a point, which has no sense"),
            Self::TooDeep { depth } => write!(
                f,
                "a world read nests {depth} reads deep, past the {MAX_WORLD_READ_DEPTH} a mate \
                 holds; read a pose of the world's geometry, or move the world less"
            ),
        }
    }
}

/// **What a mate's `to` reads** (D10, spec Q7): a pose variable read
/// off geometry of the space the copy joins, or the world.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields, bound = "")]
pub enum Target<S: crate::Slot = crate::VarId> {
    /// A pose variable of the space the copy joins.
    Pose(S::Read),
    /// The world, read inside the mate.
    World(WorldRead<S>),
}

impl<S: crate::Slot> Target<S> {
    /// The pose read, `None` for the world.
    pub fn pose(&self) -> Option<&S::Read> {
        match self {
            Self::Pose(read) => Some(read),
            Self::World(_) => None,
        }
    }

    /// [`Self::pose`], writable.
    pub fn pose_mut(&mut self) -> Option<&mut S::Read> {
        match self {
            Self::Pose(read) => Some(read),
            Self::World(_) => None,
        }
    }
}

/// **A mate** (D10, FORK-S3M): two poses of one kind, `on` read off
/// the copied shapes' geometry and `to` off the space the copy joins
/// or the world, with the contact class it declares. A mate holds no
/// number, and is a constraint of the placement that holds it, never a
/// node.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, bound = "")]
pub struct Mate<S: crate::Slot = crate::VarId> {
    /// The copy's pose.
    pub on: S::Read,
    /// The pose the copy's is equated with.
    pub to: Target<S>,
    /// The declared contact class, the kernel vocabulary
    /// ([`class_admission`] says how far each gets).
    #[serde(with = "crate::persist::kernel_wire::contact_class")]
    pub class: ContactClass,
}

impl<S: crate::Slot> Mate<S> {
    /// Bit-semantic equality (D7): the reads equal, each scalar slot
    /// equal.
    #[must_use]
    pub fn bit_eq(&self, other: &Self) -> bool {
        self == other
            && match (&self.to, &other.to) {
                (Target::World(a), Target::World(b)) => a
                    .scalars()
                    .iter()
                    .zip(b.scalars())
                    .all(|((_, x), (_, y))| x.bit_eq(y)),
                _ => true,
            }
    }

    /// **This mate in another slot form**: each pose read mapped by
    /// `read`, each world read's scalar by `slot`, `on` first.
    ///
    /// # Errors
    ///
    /// The first refusal of either.
    pub fn try_map<S2: crate::Slot, E>(
        &self,
        mut read: impl FnMut(MateSide, &S::Read) -> Result<S2::Read, E>,
        slot: &mut impl FnMut(&S) -> Result<S2, E>,
    ) -> Result<Mate<S2>, E> {
        Ok(Mate {
            on: read(MateSide::On, &self.on)?,
            to: match &self.to {
                Target::Pose(r) => Target::Pose(read(MateSide::To, r)?),
                Target::World(w) => Target::World(w.try_map(slot)?),
            },
            class: self.class,
        })
    }
}

/// **The two faces a mate rests**, when it equates two faces' planes:
/// each side a face read as its plane, through any number of plain
/// flips (a rest's faces face each other) — the face–face contact a
/// `Rest` declares (A5), `on`'s face first. `None` for a mate of another
/// kind, for a plane constructed off a face (a standoff), and for a
/// world read: such a mate places, and declares no contact.
pub(crate) fn rested_faces<P>(
    doc: &crate::doc::Doc<P>,
    mate: &Mate,
) -> Option<(crate::VarId, crate::VarId)> {
    let face = |mut var: crate::VarId| loop {
        match doc.var(var).map(crate::Var::def)? {
            crate::var::VarDef::Pose(crate::pose::PoseDef::Plane { face }) => return Some(*face),
            crate::var::VarDef::Pose(crate::pose::PoseDef::Flip {
                pose: crate::pose::RoleRead { read, role: None },
            }) => var = *read,
            _ => return None,
        }
    };
    Some((face(mate.on)?, face(*mate.to.pose()?)?))
}

/// **A bundle's slots**: each world read's scalars, addressed by the
/// constraint and the scalar's place in the read
/// ([`crate::SlotId::WorldSlide`], [`crate::SlotId::WorldSpin`]), and
/// each value's number ([`crate::SlotId::ValueSlide`],
/// [`crate::SlotId::ValueSpin`]). A trait over the bundle so the node's
/// slot table reads it through the borrow it holds.
pub(crate) trait BundleRows<S> {
    /// The rows, shared.
    fn rows(&self) -> Vec<(crate::SlotId, &S)>;
    /// The same rows, exclusive, in the same order.
    fn rows_mut(&mut self) -> Vec<(crate::SlotId, &mut S)>;
}

impl<S: crate::Slot> BundleRows<S> for Vec<Constraint<S>> {
    fn rows(&self) -> Vec<(crate::SlotId, &S)> {
        let mut out = Vec::new();
        for c in self {
            match &c.kind {
                ConstraintKind::Mate(Mate {
                    to: Target::World(w),
                    ..
                }) => {
                    for (step, (spin, e)) in w.scalars().into_iter().enumerate() {
                        out.push((world_slot(c.id, step, spin), e));
                    }
                }
                ConstraintKind::Mate(_) => {}
                ConstraintKind::Value(v) => out.push((value_slot(c.id, v.coord), &v.by)),
            }
        }
        out
    }

    fn rows_mut(&mut self) -> Vec<(crate::SlotId, &mut S)> {
        let mut out = Vec::new();
        for c in self {
            let id = c.id;
            match &mut c.kind {
                ConstraintKind::Mate(Mate {
                    to: Target::World(w),
                    ..
                }) => {
                    for (step, (spin, e)) in w.scalars_mut().into_iter().enumerate() {
                        out.push((world_slot(id, step, spin), e));
                    }
                }
                ConstraintKind::Mate(_) => {}
                ConstraintKind::Value(v) => {
                    let coord = v.coord;
                    out.push((value_slot(id, coord), &mut v.by));
                }
            }
        }
        out
    }
}

/// The slot of a value's number.
fn value_slot(constraint: crate::node::ConstraintId, coord: Coord) -> crate::SlotId {
    match coord {
        Coord::Slide(_) => crate::SlotId::ValueSlide { constraint },
        Coord::Spin => crate::SlotId::ValueSpin { constraint },
    }
}

/// The slot of a world read's `step`th scalar, a spin's or a slide's.
fn world_slot(constraint: crate::node::ConstraintId, step: usize, spin: bool) -> crate::SlotId {
    if spin {
        crate::SlotId::WorldSpin { constraint, step }
    } else {
        crate::SlotId::WorldSlide { constraint, step }
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
#[serde(rename_all = "snake_case", deny_unknown_fields, bound = "")]
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
                    a.mate == b.mate && a.coord == b.coord && a.by.bit_eq(&b.by)
                }
                _ => false,
            }
    }

    /// **This constraint in another slot form**: a mate's pose reads
    /// mapped by `read` ([`Mate::try_map`]), every scalar by `slot`, its
    /// id kept.
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
                ConstraintKind::Mate(mate) => ConstraintKind::Mate(mate.try_map(read, slot)?),
                ConstraintKind::Value(value) => ConstraintKind::Value(Value {
                    mate: value.mate,
                    coord: value.coord,
                    by: slot(&value.by)?,
                }),
            },
        })
    }
}

/// **A value** (FORK-S3M, ruling C-2): one freedom its mate leaves —
/// a slide or a spin, charted on the two bodies' own coordinates as
/// the placement carries them — set to a `Length` or an `Angle`.
///
/// The chart is the two bodies' own frames, each swung onto its side
/// of the mate by the shortest turn carrying its own `z` to the mated
/// direction ([`reference_about`]): a slide is the copy's origin
/// measured from the target's along the target's swung `x`, `y` or
/// `z`; a spin is the angle about the mated direction from the
/// target's swung `x` to the copy's. Each is a function of the
/// relative pose alone, so no order of the values is chosen, and zero
/// says the two bodies' own coordinates agree as far as the mate
/// allows.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, bound = "")]
pub struct Value<S: crate::Slot = crate::VarId> {
    /// The mate whose freedom it sets, by its id in the same bundle.
    pub mate: crate::node::ConstraintId,
    /// Which freedom.
    pub coord: Coord,
    /// The number: a `Length` for a slide, an `Angle` for a spin.
    pub by: S,
}

/// **Which freedom of its mate a value sets.**
///
/// The freedoms each kind of mate leaves: a plane mate slides along
/// `x` and `y` and spins; an axis mate slides along `z` (its axis) and
/// spins; a direction mate slides along all three and spins (its
/// residual, `Parallel`); a point mate's rotation has no chart, and a
/// frame mate leaves nothing.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Coord {
    /// A slide along the target's swung axis.
    Slide(SlideAxis),
    /// A spin about the mated direction.
    Spin,
}

/// **The axis of the target's swung frame a slide runs along.**
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum SlideAxis {
    /// The swung `x`.
    X,
    /// The swung `y`.
    Y,
    /// The swung `z`: the mated direction.
    Z,
}

impl Coord {
    /// The dimension of the number that sets it.
    #[must_use]
    pub fn dimension(self) -> crate::expr::Dimension {
        match self {
            Self::Slide(_) => crate::expr::Dimension::Length,
            Self::Spin => crate::expr::Dimension::Angle,
        }
    }

    /// Its name, for messages.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Slide(SlideAxis::X) => "slide along x",
            Self::Slide(SlideAxis::Y) => "slide along y",
            Self::Slide(SlideAxis::Z) => "slide along z",
            Self::Spin => "spin",
        }
    }

    /// **Whether a mate of `kind` leaves this freedom**, from the kind
    /// alone (ruling C-2). `Err(NoChart)` for a point mate's rotation.
    ///
    /// # Errors
    ///
    /// [`FreedomRefusal`]: the kind leaves no such freedom, or leaves it
    /// with no chart.
    pub fn admitted_by(self, kind: crate::VarKind) -> Result<(), FreedomRefusal> {
        use crate::VarKind as K;
        match (kind, self) {
            (K::Plane, Self::Slide(SlideAxis::X | SlideAxis::Y) | Self::Spin)
            | (K::Axis, Self::Slide(SlideAxis::Z) | Self::Spin)
            | (K::Direction, _) => Ok(()),
            (K::Point, Self::Spin) => Err(FreedomRefusal::NoChart),
            _ => Err(FreedomRefusal::NotAFreedom),
        }
    }
}

/// **Why a value's freedom has no chart** ([`MateFault::NoChart`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoChartWhy {
    /// A point mate's rotation: three freedoms with no direction to
    /// measure one about. A direction mate lowers it first.
    PointRotation,
    /// The mate's `to` reads two bodies: each has its own frame, and
    /// the chart is the one body's.
    TwoBodies,
}

/// **Why a mate's kind leaves a value no freedom to set.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreedomRefusal {
    /// The kind fixes it.
    NotAFreedom,
    /// The kind leaves it, with no chart to measure it on: a lone point
    /// mate's rotation.
    NoChart,
}

/// **A body's frame swung onto a mate's direction** (ruling C-1): the
/// shortest turn carrying the body's own `z` to `axis`, `axis` given in
/// the body's own coordinates, as the turn's `x` column — the body's
/// reference direction about `axis`. Its `z` is `axis` and its `y`
/// is `axis × x`.
///
/// For `axis = (a, b, c)` the swung `x` is
/// `(1 − a²/(1+c), −ab/(1+c), −a)`, with `1 + c` taken as
/// `(a² + b²)/(1 − c)` where `c < 0`, so the antipode's neighbourhood
/// loses no digit to cancellation. At the antipode itself (`axis = −z`)
/// no turn is shortest, and the half-turn about the body's own `x` is
/// taken: `x` stays. The only predicate is the decided sign of `1 + c`,
/// read as the chord from `−z` levered over `arm`; a chord the band
/// cannot decide refuses ([`crate::pose::PoseConstruction::Chart`]).
///
/// # Errors
///
/// The band's escalation, as [`crate::NodeErrorKind::PoseDegenerate`].
pub fn reference_about<T: SolveScalar>(
    axis: geom_core::UnitVec3<T>,
    arm: f64,
    band: geom_core::predicate::Band,
) -> Result<geom_core::linalg::Vec3<T>, crate::NodeErrorKind> {
    let d = axis.get();
    let (a, b, c) = (d.x, d.y, d.z);
    let one = T::one();
    let across = a.powi(2) + b.powi(2);
    // Which formula is a choice of arithmetic, not a verdict: the two
    // are one number, and the first loses no digit near the antipode.
    let one_plus_c = if c.quoted() < 0.0 {
        across / (one - c)
    } else {
        one + c
    };
    // The chord from `−z`: |d + z| = √(2(1 + c)), levered over `arm`.
    let chord = ((one + one) * one_plus_c).sqrt();
    let sign = geom_core::k_stats::decide(
        "mate_chart_reference_antipode",
        geom_core::predicate::Margin::levered(chord, T::from_computed(arm)),
        band,
    )
    .map_err(|_| crate::NodeErrorKind::PoseDegenerate {
        construction: crate::pose::PoseConstruction::Chart,
    })?;
    if sign == geom_core::predicate::Sign::Zero {
        return Ok(geom_core::linalg::Vec3::new(one, T::zero(), T::zero()));
    }
    Ok(geom_core::linalg::Vec3::new(
        one - a.powi(2) / one_plus_c,
        T::zero() - a * b / one_plus_c,
        T::zero() - a,
    ))
}

/// **The recourse a [`MateFault::Overconstrained`] ends on**: what an
/// author does about a mate the rest of its bundle already fixes.
pub const OVERCONSTRAINED_RECOURSE: &str = "delete one of the named constraints; where the two \
     copies also meet, an assertion says so";

/// **The recourse a [`MateFault::Contradictory`] ends on**: what an
/// author does about two declarations that admit no common pose.
///
/// One sentence for both shapes the arm renders — a PAIR of mates and
/// one mate named on both sides — because the repair is the same in
/// both and a sentence naming "one of the two" is false of the second. The rung is the solve door: cosets are
/// intersected exactly, so nothing here is a tolerance to widen and no
/// pose is averaged out of the two.
pub const CONTRADICTORY_RECOURSE: &str = "delete the mate that was not meant, or re-author the \
                                          datum that is wrong; the solve intersects cosets \
                                          exactly and never averages two declarations";

/// **The v1 class restriction, named.** `Fit` is specified and not
/// built; a mate cannot declare a designed clearance until the kernel
/// variant lands with its first consumer, and AQ6 is where the
/// cross-document detail is still open.
pub const CLASS_DEFERRAL: &str = "v1 mates SOLVE Rest and Tangent and ASSEMBLE Rest alone; the \
                                  cross-document detail of a designed clearance is undischarged";

/// **The recourse a [`crate::MintRefusal::NoAtRestRecord`] ends on**:
/// what an author does about a class that solves and has no record to
/// be verified by at rest.
///
/// The arm already quotes the table's reason for THIS class; this is
/// the repair, and it names the rung the way [`CLASS_DEFERRAL`] does —
/// `Rest` is the one class v1 carries all the way to the gate, and a
/// curved contact verified at rest is outside v1 rather than a
/// tolerance away.
pub const NO_AT_REST_RECORD_RECOURSE: &str = "declare the contact as a Rest, the one class v1 \
                                              mints and verifies at rest; a curved contact \
                                              verified at rest is outside v1 and is not built";

/// **How far a contact class gets in v1** — the whole class policy as
/// a value, read by both doors that enforce it.
///
/// The two doors want different things of a class: the solve needs a
/// coset the alignment table can fold, the assembly gate's mint needs
/// a KERNEL RECORD TYPE that can carry the declaration at rest. A
/// class can satisfy the first and not the second, so the admitted
/// sets differ — and the gap is stated here, once, rather than
/// asserted separately by each door's own match (which is how they
/// drift apart, and how a door comes to advertise what it cannot
/// execute).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassAdmission {
    /// Both doors: the solve folds it, and [`crate::assemble`] mints
    /// it into the product's record set as the kernel's own record
    /// type. Whether the census then CERTIFIES that record is the
    /// census's verdict, not this table's.
    Mints,
    /// The solve door only. No kernel record carries this class at
    /// rest, so [`crate::assemble`] refuses with a
    /// [`crate::MintRefusal::NoAtRestRecord`] row naming the mate — a
    /// solved placement that cannot be verified at rest, never a
    /// record minted with an invented witness.
    NoAtRestRecord {
        /// Why THIS class has none, in its own terms. Carried here so
        /// the mint door's message is the table's, never one class's
        /// reason rendered confidently over another's refusal.
        why: &'static str,
    },
    /// Neither: outside v1's vocabulary, so the solve door refuses
    /// [`MateFault::ClassNotAdmitted`]. Reaching the mint door means a
    /// mate of this class was live, which the solve door does not
    /// permit — so the mint refuses it too, with the deferral, rather
    /// than assuming the chain held.
    NotAdmitted,
}

impl ClassAdmission {
    /// Why the assembly gate carries nothing at rest for this class.
    ///
    /// Total, so the mint door never has to choose a sentence: a class
    /// that mints has no such reason and says so.
    pub fn no_record_reason(self) -> &'static str {
        match self {
            Self::Mints => "the class mints at rest",
            Self::NoAtRestRecord { why } => why,
            Self::NotAdmitted => CLASS_DEFERRAL,
        }
    }
}

/// The class table itself ([`ClassAdmission`]).
///
/// INVARIANT: every door that ENFORCES the class policy reads it
/// here, so the admitted sets cannot drift apart.
/// `ContactClass` is `#[non_exhaustive]`, so a class the kernel grows
/// arrives in the wildcard arm as [`ClassAdmission::NotAdmitted`] —
/// deferred by default, admitted only by an edit HERE that both doors
/// then obey.
pub fn class_admission(class: ContactClass) -> ClassAdmission {
    match class {
        // A rest between two placed faces IS a face–face record.
        ContactClass::Rest => ClassAdmission::Mints,
        ContactClass::Tangent => ClassAdmission::NoAtRestRecord {
            why: "a tangency between placed faces is a face-face record on two carriers, and \
                  the census at rest decides nothing along their locus for the record to cite",
        },
        _ => ClassAdmission::NotAdmitted,
    }
}

/// Why a lever could not be formed for a mate: one of its parts' reach
/// ([`MateReach::reach`]) is not in hand, the member stands on no
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

/// **What a levered clash measured, and the arm that carried it to a
/// length.** The product is the deviation; the halves are what the
/// sentence prints, because a stored product beside them would assert
/// an identity nothing enforces ([`Lever::deviation`] computes it).
///
/// A closed enum, and deliberately not a value beside a unit string:
/// its arm is the kind of number the solve levers, and another kind is
/// a new arm with its own sentence, never a free string a site could
/// set to a length.
///
/// The arm is the one the predicate was decided over: the largest
/// lever of the mates folded so far.
///
/// The solve measures a residual at its own scalar `T` and decides it
/// there; a refusal carries it at `f64` (the default).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lever<T = f64> {
    /// A dimensionless residual — a sine, a cosine, a Frobenius
    /// departure from the identity, a reachability defect — named by
    /// the predicate that measured it.
    Residual {
        /// The residual, a pure number.
        value: T,
        /// The arm, in metres.
        arm: f64,
    },
}

impl<T> Lever<T> {
    /// The arm, in metres, whichever kind of number it levered.
    pub fn arm(self) -> f64 {
        match self {
            Self::Residual { arm, .. } => arm,
        }
    }
}

impl<T: geom_core::Real> Lever<T> {
    /// **The margin this lever decides**: the pure number levered by
    /// the arm through [`Margin::levered`] — the ONE home of that
    /// multiplication for every levered predicate in the solve, at
    /// whichever scalar it runs, so the number a refusal quotes and
    /// the number the funnel decided are one value.
    pub fn margin(self) -> Margin<T> {
        match self {
            Self::Residual { value, arm } => Margin::levered(value, T::from_computed(arm)),
        }
    }
}

impl Lever {
    /// The deviation the lever measured, in metres: [`Self::margin`]'s
    /// value, computed here and nowhere stored.
    pub fn deviation(self) -> f64 {
        self.margin().value()
    }
}

/// **What a contradictory refusal measured** — closed over the three
/// ways a membership predicate refuses, so the refusal carries the
/// measurement and nothing that could disagree with it: no metre
/// figure stored beside the halves it is the product of, no sentinel
/// read back out of a number.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Clash {
    /// The predicate decided the empty intersection STRUCTURALLY
    /// (`mate_member_empty`): the empty set holds nothing and no
    /// margin decides that, so there is no measurement at all.
    Structural,
    /// A length the predicate measured outright.
    Length {
        /// The margin that should have been zero and was not, in
        /// metres.
        metres: f64,
    },
    /// A pure number the predicate levered by an arm; the deviation
    /// is the product ([`Lever::deviation`]).
    Levered(Lever),
}

impl Clash {
    /// The deviation in metres, when the predicate measured one: a
    /// length verbatim, a lever's product, and `None` for the
    /// structural refusal.
    pub fn deviation(self) -> Option<f64> {
        match self {
            Self::Structural => None,
            Self::Length { metres } => Some(metres),
            Self::Levered(lever) => Some(lever.deviation()),
        }
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

    /// A class outside v1's admitted vocabulary — `Fit { gap }` today,
    /// any later addition tomorrow. The message carries the kernel's
    /// own deferral sentence VERBATIM ([`topo::FIT_DEFERRAL`]) rather
    /// than a paraphrase of it.
    ClassNotAdmitted {
        /// The mate that declared it.
        mate: MateRef,
    },

    /// A case-split predicate landed in the ambiguity band — the typed
    /// escalation, never a silent pick.
    Indeterminate {
        /// The mate being folded when it fired.
        mate: MateRef,
        /// The predicate's diagnostics (it names itself).
        diag: Box<Indeterminate>,
    },
    /// The pair's mates meet at a pose the floating-point format
    /// cannot hold: the translation the fold solved for has no finite
    /// length. The mates can both hold; the place they meet is past the
    /// session's range.
    PoseOutOfRange {
        /// The mate already folded. **Equal to `added` when the pair
        /// has no earlier mate**, as [`Self::Contradictory`] spells it.
        held: MateRef,
        /// The mate whose intersection named the meeting point.
        added: MateRef,
    },
    /// The pair's mates intersect to the EMPTY coset (A11 rule 1's
    /// CONTRADICTORY): names both mates, the predicate that failed,
    /// and the measured clash.
    Contradictory {
        /// The mate already folded. **Equal to `added` when one mate
        /// contradicts ITSELF**: a mate whose own two poses admit no
        /// common pose dies by the same rule, and naming it on both
        /// sides is how that shape reaches this variant.
        held: MateRef,
        /// The mate whose intersection died against it.
        added: MateRef,
        /// The predicate that decided against them.
        predicate: &'static str,
        /// What it measured: a length outright, a pure number levered
        /// by an arm, or nothing — the structural refusal.
        ///
        /// A lever's arm is the mate's lever as the solve forms it —
        /// the copy's reach plus its poses' own distances from their
        /// origins, `ASSEMBLY.md` A11 rule 5's qualifier — and NOT a
        /// contact feature, so a message
        /// that names it is naming the parts' scale and nothing in the
        /// model.
        clash: Clash,
    },
    /// **A mate beyond the pin** (D10; FORK-S3O): the placement's
    /// bundle already pins its copy without each named constraint, so
    /// each is an equation the others fix. A mate places and never
    /// checks, so none of them is verified and kept: the insert door
    /// refuses the constraint being added to a bundle whose fold is
    /// already trivial, and the solve, for a state no door saw (a load,
    /// a rebind), names every constraint whose removal leaves the
    /// fold's codimension unchanged. Constraints have no order, so
    /// none of them is "the later one".
    Overconstrained {
        /// The placement.
        placement: RecipeNodeId,
        /// Every constraint the rest of the bundle already fixes.
        constraints: Vec<crate::node::ConstraintId>,
        /// What the rest of the bundle leaves, which is nothing.
        held: Subgroup,
    },
    /// **A bundle relates its copy to two spaces**: its mates' targets
    /// read geometry of two different spaces, or one the world and one
    /// a space that does not reach it. A copy joins one space, so its
    /// mates fold in that space's coordinates.
    TargetsDiffer {
        /// The placement.
        placement: RecipeNodeId,
        /// Its constraints, in bundle order.
        constraints: Vec<crate::node::ConstraintId>,
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
    /// **A mate's world read is no pose** ([`WorldRead::kind`]).
    WorldRead {
        /// The mate.
        mate: MateRef,
        /// The construction its kinds refuse.
        fault: WorldReadFault,
    },
    /// **A value names no mate of its bundle** (ruling C-2): the id is
    /// no constraint of the placement, or a value's.
    ValueNamesNoMate {
        /// The value.
        value: MateRef,
        /// The id it names.
        mate: crate::node::ConstraintId,
    },
    /// **A value sets a freedom its mate does not leave** (ruling C-2):
    /// the mate's kind fixes it (`kind` is that kind — refused at the
    /// door), or the bundle's other constraints already fix it (`None`
    /// — the placement's own refusal where it evaluates, until F's
    /// measure-free door).
    NotAFreedom {
        /// The value.
        value: MateRef,
        /// The freedom it sets.
        coord: Coord,
        /// The mate's kind, where the kind alone fixes it.
        kind: Option<crate::VarKind>,
    },
    /// **A value's freedom has no chart** (ruling C-1): a lone point
    /// mate's rotation, or a mate whose `to` reads two bodies, whose
    /// own frames do not agree on which reference to measure from.
    NoChart {
        /// The value.
        value: MateRef,
        /// Why.
        why: NoChartWhy,
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

/// The measurement a refusal quotes, as the tail of its sentence:
/// what was measured where `there` would have had to hold.
fn write_clash(f: &mut core::fmt::Formatter<'_>, clash: Clash, there: &str) -> core::fmt::Result {
    match clash {
        Clash::Structural => write!(
            f,
            "found the cosets meet in the empty set — a structural refusal, with no margin to \
             measure"
        ),
        Clash::Length { metres } if metres.is_finite() => {
            write!(f, "measured a clash of {metres} m where {there}")
        }
        // A length that is not finite is not one.
        Clash::Length { metres } => write!(
            f,
            "measured a clash that is not a finite length ({metres}) where {there}"
        ),
        Clash::Levered(lever @ Lever::Residual { value, arm }) => write!(
            f,
            "measured a dimensionless residual of {value} on a {arm} m arm, a deviation of {} m \
             where {there}",
            lever.deviation()
        ),
    }
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
            | Self::ClassNotAdmitted { .. }
            | Self::Indeterminate { .. }
            | Self::PoseOutOfRange { .. }
            | Self::Contradictory { .. }
            | Self::Overconstrained { .. }
            | Self::TargetsDiffer { .. }
            | Self::NotOnTheCopy { .. }
            | Self::Unleverable { .. }
            | Self::KindsDiffer { .. }
            | Self::WorldRead { .. }
            | Self::ValueNamesNoMate { .. }
            | Self::NotAFreedom { .. }
            | Self::NoChart { .. } => None,
        }
    }

    /// **Every refusal this fault carries, outermost first**, each with
    /// the document its node is in — the mate's own at the first level
    /// ([`crate::NodeErrorKind::carried_chain`]).
    pub fn carried_chain(&self) -> crate::CarriedChain<'_> {
        crate::CarriedChain::from_first(self.carried(), crate::CarriedIn::ThisDocument)
    }
}

/// The predicate that decides the EMPTY intersection. It is the one
/// name in the membership vocabulary that reports no measurement — the
/// empty set holds nothing and no margin decides that — so it is a
/// shared constant rather than a literal at each site, and every door
/// that needs to know reads THIS rather than inspecting a margin.
pub(crate) const MATE_MEMBER_EMPTY: &str = "mate_member_empty";

/// **A predicate the solve refuses a contradiction on: its funnel name
/// and what refusing it found, in words, declared together.** Every
/// site that decides one passes [`Refuted::name`], and the sentence
/// says [`Refuted::found`] in its place, so a predicate cannot be
/// decided under a name the sentence has no words for — both are arms
/// of one exhaustive match. The structural empty intersection
/// ([`MATE_MEMBER_EMPTY`]) is not here: no margin decides it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refuted {
    /// A coset admitting no rotation: the candidate's rotation is the
    /// identity.
    RotationIdentity,
    /// A coset admitting no motion: the candidate's translation is zero.
    TranslationZero,
    /// A prismatic coset: the translation runs along its direction.
    TranslationAlong,
    /// A coset about an axis: the rotation fixes that axis.
    AxisFixed,
    /// A planar coset: the translation stays in the plane.
    TranslationInPlane,
    /// A cylindrical coset: the axis point stays on the axis.
    PointOnAxis,
    /// A revolute coset: the axis point stays put.
    PointFixed,
    /// Two one-axis rotation constraints: one rotation reaches both.
    TwoAxisReachable,
}

impl Refuted {
    /// Every predicate, once.
    pub const ALL: [Self; 8] = [
        Self::RotationIdentity,
        Self::TranslationZero,
        Self::TranslationAlong,
        Self::AxisFixed,
        Self::TranslationInPlane,
        Self::PointOnAxis,
        Self::PointFixed,
        Self::TwoAxisReachable,
    ];

    /// The funnel's name for it — routing, which a refusal carries in
    /// its payload and never in its sentence.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::RotationIdentity => "mate_member_rotation_identity",
            Self::TranslationZero => "mate_member_translation_zero",
            Self::TranslationAlong => "mate_member_translation_along",
            Self::AxisFixed => "mate_member_axis_fixed",
            Self::TranslationInPlane => "mate_member_translation_in_plane",
            Self::PointOnAxis => "mate_member_point_on_axis",
            Self::PointFixed => "mate_member_point_fixed",
            Self::TwoAxisReachable => "mate_rotation_two_axis_reachable",
        }
    }

    /// What refusing it found, as a clause of the contradiction's
    /// sentence.
    #[must_use]
    pub const fn found(self) -> &'static str {
        match self {
            Self::RotationIdentity => "the relative rotation is not the identity",
            Self::TranslationZero => "the relative translation is not zero",
            Self::TranslationAlong => "the translation leaves the shared direction",
            Self::AxisFixed => "the rotation moves the shared axis",
            Self::TranslationInPlane => "the translation leaves the shared plane",
            Self::PointOnAxis => "the axis point leaves the shared axis",
            Self::PointFixed => "the shared point moves",
            Self::TwoAxisReachable => "no one rotation aligns both axes",
        }
    }

    /// The predicate a refusal's payload names, when it is one of these.
    #[must_use]
    pub fn of(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|r| r.name() == name)
    }
}

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
            Self::ClassNotAdmitted { mate } => write!(
                f,
                "{}'s contact class is not admitted in v1 — {}. Recourse: delete the \
                 mate, and insert it again declaring a Rest",
                mate_(*mate),
                topo::FIT_DEFERRAL
            ),
            Self::Indeterminate { mate, diag } => write!(
                f,
                "{}: a case split could not be decided — {}. Recourse: {}",
                mate_(*mate),
                diag.payload(),
                geom_core::COINCIDENCE_RECOURSE
            ),
            Self::PoseOutOfRange { held, added } => {
                if held == added {
                    write!(f, "{}'s constraints meet", mate_(*added))?;
                } else {
                    write!(f, "{} and {} meet", mate_(*held), mate_(*added))?;
                }
                write!(
                    f,
                    " at a point further away than the solve can measure a distance to. \
                     Recourse: {}",
                    geom_core::RANGE_RECOURSE
                )
            }
            Self::Contradictory {
                held,
                added,
                predicate,
                clash,
            } => {
                // One mate named on BOTH sides is a mate contradicting
                // itself, and "mate 6 and mate 6" reads as an indexing
                // fault rather than as the shape the payload states.
                if held == added {
                    write!(
                        f,
                        "{} contradicts itself — the constraints it declares admit no \
                         common pose",
                        mate_(*held)
                    )?;
                } else {
                    write!(f, "{} and {} cannot both hold", mate_(*held), mate_(*added))?;
                }
                // The predicate's name is routing and rides the
                // payload; the sentence says what it found in words.
                f.write_str(": ")?;
                if let Some(refuted) = Refuted::of(predicate) {
                    write!(f, "{} — ", refuted.found())?;
                }
                f.write_str("the solve ")?;
                // WHETHER there is a measurement to report, and of
                // which kind, is the predicate's fact and the type
                // carries it: a levered clash prints the product of
                // the two halves it shows, computed there, so the
                // sentence cannot assert an identity the payload
                // failed to keep.
                write_clash(f, *clash, "the cosets would have had to meet")?;
                // The repair is the same whichever measurement the
                // predicate had to report, so it is stated once.
                write!(f, ". Recourse: {CONTRADICTORY_RECOURSE}")
            }
            Self::Overconstrained {
                placement,
                constraints,
                held,
            } => {
                let (which, rest) = match constraints.as_slice() {
                    [_] => ("constraint", "the others leave"),
                    _ => ("any one of constraints", "without any one the others leave"),
                };
                write!(
                    f,
                    "{}'s bundle pins its copy even without {which} {}, since {rest} {}. \
                     Recourse: {OVERCONSTRAINED_RECOURSE}",
                    by.node(*placement),
                    ids(*placement, constraints),
                    held.describe()
                )
            }
            Self::TargetsDiffer {
                placement,
                constraints,
            } => write!(
                f,
                "{}'s constraints {} read targets in more than one space; a copy joins one \
                 space, so its mates fold in that space's coordinates. Recourse: mate the copy \
                 against poses of one space",
                by.node(*placement),
                ids(*placement, constraints)
            ),
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
            Self::WorldRead { mate, fault } => write!(
                f,
                "{}: {fault}. Recourse: re-author the world read",
                mate_(*mate)
            ),
            Self::ValueNamesNoMate { value, mate } => write!(
                f,
                "{} names constraint {mate}, which is no mate of its placement's bundle. \
                 Recourse: name a mate the bundle holds",
                mate_(*value)
            ),
            Self::NotAFreedom {
                value,
                coord,
                kind: Some(kind),
            } => write!(
                f,
                "{} sets the {} of a {kind} mate, which the mate fixes. Recourse: set a freedom \
                 the mate leaves",
                mate_(*value),
                coord.name()
            ),
            Self::NotAFreedom {
                value,
                coord,
                kind: None,
            } => write!(
                f,
                "{} sets its mate's {}, which the bundle's other constraints already fix. \
                 Recourse: remove the value, or the constraint that fixes it",
                mate_(*value),
                coord.name()
            ),
            Self::NoChart {
                value,
                why: NoChartWhy::PointRotation,
            } => write!(
                f,
                "{} sets a point mate's rotation, which has no chart: no direction to measure \
                 a spin about. Recourse: add a direction mate and set its spin",
                mate_(*value)
            ),
            Self::NoChart {
                value,
                why: NoChartWhy::TwoBodies,
            } => write!(
                f,
                "{} is charted on its mate's target body, and the mate's `to` reads two. \
                 Recourse: read `to` off one body, or pin the copy by a frame mate",
                mate_(*value)
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
