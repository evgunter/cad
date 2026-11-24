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
//! coordinates ([`Frame::Own`], read through no map, so the others are
//! read through their own placements and not through a widened
//! rotation's inverse on an enclosure lane), else the first operand. The
//! choice reads the frames' structure and no value, so the `f64` witness
//! and every lane over a box decide in one frame. A reading within an
//! operand that asks it about another's point
//! ([`topo::Seat::into_own`]) does send that point through the inverse.
//! Every operation over operands of one space takes it — the gathers
//! (the `⊔`, a distribute's assembly, the product), the booleans' fold
//! and subtract, measure, clearance and its witness rebuild, the
//! coincidence replay's carrier verdict — and reads the other through
//! [`Framed::relative`], the one door between two operands of a space;
//! its result is in the frame it computed in. Copies more than the
//! session box apart stand past it in either one's frame, so an
//! operation over them refuses where each stands inside it
//! (`work/onehome/an-operation-over-far-apart-copies-has-no-conditioned-computing-frame.md`).
//!
//! **A reading moves no operand.** A reader that relates two operands
//! (measure, clearance and its witness, the replay's carrier verdict,
//! the at-rest quieting rule) takes the other through
//! [`Framed::seat`]: its body as it stands and the relative motion each
//! datum it takes off it goes through ([`topo::Seat`]). An operation
//! that builds one body out of its operands hands them to the kernel's
//! door where they stand, and the door moves each one not in its
//! computing frame: the Boolean pair, the n-ary fold and the at-rest
//! intersection through a placed operand ([`topo::Placed`],
//! `topo::boolean_placed`, `topo::nary_placed`), each gather of copies
//! into one aggregate (the `⊔`, a distribute's assembly, the product)
//! through [`Framed::graft_into`], which moves the copy's data into the
//! aggregate's and asks the session box of the image. Every such motion
//! is counted once, at [`Framed::moving_into`].
//!
//! A construction computes in its own coordinates ([`Frame::Own`]). A
//! one-input construction over a body — blend, shell, split, mirror —
//! computes in that body's frame, there being no second operand to
//! condition against.
//!
//! **Where a body stands is asked by the door that puts it there**
//! ([`Framed::asked_where_it_stands`]). The box is a cube about the
//! space's origin, so it holds the convex hull of any body it holds, and
//! a result that lies within its operand's hull stands where the box
//! already admitted its operand. The doors that ask are those whose
//! result need not lie within that hull:
//! - a copy's placement and a profile's sketch frame, which stand a body
//!   by a map;
//! - a sweep lifted along its profile's `z`, which stands it by the lift;
//! - the mirror, whose reflection can carry the image off the operand's
//!   hull and past the box while its source stands inside it.
//!
//! Every other one-input operation's result lies within its operand's
//! hull. A blend or chamfer removes material at a convex edge and adds it
//! at a concave one, and what it adds lies between the two faces it joins,
//! inside their hull. A split keeps pieces of the operand. A shell is the
//! operand less its inward offset. The twist moves its body whole and
//! asks as a move does, and a tube is built in the world and finished
//! there. A gather builds one body out of its operands, each grafted
//! into its computing frame ([`Framed::graft_into`]), which asks the box
//! of each image there, and its finish asks of the whole.
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

    /// **What it holds under an isometry**, of the same kind
    /// ([`topo::FinishedBody::moved_by`], [`topo::LeafBody::moved_by`]):
    /// the verdict carried and the session box asked again.
    ///
    /// # Errors
    ///
    /// [`topo::MoveError::Map`] where the data map refuses (under a
    /// reflection, a re-minted row); [`topo::MoveError::NotFinished`]
    /// where the image reaches past the session box, or a leaf's image
    /// does not finish.
    pub fn moved_by(&self, iso: &topo::Isometry<T>, tol: Tol) -> Result<Self, topo::MoveError> {
        use topo::{FinishedBody, LeafBody};
        Ok(match self {
            Self::Finished(body) => {
                Self::Finished(Arc::new(FinishedBody::moved_by(&**body, iso, tol)?))
            }
            Self::Leaf(body) => Self::Leaf(Arc::new(LeafBody::moved_by(&**body, iso, tol)?)),
        })
    }

    /// **What it holds under a rigid motion** ([`Self::moved_by`]).
    ///
    /// # Errors
    ///
    /// The move's [`topo::NotFinished`] ([`topo::MoveError::under_a_motion`]):
    /// the image reaches past the session box, or, where it holds a
    /// leaf, the leaf's image does not finish (at `f64` and `Probe`, where
    /// a leaf is finished on its image while the box-scalar switch lives).
    pub fn moved(&self, motion: &RigidMotion<T>, tol: Tol) -> Result<Self, topo::NotFinished> {
        self.moved_by(&topo::Isometry::Proper(*motion), tol)
            .map_err(topo::MoveError::under_a_motion)
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
    /// a reader measuring the body along a direction of the space (the
    /// step that clears it) measures it along, computing in the body's
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

    /// **This value, its placement asked the session box** (D4 ¶4,
    /// `[placement.frame]`): where it stands in its space under a map,
    /// the box asked of it there with no body moved
    /// ([`topo::session_box_at`], the question a move asks of its image,
    /// read off the same data). A copy's placement is asked by the door
    /// that places it, not by the readers that read it: a placement
    /// ([`super::wire`]'s `place`), a profile's sketch frame
    /// ([`super::ProfileValue::framed`]) and a mirror of a placed copy
    /// (`wire_mirror`: its image stands, under the copy's frame, where
    /// the reflection carried it) each call this; a value in its space's own
    /// coordinates was asked by its own finish. Which one-input
    /// operations need it, and why the rest stand inside their operand,
    /// is the module's account.
    ///
    /// # Errors
    ///
    /// [`super::NodeErrorKind::NotFinished`] holding the session box's
    /// finding, or a carrier the map does not map
    /// ([`topo::MoveError::under_a_motion`]).
    pub(crate) fn asked_where_it_stands(self, tol: Tol) -> Result<Self, super::NodeErrorKind>
    where
        T: geom_core::Decide,
    {
        if let Some(map) = self.frame.map() {
            topo::session_box_at(self.body(), map, tol).map_err(|e| {
                super::NodeErrorKind::NotFinished {
                    findings: e.under_a_motion(),
                }
            })?;
        }
        Ok(self)
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

    /// **`other` as a reader in this one's frame reads it** ([`topo::Seat`]):
    /// its body as it stands, and the relative motion
    /// ([`Self::relative`]) each datum the reader takes off it is mapped
    /// through.
    ///
    /// # Errors
    ///
    /// The relative map's overflow.
    pub(crate) fn seat<'b>(&self, other: &'b Self) -> Result<topo::Seat<'b, T>, RigidMotionError> {
        Ok(topo::Seat::from_motion(other.body(), self.relative(other)?))
    }
}

/// **Why a copy did not graft into an aggregate** ([`Framed::graft_into`]):
/// it did not read into the aggregate's frame, or the graft refused.
#[derive(Debug)]
pub(crate) enum GraftIn {
    /// The relative motion overflowed.
    Motion(RigidMotionError),
    /// The copy's image refused its move: it reaches past the session
    /// box.
    Moved(topo::NotFinished),
    /// The graft's own refusal.
    Graft(Box<topo::BooleanError>),
}

impl From<GraftIn> for super::NodeErrorKind {
    /// The refusal as an operation that grafts the copy raises it.
    fn from(refusal: GraftIn) -> Self {
        match refusal {
            GraftIn::Motion(e) => Self::Motion(e),
            GraftIn::Moved(findings) => Self::NotFinished { findings },
            GraftIn::Graft(e) => Self::Boolean(*e),
        }
    }
}

impl<T: topo::AtRestPolicy> Held<T> {
    /// Which kind of value it is ([`topo::HeldKind`]).
    pub(crate) fn kind(&self) -> topo::HeldKind {
        match self {
            Self::Finished(_) => topo::HeldKind::Finished,
            Self::Leaf(_) => topo::HeldKind::Leaf,
        }
    }
}

impl<T: topo::AtRestPolicy> Framed<T> {
    /// **The motion that moves this body into `frame`'s coordinates**,
    /// `None` where the two are one frame, counted as the one rigid map
    /// it costs where it is a motion (`counters::rigid_map`): the one
    /// home of that count. It counts each placement the evaluator makes
    /// ([`Self::graft_into`], [`Self::placed_in`]), once per operand and
    /// operation, not each move a kernel door makes of it: the at-rest
    /// localization places a copy once and hands it to every pair it is
    /// in, whose door moves it per pair.
    ///
    /// # Errors
    ///
    /// The relative map's overflow.
    pub(crate) fn moving_into(
        &self,
        frame: &Frame<T>,
    ) -> Result<Option<RigidMotion<T>>, RigidMotionError> {
        let motion = frame.relative(&self.frame)?;
        if motion.is_some() {
            super::counters::rigid_map();
        }
        Ok(motion)
    }
}

impl<T: topo::AtRestPolicy> Framed<T> {
    /// **`held`, this value's body finished, placed in `frame`**
    /// ([`topo::Placed`]): under the motion into `frame` ([`Self::moving_into`],
    /// where that motion is counted), or alone where the two are one frame.
    /// The door it is handed to moves it.
    ///
    /// # Errors
    ///
    /// The relative map's overflow.
    pub(crate) fn placed_in<'a>(
        &self,
        frame: &Frame<T>,
        held: &'a T::Finished,
    ) -> Result<topo::Placed<'a, T::Finished>, RigidMotionError> {
        Ok(topo::Placed::new(held, self.moving_into(frame)?))
    }
}

/// **What a gather of copies made** ([`Framed::gathered`]): the aggregate
/// finished by carrying its members' verdicts, or, where some member is
/// not handed over finished, the aggregate as construction-state data,
/// which the door that gathered it ends as it ends a body it built.
#[derive(Debug)]
pub(crate) enum Gathered<T: topo::AtRestPolicy> {
    /// Every member was a finished body ([`topo::boolean::gather_finished`]).
    Finished(T::Finished),
    /// Some member was a leaf the switch keeps unfinished at the box
    /// scalars: grafted as data ([`Framed::graft_into`]).
    Grafted(Box<Body<T>>),
}

impl<T: topo::AtRestPolicy> core::ops::Deref for Gathered<T> {
    type Target = Body<T>;

    fn deref(&self) -> &Body<T> {
        match self {
            Self::Finished(body) => body,
            Self::Grafted(body) => body,
        }
    }
}

impl<T: topo::AtRestPolicy> Held<T> {
    /// **Whether a gather takes this as a finished body**, the same at
    /// every lane of one build: a finished body always; a leaf where the
    /// box scalars' leaf is finished too ([`topo::LEAVES_ARE_FINISHED`]),
    /// so no lane carries what another derives.
    fn gathers_finished(&self) -> bool {
        match self {
            Self::Finished(_) => true,
            Self::Leaf(_) => topo::LEAVES_ARE_FINISHED,
        }
    }
}

impl<T: topo::AtRestPolicy + geom_core::Decide> Framed<T> {
    /// **`members` gathered into one body computed in `frame`**, in order,
    /// with the keys each member's graft minted: by carrying
    /// ([`topo::boolean::gather_finished`]) where every member is handed over finished
    /// ([`Held::gathers_finished`], a leaf through the one taker door), as
    /// data through each member's placement ([`Self::graft_into`])
    /// otherwise, and where there is no member.
    ///
    /// # Errors
    ///
    /// The member at fault, by position, and its refusal as
    /// [`Self::graft_into`] states it; a leaf's take, as
    /// [`GraftIn::Moved`]; at a dual, the aggregate's tier 2 as
    /// [`GraftIn::Moved`] of its first member.
    pub(crate) fn gathered(
        frame: &Frame<T>,
        members: &[&Self],
        tol: Tol,
    ) -> Result<(Gathered<T>, Vec<topo::GraftKeys>), (usize, GraftIn)> {
        if members.is_empty() || !members.iter().all(|member| member.body.gathers_finished()) {
            let mut fused = Body::new();
            let keys = members
                .iter()
                .enumerate()
                .map(|(at, member)| {
                    Self::graft_into(frame, &mut fused, member, tol).map_err(|e| (at, e))
                })
                .collect::<Result<Vec<_>, _>>()?;
            return Ok((Gathered::Grafted(Box::new(fused)), keys));
        }
        let finished = members
            .iter()
            .enumerate()
            .map(|(at, member)| match &member.body {
                Held::Finished(body) => Ok(Arc::clone(body)),
                Held::Leaf(leaf) => topo::take::<T>(leaf, tol).map_err(|e| (at, GraftIn::Moved(e))),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let placed = members
            .iter()
            .zip(&finished)
            .enumerate()
            .map(|(at, (member, held))| {
                member
                    .placed_in(frame, held)
                    .map_err(|e| (at, GraftIn::Motion(e)))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let (body, keys) = topo::boolean::gather_finished(&placed, tol).map_err(|e| match e {
            topo::boolean::GatherError::Moved { member, findings } => {
                (member, GraftIn::Moved(findings))
            }
            topo::boolean::GatherError::Graft { member, error } => (member, GraftIn::Graft(error)),
            topo::boolean::GatherError::Aggregate(findings) => (0, GraftIn::Moved(findings)),
        })?;
        Ok((Gathered::Finished(body), keys))
    }
}

impl<T: topo::AtRestPolicy + geom_core::Decide> Framed<T> {
    /// **`other` grafted into `dst`, an aggregate computed in `frame`**
    /// ([`Self::computing_among`], [`topo::boolean::graft_placed`]): its
    /// body as it stands where `other` is in that frame, else its data
    /// moved through its placement into it and the image asked the
    /// session box, as a move of what `other` holds asks and logs it. The
    /// image is the aggregate's data, so this door, which places it,
    /// asks. The aggregate's own finish decides the rest.
    ///
    /// # Errors
    ///
    /// [`GraftIn::Motion`]: the motion's overflow;
    /// [`GraftIn::Moved`]: the image is past the session box;
    /// [`GraftIn::Graft`]: the graft's.
    pub(crate) fn graft_into(
        frame: &Frame<T>,
        dst: &mut Body<T>,
        other: &Self,
        tol: Tol,
    ) -> Result<topo::GraftKeys, GraftIn> {
        let seat = topo::Seat::from_motion(
            other.body(),
            other.moving_into(frame).map_err(GraftIn::Motion)?,
        );
        topo::boolean::graft_placed(dst, seat, other.body.kind(), tol).map_err(|e| match e {
            topo::boolean::PlacedGraftError::Move(e) => {
                GraftIn::Moved(topo::MoveError::under_a_motion(e))
            }
            topo::boolean::PlacedGraftError::Graft(e) => GraftIn::Graft(Box::new(e)),
        })
    }
}
