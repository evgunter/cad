//! **A body value and the frame it computes in** (D10
//! `[placement.frame]`).
//!
//! A body value is [`Framed`]: the body in its own coordinates, shared
//! by `Arc`, and one map from them into its space's coordinates. A copy
//! keeps its source's `Arc` under the composed map and is never
//! rebuilt.
//!
//! **The computing frame is a conditioning choice.** `[placement.frame]`
//! has an operation over operands in one space compute in a frame
//! "chosen so its arithmetic is well conditioned", the first operand's
//! "where conditioning does not decide". The choice here is
//! [`Framed::computing_among`]: the first operand in its space's
//! coordinates ([`Frame::Own`], read through no map, so no widened
//! rotation's inverse reaches the others on an enclosure lane), else the
//! first operand. It reads the frames' structure and no value, so the
//! `f64` witness and every lane over a box decide in one frame. Every
//! operation over two or more operands of one space takes it — the
//! booleans' fold, subtract and distribute, measure, the product gather,
//! clearance and its witness rebuild, the coincidence replay's carrier
//! verdict — and reads every other operand through
//! [`Framed::relative`], the one door between two operands of a space;
//! its result is in the frame it computed in.
//!
//! A construction computes in its own coordinates ([`Frame::Own`]). A
//! one-input construction over a body — blend, shell, split, mirror —
//! computes in that body's frame, there being no second operand to
//! condition against.
//!
//! Nothing here composes a body into its space's coordinates for a
//! reader outside the model: that is `crate::mate::in_space`'s, and only
//! export, the façade's read-outs (`pncad::export`) and the viewer's
//! picture call it.

use std::sync::Arc;

use geom_core::{Real, RigidMotion, RigidMotionError, Tol};
use topo::Body;

/// **The map from a body's coordinates into its space's.**
#[derive(Debug, Clone)]
pub(crate) enum Frame<T: Real> {
    /// The identity: the body is in its space's coordinates. A
    /// construction's frame, and a copy's under the identity bundle as
    /// written.
    Own,
    /// A rigid map, shared: every value computed in one frame holds the
    /// same `Arc`, so two such values relate by the identity without
    /// arithmetic.
    Map(Arc<RigidMotion<T>>),
}

impl<T: Real> Frame<T> {
    /// **Whether the two are one frame**: both `Own`, or one shared map.
    fn same(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Own, Self::Own) => true,
            (Self::Map(a), Self::Map(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }

    /// The map, `None` for [`Frame::Own`].
    pub(crate) fn map(&self) -> Option<&RigidMotion<T>> {
        match self {
            Self::Own => None,
            Self::Map(m) => Some(m),
        }
    }

    /// **This frame moved by `motion`**: `motion ∘ self`.
    ///
    /// # Errors
    ///
    /// The composition's overflow.
    pub(crate) fn moved(&self, motion: &RigidMotion<T>) -> Result<Self, RigidMotionError> {
        Ok(Self::Map(Arc::new(match self {
            Self::Own => *motion,
            Self::Map(inner) => motion.compose(**inner)?,
        })))
    }

    /// **`self⁻¹ ∘ other`**: the map from `other`'s coordinates into
    /// `self`'s, `None` where the two are one frame.
    ///
    /// # Errors
    ///
    /// The inverse's or the composition's overflow.
    pub(crate) fn relative(
        &self,
        other: &Self,
    ) -> Result<Option<RigidMotion<T>>, RigidMotionError> {
        if self.same(other) {
            return Ok(None);
        }
        Ok(Some(match (self.map(), other.map()) {
            (None, Some(b)) => *b,
            (Some(a), None) => a.inverse()?,
            (Some(a), Some(b)) => a.inverse()?.compose(*b)?,
            (None, None) => unreachable!("two `Own` frames are one frame"),
        }))
    }
}

/// **The body a value holds**: a finished body, or its scalar's leaf
/// ([`topo::AtRestPolicy::Leaf`]) where a sweep door, a placement's
/// image or an assemble's graft built it. Read as a body through
/// [`Deref`](core::ops::Deref); a consumer that takes a finished body
/// takes it through [`Held::finished`], the evaluator's one taker door.
///
/// At `f64` and `Probe` both arms hold a finished body. At the box
/// scalars a leaf is tier 2 alone (the switch at
/// [`topo::AtRestPolicy::leaf`]), and a finished body stays finished:
/// its taker does not run tier 3 again, at either lane.
#[derive(Debug)]
pub enum Held<T: topo::AtRestPolicy> {
    /// A finished body: a Boolean's, a split's, a shell's, a blend's.
    Finished(Arc<T::Finished>),
    /// The scalar's leaf.
    Leaf(Arc<T::Leaf>),
}

impl<T: topo::AtRestPolicy> Clone for Held<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Finished(body) => Self::Finished(Arc::clone(body)),
            Self::Leaf(body) => Self::Leaf(Arc::clone(body)),
        }
    }
}

impl<T: topo::AtRestPolicy> core::ops::Deref for Held<T> {
    type Target = Body<T>;

    fn deref(&self) -> &Body<T> {
        match self {
            Self::Finished(body) => body,
            Self::Leaf(body) => body,
        }
    }
}

impl<T: topo::AtRestPolicy> Held<T> {
    /// **The finished body**, as a door that takes finished operands
    /// takes it: the held one, or the leaf through the one taker door
    /// ([`topo::take`]: its finish record relogged at `f64` and `Probe`,
    /// tier 3 run and logged at the box scalars).
    ///
    /// # Errors
    ///
    /// The finish's findings, at the box scalars; an evaluator node
    /// reads them as [`super::NodeErrorKind::UnfinishedLeaf`].
    pub fn finished(&self, tol: Tol) -> Result<Arc<T::Finished>, topo::NotFinished> {
        match self {
            Self::Finished(body) => Ok(Arc::clone(body)),
            Self::Leaf(leaf) => topo::take::<T>(leaf, tol),
        }
    }

    /// Whether the two hold one body, by `Arc`.
    #[must_use]
    pub fn same(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Finished(a), Self::Finished(b)) => Arc::ptr_eq(a, b),
            (Self::Leaf(a), Self::Leaf(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl Held<f64> {
    /// **The finished body at `f64`**, where both arms hold one: read as
    /// it is, for an `f64`-only reader that is no taker (no relog).
    #[must_use]
    pub fn at_f64(&self) -> &topo::AtRestBody<f64> {
        self.arc()
    }

    /// [`Self::at_f64`]'s `Arc`: what two values are compared by.
    #[must_use]
    pub fn arc(&self) -> &Arc<topo::AtRestBody<f64>> {
        match self {
            Self::Finished(body) | Self::Leaf(body) => body,
        }
    }
}

/// **A body value** (the copies ruling): the body in its own
/// coordinates ([`Held`]) and the map from them into its space's.
#[derive(Debug, Clone)]
pub struct Framed<T: topo::AtRestPolicy> {
    body: Held<T>,
    frame: Frame<T>,
}

impl<T: topo::AtRestPolicy> Framed<T> {
    /// A finished body in its space's coordinates: a construction's,
    /// computed in its own.
    #[must_use]
    pub fn own(body: Arc<T::Finished>) -> Self {
        Self::held(Held::Finished(body), Frame::Own)
    }

    /// A leaf in its space's coordinates ([`topo::AtRestPolicy::leaf`]).
    #[must_use]
    pub fn own_leaf(body: Arc<T::Leaf>) -> Self {
        Self::held(Held::Leaf(body), Frame::Own)
    }

    /// A finished `body` in `frame`: an operation's result, computed in
    /// the frame of the operand it computed in.
    pub(crate) fn in_frame(body: Arc<T::Finished>, frame: Frame<T>) -> Self {
        Self::held(Held::Finished(body), frame)
    }

    /// A leaf `body` in `frame`.
    pub(crate) fn in_frame_leaf(body: Arc<T::Leaf>, frame: Frame<T>) -> Self {
        Self::held(Held::Leaf(body), frame)
    }

    /// `body` in `frame`.
    pub(crate) fn held(body: Held<T>, frame: Frame<T>) -> Self {
        Self { body, frame }
    }

    /// **The body, in its own coordinates.** A reader that relates it to
    /// another body of its space reads that one through
    /// [`Self::relative`].
    #[must_use]
    pub fn body(&self) -> &Body<T> {
        &self.body
    }

    /// What it holds ([`Held`]); [`Held::finished`] is the taker door.
    #[must_use]
    pub fn holding(&self) -> &Held<T> {
        &self.body
    }

    /// **Another body in this one's frame** (`test-support`): for rows
    /// that write a value by hand where an evaluation put one.
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn with_body(&self, body: Arc<T::Finished>) -> Self {
        Self {
            body: Held::Finished(body),
            frame: self.frame.clone(),
        }
    }

    /// The frame it is in.
    pub(crate) fn frame(&self) -> &Frame<T> {
        &self.frame
    }

    /// **Its space's origin, in the body's coordinates**: where a reach
    /// in the space is measured from.
    ///
    /// # Errors
    ///
    /// The inverse's overflow.
    pub(crate) fn space_origin(&self) -> Result<geom_core::Point3<T>, RigidMotionError> {
        Ok(match self.frame.map() {
            None => geom_core::Point3::origin(),
            Some(map) => map.inverse()?.origin(),
        })
    }

    /// **A point of the body, in its space's coordinates**: what a
    /// reader that states a position in the space compares against.
    pub(crate) fn point_in_space(&self, p: geom_core::Point3<T>) -> geom_core::Point3<T> {
        self.frame.map().map_or(p, |map| map.transform_point(p))
    }

    /// **A direction of the body, in its space's coordinates.**
    pub(crate) fn vec_in_space(&self, v: geom_core::Vec3<T>) -> geom_core::Vec3<T> {
        self.frame.map().map_or(v, |map| map.transform_vec(v))
    }

    /// **A heading stated in its space, in the body's coordinates**: what
    /// a reader measuring the body along a direction of the space (a
    /// duplicate's step) measures it along, computing in the body's
    /// frame.
    ///
    /// # Errors
    ///
    /// The inverse's overflow.
    pub fn heading_in_own(
        &self,
        v: geom_core::Vec3<T>,
    ) -> Result<geom_core::Vec3<T>, RigidMotionError> {
        Ok(match self.frame.map() {
            None => v,
            Some(map) => map.inverse()?.transform_vec(v),
        })
    }

    /// **The same body, its frame moved by `motion`**: a placement's
    /// copy, sharing its source's `Arc`.
    ///
    /// # Errors
    ///
    /// The composition's overflow.
    pub(crate) fn moved(&self, motion: &RigidMotion<T>) -> Result<Self, RigidMotionError> {
        Ok(Self {
            body: self.body.clone(),
            frame: self.frame.moved(motion)?,
        })
    }

    /// **Whether it is in its space's coordinates** ([`Frame::Own`]): a
    /// construction, or a copy under no map. Structural, so every scalar
    /// lane answers it alike.
    pub(crate) fn in_own_frame(&self) -> bool {
        matches!(self.frame, Frame::Own)
    }

    /// **The operand an operation over `operands` computes in**
    /// (`[placement.frame]`, conditioning): the first that is in its
    /// space's coordinates, so no such operand is read through another's
    /// map (a widened rotation's, on an enclosure lane); the first where
    /// none is. The choice reads the frames' structure and no value, so
    /// the `f64` witness and every lane over a box compute in one frame
    /// and take one set of decisions.
    pub fn computing_among<'a>(
        operands: impl IntoIterator<Item = &'a Self> + Clone,
    ) -> Option<&'a Self>
    where
        T: 'a,
    {
        operands
            .clone()
            .into_iter()
            .find(|f| f.in_own_frame())
            .or_else(|| operands.into_iter().next())
    }

    /// **The one door between two operands of one space**: the map from
    /// `other`'s body coordinates into this one's
    /// (`self.frame⁻¹ ∘ other.frame`), `None` where the two are in one
    /// frame and the map is the identity.
    ///
    /// # Errors
    ///
    /// The inverse's or the composition's overflow.
    pub fn relative(&self, other: &Self) -> Result<Option<RigidMotion<T>>, RigidMotionError> {
        self.frame.relative(&other.frame)
    }
}

impl<T: topo::AtRestPolicy> Framed<T> {
    /// **`other`'s body in this one's coordinates** ([`ReadIn`]): what it
    /// holds, as it holds it, where the two are in one frame; else its
    /// body mapped once ([`Self::relative`]), construction state no door
    /// has finished. A reader reads it as a body; a taker finishes it,
    /// once, through [`ReadIn::taken`].
    ///
    /// # Errors
    ///
    /// The relative map's overflow, or the kernel's own
    /// [`topo::transform::TransformError`].
    pub(crate) fn read_in(&self, other: &Self, tol: Tol) -> Result<ReadIn<T>, super::NodeErrorKind> {
        match self.relative(other).map_err(super::NodeErrorKind::Motion)? {
            None => Ok(ReadIn::Held(other.body.clone())),
            Some(map) => {
                super::counters::rigid_map();
                topo::transform::transform_rigid(&other.body, &map, tol)
                    .map(|mapped| ReadIn::Mapped(Arc::new(mapped)))
                    .map_err(super::NodeErrorKind::Transform)
            }
        }
    }
}

/// **A body read into a frame** ([`Framed::read_in`]): a value's own
/// [`Held`], or its body mapped into the frame, which is construction
/// state. ReadIn as a body through [`Deref`](core::ops::Deref); a door
/// that takes a finished body takes it through [`ReadIn::taken`], which
/// finishes a mapped body once, where it is taken.
#[derive(Debug)]
pub enum ReadIn<T: topo::AtRestPolicy> {
    /// What the value holds, as it holds it: the two are in one frame.
    Held(Held<T>),
    /// The value's body mapped into the frame: construction state.
    Mapped(Arc<Body<T>>),
}

impl<T: topo::AtRestPolicy> Clone for ReadIn<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Held(held) => Self::Held(held.clone()),
            Self::Mapped(body) => Self::Mapped(Arc::clone(body)),
        }
    }
}

impl<T: topo::AtRestPolicy> core::ops::Deref for ReadIn<T> {
    type Target = Body<T>;

    fn deref(&self) -> &Body<T> {
        match self {
            Self::Held(held) => held,
            Self::Mapped(body) => body,
        }
    }
}

impl<T: topo::AtRestPolicy> ReadIn<T> {
    /// **The finished body**, as a door that takes finished operands
    /// takes it: a held one through [`Held::finished`], a mapped one
    /// finished here ([`topo::finish`], logged where it is taken).
    ///
    /// # Errors
    ///
    /// The finish's findings; an evaluator node reads them as
    /// [`super::NodeErrorKind::UnfinishedLeaf`].
    pub fn taken(self, tol: Tol) -> Result<Arc<T::Finished>, topo::NotFinished> {
        match self {
            Self::Held(held) => held.finished(tol),
            Self::Mapped(body) => topo::finish::<T>(Arc::unwrap_or_clone(body), tol).map(Arc::new),
        }
    }

    /// The value's own [`Held`], where it was read as it holds it.
    #[must_use]
    pub fn held(&self) -> Option<&Held<T>> {
        match self {
            Self::Held(held) => Some(held),
            Self::Mapped(_) => None,
        }
    }
}
